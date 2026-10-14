//! Targeted verification tests for PDG DOT and JSON topology export.

use rparse_cfg::build_cfg;
use rparse_core::JsonValue;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg, PdgGraphInit, ProgramDependenceGraph};
use rparse_scope::resolve_scopes;
use std::collections::BTreeMap;

fn construct_pdg(source: &str, seeds: &[DataFrameSeed]) -> ProgramDependenceGraph {
    let program = parse_source(source).expect("parse source");
    let scope_model = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("cfg build");
    let def_use = collect_def_use(&program, &scope_model, &cfg);
    let reaching_defs = compute_reaching_definitions(&cfg, &def_use);
    let post_dominators = compute_post_dominators(&cfg);
    let control_deps = compute_control_dependencies(&cfg, &post_dominators);
    let df_analysis = analyze_dataframes(&program, &scope_model, seeds);

    let pdg = build_pdg(
        &program,
        &scope_model,
        &cfg,
        &def_use,
        &reaching_defs,
        &control_deps,
        &df_analysis,
    );
    assert!(validate_pdg(&pdg).is_ok(), "PDG invariants must hold");
    pdg
}

#[test]
fn test_empty_pdg_export() {
    let empty_init = PdgGraphInit {
        nodes: Vec::new(),
        edges: Vec::new(),
        incoming: BTreeMap::new(),
        outgoing: BTreeMap::new(),
        column_ports: BTreeMap::new(),
        stmt_nodes: BTreeMap::new(),
        expr_nodes: BTreeMap::new(),
        var_def_nodes: BTreeMap::new(),
        var_use_nodes: BTreeMap::new(),
        row_selection_nodes: BTreeMap::new(),
        unknown_nodes: Vec::new(),
    };
    let pdg = ProgramDependenceGraph::new(empty_init);

    let dot = pdg.to_dot();
    assert!(dot.starts_with("digraph PDG {"));
    assert!(dot.ends_with("}\n"));

    let json_str = pdg.to_json();
    let parsed = JsonValue::parse(&json_str).expect("parse empty pdg json");
    assert_eq!(parsed.get("graph_type").unwrap().as_str(), Some("pdg"));
    assert_eq!(parsed.get("nodes").unwrap().as_array().unwrap().len(), 0);
    assert_eq!(parsed.get("edges").unwrap().as_array().unwrap().len(), 0);
    let summary = parsed.get("summary").unwrap();
    assert_eq!(summary.get("node_count").unwrap().as_u64(), Some(0));
    assert_eq!(summary.get("edge_count").unwrap().as_u64(), Some(0));
}

#[test]
fn test_populated_pdg_dot_export_determinism_and_content() {
    let code = r#"
        df <- data.frame(price = 10, quantity = 5)
        df$total <- df$price * df$quantity
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price".to_string(), "quantity".to_string()],
    )];
    let pdg = construct_pdg(code, &seeds);

    let dot1 = pdg.to_dot();
    let dot2 = pdg.to_dot();
    // Deterministic export invariant
    assert_eq!(dot1, dot2, "DOT export must be 100% deterministic");

    assert!(dot1.contains("digraph PDG {"));
    // Must contain column port nodes
    assert!(dot1.contains("ColumnPort"));
    assert!(dot1.contains("shape=ellipse"));
    // Must contain statement nodes
    assert!(dot1.contains("Statement"));
    // Must contain directional edges
    assert!(dot1.contains(" -> "));
    // Must contain dependency category labels
    assert!(
        dot1.contains("col_deriv")
            || dot1.contains("stmt_col_binding")
            || dot1.contains("var_data")
    );
}

#[test]
fn test_populated_pdg_json_topology_export() {
    let code = r#"
        df <- data.frame(price = 10, quantity = 5)
        df$total <- df$price * df$quantity
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price".to_string(), "quantity".to_string()],
    )];
    let pdg = construct_pdg(code, &seeds);

    let json1 = pdg.to_json();
    let json2 = pdg.to_json();
    assert_eq!(json1, json2, "JSON export must be 100% deterministic");

    let parsed = JsonValue::parse(&json1).expect("parse json");
    assert_eq!(parsed.get("graph_type").unwrap().as_str(), Some("pdg"));

    let nodes = parsed.get("nodes").unwrap().as_array().unwrap();
    assert!(!nodes.is_empty());
    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.get("id").unwrap().as_u64(), Some(i as u64));
        assert!(node.get("label").is_some());
        assert!(node.get("kind").is_some());
        assert!(node.get("span").is_some());
    }

    let edges = parsed.get("edges").unwrap().as_array().unwrap();
    assert!(!edges.is_empty());
    for (i, edge) in edges.iter().enumerate() {
        assert_eq!(edge.get("id").unwrap().as_u64(), Some(i as u64));
        assert!(edge.get("from").is_some());
        assert!(edge.get("to").is_some());
        assert_eq!(
            edge.get("direction").unwrap().as_str(),
            Some("source_to_dependent")
        );
        assert!(edge.get("kind").is_some());
    }

    let summary = parsed.get("summary").unwrap();
    assert!(summary.get("node_count").unwrap().as_u64().unwrap() > 0);
    assert!(summary.get("edge_count").unwrap().as_u64().unwrap() > 0);
    assert!(summary.get("column_port_count").unwrap().as_u64().unwrap() >= 2);
}

#[test]
fn test_unknown_pdg_nodes_rendered_distinctly() {
    let code = r#"
        df <- data.frame(a = 1)
        res <- eval(parse(text = "df$b <- 2"))
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["a".to_string()])];
    let pdg = construct_pdg(code, &seeds);

    let dot = pdg.to_dot();
    // Unknown nodes must have distinct visual styling
    if !pdg.unknown_nodes().is_empty() {
        assert!(
            dot.contains("shape=octagon"),
            "Unknown nodes must use distinct octagon shape in DOT"
        );
        assert!(
            dot.contains("#d32f2f"),
            "Unknown nodes must use distinct red styling"
        );
    }

    let json = pdg.to_json();
    let parsed = JsonValue::parse(&json).expect("parse json");
    let summary = parsed.get("summary").unwrap();
    let unk_count = summary.get("unknown_node_count").unwrap().as_u64().unwrap();
    assert_eq!(unk_count, pdg.unknown_nodes().len() as u64);
}
