//! Integration tests for column provenance, filter row-selection dependencies,
//! sequential filters, direct Base R assignments, Base R projections, transform(),
//! and native pipe pipelines.

use rparse_dataframe::{
    analyze_dataframes, DataFrameAnalysis, DataFrameId, DataFrameSeed, StaticKnowledge,
    TransformationType,
};
use rparse_ir::parse_source;
use rparse_scope::resolve_scopes;
use std::collections::BTreeSet;

fn run_analysis(source: &str, seeds: &[DataFrameSeed]) -> DataFrameAnalysis {
    let program = parse_source(source).expect("R program should parse");
    let scope_model = resolve_scopes(&program);
    analyze_dataframes(&program, &scope_model, seeds)
}

#[test]
fn test_7_filter_dependency() {
    // TEST 7 — Filter Dependency
    // Input: clean <- df %>% filter(valid == TRUE)
    // Expected:
    // - output schema contains the same columns
    // - `valid` is recorded in RowSelectionDependency
    // - filter is NOT represented as CFG control dependence
    // - filter is NOT represented as a normal column derivation to output columns
    let code = r#"
        clean <- df %>% filter(valid == TRUE)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity", "valid"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();
    let valid_v0 = *df0.schema.get("valid").unwrap();

    let clean = analysis.active_state_for_symbol("clean").unwrap();
    assert_eq!(clean.version, 1);
    assert_eq!(clean.schema.len(), 3);

    // Existing column versions carried forward identically
    assert_eq!(clean.schema.get("price"), Some(&price_v0));
    assert_eq!(clean.schema.get("quantity"), Some(&quantity_v0));
    assert_eq!(clean.schema.get("valid"), Some(&valid_v0));

    // Filter did NOT add standard column derivations
    assert_eq!(analysis.provenance().edges().len(), 0);

    // Filter introduces first-class RowSelectionDependency
    assert_eq!(clean.row_selections.len(), 1);
    let row_dep = &clean.row_selections[0];
    assert_eq!(row_dep.dataframe_id, clean.id);
    assert!(row_dep.predicate_columns.contains(&valid_v0));
    assert_eq!(row_dep.predicate_columns.len(), 1);
    assert_eq!(row_dep.knowledge, StaticKnowledge::KnownStatic);
}

#[test]
fn test_10_sequential_filters() {
    // TEST 10 — Sequential Filters
    // Input:
    // df2 <- df %>% filter(valid == TRUE)
    // df3 <- df2 %>% filter(age > 18)
    // Expected:
    // - df2 preserves the first row-selection dependency
    // - df3 preserves both row-selection dependencies (chained)
    let code = r#"
        df2 <- df %>% filter(valid == TRUE)
        df3 <- df2 %>% filter(age > 18)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["valid", "age", "score"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let valid_v0 = *df0.schema.get("valid").unwrap();
    let age_v0 = *df0.schema.get("age").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    assert_eq!(df2.row_selections.len(), 1);
    assert!(df2.row_selections[0].predicate_columns.contains(&valid_v0));

    let df3 = analysis.active_state_for_symbol("df3").unwrap();
    assert_eq!(df3.row_selections.len(), 2);
    // Chain preserved: first filter on valid, second filter on age
    assert!(df3.row_selections[0].predicate_columns.contains(&valid_v0));
    assert!(df3.row_selections[1].predicate_columns.contains(&age_v0));
}

#[test]
fn test_11_direct_base_r_assignment() {
    // TEST 11 — Direct Base R Assignment
    // Input: df$total <- df$price * df$quantity
    // Expected:
    // - creates new total column version
    // - creates provenance edge with BaseRDirectAssignment
    // - creates new dataframe state
    // - rebinds df to new state
    let code = r#"
        df$total <- df$price * df$quantity
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df_active = analysis.active_state_for_symbol("df").unwrap();
    assert_eq!(df_active.version, 1);
    assert_eq!(df_active.schema.len(), 3);
    assert_eq!(df_active.schema.get("price"), Some(&price_v0));
    assert_eq!(df_active.schema.get("quantity"), Some(&quantity_v0));

    let total_id = *df_active.schema.get("total").expect("total in df");
    let total_col = analysis.column_version(total_id).unwrap();
    assert_eq!(total_col.name, "total");
    assert_eq!(total_col.version, 0);

    let edge_id = total_col.provenance_edge.unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();
    assert_eq!(edge.output_column, total_id);
    assert_eq!(
        edge.transform_type,
        TransformationType::BaseRDirectAssignment
    );

    let mut expected_inputs = BTreeSet::new();
    expected_inputs.insert(price_v0);
    expected_inputs.insert(quantity_v0);
    assert_eq!(edge.input_columns, expected_inputs);
}

#[test]
fn test_12_base_r_projection() {
    // TEST 12 — Base R Projection
    // Input: df2 <- df[, c("price", "quantity")]
    // Expected:
    // - only selected columns remain
    // - nonselected columns absent
    // - retained column version IDs preserved
    let code = r#"
        df2 <- df[, c("price", "quantity")]
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price", "quantity", "extra_col"],
    )];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    assert_eq!(df2.schema.len(), 2);
    assert_eq!(df2.schema.get("price"), Some(&price_v0));
    assert_eq!(df2.schema.get("quantity"), Some(&quantity_v0));
    assert!(!df2.schema.contains_key("extra_col"));
}

#[test]
fn test_13_transform_function() {
    // TEST 13 — transform()
    // Input: df2 <- transform(df, total = price * quantity)
    // Expected:
    // - modeled equivalently to mutate
    // - input dataframe state, dependency extraction, new column version, provenance edge
    let code = r#"
        df2 <- transform(df, total = price * quantity)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let total_id = *df2.schema.get("total").unwrap();

    let edge_id = analysis
        .column_version(total_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();
    assert_eq!(edge.output_column, total_id);
    assert_eq!(edge.transform_type, TransformationType::Transform);

    let mut expected_inputs = BTreeSet::new();
    expected_inputs.insert(price_v0);
    expected_inputs.insert(quantity_v0);
    assert_eq!(edge.input_columns, expected_inputs);
}

#[test]
fn test_base_r_direct_column_overwrite() {
    // df$price <- df$quantity * 2
    // New price column version must NOT overwrite identity of previous price version.
    let code = r#"
        df$price <- df$quantity * 2
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df_active = analysis.active_state_for_symbol("df").unwrap();
    let price_v1 = *df_active.schema.get("price").unwrap();
    assert_ne!(price_v1, price_v0);

    let price_col_v1 = analysis.column_version(price_v1).unwrap();
    assert_eq!(price_col_v1.version, 1);
    let edge_id = price_col_v1.provenance_edge.unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();
    assert!(edge.input_columns.contains(&quantity_v0));
    assert!(!edge.input_columns.contains(&price_v0));

    // Old version still available
    let price_col_v0 = analysis.column_version(price_v0).unwrap();
    assert_eq!(price_col_v0.version, 0);
}

#[test]
fn test_native_pipe_pipeline() {
    // Pipeline using native pipe `|>`
    let code = r#"
        df2 <- df |>
            mutate(total = price * quantity) |>
            filter(total > 100)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    assert!(df2.schema.contains_key("total"));
    let total_id = *df2.schema.get("total").unwrap();

    // Contains row selection from filter
    assert_eq!(df2.row_selections.len(), 1);
    assert!(df2.row_selections[0].predicate_columns.contains(&total_id));
}
