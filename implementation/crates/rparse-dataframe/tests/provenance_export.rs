//! Targeted verification tests for Column Provenance DOT and JSON topology export.

use rparse_core::{JsonValue, NodeId, Span};
use rparse_dataframe::{
    analyze_dataframes, ColumnVersionId, DataFrameSeed, ProvenanceEdge, ProvenanceEdgeInit,
    ProvenanceGraph, StaticKnowledge, TransformationType,
};
use rparse_ir::parse_source;
use rparse_scope::resolve_scopes;
use std::collections::BTreeSet;

#[test]
fn test_empty_provenance_graph_export() {
    let graph = ProvenanceGraph::new();

    let dot = graph.to_dot();
    assert!(dot.starts_with("digraph ColumnProvenance {"));
    assert!(dot.ends_with("}\n"));

    let json_str = graph.to_json();
    let parsed = JsonValue::parse(&json_str).expect("parse empty provenance json");
    assert_eq!(
        parsed.get("graph_type").unwrap().as_str(),
        Some("provenance")
    );
    assert_eq!(parsed.get("columns").unwrap().as_array().unwrap().len(), 0);
    assert_eq!(
        parsed.get("derivations").unwrap().as_array().unwrap().len(),
        0
    );
    let summary = parsed.get("summary").unwrap();
    assert_eq!(summary.get("column_count").unwrap().as_u64(), Some(0));
    assert_eq!(summary.get("derivation_count").unwrap().as_u64(), Some(0));
}

#[test]
fn test_multi_input_column_provenance_export_and_grouping() {
    let mut graph = ProvenanceGraph::new();

    let mut in_cols = BTreeSet::new();
    in_cols.insert(ColumnVersionId(0)); // price@v0
    in_cols.insert(ColumnVersionId(1)); // quantity@v0

    let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
        id: rparse_dataframe::ProvenanceEdgeId(0),
        input_columns: in_cols,
        input_variables: BTreeSet::new(),
        output_column: ColumnVersionId(2), // total@v1
        expression_node: Some(NodeId::new(42)),
        span: Span::new(10, 30, 2, 1, 2, 21),
        transform_type: TransformationType::Mutate,
        knowledge: StaticKnowledge::KnownStatic,
    });
    graph.add_edge(edge);

    // Test DOT
    let dot1 = graph.to_dot();
    let dot2 = graph.to_dot();
    assert_eq!(dot1, dot2, "DOT export must be deterministic");

    // Must declare columns
    assert!(dot1.contains("col_0"));
    assert!(dot1.contains("col_1"));
    assert!(dot1.contains("col_2"));

    // Must declare derivation activity node
    assert!(dot1.contains("pedge_0"));
    assert!(dot1.contains("Mutate"));

    // Grouping integrity: both input columns point into pedge_0
    assert!(dot1.contains("col_0 -> pedge_0"));
    assert!(dot1.contains("col_1 -> pedge_0"));
    // Derivation node points to output column
    assert!(dot1.contains("pedge_0 -> col_2"));

    // Test JSON
    let json_str = graph.to_json();
    let parsed = JsonValue::parse(&json_str).expect("parse provenance json");
    assert_eq!(
        parsed.get("graph_type").unwrap().as_str(),
        Some("provenance")
    );

    let cols = parsed.get("columns").unwrap().as_array().unwrap();
    assert_eq!(cols.len(), 3);

    let derivs = parsed.get("derivations").unwrap().as_array().unwrap();
    assert_eq!(derivs.len(), 1);
    let deriv = &derivs[0];
    assert_eq!(deriv.get("id").unwrap().as_u64(), Some(0));
    assert_eq!(
        deriv.get("transform_type").unwrap().as_str(),
        Some("Mutate")
    );
    assert_eq!(deriv.get("output_column").unwrap().as_u64(), Some(2));
    assert_eq!(deriv.get("is_unknown").unwrap().as_bool(), Some(false));

    let in_arr = deriv.get("input_columns").unwrap().as_array().unwrap();
    assert_eq!(in_arr.len(), 2);
    assert_eq!(in_arr[0].as_u64(), Some(0));
    assert_eq!(in_arr[1].as_u64(), Some(1));
}

#[test]
fn test_unknown_provenance_derivation_distinct_export() {
    let mut graph = ProvenanceGraph::new();

    let mut in_cols = BTreeSet::new();
    in_cols.insert(ColumnVersionId(0));

    let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
        id: rparse_dataframe::ProvenanceEdgeId(0),
        input_columns: in_cols,
        input_variables: BTreeSet::new(),
        output_column: ColumnVersionId(1),
        expression_node: None,
        span: Span::new(0, 10, 1, 1, 1, 11),
        transform_type: TransformationType::Mutate,
        knowledge: StaticKnowledge::UnknownStatic,
    });
    graph.add_edge(edge);

    let dot = graph.to_dot();
    // Must contain distinct styling for unknown derivations
    assert!(dot.contains("UNKNOWN_STATIC"));
    assert!(dot.contains("fillcolor=\"#ffebee\""));
    assert!(dot.contains("color=\"#d32f2f\""));
    assert!(dot.contains("dashed,bold"));

    let json_str = graph.to_json();
    let parsed = JsonValue::parse(&json_str).expect("parse json");
    let summary = parsed.get("summary").unwrap();
    assert_eq!(
        summary.get("uncertain_derivation_count").unwrap().as_u64(),
        Some(1)
    );

    let derivs = parsed.get("derivations").unwrap().as_array().unwrap();
    assert_eq!(derivs[0].get("is_unknown").unwrap().as_bool(), Some(true));
    assert_eq!(
        derivs[0].get("knowledge").unwrap().as_str(),
        Some("UNKNOWN_STATIC")
    );
}

#[test]
fn test_dataframe_analysis_provenance_labels() {
    let code = r#"
        df <- data.frame(price = 10, quantity = 5)
        df$total <- df$price * df$quantity
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price".to_string(), "quantity".to_string()],
    )];
    let program = parse_source(code).expect("parse source");
    let scope_model = resolve_scopes(&program);
    let df_analysis = analyze_dataframes(&program, &scope_model, &seeds);

    let dot = df_analysis.provenance_to_dot();
    // Readable labels should include symbol and column names
    assert!(dot.contains("df$price") || dot.contains("price"));
    assert!(dot.contains("df$total") || dot.contains("total"));

    let json = df_analysis.provenance_to_json();
    let parsed = JsonValue::parse(&json).expect("parse json");
    let cols = parsed.get("columns").unwrap().as_array().unwrap();
    assert!(cols
        .iter()
        .any(|c| c.get("label").unwrap().as_str().unwrap().contains("price")));
    assert!(cols
        .iter()
        .any(|c| c.get("label").unwrap().as_str().unwrap().contains("total")));
}
