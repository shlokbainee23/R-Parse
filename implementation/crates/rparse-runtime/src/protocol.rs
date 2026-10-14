//! Length-prefixed JSON-RPC 2.0 wire protocol for Rust <-> GNU R communication.

use std::collections::BTreeMap;
use std::io::{BufRead, Write};

use crate::error::RuntimeError;
use crate::evidence::{ObservationRecord, ObservedColumn};
use crate::json::JsonValue;

/// Standard JSON-RPC 2.0 request envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcRequest {
    /// JSON-RPC version ("2.0").
    pub jsonrpc: String,
    /// Unique sequential request ID.
    pub id: u64,
    /// Operation name.
    pub method: String,
    /// Structured operation parameters.
    pub params: JsonValue,
}

impl RpcRequest {
    /// Creates a new JSON-RPC 2.0 request.
    pub fn new(id: u64, method: impl Into<String>, params: JsonValue) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params,
        }
    }

    /// Converts to a JSON value.
    pub fn to_json(&self) -> JsonValue {
        let mut map = BTreeMap::new();
        map.insert(
            "jsonrpc".to_string(),
            JsonValue::String(self.jsonrpc.clone()),
        );
        map.insert("id".to_string(), JsonValue::Number(self.id as f64));
        map.insert("method".to_string(), JsonValue::String(self.method.clone()));
        map.insert("params".to_string(), self.params.clone());
        JsonValue::Object(map)
    }

    /// Serializes this request to a JSON string.
    pub fn to_json_string(&self) -> String {
        self.to_json().to_json_string()
    }
}

/// Standard JSON-RPC 2.0 response envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcResponse {
    /// JSON-RPC version ("2.0").
    pub jsonrpc: String,
    /// Matching request ID.
    pub id: u64,
    /// Successful result payload.
    pub result: Option<JsonValue>,
    /// Error payload if operation failed.
    pub error: Option<RpcError>,
}

impl RpcResponse {
    /// Parses an `RpcResponse` from a JSON string.
    pub fn parse(s: &str) -> Result<Self, RuntimeError> {
        let val = JsonValue::parse(s).map_err(RuntimeError::SerializationError)?;
        let jsonrpc = val
            .get("jsonrpc")
            .and_then(|v| v.as_str())
            .unwrap_or("2.0")
            .to_string();
        let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
        let result = val.get("result").cloned();
        let error = val.get("error").map(|v| {
            let code = v.get("code").and_then(|c| c.as_i64()).unwrap_or(-1) as i32;
            let message = v
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            let data = v.get("data").cloned();
            RpcError {
                code,
                message,
                data,
            }
        });

        Ok(Self {
            jsonrpc,
            id,
            result,
            error,
        })
    }

    /// Serializes to a JSON string.
    pub fn to_json_string(&self) -> String {
        let mut map = BTreeMap::new();
        map.insert(
            "jsonrpc".to_string(),
            JsonValue::String(self.jsonrpc.clone()),
        );
        map.insert("id".to_string(), JsonValue::Number(self.id as f64));
        if let Some(res) = &self.result {
            map.insert("result".to_string(), res.clone());
        }
        if let Some(err) = &self.error {
            let mut err_map = BTreeMap::new();
            err_map.insert("code".to_string(), JsonValue::Number(err.code as f64));
            err_map.insert(
                "message".to_string(),
                JsonValue::String(err.message.clone()),
            );
            if let Some(d) = &err.data {
                err_map.insert("data".to_string(), d.clone());
            }
            map.insert("error".to_string(), JsonValue::Object(err_map));
        }
        JsonValue::Object(map).to_json_string()
    }
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcError {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable error description.
    pub message: String,
    /// Optional structured diagnostic data.
    pub data: Option<JsonValue>,
}

/// Parameters for evaluating arbitrary R code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalParams {
    /// The R expression or code block to evaluate.
    pub expression: String,
    /// Execution timeout in milliseconds.
    pub timeout_ms: Option<u64>,
}

impl EvalParams {
    /// Converts parameters to JSON value.
    pub fn to_json(&self) -> JsonValue {
        let mut map = BTreeMap::new();
        map.insert(
            "expression".to_string(),
            JsonValue::String(self.expression.clone()),
        );
        if let Some(t) = self.timeout_ms {
            map.insert("timeout_ms".to_string(), JsonValue::Number(t as f64));
        }
        JsonValue::Object(map)
    }
}

