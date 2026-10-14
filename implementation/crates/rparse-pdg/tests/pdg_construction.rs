//! Tests for PDG construction, invariant validation, and deterministic graph structure.

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg, ProgramDependenceGraph};
use rparse_scope::resolve_scopes;

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
fn test_pdg_construction_and_validation_basic() {
    let code = r#"
        x <- 10
        y <- x + 5
        z <- y * 2
    "#;
    let pdg = construct_pdg(code, &[]);

    assert!(pdg.nodes().len() >= 3);
    assert!(!pdg.edges().is_empty());
    assert!(validate_pdg(&pdg).is_ok());

    // Verify sequential node and edge IDs
    for (i, node) in pdg.nodes().iter().enumerate() {
        assert_eq!(node.id.0, i as u32);
    }
    for (i, edge) in pdg.edges().iter().enumerate() {
        assert_eq!(edge.id.0, i as u32);
    }
}

#[test]
fn test_pdg_construction_determinism() {
    let code = r#"
        raw <- read.csv("file.csv")
        clean <- raw %>% filter(valid == TRUE)
        df2 <- clean %>% mutate(target = price * count, unused = age + 10)
        res <- df2$target
    "#;
    let seeds = vec![DataFrameSeed::new(
        "raw",
        vec!["valid", "price", "count", "unused", "age"],
    )];

    let pdg1 = construct_pdg(code, &seeds);
    let pdg2 = construct_pdg(code, &seeds);

    assert_eq!(pdg1.nodes().len(), pdg2.nodes().len());
    assert_eq!(pdg1.edges().len(), pdg2.edges().len());

    for (n1, n2) in pdg1.nodes().iter().zip(pdg2.nodes().iter()) {
        assert_eq!(n1.id, n2.id);
        assert_eq!(n1.kind, n2.kind);
        assert_eq!(n1.span, n2.span);
    }

    for (e1, e2) in pdg1.edges().iter().zip(pdg2.edges().iter()) {
        assert_eq!(e1.id, e2.id);
        assert_eq!(e1.from, e2.from);
        assert_eq!(e1.to, e2.to);
        assert_eq!(e1.kind, e2.kind);
    }
}
