//! Integration tests for expression dependency extraction:
//! scalar vs column separation, .data access, .env access, operators, and df$col expressions.

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
fn test_14_scalar_variable_plus_column() {
    // TEST 14 — Scalar Variable + Column
    // Input:
    // rate <- 0.18
    // df2 <- df %>% mutate(tax = price * rate)
    // Expected:
    // column dependency: price
    // lexical dependency: rate
    // rate MUST NOT be classified as a dataframe column
    let code = r#"
        rate <- 0.18
        df2 <- df %>% mutate(tax = price * rate)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let tax_id = *df2.schema.get("tax").unwrap();

    let tax_edge_id = analysis
        .column_version(tax_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let tax_edge = analysis.provenance().get_edge(tax_edge_id).unwrap();

    // 1. Column input contains price@v0
    assert_eq!(tax_edge.input_columns.len(), 1);
    assert!(tax_edge.input_columns.contains(&price_v0));

    // 2. Lexical variable input contains rate SymbolId
    assert_eq!(tax_edge.input_variables.len(), 1);

    // 3. Rate is NOT in input_columns
    assert!(!tax_edge
        .input_columns
        .iter()
        .any(|&c| { analysis.column_version(c).map(|col| col.name.as_str()) == Some("rate") }));
}

#[test]
fn test_18_static_dot_data_access() {
    // TEST 18 — Static .data Access
    // Input: df2 <- df %>% mutate(total = .data[["price"]] * quantity)
    // Expected: price and quantity both resolve correctly
    let code = r#"
        df2 <- df %>% mutate(total = .data[["price"]] * quantity)
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

    assert!(edge.input_columns.contains(&price_v0));
    assert!(edge.input_columns.contains(&quantity_v0));
    assert_eq!(edge.input_columns.len(), 2);
    assert_eq!(edge.knowledge, StaticKnowledge::KnownStatic);
}

#[test]
fn test_static_dot_data_dollar_access() {
    // .data$col access
    let code = r#"
        df2 <- df %>% mutate(total = .data$price * quantity)
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

    assert!(edge.input_columns.contains(&price_v0));
    assert!(edge.input_columns.contains(&quantity_v0));
    assert_eq!(edge.knowledge, StaticKnowledge::KnownStatic);
}

#[test]
fn test_19_dot_env_access() {
    // TEST 19 — .env Access
    // Input:
    // rate <- 0.18
    // df2 <- df %>% mutate(tax = price * .env$rate)
    // Expected:
    // price = column dependency
    // rate = lexical variable dependency (.env explicitly bypasses data masking)
    let code = r#"
        rate <- 0.18
        df2 <- df %>% mutate(tax = price * .env$rate)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let tax_id = *df2.schema.get("tax").unwrap();

    let edge_id = analysis
        .column_version(tax_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();

    assert_eq!(edge.input_columns.len(), 1);
    assert!(edge.input_columns.contains(&price_v0));
    assert_eq!(edge.input_variables.len(), 1);
}

#[test]
fn test_binary_and_unary_operators() {
    // Binary operators +, -, *, /, &, | and unary !
    let code = r#"
        df2 <- df %>% mutate(flag = !(price > 100) & (quantity <= 5))
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let flag_id = *df2.schema.get("flag").unwrap();

    let edge_id = analysis
        .column_version(flag_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();

    assert!(edge.input_columns.contains(&price_v0));
    assert!(edge.input_columns.contains(&quantity_v0));
    assert_eq!(edge.input_columns.len(), 2);
}

#[test]
fn test_explicit_dataframe_dollar_access_in_expression() {
    // df$price inside expression
    let code = r#"
        df2 <- df %>% mutate(total = df$price * quantity)
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

    assert!(edge.input_columns.contains(&price_v0));
    assert!(edge.input_columns.contains(&quantity_v0));
}
