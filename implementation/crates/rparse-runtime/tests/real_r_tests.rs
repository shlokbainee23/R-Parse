use rparse_runtime::bridge::RuntimeBridge;
use rparse_runtime::config::{AnalysisMode, RuntimeConfig};
use rparse_runtime::discovery::discover_r_executable;
use rparse_runtime::error::RuntimeError;

fn get_r_bridge(timeout_ms: u64) -> Option<RuntimeBridge> {
    if let Ok(r_path) = discover_r_executable(None) {
        let config = RuntimeConfig::new()
            .with_mode(AnalysisMode::Assisted)
            .with_r_executable(r_path)
            .with_timeout_ms(timeout_ms);
        Some(RuntimeBridge::new(config))
    } else {
        println!("Skipping GNU R test: GNU R executable not discovered on host");
        None
    }
}

#[test]
fn test_real_r_evaluation_and_startup() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    let eval_res = bridge
        .evaluate("result <- 10 * 20; result")
        .expect("R evaluation failed");
    assert_eq!(eval_res.status, "success");
    assert!(eval_res.output.as_deref().unwrap_or("").contains("200"));
}

#[test]
fn test_real_r_object_inspection() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    let obj = bridge
        .inspect_object("vec", Some("vec <- c(10, 20, 30)"))
        .expect("Object inspection failed");

    assert!(obj.exists);
    assert_eq!(obj.symbol, "vec");
    assert_eq!(obj.length, 3);
    assert_eq!(obj.type_name, "double");
    assert!(!obj.is_dataframe);
}

#[test]
fn test_real_r_dataframe_schema_discovery() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    let schema = bridge
        .inspect_dataframe(
            "sales",
            Some("sales <- data.frame(price = c(10.5, 20.0), count = c(1L, 2L))"),
        )
        .expect("DataFrame inspection failed");

    assert!(schema.exists);
    assert_eq!(schema.status, "success");
    assert_eq!(schema.target_symbol, "sales");
    assert_eq!(schema.observed_columns.len(), 2);
    assert_eq!(schema.row_count_estimate, Some(2));

    let price_col = schema
        .observed_columns
        .iter()
        .find(|c| c.name == "price")
        .unwrap();
    assert_eq!(price_col.r_type, "double");

    let count_col = schema
        .observed_columns
        .iter()
        .find(|c| c.name == "count")
        .unwrap();
    assert_eq!(count_col.r_type, "integer");
}

#[test]
fn test_real_r_multiple_columns_and_empty_dataframe() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    // Multiple column types
    let schema_multi = bridge
        .inspect_dataframe(
            "full_df",
            Some("full_df <- data.frame(id = 1L, name = 'test', cost = 9.99, flag = TRUE)"),
        )
        .expect("Multiple column inspection failed");
    assert_eq!(schema_multi.observed_columns.len(), 4);

    // Empty dataframe
    let schema_empty = bridge
        .inspect_dataframe(
            "empty_df",
            Some("empty_df <- data.frame(x = numeric(0), y = character(0))"),
        )
        .expect("Empty dataframe inspection failed");
    assert_eq!(schema_empty.observed_columns.len(), 2);
    assert_eq!(schema_empty.row_count_estimate, Some(0));
}

#[test]
fn test_real_r_object_not_found() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    let obj = bridge
        .inspect_object("non_existent_symbol_12345", None)
        .expect("Inspection call should succeed with exists=false");
    assert!(!obj.exists);

    let df_res = bridge
        .inspect_dataframe("non_existent_df_12345", None)
        .expect("Dataframe inspection call should succeed with exists=false");
    assert!(!df_res.exists);
    assert_eq!(df_res.status, "error");
}

#[test]
fn test_real_r_invalid_code_and_runtime_exception() {
    let mut bridge = match get_r_bridge(5000) {
        Some(b) => b,
        None => return,
    };

    // Syntax error
    let res_syntax = bridge.evaluate("1 + ");
    assert!(res_syntax.is_ok());
    let err_val = res_syntax.unwrap();
    assert_eq!(err_val.status, "error");

    // Runtime exception
    let res_exception = bridge.evaluate("stop('custom runtime exception')");
    assert!(res_exception.is_ok());
    let err_val2 = res_exception.unwrap();
    assert_eq!(err_val2.status, "error");
    assert!(err_val2
        .output
        .as_deref()
        .unwrap_or("")
        .contains("custom runtime exception"));
}

#[test]
fn test_real_r_timeout_enforcement() {
    // 100 ms timeout against a 5-second sleep
    let mut bridge = match get_r_bridge(100) {
        Some(b) => b,
        None => return,
    };

    let res = bridge.evaluate("Sys.sleep(5)");
    match res {
        Err(RuntimeError::Timeout {
            timeout_ms,
            operation,
        }) => {
            assert_eq!(timeout_ms, 100);
            assert_eq!(operation, "evaluate");
        }
        other => panic!("Expected timeout error, got {:?}", other),
    }

    // After timeout, the backend must be shut down and child terminated
    assert!(!bridge.is_connected());
}
