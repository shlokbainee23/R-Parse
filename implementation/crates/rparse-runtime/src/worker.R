# R-PARSE GNU R Runtime Bridge Worker
# Runs inside the GNU R child process, listening on stdin for JSON-RPC 2.0 requests.
# Communicates using Content-Length framed JSON or newline-delimited JSON.

options(warn = 1, stringsAsFactors = FALSE)

# Controlled evaluation environment (isolated from .GlobalEnv directly)
eval_env <- new.env(parent = globalenv())

has_jsonlite <- requireNamespace("jsonlite", quietly = TRUE)

# Minimal base R JSON serializer fallback
escape_str <- function(s) {
  s <- gsub("\\\\", "\\\\\\\\", s)
  s <- gsub('"', '\\\\"', s)
  s <- gsub("\r", "\\\\r", s)
  s <- gsub("\n", "\\\\n", s)
  s <- gsub("\t", "\\\\t", s)
  paste0('"', s, '"')
}

to_json_val <- function(x) {
  if (is.null(x) || length(x) == 0) {
    return("null")
  }
  if (is.logical(x)) {
    return(if (x[1]) "true" else "false")
  }
  if (is.numeric(x)) {
    return(as.character(x[1]))
  }
  if (is.character(x)) {
    if (length(x) == 1 && is.null(names(x))) {
      return(escape_str(x))
    }
  }
  if (is.list(x)) {
    nms <- names(x)
    if (is.null(nms)) {
      # Array
      items <- vapply(x, to_json_val, character(1))
      return(paste0("[", paste(items, collapse = ","), "]"))
    } else {
      # Object
      items <- character(length(x))
      for (i in seq_along(x)) {
        key <- escape_str(nms[i])
        val <- to_json_val(x[[i]])
        items[i] <- paste0(key, ":", val)
      }
      return(paste0("{", paste(items, collapse = ","), "}"))
    }
  }
  if (is.vector(x) && length(x) > 1) {
    items <- vapply(x, to_json_val, character(1))
    return(paste0("[", paste(items, collapse = ","), "]"))
  }
  escape_str(as.character(x[1]))
}

encode_json <- function(obj) {
  if (has_jsonlite) {
    return(as.character(jsonlite::toJSON(obj, auto_unbox = TRUE, null = "null")))
  } else {
    return(to_json_val(obj))
  }
}

decode_json <- function(txt) {
  if (has_jsonlite) {
    return(jsonlite::fromJSON(txt, simplifyVector = FALSE))
  } else {
    # Simple base R parser for the fixed request schema
    req <- list()
    id_match <- regmatches(txt, regexec('"id"\\s*:\\s*([0-9]+)', txt))[[1]]
    if (length(id_match) >= 2) {
      req$id <- as.integer(id_match[2])
    } else {
      req$id <- 0
    }
    method_match <- regmatches(txt, regexec('"method"\\s*:\\s*"([^"]+)"', txt))[[1]]
    if (length(method_match) >= 2) {
      req$method <- method_match[2]
    } else {
      req$method <- "unknown"
    }
    expr_match <- regmatches(txt, regexec('"expression"\\s*:\\s*"((?:[^"\\\\]|\\\\.)*)"', txt))[[1]]
    params <- list()
    if (length(expr_match) >= 2) {
      expr_val <- gsub('\\\\"', '"', expr_match[2])
      expr_val <- gsub('\\\\n', '\n', expr_val)
      expr_val <- gsub('\\\\t', '\t', expr_val)
      expr_val <- gsub('\\\\\\\\', '\\\\', expr_val)
      params$expression <- expr_val
    }
    sym_match <- regmatches(txt, regexec('"target_symbol"\\s*:\\s*"([^"]+)"', txt))[[1]]
    if (length(sym_match) >= 2) {
      params$target_symbol <- sym_match[2]
    }
    req$params <- params
    return(req)
  }
}

send_response <- function(resp_obj) {
  json_txt <- encode_json(resp_obj)
  nbytes <- nchar(json_txt, type = "bytes")
  header <- sprintf("Content-Length: %d\r\n\r\n", nbytes)
  cat(header, file = stdout())
  cat(json_txt, file = stdout())
  flush(stdout())
}

send_error <- function(id, code, message) {
  err_resp <- list(
    jsonrpc = "2.0",
    id = id,
    error = list(
      code = code,
      message = message
    )
  )
  send_response(err_resp)
}

# Main event loop
stdin_con <- file("stdin", open = "r")

