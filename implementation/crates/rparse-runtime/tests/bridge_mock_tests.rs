use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use rparse_dataframe::{DataFrameId, DataFrameState, StaticKnowledge};
use rparse_runtime::bridge::RuntimeBridge;
use rparse_runtime::config::{AnalysisMode, RuntimeConfig};
use rparse_runtime::discovery::discover_r_executable;
use rparse_runtime::error::RuntimeError;
use rparse_runtime::evidence::{GroundedEvidence, ObservedColumn, RuntimeEvidence};
use rparse_runtime::integration::{AnnotatedDataFrameState, CorrelationStatus};
use rparse_runtime::json::JsonValue;
use rparse_runtime::mock::MockBackend;

#[test]
fn test_requirement_o_static_mode_never_executes_r() {
    let config = RuntimeConfig::new().with_mode(AnalysisMode::Static);
    let mut bridge = RuntimeBridge::new(config);

    // evaluate must fail immediately without launching R
    let res = bridge.evaluate("1 + 1");
    assert_eq!(res, Err(RuntimeError::ExecutionForbiddenInStaticMode));

    // inspect_object must fail
    let res2 = bridge.inspect_object("x", None);
    assert_eq!(res2, Err(RuntimeError::ExecutionForbiddenInStaticMode));

    // inspect_dataframe must fail
    let res3 = bridge.inspect_dataframe("df", None);
    assert_eq!(res3, Err(RuntimeError::ExecutionForbiddenInStaticMode));

    // observe_dataframe must safely return RuntimeEvidence::None
    let evidence = bridge.observe_dataframe("df", None);
    assert_eq!(evidence, RuntimeEvidence::None);
    assert!(!bridge.is_connected());
}

#[test]
fn test_requirement_p_assisted_mode_records_runtime_evidence() {
    let config = RuntimeConfig::new().with_mode(AnalysisMode::Assisted);
    let mut mock = MockBackend::new();

    // Setup mock response for inspect_dataframe
    let schema_json = r#"
    {
        "status": "success",
        "exists": true,
        "target_symbol": "clean",
        "observed_columns": [
            { "name": "price", "r_type": "numeric", "class_name": ["numeric"] },
            { "name": "count", "r_type": "integer", "class_name": ["integer"] }
        ],
        "row_count_estimate": 100,
        "execution_time_ms": 12
    }
    "#;
    mock.queue_result(JsonValue::parse(schema_json).unwrap());

    let mut bridge = RuntimeBridge::with_backend(config, Box::new(mock));
    let evidence = bridge.observe_dataframe("clean", Some("clean <- read.csv('data.csv')"));

    assert!(evidence.is_observed());
    let record = evidence.as_observed().unwrap();
    assert_eq!(record.target_symbol, "clean");
    assert!(record.status.is_success());
    assert_eq!(record.observed_columns.len(), 2);
    assert_eq!(record.observed_columns[0].name, "price");
    assert_eq!(record.observed_columns[1].name, "count");
}

#[test]
fn test_requirement_q_execute_mode_records_runtime_evidence() {
    let config = RuntimeConfig::new().with_mode(AnalysisMode::Execute);
    let mut mock = MockBackend::new();

    let eval_json = r#"
    {
        "status": "success",
        "output": "[1] 42",
        "value_type": "integer",
        "execution_time_ms": 5
    }
    "#;
    mock.queue_result(JsonValue::parse(eval_json).unwrap());

    let mut bridge = RuntimeBridge::with_backend(config, Box::new(mock));
    let eval_res = bridge.evaluate("answer <- 42").unwrap();

    assert_eq!(eval_res.status, "success");
    assert_eq!(eval_res.output.as_deref(), Some("[1] 42"));
    assert_eq!(eval_res.value_type.as_deref(), Some("integer"));
}

