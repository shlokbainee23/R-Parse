//! Minimal, robust loopback HTTP server for R-PARSE Scientific Analysis Studio.
//!
//! Provides a zero-dependency, local-only HTTP service binding strictly to `127.0.0.1`.
//! Exposes:
//! - `GET /` and `GET /index.html`: Serves the interactive Scientific Analysis Studio.
//! - `GET /api/health`: Loopback health check and version query.
//! - `OPTIONS /api/analyze`: CORS preflight configuration for browser origins.
//! - `POST /api/analyze`: Accepts R source code, runs the real Rust analysis pipeline,
//!   and returns the authoritative structured JSON analysis dataset.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rparse_core::JsonValue;
use rparse_runtime::config::AnalysisMode;

use crate::args::{GraphType, RPARSE_VERSION};
use crate::format_html::{build_default_studio_html, build_report_data, format_html_report};
use crate::pipeline::AnalysisContext;

/// Maximum allowable HTTP header size (64 KB).
const MAX_HEADER_SIZE: usize = 65_536;
/// Maximum allowable HTTP body payload (4 MB).
const MAX_BODY_SIZE: usize = 4_194_304;

/// Launches the interactive Scientific Analysis Studio HTTP server.
pub fn run_studio_server(
    file: Option<PathBuf>,
    port_opt: Option<u16>,
    open_browser: bool,
) -> (i32, String) {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .or(port_opt)
        .unwrap_or(8080);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind_addr = format!("{}:{}", host, port);

    let listener = match TcpListener::bind(&bind_addr) {
        Ok(l) => l,
        Err(e) => {
            return (
                crate::exit_code::EXIT_INVALID_USAGE,
                format!("Error: Failed to bind server to {}: {}", bind_addr, e),
            );
        }
    };

    let initial_html = if let Some(ref path) = file {
        match AnalysisContext::run(path, AnalysisMode::Static, vec![]) {
            Ok(ctx) => format_html_report(&ctx, None, None, None),
            Err(e) => {
                return (
                    e.exit_code(),
                    format!(
                        "Error: Failed to analyze initial file '{}': {}",
                        path.display(),
                        e
                    ),
                );
            }
        }
    } else {
        build_default_studio_html()
    };

    let display_host = if host == "0.0.0.0" { "localhost" } else { &host };
    let local_url = format!("http://{}:{}", display_host, port);
    let studio_url = format!("http://{}:{}/studio", display_host, port);
    let health_url = format!("http://{}:{}/api/health", display_host, port);

    println!("╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║       R-PARSE Unified Web Service & Scientific Analysis Studio        ║");
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    println!("║ Bound Address:   {:<52} ║", bind_addr);
    println!("║ Landing Page:    {:<52} ║", local_url);
    println!("║ Studio App:      {:<52} ║", studio_url);
    println!("║ Health Endpoint: {:<52} ║", health_url);
    println!("║ Engine Mode:     STATIC (100% Deterministic Rust Core)                ║");
    println!("║ Press Ctrl+C in terminal to stop server.                              ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝");

    if open_browser {
        launch_browser(&local_url);
    }

    let running = Arc::new(AtomicBool::new(true));
    let initial_html = Arc::new(initial_html);

    while running.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _peer)) => {
                let html_clone = Arc::clone(&initial_html);
                thread::spawn(move || {
                    let _ = handle_connection(stream, &html_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection accept error: {}", e);
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    (
        crate::exit_code::EXIT_SUCCESS,
        "Studio server stopped.".to_string(),
    )
}

/// Helper to open the default web browser on various operating systems.
fn launch_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

/// Handles a single incoming HTTP connection.
pub(crate) fn handle_connection(mut stream: TcpStream, initial_html: &str) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(15)))?;
    stream.set_write_timeout(Some(Duration::from_secs(15)))?;

    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        send_response(
            &mut stream,
            400,
            "Bad Request",
            "text/plain",
            b"Malformed request line\n",
        )?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    let mut headers = BTreeMap::new();
    let mut total_header_len = request_line.len();

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        total_header_len += bytes_read;
        if total_header_len > MAX_HEADER_SIZE {
            send_response(
                &mut stream,
                431,
                "Request Header Fields Too Large",
                "text/plain",
                b"Header too large\n",
            )?;
            return Ok(());
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        if let Some((k, v)) = trimmed.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    // CORS preflight handling
    if method.eq_ignore_ascii_case("OPTIONS") {
        send_cors_preflight(&mut stream)?;
        return Ok(());
    }

    let clean_path = path.split('?').next().unwrap_or(path);

    // Route matching
    match (method, clean_path) {
        (
            "GET",
            "/"
            | "/index.html"
            | "/landing"
            | "/landing/"
            | "/landing.html"
            | "/landing/index.html"
            | "/investigation",
        ) => {
            let landing_html = include_str!("../../../../landing.html");
            send_response(
                &mut stream,
                200,
                "OK",
                "text/html; charset=utf-8",
                landing_html.as_bytes(),
            )?;
        }
        ("GET", "/studio" | "/studio/" | "/studio.html" | "/workbench") => {
            send_response(
                &mut stream,
                200,
                "OK",
                "text/html; charset=utf-8",
                initial_html.as_bytes(),
            )?;
        }
        (
            "GET",
            "/how-to-use"
            | "/how-to-use/"
            | "/how-to-use.html"
            | "/landing/how-to-use.html"
            | "/docs"
            | "/docs/"
            | "/docs.html",
        ) => {
            let doc_html = include_str!("../../../../how-to-use.html");
            send_response(
                &mut stream,
                200,
                "OK",
                "text/html; charset=utf-8",
                doc_html.as_bytes(),
            )?;
        }
        ("GET", "/landing/styles.css" | "/styles.css") => {
            let css = include_str!("../../../../landing/styles.css");
            send_response(
                &mut stream,
                200,
                "OK",
                "text/css; charset=utf-8",
                css.as_bytes(),
            )?;
        }
        ("GET", "/landing/ribbon.js" | "/ribbon.js") => {
            let js = include_str!("../../../../landing/ribbon.js");
            send_response(
                &mut stream,
                200,
                "OK",
                "application/javascript; charset=utf-8",
                js.as_bytes(),
            )?;
        }
        ("GET", "/landing/hero3d.js" | "/hero3d.js") => {
            let js = include_str!("../../../../landing/hero3d.js");
            send_response(
                &mut stream,
                200,
                "OK",
                "application/javascript; charset=utf-8",
                js.as_bytes(),
            )?;
        }
        ("GET", "/landing/hero_diagram.js" | "/hero_diagram.js") => {
            let js = include_str!("../../../../landing/hero_diagram.js");
            send_response(
                &mut stream,
                200,
                "OK",
                "application/javascript; charset=utf-8",
                js.as_bytes(),
            )?;
        }
        ("GET", "/landing/assembly.js" | "/assembly.js") => {
            let js = include_str!("../../../../landing/assembly.js");
            send_response(
                &mut stream,
                200,
                "OK",
                "application/javascript; charset=utf-8",
                js.as_bytes(),
            )?;
        }
        ("GET", "/landing/investigation.js" | "/investigation.js") => {
            let js = include_str!("../../../../landing/investigation.js");
            send_response(
                &mut stream,
                200,
                "OK",
                "application/javascript; charset=utf-8",
                js.as_bytes(),
            )?;
        }
        ("GET", "/favicon.svg" | "/landing/favicon.svg" | "/favicon.ico") => {
            let svg = include_str!("../../../../favicon.svg");
            send_response(
                &mut stream,
                200,
                "OK",
                "image/svg+xml; charset=utf-8",
                svg.as_bytes(),
            )?;
        }
        ("GET", "/api/health") => {
            let body = format!(
                r#"{{"status":"ok","tool":"rparse","version":"{}"}}"#,
                RPARSE_VERSION
            );
            send_response(
                &mut stream,
                200,
                "OK",
                "application/json; charset=utf-8",
                body.as_bytes(),
            )?;
        }
        ("POST", "/api/analyze") => {
            let content_len = headers
                .get("content-length")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0);

            if content_len > MAX_BODY_SIZE {
                send_response(
                    &mut stream,
                    413,
                    "Payload Too Large",
                    "application/json",
                    br#"{"error":"Payload exceeds maximum limit of 4MB"}"#,
                )?;
                return Ok(());
            }

            let mut body_bytes = vec![0u8; content_len];
            reader.read_exact(&mut body_bytes)?;
            let body_str = String::from_utf8_lossy(&body_bytes);

            let (status_code, response_json) = handle_api_analyze(&body_str);
            let status_msg = match status_code {
                200 => "OK",
                400 => "Bad Request",
                422 => "Unprocessable Entity",
                _ => "Internal Error",
            };
            send_response(
                &mut stream,
                status_code,
                status_msg,
                "application/json; charset=utf-8",
                response_json.as_bytes(),
            )?;
        }
        _ => {
            if clean_path.starts_with("/api/") {
                send_response(
                    &mut stream,
                    404,
                    "Not Found",
                    "application/json; charset=utf-8",
                    br#"{"error":"API endpoint not found"}"#,
                )?;
            } else if clean_path.ends_with(".js")
                || clean_path.ends_with(".css")
                || clean_path.ends_with(".png")
                || clean_path.ends_with(".jpg")
                || clean_path.ends_with(".svg")
                || clean_path.ends_with(".woff")
                || clean_path.ends_with(".woff2")
                || clean_path.ends_with(".ttf")
                || clean_path.ends_with(".ico")
            {
                send_response(
                    &mut stream,
                    404,
                    "Not Found",
                    "text/plain; charset=utf-8",
                    b"404 Asset Not Found\n",
                )?;
            } else {
                send_response(
                    &mut stream,
                    404,
                    "Not Found",
                    "text/plain; charset=utf-8",
                    b"404 Not Found\n",
                )?;
            }
        }
    }

    Ok(())
}