/// Result of evaluating R code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalResult {
    /// Execution status ("success" or "error").
    pub status: String,
    /// Captured stdout/printed text.
    pub output: Option<String>,
    /// R type of the final expression value (`typeof`).
    pub value_type: Option<String>,
    /// Execution duration in milliseconds.
    pub execution_time_ms: u64,
}

impl EvalResult {
    /// Parses an `EvalResult` from a `JsonValue`.
    pub fn from_json(val: &JsonValue) -> Result<Self, RuntimeError> {
        let status = val
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("error")
            .to_string();
        let output = val
            .get("output")
            .and_then(|o| o.as_str())
            .map(|s| s.to_string());
        let value_type = val
            .get("value_type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let execution_time_ms = val
            .get("execution_time_ms")
            .and_then(|t| t.as_u64())
            .unwrap_or(0);

        Ok(Self {
            status,
            output,
            value_type,
            execution_time_ms,
        })
    }
}

/// Parameters for inspecting an R object in the evaluation environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectObjectParams {
    /// Target variable or symbol name.
    pub target_symbol: String,
    /// Optional expression to evaluate before inspection.
    pub expression: Option<String>,
    /// Execution timeout in milliseconds.
    pub timeout_ms: Option<u64>,
}

impl InspectObjectParams {
    /// Converts parameters to JSON value.
    pub fn to_json(&self) -> JsonValue {
        let mut map = BTreeMap::new();
        map.insert(
            "target_symbol".to_string(),
            JsonValue::String(self.target_symbol.clone()),
        );
        if let Some(e) = &self.expression {
            map.insert("expression".to_string(), JsonValue::String(e.clone()));
        }
        if let Some(t) = self.timeout_ms {
            map.insert("timeout_ms".to_string(), JsonValue::Number(t as f64));
        }
        JsonValue::Object(map)
    }
}

/// Structural metadata for an inspected R object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMetadata {
    /// True if the symbol was found.
    pub exists: bool,
    /// Symbol name.
    pub symbol: String,
    /// Class hierarchy (`class(x)`).
    pub class_name: Vec<String>,
    /// Base R storage type (`typeof(x)`).
    pub type_name: String,
    /// Vector length or dimension count.
    pub length: usize,
    /// True if the object inherits from `data.frame`.
    pub is_dataframe: bool,
}

impl ObjectMetadata {
    /// Parses an `ObjectMetadata` from a `JsonValue`.
    pub fn from_json(val: &JsonValue) -> Result<Self, RuntimeError> {
        let exists = val.get("exists").and_then(|e| e.as_bool()).unwrap_or(false);
        let symbol = val
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        let class_name = val
            .get("class_name")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let type_name = val
            .get("type_name")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        let length = val.get("length").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
        let is_dataframe = val
            .get("is_dataframe")
            .and_then(|d| d.as_bool())
            .unwrap_or(false);

        Ok(Self {
            exists,
            symbol,
            class_name,
            type_name,
            length,
            is_dataframe,
        })
    }
}

/// Parameters for discovering dataframe schema information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectDataFrameParams {
    /// Target variable or symbol name (e.g. "df").
    pub target_symbol: String,
    /// Optional expression to evaluate before inspection (e.g. `df <- read.csv("data.csv")`).
    pub expression: Option<String>,
    /// Execution timeout in milliseconds.
    pub timeout_ms: Option<u64>,
}

impl InspectDataFrameParams {
    /// Converts parameters to JSON value.
    pub fn to_json(&self) -> JsonValue {
        let mut map = BTreeMap::new();
        map.insert(
            "target_symbol".to_string(),
            JsonValue::String(self.target_symbol.clone()),
        );
        if let Some(e) = &self.expression {
            map.insert("expression".to_string(), JsonValue::String(e.clone()));
        }
        if let Some(t) = self.timeout_ms {
            map.insert("timeout_ms".to_string(), JsonValue::Number(t as f64));
        }
        JsonValue::Object(map)
    }
}

/// Observed dataframe schema response payload from GNU R.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFrameSchemaObservation {
    /// Observation status ("success" or "error").
    pub status: String,
    /// True if target object exists and is a dataframe.
    pub exists: bool,
    /// Symbol name inspected.
    pub target_symbol: String,
    /// List of observed columns with their types.
    pub observed_columns: Vec<ObservedColumn>,
    /// Number of rows if safely countable.
    pub row_count_estimate: Option<usize>,
    /// Execution time in milliseconds.
    pub execution_time_ms: u64,
    /// Error message if inspection failed.
    pub error_message: Option<String>,
}