#[test]
fn test_requirement_r_and_s_and_t_static_and_runtime_separation() {
    // Statically unknown schema (e.g. dynamic .data[[get_col()]])
    let static_knowledge = StaticKnowledge::UnknownStatic;

    // Runtime observed evidence
    let observed_col = ObservedColumn {
        name: "dynamic_target".to_string(),
        r_type: "numeric".to_string(),
        class_name: vec!["numeric".to_string()],
    };
    let record = rparse_runtime::evidence::ObservationRecord::success(
        "df",
        Some("df$dynamic_target <- 1".to_string()),
        vec![observed_col],
        Some(10),
        8,
    );
    let runtime_evidence = RuntimeEvidence::Observed(record);

    let grounded = GroundedEvidence::new(static_knowledge, runtime_evidence.clone());

    // Invariant: static knowledge is still UnknownStatic! It has NOT been overwritten!
    assert_eq!(grounded.static_knowledge, StaticKnowledge::UnknownStatic);
    assert!(grounded.runtime_evidence.is_observed());
    assert!(grounded.is_empirically_grounded());
    assert!(!grounded.is_confirmed()); // Not confirmed because statically it was unknown
}

#[test]
fn test_requirement_m_timeout_handling() {
    let config = RuntimeConfig::new()
        .with_mode(AnalysisMode::Assisted)
        .with_timeout_ms(50);
    let mut mock = MockBackend::new();
    mock.set_delay(Duration::from_millis(150));

    let mut bridge = RuntimeBridge::with_backend(config, Box::new(mock));
    let res = bridge.evaluate("Sys.sleep(10)");

    match res {
        Err(RuntimeError::Timeout {
            timeout_ms,
            operation,
        }) => {
            assert_eq!(timeout_ms, 50);
            assert_eq!(operation, "evaluate");
        }
        other => panic!("Expected Timeout error, got {:?}", other),
    }
}

#[test]
fn test_requirement_l_process_termination() {
    let config = RuntimeConfig::new().with_mode(AnalysisMode::Assisted);
    let mut mock = MockBackend::new();
    mock.simulate_termination();

    let mut bridge = RuntimeBridge::with_backend(config, Box::new(mock));
    let res = bridge.evaluate("x <- 1");

    match res {
        Err(RuntimeError::ProcessTerminated { .. }) => {}
        other => panic!("Expected ProcessTerminated error, got {:?}", other),
    }
}

#[test]
fn test_requirement_j_missing_gnu_r_executable() {
    let invalid_path = Path::new("C:/non_existent_dir_12345/non_existent_Rscript.exe");
    let res = discover_r_executable(Some(invalid_path));

    match res {
        Err(RuntimeError::ExecutableNotFound { searched }) => {
            assert!(!searched.is_empty());
        }
        other => panic!("Expected ExecutableNotFound error, got {:?}", other),
    }
}

#[test]
fn test_correlation_model() {
    let mut schema = BTreeMap::new();
    schema.insert("price".to_string(), rparse_dataframe::ColumnVersionId(0));
    schema.insert(
        "unknown_ext".to_string(),
        rparse_dataframe::ColumnVersionId(1),
    );

    let static_state = DataFrameState {
        id: DataFrameId(0),
        version: 0,
        symbol_id: None,
        symbol_name: "df".to_string(),
        schema,
        creation_node: rparse_core::NodeId::new(1),
        span: rparse_core::Span::default(),
        parent_state: None,
        row_selections: Vec::new(),
        knowledge: StaticKnowledge::KnownStatic,
    };

    let observed_col = ObservedColumn {
        name: "price".to_string(),
        r_type: "numeric".to_string(),
        class_name: vec!["numeric".to_string()],
    };
    let new_col = ObservedColumn {
        name: "runtime_only".to_string(),
        r_type: "character".to_string(),
        class_name: vec!["character".to_string()],
    };

    let record = rparse_runtime::evidence::ObservationRecord::success(
        "df",
        None,
        vec![observed_col, new_col],
        Some(50),
        5,
    );

    let annotated = AnnotatedDataFrameState::new(static_state, RuntimeEvidence::Observed(record));
    let correlations = annotated.correlate_columns();

    let price_corr = correlations.iter().find(|c| c.name == "price").unwrap();
    assert_eq!(price_corr.status, CorrelationStatus::Confirmed);
    assert_eq!(price_corr.observed_type.as_deref(), Some("numeric"));

    let ext_corr = correlations
        .iter()
        .find(|c| c.name == "unknown_ext")
        .unwrap();
    assert_eq!(ext_corr.status, CorrelationStatus::StaticOnly);

    let runtime_only_corr = correlations
        .iter()
        .find(|c| c.name == "runtime_only")
        .unwrap();
    assert_eq!(runtime_only_corr.status, CorrelationStatus::EmpiricalOnly);
}
