//! Tests for control dependencies and separation from row-selection dependencies.

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
fn test_cfg_control_dependency_on_branch() {
    let code = r#"
        if (enabled) {
            x <- 10
        }
        res <- x
    "#;
    let pdg = setup_pdg(code, &[]);

    // Find control edges
    let control_edges: Vec<_> = pdg.edges().iter().filter(|e| e.is_control()).collect();
    assert!(
        !control_edges.is_empty(),
        "must have control dependency edges"
    );
}

#[test]
fn test_filter_is_row_selection_not_control_dependency() {
    let code = r#"
        raw <- read.csv("data.csv")
        clean <- raw %>% filter(valid == TRUE)
        df2 <- clean %>% mutate(target = price * count)
    "#;
    let seeds = vec![DataFrameSeed::new("raw", vec!["valid", "price", "count"])];
    let pdg = setup_pdg(code, &seeds);

    // There is no if-statement in this code, so there must be 0 ControlDependency edges!
    let control_edges: Vec<_> = pdg.edges().iter().filter(|e| e.is_control()).collect();
    assert_eq!(
        control_edges.len(),
        0,
        "filter must not generate CFG control edges"
    );

    // But there MUST be RowSelectionDependency edges!
    let row_edges: Vec<_> = pdg
        .edges()
        .iter()
        .filter(|e| e.is_row_selection())
        .collect();
    assert!(
        !row_edges.is_empty(),
        "filter must generate RowSelectionDependency edges"
    );
}
