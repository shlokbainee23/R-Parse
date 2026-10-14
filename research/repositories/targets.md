# targets — Repository Analysis

## 1. Project Identity

- **Name:** targets
- **Repository:** https://github.com/ropensci/targets
- **Organization:** rOpenSci (William Michael Landau)
- **Language:** R (100%)
- **License:** MIT
- **Status:** Active, industry-standard pipeline toolkit for reproducible data science in R
- **Size:** 284 R source files, ~1.4 MB codebase
- **Dependencies:** `codetools`, `callr`, `cli`, `digest`, `igraph`, `rlang`, `tidyselect`, `vctrs`

---

## 2. Purpose & Problem Solved

`targets` is a Make-like pipeline toolkit for statistics and data science in R. It orchestrates complex multi-step computation DAGs (data downloading, wrangling, modeling, reporting). It statically detects which target steps depend on which variables, functions, and files, hashes inputs and code, and only executes outdated targets when upstream dependencies change.

---

## 3. Architecture & Execution Flow

```text
_targets.R (Pipeline Script)
      │
      ▼
tar_manifest() / tar_make() [R/tar_make.R]
      │
      ├─► Static Dependency Detection [R/tar_deps.R line 29]
      │     ├─► embody_expr(expr): wraps expression in dummy function() [R/utils_language.R line 71]
      │     ├─► Replaces `~` with `identity` in AST [R/utils_language.R line 38]
      │     └─► codetools::findGlobals(fun) -> global variable names [R/utils_language.R line 40]
      │
      ├─► DAG Construction [R/class_pipeline.R, R/utils_igraph.R]
      │     └─► Builds igraph directed acyclic graph linking target names
      │
      ├─► Cryptographic Hashing [R/class_record.R, R/utils_hash.R]
      │     └─► Hashes function bodies (via deparsing) and data objects
      │
      └─► Execution Scheduler [R/class_scheduler.R, R/tar_crew.R]
            └─► Dispatches out-of-date targets to background workers
```

---

## 4. Parsing & Static Analysis Mechanism

Targets uses a clever, lightweight static analysis technique:
1. It does **not** maintain a custom parser or AST walker.
2. In `tar_deps_raw(expr)` (`R/tar_deps_raw.R`), if an expression is not already a function, it converts it into a function via:
   ```r
   embody_expr <- function(expr) {
     fun <- function() {}
     body(fun) <- expr
     fun
   }
   ```
3. In `deps_function(fun)` (`R/utils_language.R` lines 37-41):
   ```r
   deps_function <- function(fun) {
     env <- list(`~` = `identity`)
     body(fun) <- eval(call("substitute", body(fun), env), envir = baseenv())
     sort_chr(codetools::findGlobals(fun))
   }
   ```
4. It relies on base R's `codetools::findGlobals()`, which traverses compiled byte-code / S-expressions to find free variables.

---

## 5. Scope & Domain Understanding

- **Target-Level Coarseness:** It only analyzes dependencies between named targets defined in `tar_target(name, command)`. It does not analyze intra-command statement sequences or fine-grained dataflow.
- **Formulas:** Explicitly handles `~` by replacing it with `identity` in the AST so that variables on both LHS and RHS of formulas are recognized as dependencies.
- **Tidy Evaluation:** Uses `tar_tidy_eval()` (`R/utils_language.R` line 85) via `rlang::quo_squash()` to unquote expressions before analysis.

---

## 6. What targets Does NOT Do (Verified Gaps)

1. **No Intra-Function Dataflow:** Treats each target command as an opaque black box returning a single value.
2. **False Dependency Prone in NSE:** In `mutate(df, x = y + 1)`, `y` is flagged as a global dependency by `codetools::findGlobals` if `y` is not bound locally inside the expression, requiring the user to avoid variable/column name collisions.
3. **Deparse-Based Hashing Fragility:** Function changes are detected by hashing deparsed text (`tar_deparse_safe()`). Formatting changes or comment edits can falsely trigger pipeline invalidation unless carefully normalized.

---

## 7. Strengths

- **Simplicity & Speed:** Using `codetools::findGlobals()` on embodied functions provides near-instant dependency extraction for thousands of pipeline stages.
- **Robust DAG Management:** World-class execution caching, cloud integration (AWS S3, GCP GCS), and parallel workers (`crew`, `future`, `clustermq`).

---

## 8. Limitations & Failure Modes

- **Column vs Global Ambiguity:** In modern tidyverse code, data frame column names inside dplyr verbs are frequently mistaken for missing global dependencies.
- **Dynamic Branching Blindness:** Dynamic branches (`tar_target(..., pattern = map(x))`) cannot be fully determined until runtime evaluation of the parent target completes.

---

## 9. Relevance to R-PARSE

- **Workflow Extraction Paradigm:** Demonstrates the value of extracting DAG dependencies from code.
- **Opportunity for Precision:** R-PARSE's fine-grained dataflow and NSE-aware scope resolution could eliminate the false positives that plague `codetools::findGlobals()` in `targets`.
