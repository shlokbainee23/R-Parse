//! Tests for UNKNOWN uncertainty handling in backward slicing (Benchmark 6).

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg};
use rparse_scope::resolve_scopes;
use rparse_slicing::{compute_column_slice, SliceConfidence};

#[test]
fn test_benchmark_6_unknown_backward_slice() {
    // BENCHMARK 6 — UNKNOWN
    // Input:
    // df2 <- df %>%
    //     mutate(
    //         dynamic_col = .data[[get_col_name()]]
    //     )
    // Criterion: df2$dynamic_col
    // Expected:
    // dynamic_col retained
    // UNKNOWN retained
    // origin statement retained
    // contains_unknowns = true
    // confidence = PartialContainsUnknowns
    // No guessed source column.
    let code = r#"
        df2 <- df %>%
            mutate(
                dynamic_col = .data[[get_col_name()]]
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["col_a", "col_b"])];

    let program = parse_source(code).expect("parse source");
    let scope_model = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("cfg build");
    let def_use = collect_def_use(&program, &scope_model, &cfg);
    let reaching_defs = compute_reaching_definitions(&cfg, &def_use);
    let post_dominators = compute_post_dominators(&cfg);
    let control_deps = compute_control_dependencies(&cfg, &post_dominators);
    let df_analysis = analyze_dataframes(&program, &scope_model, &seeds);

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

    let slice = compute_column_slice("df2", "dynamic_col", &pdg, &df_analysis, &scope_model)
        .expect("slice should compute");

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let dyn_cid = df2_st.column("dynamic_col").unwrap();

    assert!(slice.is_column_retained(dyn_cid));
    assert!(slice.contains_unknowns, "slice must flag contains_unknowns");
    assert_eq!(
        slice.confidence,
        SliceConfidence::PartialContainsUnknowns,
        "confidence must be PartialContainsUnknowns"
    );

    // Verify an Unknown node is explicitly retained
    let has_unknown_elem = slice
        .retained_elements
        .iter()
        .any(|e| e.kind.to_string().contains("Unknown"));
    assert!(has_unknown_elem, "Unknown node must be retained in slice");

    // Verify neither col_a nor col_b was guessed as the source!
    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let cola_cid = df0_st.column("col_a").unwrap();
    let colb_cid = df0_st.column("col_b").unwrap();
    assert!(
        !slice.is_column_retained(cola_cid),
        "col_a must NOT be guessed"
    );
    assert!(
        !slice.is_column_retained(colb_cid),
        "col_b must NOT be guessed"
    );
}