impl DataFrameSchemaObservation {
    /// Parses a `DataFrameSchemaObservation` from a `JsonValue`.
    pub fn from_json(val: &JsonValue) -> Result<Self, RuntimeError> {
        let status = val
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("error")
            .to_string();
        let exists = val.get("exists").and_then(|e| e.as_bool()).unwrap_or(false);
        let target_symbol = val
            .get("target_symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        let mut observed_columns = Vec::new();
        if let Some(cols_arr) = val.get("observed_columns").and_then(|c| c.as_array()) {
            for col_val in cols_arr {
                let name = col_val
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();
                let r_type = col_val
                    .get("r_type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let class_name = col_val
                    .get("class_name")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|item| item.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                observed_columns.push(ObservedColumn {
                    name,
                    r_type,
                    class_name,
                });
            }
        }

        let row_count_estimate = val
            .get("row_count_estimate")
            .and_then(|r| r.as_u64())
            .map(|u| u as usize);
        let execution_time_ms = val
            .get("execution_time_ms")
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        let error_message = val
            .get("error_message")
            .and_then(|e| e.as_str())
            .map(|s| s.to_string());

        Ok(Self {
            status,
            exists,
            target_symbol,
            observed_columns,
            row_count_estimate,
            execution_time_ms,
            error_message,
        })
    }

    /// Converts this schema observation into an empirical [`ObservationRecord`].
    pub fn into_record(self, expr: Option<String>) -> ObservationRecord {
        if self.status == "success" && self.exists {
            ObservationRecord::success(
                self.target_symbol,
                expr,
                self.observed_columns,
                self.row_count_estimate,
                self.execution_time_ms,
            )
        } else {
            let msg = self.error_message.unwrap_or_else(|| {
                "Target object does not exist or is not a dataframe".to_string()
            });
            ObservationRecord::error(self.target_symbol, expr, msg, self.execution_time_ms)
        }
    }
}

/// Encodes and writes a JSON-RPC request using `Content-Length` framing.
pub fn write_message<W: Write>(writer: &mut W, request: &RpcRequest) -> Result<(), RuntimeError> {
    let payload = request.to_json_string();
    let header = format!("Content-Length: {}\r\n\r\n", payload.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(payload.as_bytes())?;
    writer.flush()?;
    Ok(())
}

/// Reads a JSON-RPC response from a buffered reader supporting `Content-Length` framing
/// and fallback newline-delimited JSON.
pub fn read_message<R: BufRead>(reader: &mut R) -> Result<RpcResponse, RuntimeError> {
    let mut header_line = String::new();

    // Read initial line (skipping any leading blank lines)
    loop {
        header_line.clear();
        let bytes_read = reader.read_line(&mut header_line)?;
        if bytes_read == 0 {
            return Err(RuntimeError::UnexpectedEof);
        }
        let trimmed = header_line.trim();
        if !trimmed.is_empty() {
            break;
        }
    }

    let trimmed = header_line.trim();

    // Check if line is directly a JSON object (line-delimited fallback)
    if trimmed.starts_with('{') {
        let resp = RpcResponse::parse(trimmed)?;
        return Ok(resp);
    }

    // Otherwise, parse "Content-Length: <n>"
    if !trimmed.to_ascii_lowercase().starts_with("content-length:") {
        return Err(RuntimeError::ProtocolError(format!(
            "Expected 'Content-Length:' header, got: '{}'",
            trimmed
        )));
    }

    let len_str = trimmed["content-length:".len()..].trim();
    let content_len: usize = len_str.parse().map_err(|_| {
        RuntimeError::ProtocolError(format!("Invalid Content-Length value: '{}'", len_str))
    })?;

    // Read headers until blank line delimiter (\r\n\r\n)
    loop {
        header_line.clear();
        let bytes_read = reader.read_line(&mut header_line)?;
        if bytes_read == 0 {
            return Err(RuntimeError::UnexpectedEof);
        }
        if header_line.trim().is_empty() {
            break;
        }
    }

    // Read exactly content_len bytes
    let mut buffer = vec![0u8; content_len];
    reader.read_exact(&mut buffer)?;

    let payload_str = std::str::from_utf8(&buffer)
        .map_err(|e| RuntimeError::ProtocolError(format!("Payload is not valid UTF-8: {}", e)))?;

    let resp = RpcResponse::parse(payload_str)?;
    Ok(resp)
}