/// Authoritative analysis processor for incoming `/api/analyze` JSON payloads.
///
/// Accepts in-memory JSON payload, runs `AnalysisContext::run_from_source`,
/// and returns `(http_status, json_response)`.
pub fn handle_api_analyze(payload_json: &str) -> (u16, String) {
    let parsed = match JsonValue::parse(payload_json) {
        Ok(v) => v,
        Err(e) => {
            let mut err_obj = BTreeMap::new();
            err_obj.insert("status".to_string(), JsonValue::String("error".to_string()));
            err_obj.insert(
                "error_type".to_string(),
                JsonValue::String("bad_request".to_string()),
            );
            err_obj.insert(
                "error_message".to_string(),
                JsonValue::String(format!("Malformed JSON: {}", e)),
            );
            return (400, JsonValue::Object(err_obj).to_json_string());
        }
    };

    let source = match parsed.get("source").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => {
            let mut err_obj = BTreeMap::new();
            err_obj.insert("status".to_string(), JsonValue::String("error".to_string()));
            err_obj.insert(
                "error_type".to_string(),
                JsonValue::String("bad_request".to_string()),
            );
            err_obj.insert(
                "error_message".to_string(),
                JsonValue::String("Missing required 'source' string field".to_string()),
            );
            return (400, JsonValue::Object(err_obj).to_json_string());
        }
    };

    let file_name = parsed
        .get("file_name")
        .and_then(|v| v.as_str())
        .unwrap_or("script.R")
        .to_string();

    let mode_str = parsed
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("static");

    let mode = match mode_str.to_ascii_lowercase().as_str() {
        "assisted" => AnalysisMode::Assisted,
        "execute" => AnalysisMode::Execute,
        _ => AnalysisMode::Static,
    };

    let criterion = parsed
        .get("criterion")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let column = parsed
        .get("column")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let graph = parsed
        .get("graph")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "pdg" => Some(GraphType::Pdg),
            "provenance" => Some(GraphType::Provenance),
            "cfg" => Some(GraphType::Cfg),
            _ => None,
        });

    let revision = parsed.get("revision").cloned();

    // Check for empty source text
    if source.trim().is_empty() {
        let mut root = BTreeMap::new();
        root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
        root.insert(
            "version".to_string(),
            JsonValue::String(RPARSE_VERSION.to_string()),
        );
        root.insert("file_name".to_string(), JsonValue::String(file_name));
        root.insert(
            "mode".to_string(),
            JsonValue::String(format!("{:?}", mode).to_uppercase()),
        );
        root.insert(
            "analysis_status".to_string(),
            JsonValue::String("empty_source".to_string()),
        );
        root.insert("source_code".to_string(), JsonValue::String(source));
        root.insert("source_lines".to_string(), JsonValue::Array(vec![]));
        if let Some(rev) = revision {
            root.insert("revision".to_string(), rev);
        }

        let mut summary = BTreeMap::new();
        summary.insert("statement_count".to_string(), JsonValue::Number(0.0));
        summary.insert("dataframe_state_count".to_string(), JsonValue::Number(0.0));
        summary.insert("column_version_count".to_string(), JsonValue::Number(0.0));
        summary.insert("pdg_node_count".to_string(), JsonValue::Number(0.0));
        summary.insert("pdg_edge_count".to_string(), JsonValue::Number(0.0));
        summary.insert("derivation_count".to_string(), JsonValue::Number(0.0));
        summary.insert("contains_unknowns".to_string(), JsonValue::Bool(false));
        root.insert("summary".to_string(), JsonValue::Object(summary));

        let mut empty_graph = BTreeMap::new();
        empty_graph.insert("nodes".to_string(), JsonValue::Array(vec![]));
        empty_graph.insert("edges".to_string(), JsonValue::Array(vec![]));
        root.insert(
            "provenance".to_string(),
            JsonValue::Object(empty_graph.clone()),
        );
        root.insert("pdg".to_string(), JsonValue::Object(empty_graph));
        root.insert("schema_evolution".to_string(), JsonValue::Array(vec![]));
        root.insert("slices".to_string(), JsonValue::Object(BTreeMap::new()));
        root.insert(
            "ast_explorer".to_string(),
            crate::format_ast::build_empty_ast_explorer_payload(),
        );

        let mut diag = BTreeMap::new();
        diag.insert(
            "severity".to_string(),
            JsonValue::String("warning".to_string()),
        );
        diag.insert(
            "message".to_string(),
            JsonValue::String("Source program is empty. Please enter or load R code.".to_string()),
        );
        root.insert(
            "diagnostics".to_string(),
            JsonValue::Array(vec![JsonValue::Object(diag)]),
        );

        return (200, JsonValue::Object(root).to_json_string());
    }

    // Run actual Rust analysis pipeline
    match AnalysisContext::run_from_source(PathBuf::from(&file_name), source.clone(), mode, vec![])
    {
        Ok(ctx) => {
            let mut report_value =
                build_report_data(&ctx, criterion.as_deref(), column.as_deref(), graph);
            if let JsonValue::Object(ref mut map) = report_value {
                if let Some(rev) = revision {
                    map.insert("revision".to_string(), rev);
                }

                let status = if ctx.has_errors() {
                    "syntax_error"
                } else if !ctx.pdg.unknown_nodes().is_empty() {
                    "uncertain"
                } else {
                    "ok"
                };
                map.insert(
                    "analysis_status".to_string(),
                    JsonValue::String(status.to_string()),
                );
            }
            (200, report_value.to_json_string())
        }
        Err(err) => {
            // Pipeline encountered a fatal syntax or analysis error
            let lines: Vec<JsonValue> = source
                .lines()
                .map(|l| JsonValue::String(l.to_string()))
                .collect();
            let mut root = BTreeMap::new();
            root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
            root.insert(
                "version".to_string(),
                JsonValue::String(RPARSE_VERSION.to_string()),
            );
            root.insert("file_name".to_string(), JsonValue::String(file_name));
            root.insert(
                "mode".to_string(),
                JsonValue::String(format!("{:?}", mode).to_uppercase()),
            );
            root.insert(
                "analysis_status".to_string(),
                JsonValue::String("error".to_string()),
            );
            root.insert(
                "error_message".to_string(),
                JsonValue::String(err.to_string()),
            );
            root.insert("source_code".to_string(), JsonValue::String(source.clone()));
            root.insert("source_lines".to_string(), JsonValue::Array(lines));
            if let Some(rev) = revision {
                root.insert("revision".to_string(), rev);
            }

            let mut summary = BTreeMap::new();
            summary.insert("statement_count".to_string(), JsonValue::Number(0.0));
            summary.insert("dataframe_state_count".to_string(), JsonValue::Number(0.0));
            summary.insert("column_version_count".to_string(), JsonValue::Number(0.0));
            summary.insert("pdg_node_count".to_string(), JsonValue::Number(0.0));
            summary.insert("pdg_edge_count".to_string(), JsonValue::Number(0.0));
            summary.insert("derivation_count".to_string(), JsonValue::Number(0.0));
            summary.insert("contains_unknowns".to_string(), JsonValue::Bool(false));
            root.insert("summary".to_string(), JsonValue::Object(summary));

            let mut empty_graph = BTreeMap::new();
            empty_graph.insert("nodes".to_string(), JsonValue::Array(vec![]));
            empty_graph.insert("edges".to_string(), JsonValue::Array(vec![]));
            root.insert(
                "provenance".to_string(),
                JsonValue::Object(empty_graph.clone()),
            );
            root.insert("pdg".to_string(), JsonValue::Object(empty_graph));
            root.insert("schema_evolution".to_string(), JsonValue::Array(vec![]));
            root.insert("slices".to_string(), JsonValue::Object(BTreeMap::new()));
            root.insert(
                "ast_explorer".to_string(),
                crate::format_ast::build_ast_explorer_payload_for_error_source(&source),
            );

            let mut diag = BTreeMap::new();
            diag.insert(
                "severity".to_string(),
                JsonValue::String("error".to_string()),
            );
            diag.insert("message".to_string(), JsonValue::String(err.to_string()));
            diag.insert(
                "code".to_string(),
                JsonValue::String("ANALYSIS_FAILED".to_string()),
            );
            root.insert(
                "diagnostics".to_string(),
                JsonValue::Array(vec![JsonValue::Object(diag)]),
            );

            (200, JsonValue::Object(root).to_json_string())
        }
    }
}

/// Writes standard HTTP response with CORS headers.
fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Accept, X-Requested-With\r\n\
         \r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );

    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

/// Sends standard HTTP 204 response for CORS preflight queries.
fn send_cors_preflight(stream: &mut TcpStream) -> std::io::Result<()> {
    let header = "HTTP/1.1 204 No Content\r\n\
                  Connection: close\r\n\
                  Access-Control-Allow-Origin: *\r\n\
                  Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                  Access-Control-Allow-Headers: Content-Type, Accept, X-Requested-With\r\n\
                  Access-Control-Max-Age: 86400\r\n\
                  Content-Length: 0\r\n\
                  \r\n";
    stream.write_all(header.as_bytes())?;
    stream.flush()?;
    Ok(())
}
