//! Canonical benchmarks for column-aware static backward slicing.

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameAnalysis, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg, ProgramDependenceGraph};
use rparse_scope::{resolve_scopes, ScopeModel};
use rparse_slicing::{
    compute_backward_slice, compute_column_slice, SliceConfidence, SliceCriterion,
};

fn run_pipeline(
    source: &str,
    seeds: &[DataFrameSeed],
) -> (ScopeModel, DataFrameAnalysis, ProgramDependenceGraph) {
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
    (scope_model, df_analysis, pdg)
}

#[test]
fn test_benchmark_1_direct_column_derivation() {
    // BENCHMARK 1 — Direct Column Derivation
    // Input: df2 <- df %>% mutate(total = price * quantity)
    // Seed: df = {price, quantity}
    // Criterion: df2$total
    // Expected: price, quantity, total, defining statement. No unrelated columns.
    let code = r#"
        df2 <- df %>%
            mutate(
                total = price * quantity
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let result = compute_column_slice("df2", "total", &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    assert_eq!(result.confidence, SliceConfidence::Complete);
    assert!(!result.contains_unknowns);

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let total_cid = df2_st.column("total").unwrap();
    assert!(result.is_column_retained(total_cid));

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let qty_cid = df0_st.column("quantity").unwrap();
    assert!(result.is_column_retained(price_cid));
    assert!(result.is_column_retained(qty_cid));
}

#[test]
fn test_benchmark_2_multi_step_provenance() {
    // BENCHMARK 2 — Multi-Step Provenance
    // Input:
    // df2 <- df %>% mutate(total = price * quantity)
    // df3 <- df2 %>% mutate(tax = total * 0.18)
    // Criterion: df3$tax
    // Expected backward chain: tax -> total -> price, quantity.
    let code = r#"
        df2 <- df %>% mutate(total = price * quantity)
        df3 <- df2 %>% mutate(tax = total * 0.18)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let result = compute_column_slice("df3", "tax", &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    assert_eq!(result.confidence, SliceConfidence::Complete);

    let df3_st = df_analysis.active_state_for_symbol("df3").unwrap();
    let tax_cid = df3_st.column("tax").unwrap();
    assert!(result.is_column_retained(tax_cid));

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let total_cid = df2_st.column("total").unwrap();
    assert!(result.is_column_retained(total_cid));

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    let qty_cid = df0_st.column("quantity").unwrap();
    assert!(result.is_column_retained(price_cid));
    assert!(result.is_column_retained(qty_cid));
}

#[test]
fn test_benchmark_3_rename_plus_select() {
    // BENCHMARK 3 — Rename + Select
    // Input:
    // df2 <- df %>%
    //     rename(cost = price) %>%
    //     select(cost, customer_id)
    // Criterion: df2$cost
    // Expected: price ancestry retained through rename, cost retained, customer_id excluded.
    let code = r#"
        df2 <- df %>%
            rename(cost = price) %>%
            select(cost, customer_id)
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price", "customer_id", "unrelated"],
    )];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let result = compute_column_slice("df2", "cost", &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let cost_cid = df2_st.column("cost").unwrap();
    let cust_cid = df2_st.column("customer_id").unwrap();

    assert!(result.is_column_retained(cost_cid));
    // customer_id is irrelevant to cost, so it must be EXCLUDED!
    assert!(!result.is_column_retained(cust_cid));
    assert!(result.is_column_excluded(cust_cid));

    // Ancestor price must be retained through rename
    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_cid = df0_st.column("price").unwrap();
    assert!(result.is_column_retained(price_cid));
}

#[test]
fn test_benchmark_4_multi_input_expression() {
    // BENCHMARK 4 — Multi-Input Expression
    // Input: df2 <- df %>% mutate(full_name = paste(first_name, last_name))
    // Criterion: df2$full_name
    // Expected: first_name, last_name, full_name
    let code = r#"
        df2 <- df %>%
            mutate(
                full_name = paste(first_name, last_name)
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["first_name", "last_name"])];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let result = compute_column_slice("df2", "full_name", &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let fn_cid = df2_st.column("full_name").unwrap();
    assert!(result.is_column_retained(fn_cid));

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let fst_cid = df0_st.column("first_name").unwrap();
    let lst_cid = df0_st.column("last_name").unwrap();
    assert!(result.is_column_retained(fst_cid));
    assert!(result.is_column_retained(lst_cid));
}

#[test]
fn test_benchmark_5_column_independence() {
    // BENCHMARK 5 — Column Independence
    // Input:
    // df2 <- df %>%
    //     mutate(
    //         a = x + 1,
    //         b = y + 1
    //     )
    // Criterion: df2$a
    // Expected: x retained, a retained, y NOT retained, b NOT retained.
    let code = r#"
        df2 <- df %>%
            mutate(
                a = x + 1,
                b = y + 1
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["x", "y"])];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let result = compute_column_slice("df2", "a", &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let a_cid = df2_st.column("a").unwrap();
    let b_cid = df2_st.column("b").unwrap();

    let df0_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let x_cid = df0_st.column("x").unwrap();
    let y_cid = df0_st.column("y").unwrap();

    // a and x retained
    assert!(result.is_column_retained(a_cid));
    assert!(result.is_column_retained(x_cid));

    // b and y EXCLUDED!
    assert!(!result.is_column_retained(b_cid));
    assert!(!result.is_column_retained(y_cid));
    assert!(result.is_column_excluded(b_cid));
    assert!(result.is_column_excluded(y_cid));
}

#[test]
fn test_benchmark_7_frozen_column_aware_slice() {
    // BENCHMARK 7 — FROZEN COLUMN-AWARE SLICE
    // Input:
    // raw <- read.csv("data.csv")
    // clean <- raw %>% filter(valid == TRUE)
    // df2 <- clean %>% mutate(target = price * count,
    //                         unused = age + 10)
    // res <- df2$target
    //
    // Criterion: df2$target
    //
    // Expected RETAINED:
    // - raw
    // - clean
    // - valid
    // - price
    // - count
    // - target
    // - relevant filter statement
    // - relevant mutate expression
    // - result/use of target
    //
    // Expected EXCLUDED:
    // - unused
    // - age
    // - unrelated unused = age + 10 expression
    //
    // IMPORTANT:
    // The filter must be retained because it determines row survival.
    // But `valid` must appear through a RowSelectionDependency, NOT a CFG ControlDependency.
    let code = r#"
        raw <- read.csv("data.csv")
        clean <- raw %>% filter(valid == TRUE)
        df2 <- clean %>% mutate(target = price * count,
                                unused = age + 10)
        res <- df2$target
    "#;
    let seeds = vec![DataFrameSeed::new(
        "raw",
        vec!["valid", "price", "count", "unused", "age"],
    )];
    let (scope_model, df_analysis, pdg) = run_pipeline(code, &seeds);

    let criterion = SliceCriterion::ColumnByName {
        dataframe_symbol: "df2".to_string(),
        column_name: "target".to_string(),
    };
    let result = compute_backward_slice(&criterion, &pdg, &df_analysis, &scope_model)
        .expect("slice computation should succeed");

    assert_eq!(result.confidence, SliceConfidence::Complete);
    assert!(!result.contains_unknowns);

    let df2_st = df_analysis.active_state_for_symbol("df2").unwrap();
    let target_cid = df2_st.column("target").unwrap();
    let unused_cid = df2_st.column("unused").unwrap();

    let clean_st = df_analysis.active_state_for_symbol("clean").unwrap();
    let clean_valid_cid = clean_st.column("valid").unwrap();
    let clean_price_cid = clean_st.column("price").unwrap();
    let clean_count_cid = clean_st.column("count").unwrap();
    let clean_age_cid = clean_st.column("age").unwrap();

    let raw_st = df_analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let raw_valid_cid = raw_st.column("valid").unwrap();
    let raw_price_cid = raw_st.column("price").unwrap();
    let raw_count_cid = raw_st.column("count").unwrap();
    let raw_age_cid = raw_st.column("age").unwrap();
    let raw_unused_cid = raw_st.column("unused").unwrap();

    // 1. Verify RETAINED columns:
    assert!(
        result.is_column_retained(target_cid),
        "target must be retained"
    );
    assert!(
        result.is_column_retained(clean_price_cid),
        "clean$price must be retained"
    );
    assert!(
        result.is_column_retained(clean_count_cid),
        "clean$count must be retained"
    );
    assert!(
        result.is_column_retained(clean_valid_cid),
        "clean$valid must be retained via row selection"
    );
    assert!(
        result.is_column_retained(raw_price_cid),
        "raw$price must be retained"
    );
    assert!(
        result.is_column_retained(raw_count_cid),
        "raw$count must be retained"
    );
    assert!(
        result.is_column_retained(raw_valid_cid),
        "raw$valid must be retained"
    );

    // 2. Verify EXCLUDED columns:
    assert!(
        !result.is_column_retained(unused_cid),
        "df2$unused must NOT be retained"
    );
    assert!(
        result.is_column_excluded(unused_cid),
        "df2$unused must be in excluded_column_versions"
    );
    assert!(
        !result.is_column_retained(clean_age_cid),
        "clean$age must NOT be retained"
    );
    assert!(
        !result.is_column_retained(raw_age_cid),
        "raw$age must NOT be retained"
    );
    assert!(
        !result.is_column_retained(raw_unused_cid),
        "raw$unused must NOT be retained"
    );

    // 3. Verify Filter is retained via RowSelectionDependency, NOT ControlDependency:
    let rsel_nodes: Vec<_> = pdg
        .nodes()
        .iter()
        .filter(|n| n.is_row_selection())
        .collect();
    assert!(!rsel_nodes.is_empty(), "RowSelection node must exist");
    for rnode in &rsel_nodes {
        assert!(
            result.is_node_retained(rnode.id),
            "RowSelection node must be retained"
        );
    }

    // Verify ZERO CFG control dependencies exist in this entire PDG
    let control_edges: Vec<_> = pdg.edges().iter().filter(|e| e.is_control()).collect();
    assert_eq!(
        control_edges.len(),
        0,
        "filter must NOT be modeled as CFG control dependence"
    );

    // 4. Verify partial retention of mutate statement:
    // In df2 <- clean %>% mutate(target = price * count, unused = age + 10)
    // The target expression is retained, but the unused expression is excluded!
    let mut mutate_stmt_found = false;
    for stmt_slice in result.statement_slices.values() {
        if !stmt_slice.retained_subnodes.is_empty() && !stmt_slice.excluded_subnodes.is_empty() {
            assert_eq!(
                stmt_slice.retention,
                rparse_slicing::RetentionState::PartiallyRetained
            );
            mutate_stmt_found = true;
        }
    }
    assert!(
        mutate_stmt_found,
        "mutate statement must be PARTIALLY_RETAINED"
    );
}
