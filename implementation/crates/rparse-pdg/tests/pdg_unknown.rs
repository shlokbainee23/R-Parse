//! Tests for PDG unknown dependency integration.

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg, ProgramDependenceGraph};
use rparse_scope::resolve_scopes;

fn setup_pdg(source: &str, seeds: &[DataFrameSeed]) -> ProgramDependenceGraph {
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
    assert!(validate_pdg(&pdg).is_ok());
    pdg
}

#[test]
fn test_unknown_node_and_edge_created_for_dynamic_indexing() {
    let code = r#"
        df2 <- df %>%
            mutate(
                dynamic_col = .data[[get_col_name()]]
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["x", "y"])];
    let pdg = setup_pdg(code, &seeds);

    assert!(!pdg.unknown_nodes().is_empty());
    let unknown_edges: Vec<_> = pdg.edges().iter().filter(|e| e.is_unknown()).collect();
    assert!(
        !unknown_edges.is_empty(),
        "must have unknown dependency edges"
    );
}