while (TRUE) {
  header_line <- readLines(stdin_con, n = 1, warn = FALSE)
  if (length(header_line) == 0) {
    break # EOF
  }
  trimmed <- trimws(header_line)
  if (nchar(trimmed) == 0) {
    next
  }

  payload <- ""
  if (startsWith(trimmed, "{")) {
    payload <- trimmed
  } else if (startsWith(tolower(trimmed), "content-length:")) {
    len_str <- trimws(substring(trimmed, 16))
    content_len <- as.integer(len_str)
    # Read until blank line
    while (TRUE) {
      hdr <- readLines(stdin_con, n = 1, warn = FALSE)
      if (length(hdr) == 0 || nchar(trimws(hdr)) == 0) {
        break
      }
    }
    payload <- readChar(stdin_con, nchars = content_len, useBytes = TRUE)
  } else {
    next
  }

  req <- tryCatch({
    decode_json(payload)
  }, error = function(e) {
    NULL
  })

  if (is.null(req)) {
    send_error(0, -32700, "Parse error")
    next
  }

  req_id <- if (!is.null(req$id)) req$id else 0
  method <- if (!is.null(req$method)) req$method else ""
  params <- if (!is.null(req$params)) req$params else list()

  t_start <- proc.time()[[3]]

  if (method == "ping") {
    send_response(list(
      jsonrpc = "2.0",
      id = req_id,
      result = list(status = "pong")
    ))
  } else if (method == "exit") {
    send_response(list(
      jsonrpc = "2.0",
      id = req_id,
      result = list(status = "exiting")
    ))
    break
  } else if (method == "evaluate") {
    expr_text <- params$expression
    if (is.null(expr_text) || nchar(expr_text) == 0) {
      send_error(req_id, -32602, "Missing 'expression' parameter")
      next
    }

    captured_out <- ""
    val_type <- "NULL"
    eval_err <- NULL

    tryCatch({
      parsed <- parse(text = expr_text)
      captured_out <- paste(capture.output({
        vis <- withVisible(eval(parsed, envir = eval_env))
        if (isTRUE(vis$visible) && !is.null(vis$value)) {
          print(vis$value)
        }
        val_type <- typeof(vis$value)
      }), collapse = "\n")
    }, error = function(e) {
      eval_err <<- conditionMessage(e)
    })

    t_end <- proc.time()[[3]]
    elapsed_ms <- as.integer(round((t_end - t_start) * 1000))

    if (!is.null(eval_err)) {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          status = "error",
          output = eval_err,
          value_type = NULL,
          execution_time_ms = elapsed_ms
        )
      ))
    } else {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          status = "success",
          output = captured_out,
          value_type = val_type,
          execution_time_ms = elapsed_ms
        )
      ))
    }
  } else if (method == "inspect_object") {
    sym_name <- params$target_symbol
    expr_text <- params$expression

    if (!is.null(expr_text) && nchar(expr_text) > 0) {
      tryCatch({
        eval(parse(text = expr_text), envir = eval_env)
      }, error = function(e) {})
    }

    obj_exists <- exists(sym_name, envir = eval_env, inherits = FALSE)
    t_end <- proc.time()[[3]]
    elapsed_ms <- as.integer(round((t_end - t_start) * 1000))

    if (!obj_exists) {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          exists = FALSE,
          symbol = sym_name,
          class_name = list(),
          type_name = "NULL",
          length = 0,
          is_dataframe = FALSE
        )
      ))
    } else {
      obj <- get(sym_name, envir = eval_env, inherits = FALSE)
      is_df <- is.data.frame(obj)
      classes <- as.list(class(obj))
      type_nm <- typeof(obj)
      len <- length(obj)
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          exists = TRUE,
          symbol = sym_name,
          class_name = classes,
          type_name = type_nm,
          length = len,
          is_dataframe = is_df
        )
      ))
    }
  } else if (method == "inspect_dataframe") {
    sym_name <- params$target_symbol
    expr_text <- params$expression

    eval_err <- NULL
    if (!is.null(expr_text) && nchar(expr_text) > 0) {
      tryCatch({
        eval(parse(text = expr_text), envir = eval_env)
      }, error = function(e) {
        eval_err <<- conditionMessage(e)
      })
    }

    t_end <- proc.time()[[3]]
    elapsed_ms <- as.integer(round((t_end - t_start) * 1000))

    if (!is.null(eval_err)) {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          status = "error",
          exists = FALSE,
          target_symbol = sym_name,
          observed_columns = list(),
          row_count_estimate = NULL,
          execution_time_ms = elapsed_ms,
          error_message = eval_err
        )
      ))
      next
    }

    obj_exists <- exists(sym_name, envir = eval_env, inherits = FALSE)
    if (!obj_exists) {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          status = "error",
          exists = FALSE,
          target_symbol = sym_name,
          observed_columns = list(),
          row_count_estimate = NULL,
          execution_time_ms = elapsed_ms,
          error_message = sprintf("Object '%s' not found", sym_name)
        )
      ))
      next
    }

    obj <- get(sym_name, envir = eval_env, inherits = FALSE)
    if (!is.data.frame(obj)) {
      send_response(list(
        jsonrpc = "2.0",
        id = req_id,
        result = list(
          status = "error",
          exists = TRUE,
          target_symbol = sym_name,
          observed_columns = list(),
          row_count_estimate = NULL,
          execution_time_ms = elapsed_ms,
          error_message = sprintf("Object '%s' is not a data.frame (class: %s)", sym_name, paste(class(obj), collapse = ", "))
        )
      ))
      next
    }

    col_names <- names(obj)
    cols <- vector("list", length(col_names))
    if (length(col_names) > 0) {
      for (i in seq_along(col_names)) {
        cn <- col_names[i]
        col_val <- obj[[cn]]
        cols[[i]] <- list(
          name = cn,
          r_type = typeof(col_val),
          class_name = as.list(class(col_val))
        )
      }
    }

    row_count <- nrow(obj)

    send_response(list(
      jsonrpc = "2.0",
      id = req_id,
      result = list(
        status = "success",
        exists = TRUE,
        target_symbol = sym_name,
        observed_columns = cols,
        row_count_estimate = row_count,
        execution_time_ms = elapsed_ms,
        error_message = NULL
      )
    ))
  } else {
    send_error(req_id, -32601, sprintf("Method not found: '%s'", method))
  }
}

close(stdin_con)
q(save = "no", status = 0)
