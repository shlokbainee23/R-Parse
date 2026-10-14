//! Tests for whole-object vs column-aware slicing distinction (Sections 23 and 24).

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg};
use rparse_scope::resolve_scopes;
use rparse_slicing::{compute_column_slice, compute_object_slice};

#[test]
fn test_object_vs_column_slice_distinction() {
    let code = r#"
        df2 <- df %>%
            mutate(
                target = price * quantity,
                unused = age + 10
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity", "age"])];

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

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let target_cid = df2_st.column("target").unwrap();
    let unused_cid = df2_st.column("unused").unwrap();

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let qty_cid = df0_st.column("quantity").unwrap();
    let age_cid = df0_st.column("age").unwrap();

    // 1. Column-aware slice for df2$target
    let col_slice =
        compute_column_slice("df2", "target", &pdg, &df_analysis, &scope_model).expect("col slice");

    assert!(col_slice.is_column_retained(target_cid));
    assert!(col_slice.is_column_retained(price_cid));
    assert!(col_slice.is_column_retained(qty_cid));
    assert!(!col_slice.is_column_retained(unused_cid));
    assert!(!col_slice.is_column_retained(age_cid));
    assert!(col_slice.is_column_excluded(unused_cid));
    assert!(col_slice.is_column_excluded(age_cid));

    // 2. Whole-object slice for df2
    let obj_slice =
        compute_object_slice("df2", &pdg, &df_analysis, &scope_model).expect("object slice");

    // Whole-object slice MUST retain all columns in df2's active schema!
    assert!(obj_slice.is_column_retained(target_cid));
    assert!(obj_slice.is_column_retained(unused_cid));
    assert!(obj_slice.is_column_retained(price_cid));
    assert!(obj_slice.is_column_retained(qty_cid));
    assert!(obj_slice.is_column_retained(age_cid));
    assert_eq!(obj_slice.excluded_column_versions.len(), 0);
}
