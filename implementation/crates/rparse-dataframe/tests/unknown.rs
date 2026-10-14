//! Integration tests for explicit UNKNOWN_STATIC handling, unknown propagation,
//! unknown functions, dynamic indexing, branch ambiguity, and determinism.

use rparse_dataframe::{
    analyze_dataframes, DataFrameAnalysis, DataFrameId, DataFrameSeed, StaticKnowledge,
};
use rparse_ir::parse_source;
use rparse_scope::resolve_scopes;

fn run_analysis(source: &str, seeds: &[DataFrameSeed]) -> DataFrameAnalysis {
    let program = parse_source(source).expect("R program should parse");
    let scope_model = resolve_scopes(&program);
    analyze_dataframes(&program, &scope_model, seeds)
}

#[test]
fn test_6_explicit_unknown_dynamic_indexing() {
    // TEST 6 — Explicit UNKNOWN
    // Input: df2 <- df %>% mutate(dynamic_col = .data[[get_col_name()]])
    // Expected:
    // dynamic_col contains UNKNOWN_STATIC dependency.
    // Do not guess which dataframe column get_col_name() returns.
    // The analyzer must complete rather than abort.
    let code = r#"
        df2 <- df %>% mutate(dynamic_col = .data[[get_col_name()]])
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    assert!(df2.schema.contains_key("dynamic_col"));

    let dynamic_col_id = *df2.schema.get("dynamic_col").unwrap();
    let col = analysis.column_version(dynamic_col_id).unwrap();

    assert_eq!(col.knowledge, StaticKnowledge::UnknownStatic);

    let edge_id = col.provenance_edge.unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();
    assert_eq!(edge.knowledge, StaticKnowledge::UnknownStatic);
}

#[test]
fn test_17_unknown_function_handling() {
    // TEST 17 — Unknown Function
    // Input: df2 <- df %>% mutate(result = custom_transform(price, quantity))
    // Expected:
    // - price and quantity remain visible as explicit dependencies
    // - unknown semantics are represented conservatively
    // - no fabricated precise function semantics
    // - no panic
    let code = r#"
        df2 <- df %>% mutate(result = custom_transform(price, quantity))
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let result_id = *df2.schema.get("result").unwrap();

    let result_col = analysis.column_version(result_id).unwrap();
    assert_eq!(result_col.knowledge, StaticKnowledge::UnknownStatic);

    let edge_id = result_col.provenance_edge.unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();

    // price and quantity remain visible as explicit dependencies!
    assert!(edge.input_columns.contains(&price_v0));
    assert!(edge.input_columns.contains(&quantity_v0));
    assert_eq!(edge.input_columns.len(), 2);

    // Conservative unknown static knowledge preserved
    assert_eq!(edge.knowledge, StaticKnowledge::UnknownStatic);
}

#[test]
fn test_32_unknown_propagation_with_known_dependencies() {
    // TEST 32 — UNKNOWN Propagation
    // Do not discard known dependencies merely because UNKNOWN is present.
    // df2 <- df %>% mutate(a = price + .data[[get_name()]])
    let code = r#"
        df2 <- df %>% mutate(a = price + .data[[get_name()]])
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let a_id = *df2.schema.get("a").unwrap();

    let edge_id = analysis
        .column_version(a_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();

    // price is preserved as a known input column
    assert!(edge.input_columns.contains(&price_v0));
    // And certainty is marked UnknownStatic
    assert_eq!(edge.knowledge, StaticKnowledge::UnknownStatic);
}

#[test]
fn test_dynamic_select_helper_unknown() {
    // Dynamic selector helper where column names cannot be resolved statically
    let code = r#"
        df2 <- df %>% select(matches("^prefix"))
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["prefix_a", "other_b"])];
    let analysis = run_analysis(code, &seeds);

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    assert_eq!(df2.knowledge, StaticKnowledge::UnknownStatic);
}

#[test]
fn test_branch_aware_dataframe_states() {
    // Section 27: Branch-Aware DataFrame States
    // If column identity differs across possible states at a join site:
    // preserve candidate/common versions where safely representable, otherwise emit UNKNOWN_STATIC.
    let code = r#"
        if (cond) {
            df <- df %>% mutate(total = price * quantity)
        }
        res <- df
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let res_state = analysis.active_state_for_symbol("df").unwrap();
    // After conditional mutate, df state at join has UnknownStatic knowledge because schemas differ
    assert_eq!(res_state.knowledge, StaticKnowledge::UnknownStatic);
    // But common columns price and quantity are safely preserved
    assert!(res_state.schema.contains_key("price"));
    assert!(res_state.schema.contains_key("quantity"));
}

#[test]
fn test_34_determinism() {
    // TEST 34 — Determinism
    // Running the same analysis twice on identical source and identical static
    // schema seeds must produce logically identical:
    // - DataFrameIds
    // - ColumnVersionIds
    // - provenance edge IDs
    // - schemas
    // - dependency sets
    // - row-selection dependencies
    let code = r#"
        df2 <- df %>%
            mutate(total = price * quantity, tax = total * 0.18) %>%
            filter(tax > 50)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];

    let run1 = run_analysis(code, &seeds);
    let run2 = run_analysis(code, &seeds);

    // 1. Same active symbol mapping
    assert_eq!(run1.active_symbol_states, run2.active_symbol_states);

    // 2. Same states
    assert_eq!(run1.registry.states.len(), run2.registry.states.len());
    for (id, state1) in &run1.registry.states {
        let state2 = run2
            .registry
            .states
            .get(id)
            .expect("matching state in run2");
        assert_eq!(state1.id, state2.id);
        assert_eq!(state1.version, state2.version);
        assert_eq!(state1.schema, state2.schema);
        assert_eq!(state1.knowledge, state2.knowledge);
        assert_eq!(state1.row_selections, state2.row_selections);
    }

    // 3. Same column versions
    assert_eq!(run1.registry.columns.len(), run2.registry.columns.len());
    for (col1, col2) in run1
        .registry
        .columns
        .iter()
        .zip(run2.registry.columns.iter())
    {
        assert_eq!(col1, col2);
    }

    // 4. Same provenance edges
    assert_eq!(run1.provenance().edges(), run2.provenance().edges());
}
