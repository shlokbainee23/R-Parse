use rparse_runtime::error::RuntimeError;
use rparse_runtime::json::JsonValue;
use rparse_runtime::protocol::{
    read_message, write_message, DataFrameSchemaObservation, RpcRequest, RpcResponse,
};
use std::io::Cursor;

#[test]
fn test_rpc_request_serialization() {
    let mut params = std::collections::BTreeMap::new();
    params.insert(
        "expression".to_string(),
        JsonValue::String("x <- 1".to_string()),
    );
    let req = RpcRequest::new(1, "evaluate", JsonValue::Object(params));

    let json_str = req.to_json_string();
    assert!(json_str.contains(r#""jsonrpc":"2.0""#));
    assert!(json_str.contains(r#""id":1"#));
    assert!(json_str.contains(r#""method":"evaluate""#));
}

#[test]
fn test_rpc_response_deserialization_success() {
    let json_resp =
        r#"{"jsonrpc":"2.0","id":42,"result":{"status":"success","execution_time_ms":10}}"#;
    let resp = RpcResponse::parse(json_resp).expect("Failed to parse response");
    assert_eq!(resp.id, 42);
    assert!(resp.error.is_none());
    let res = resp.result.expect("Missing result");
    assert_eq!(res.get("status").and_then(|v| v.as_str()), Some("success"));
}

#[test]
fn test_rpc_response_deserialization_error() {
    let json_resp =
        r#"{"jsonrpc":"2.0","id":7,"error":{"code":-32601,"message":"Method not found"}}"#;
    let resp = RpcResponse::parse(json_resp).expect("Failed to parse error response");
    assert_eq!(resp.id, 7);
    assert!(resp.result.is_none());
    let err = resp.error.expect("Missing error");
    assert_eq!(err.code, -32601);
    assert_eq!(err.message, "Method not found");
}

#[test]
fn test_framed_message_roundtrip() {
    let mut params = std::collections::BTreeMap::new();
    params.insert(
        "target_symbol".to_string(),
        JsonValue::String("df".to_string()),
    );
    let req = RpcRequest::new(99, "inspect_object", JsonValue::Object(params));

    let mut buffer = Vec::new();
    write_message(&mut buffer, &req).expect("Write failed");

    let header_str = String::from_utf8_lossy(&buffer);
    assert!(header_str.starts_with("Content-Length: "));
    assert!(header_str.contains("\r\n\r\n"));

    // Now test reading it back
    // Construct a valid response framed the same way
    let resp_payload =
        r#"{"jsonrpc":"2.0","id":99,"result":{"exists":true,"symbol":"df","length":5}}"#;
    let framed_resp = format!(
        "Content-Length: {}\r\n\r\n{}",
        resp_payload.len(),
        resp_payload
    );
    let mut cursor = Cursor::new(framed_resp.into_bytes());

    let parsed_resp = read_message(&mut cursor).expect("Read message failed");
    assert_eq!(parsed_resp.id, 99);
    let res = parsed_resp.result.unwrap();
    assert_eq!(res.get("exists").and_then(|v| v.as_bool()), Some(true));
}

#[test]
fn test_line_delimited_json_fallback() {
    let resp_payload = "{\"jsonrpc\":\"2.0\",\"id\":12,\"result\":{\"status\":\"pong\"}}\n";
    let mut cursor = Cursor::new(resp_payload.as_bytes());

    let parsed_resp = read_message(&mut cursor).expect("Line delimited read failed");
    assert_eq!(parsed_resp.id, 12);
}

#[test]
fn test_malformed_protocol_header() {
    let bad_data = "Invalid-Header: 123\r\n\r\n{}";
    let mut cursor = Cursor::new(bad_data.as_bytes());
    let res = read_message(&mut cursor);
    assert!(matches!(res, Err(RuntimeError::ProtocolError(_))));
}

#[test]
fn test_unexpected_eof() {
    let empty_data = "";
    let mut cursor = Cursor::new(empty_data.as_bytes());
    let res = read_message(&mut cursor);
    assert!(matches!(res, Err(RuntimeError::UnexpectedEof)));
}

#[test]
fn test_truncated_payload() {
    let truncated_data = "Content-Length: 100\r\n\r\n{short}";
    let mut cursor = Cursor::new(truncated_data.as_bytes());
    let res = read_message(&mut cursor);
    assert!(res.is_err());
}

#[test]
fn test_dataframe_schema_observation_from_json() {
    let json_text = r#"
    {
        "status": "success",
        "exists": true,
        "target_symbol": "sales",
        "observed_columns": [
            { "name": "price", "r_type": "numeric", "class_name": ["numeric"] },
            { "name": "item", "r_type": "character", "class_name": ["character"] }
        ],
        "row_count_estimate": 42,
        "execution_time_ms": 15
    }
    "#;
    let json_val = JsonValue::parse(json_text).unwrap();
    let schema = DataFrameSchemaObservation::from_json(&json_val).unwrap();

    assert_eq!(schema.status, "success");
    assert!(schema.exists);
    assert_eq!(schema.target_symbol, "sales");
    assert_eq!(schema.observed_columns.len(), 2);
    assert_eq!(schema.observed_columns[0].name, "price");
    assert_eq!(schema.observed_columns[0].r_type, "numeric");
    assert_eq!(schema.row_count_estimate, Some(42));
    assert_eq!(schema.execution_time_ms, 15);
}
