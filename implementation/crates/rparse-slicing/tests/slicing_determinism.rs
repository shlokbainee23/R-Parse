//! Tests for determinism, scalar crossing, sequential filters, and rebinding.

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg};
use rparse_scope::resolve_scopes;
use rparse_slicing::{compute_backward_slice, compute_column_slice, SliceCriterion};

#[test]
fn test_slicing_determinism_identical_runs() {
    let code = r#"
        raw <- read.csv("data.csv")
        clean <- raw %>% filter(valid == TRUE)
        df2 <- clean %>% mutate(target = price * count, unused = age + 10)
        res <- df2$target
    "#;
    let seeds = vec![DataFrameSeed::new(
        "raw",
        vec!["valid", "price", "count", "unused", "age"],
    )];

    let run_slice = || {
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
        let slice =
            compute_column_slice("df2", "target", &pdg, &df_analysis, &scope_model).expect("slice");
        let report = slice.format_report(&pdg);
        (slice, report)
    };

    let (s1, r1) = run_slice();
    let (s2, r2) = run_slice();

    assert_eq!(s1.retained_nodes, s2.retained_nodes);
    assert_eq!(s1.retained_column_versions, s2.retained_column_versions);
    assert_eq!(s1.excluded_column_versions, s2.excluded_column_versions);
    assert_eq!(s1.confidence, s2.confidence);
    assert_eq!(s1.contains_unknowns, s2.contains_unknowns);
    assert_eq!(
        r1, r2,
        "deterministic textual reports must match identically"
    );
}

#[test]
fn test_benchmark_26_scalar_dependency() {
    // BENCHMARK 26 — Scalar Dependency Benchmark
    // Input:
    // rate <- 0.18
    // df2 <- df %>% mutate(tax = price * rate)
    // Criterion: df2$tax
    // Expected:
    // price retained as column dependency
    // rate retained as scalar lexical dependency
    // tax retained
    // Do not classify rate as a dataframe column.
    let code = r#"
        rate <- 0.18
        df2 <- df %>%
            mutate(
                tax = price * rate
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "unused"])];

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

    let slice =
        compute_column_slice("df2", "tax", &pdg, &df_analysis, &scope_model).expect("slice");

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let tax_cid = df2_st.column("tax").unwrap();
    assert!(slice.is_column_retained(tax_cid));

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let unused_cid = df0_st.column("unused").unwrap();
    assert!(slice.is_column_retained(price_cid));
    assert!(!slice.is_column_retained(unused_cid));

    // rate is retained as a lexical scalar variable, NOT a dataframe column!
    let rate_retained = slice.retained_elements.iter().any(|e| {
        e.kind.name() == Some("rate") && (e.kind.is_variable_def() || e.kind.is_variable_use())
    });
    assert!(rate_retained, "rate must be retained as scalar variable");

    // rate is NOT a column version
    let rate_is_col = slice
        .retained_elements
        .iter()
        .any(|e| e.kind.name() == Some("rate") && e.kind.is_column_version());
    assert!(
        !rate_is_col,
        "rate must NOT be classified as a dataframe column"
    );
}

#[test]
fn test_benchmark_30_sequential_filters() {
    // BENCHMARK 30 — Sequential Filter Benchmark
    // Input:
    // df2 <- df %>% filter(valid == TRUE)
    // df3 <- df2 %>% filter(age > 18)
    // df4 <- df3 %>% mutate(target = price * count)
    // Criterion: df4$target
    // Expected:
    // - valid row-selection retained
    // - age row-selection retained
    // - price/count/target retained
    // - irrelevant columns not retained
    // - neither filter represented as CFG control dependence
    let code = r#"
        df2 <- df %>%
            filter(valid == TRUE)
        df3 <- df2 %>%
            filter(age > 18)
        df4 <- df3 %>%
            mutate(target = price * count)
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["valid", "age", "price", "count", "irrelevant"],
    )];

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

    let slice =
        compute_column_slice("df4", "target", &pdg, &df_analysis, &scope_model).expect("slice");

    let df4_st = df_analysis.active_state_for_symbol("df4").unwrap();
    let target_cid = df4_st.column("target").unwrap();
    assert!(slice.is_column_retained(target_cid));

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let valid_cid = df0_st.column("valid").unwrap();
    let age_cid = df0_st.column("age").unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let count_cid = df0_st.column("count").unwrap();
    let irr_cid = df0_st.column("irrelevant").unwrap();

    // Retained through row-selection and derivation
    assert!(
        slice.is_column_retained(valid_cid),
        "valid retained via row selection"
    );
    assert!(
        slice.is_column_retained(age_cid),
        "age retained via row selection"
    );
    assert!(slice.is_column_retained(price_cid), "price retained");
    assert!(slice.is_column_retained(count_cid), "count retained");

    // Irrelevant column EXCLUDED!
    assert!(
        !slice.is_column_retained(irr_cid),
        "irrelevant column excluded"
    );
    assert!(slice.is_column_excluded(irr_cid));

    // Neither filter is CFG control dependence
    let control_edges: Vec<_> = pdg.edges().iter().filter(|e| e.is_control()).collect();
    assert_eq!(
        control_edges.len(),
        0,
        "no CFG control dependencies for filters"
    );
}

#[test]
fn test_benchmark_31_rebinding_same_symbol() {
    // BENCHMARK 31 — Rebinding Benchmark
    // Input:
    // df <- df %>% mutate(total = price * quantity)
    // df <- df %>% mutate(tax = total * 0.18)
    // Criterion: df$tax
    // Expected: tax -> total -> price, quantity.
    // The old dataframe version remains part of ancestry.
    let code = r#"
        df <- df %>%
            mutate(
                total = price * quantity
            )
        df <- df %>%
            mutate(
                tax = total * 0.18
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];

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

    let slice = compute_column_slice("df", "tax", &pdg, &df_analysis, &scope_model).expect("slice");

    let df_active = df_analysis.active_state_for_symbol("df").unwrap();
    let tax_cid = df_active.column("tax").unwrap();
    let total_cid = df_active.column("total").unwrap();

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let qty_cid = df0_st.column("quantity").unwrap();

    assert!(slice.is_column_retained(tax_cid));
    assert!(slice.is_column_retained(total_cid));
    assert!(slice.is_column_retained(price_cid));
    assert!(slice.is_column_retained(qty_cid));
}

#[test]
fn test_benchmark_32_branch_aware_dataframe_state() {
    // BENCHMARK 32 — Branch-Aware DataFrame Benchmark
    // Input:
    // if (cond) {
    //     df <- df %>% mutate(target = price * quantity)
    // }
    // res <- df$target
    let code = r#"
        if (cond) {
            df <- df %>%
                mutate(
                    target = price * quantity
                )
        }
        res <- df$target
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];

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

    // In df_analysis, the merge at the branch resulted in UnknownStatic because target
    // only existed on one branch!
    let df_active = df_analysis.active_state_for_symbol("df").unwrap();
    assert_eq!(
        df_active.knowledge,
        rparse_dataframe::StaticKnowledge::UnknownStatic
    );

    let criterion = SliceCriterion::ScalarByName {
        symbol_name: "res".to_string(),
    };
    let slice =
        compute_backward_slice(&criterion, &pdg, &df_analysis, &scope_model).expect("slice");

    // Slice retains the res definition and cond
    let has_cond = slice.retained_elements.iter().any(|e| {
        e.kind.to_string().contains("cond") || e.kind.to_string().contains("if_condition")
    });
    assert!(has_cond, "condition must be retained in slice");
}
