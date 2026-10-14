//! Integration tests for dataframe semantic states, mutate, transmute, select, rename,
//! left-to-right evaluation, overwrite semantics, and symbol rebinding.

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
fn test_1_direct_column_derivation() {
    // TEST 1 — Direct Column Derivation
    // Input: df2 <- df %>% mutate(total = price * quantity)
    // Seed: df = {price, quantity}
    let code = r#"
        df2 <- df %>% mutate(total = price * quantity)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis
        .state(DataFrameId(0))
        .expect("initial df state exists");
    assert_eq!(df0.version, 0);
    assert_eq!(df0.schema.len(), 2);
    let price_v0 = *df0.schema.get("price").expect("price in df0");
    let quantity_v0 = *df0.schema.get("quantity").expect("quantity in df0");

    let df2 = analysis
        .active_state_for_symbol("df2")
        .expect("df2 state exists");
    assert_eq!(df2.version, 1);
    assert_eq!(df2.schema.len(), 3);
    assert_eq!(df2.schema.get("price"), Some(&price_v0));
    assert_eq!(df2.schema.get("quantity"), Some(&quantity_v0));

    let total_v1 = *df2.schema.get("total").expect("total exists in df2");
    assert_ne!(total_v1, price_v0);
    assert_ne!(total_v1, quantity_v0);

    let total_col = analysis
        .column_version(total_v1)
        .expect("total col version");
    assert_eq!(total_col.name, "total");
    assert_eq!(total_col.version, 0);
    assert_eq!(total_col.knowledge, StaticKnowledge::KnownStatic);
    assert!(total_col.provenance_edge.is_some());

    let edge_id = total_col.provenance_edge.unwrap();
    let edge = analysis
        .provenance()
        .get_edge(edge_id)
        .expect("edge exists");
    assert_eq!(edge.output_column, total_v1);
    assert_eq!(edge.transform_type, TransformationType::Mutate);
    assert_eq!(edge.knowledge, StaticKnowledge::KnownStatic);

    let mut expected_inputs = BTreeSet::new();
    expected_inputs.insert(price_v0);
    expected_inputs.insert(quantity_v0);
    assert_eq!(edge.input_columns, expected_inputs);
    assert!(edge.input_variables.is_empty());
}

#[test]
fn test_2_multi_step_propagation() {
    // TEST 2 — Multi-Step Propagation
    // Input:
    // df2 <- df %>% mutate(total = price * quantity)
    // df3 <- df2 %>% mutate(tax = total * 0.18)
    let code = r#"
        df2 <- df %>% mutate(total = price * quantity)
        df3 <- df2 %>% mutate(tax = total * 0.18)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let total_v1 = *df2.schema.get("total").unwrap();

    let df3 = analysis.active_state_for_symbol("df3").unwrap();
    let tax_v2 = *df3.schema.get("tax").unwrap();

    // 1. Immediate derivation for total: inputs = {price@v0, quantity@v0}
    let total_edge_id = analysis
        .column_version(total_v1)
        .unwrap()
        .provenance_edge
        .unwrap();
    let total_edge = analysis.provenance().get_edge(total_edge_id).unwrap();
    let mut total_expected_inputs = BTreeSet::new();
    total_expected_inputs.insert(price_v0);
    total_expected_inputs.insert(quantity_v0);
    assert_eq!(total_edge.input_columns, total_expected_inputs);

    // 2. Immediate derivation for tax: inputs = {total@v1} ONLY (not prematurely flattened!)
    let tax_edge_id = analysis
        .column_version(tax_v2)
        .unwrap()
        .provenance_edge
        .unwrap();
    let tax_edge = analysis.provenance().get_edge(tax_edge_id).unwrap();
    let mut tax_expected_inputs = BTreeSet::new();
    tax_expected_inputs.insert(total_v1);
    assert_eq!(tax_edge.input_columns, tax_expected_inputs);

    // 3. Transitive ancestor traversal includes total, price, and quantity
    let ancestors = analysis.provenance().ancestor_columns(tax_v2);
    assert!(ancestors.contains(&total_v1));
    assert!(ancestors.contains(&price_v0));
    assert!(ancestors.contains(&quantity_v0));
}

#[test]
fn test_3_rename_plus_select() {
    // TEST 3 — Rename + Select
    // Input:
    // df2 <- df %>%
    //     rename(cost = price) %>%
    //     select(cost, customer_id)
    // Seed: df = {price, customer_id, unused}
    let code = r#"
        df2 <- df %>%
            rename(cost = price) %>%
            select(cost, customer_id)
    "#;
    let seeds = vec![DataFrameSeed::new(
        "df",
        vec!["price", "customer_id", "unused"],
    )];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let cust_v0 = *df0.schema.get("customer_id").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();

    // Verification:
    // - cost exists
    assert!(df2.schema.contains_key("cost"));
    // - price is absent in output schema
    assert!(!df2.schema.contains_key("price"));
    // - customer_id exists and retains its original ColumnVersionId
    assert_eq!(df2.schema.get("customer_id"), Some(&cust_v0));
    // - unused does not exist
    assert!(!df2.schema.contains_key("unused"));

    // - cost remains linked to original price ancestry
    let cost_col_id = *df2.schema.get("cost").unwrap();
    let cost_col = analysis.column_version(cost_col_id).unwrap();
    assert_eq!(cost_col.name, "cost");
    assert!(cost_col.provenance_edge.is_some());

    let rename_edge = analysis
        .provenance()
        .get_edge(cost_col.provenance_edge.unwrap())
        .unwrap();
    assert_eq!(
        rename_edge.transform_type,
        TransformationType::Rename {
            old_name: "price".to_string()
        }
    );
    assert!(rename_edge.input_columns.contains(&price_v0));
}

#[test]
fn test_4_multi_column_combination() {
    // TEST 4 — Multi-Column Combination
    // Input: df2 <- df %>% mutate(full_name = paste(first_name, last_name))
    let code = r#"
        df2 <- df %>% mutate(full_name = paste(first_name, last_name))
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["first_name", "last_name"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let fn_v0 = *df0.schema.get("first_name").unwrap();
    let ln_v0 = *df0.schema.get("last_name").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let full_name_id = *df2.schema.get("full_name").unwrap();

    let edge_id = analysis
        .column_version(full_name_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let edge = analysis.provenance().get_edge(edge_id).unwrap();

    let mut expected_inputs = BTreeSet::new();
    expected_inputs.insert(fn_v0);
    expected_inputs.insert(ln_v0);
    assert_eq!(edge.input_columns, expected_inputs);
}

#[test]
fn test_5_column_independence() {
    // TEST 5 — Column Independence
    // Input:
    // df2 <- df %>%
    //     mutate(
    //         a = x + 1,
    //         b = y + 1
    //     )
    let code = r#"
        df2 <- df %>%
            mutate(
                a = x + 1,
                b = y + 1
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["x", "y"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let x_v0 = *df0.schema.get("x").unwrap();
    let y_v0 = *df0.schema.get("y").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let a_id = *df2.schema.get("a").unwrap();
    let b_id = *df2.schema.get("b").unwrap();

    let a_edge_id = analysis
        .column_version(a_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let a_edge = analysis.provenance().get_edge(a_edge_id).unwrap();
    // a depends on x; a MUST NOT depend on y
    assert!(a_edge.input_columns.contains(&x_v0));
    assert!(!a_edge.input_columns.contains(&y_v0));

    let b_edge_id = analysis
        .column_version(b_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let b_edge = analysis.provenance().get_edge(b_edge_id).unwrap();
    // b depends on y; b MUST NOT depend on x
    assert!(b_edge.input_columns.contains(&y_v0));
    assert!(!b_edge.input_columns.contains(&x_v0));
}

#[test]
fn test_8_left_to_right_mutate() {
    // TEST 8 — Left-to-Right Mutate
    // Input:
    // df2 <- df %>%
    //     mutate(
    //         total = price * quantity,
    //         tax = total * 0.18
    //     )
    let code = r#"
        df2 <- df %>%
            mutate(
                total = price * quantity,
                tax = total * 0.18
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let total_id = *df2.schema.get("total").unwrap();
    let tax_id = *df2.schema.get("tax").unwrap();

    let tax_edge_id = analysis
        .column_version(tax_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let tax_edge = analysis.provenance().get_edge(tax_edge_id).unwrap();

    // tax MUST depend on the NEW total version created within the same mutate call
    assert!(tax_edge.input_columns.contains(&total_id));
}

#[test]
fn test_9_mutation_overwrite() {
    // TEST 9 — Mutation Overwrite
    // Input:
    // df2 <- df %>%
    //     mutate(
    //         total = price * quantity,
    //         price = quantity * 2
    //     )
    let code = r#"
        df2 <- df %>%
            mutate(
                total = price * quantity,
                price = quantity * 2
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df0 = analysis.state(DataFrameId(0)).unwrap();
    let price_v0 = *df0.schema.get("price").unwrap();
    let quantity_v0 = *df0.schema.get("quantity").unwrap();

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    let total_id = *df2.schema.get("total").unwrap();
    let price_v1 = *df2.schema.get("price").unwrap();

    // price was overwritten: price_v1 is distinct from price_v0
    assert_ne!(price_v1, price_v0);

    // total must depend on OLD price version (price_v0)
    let total_edge_id = analysis
        .column_version(total_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let total_edge = analysis.provenance().get_edge(total_edge_id).unwrap();
    assert!(total_edge.input_columns.contains(&price_v0));
    assert!(!total_edge.input_columns.contains(&price_v1));

    // new price version depends on quantity
    let new_price_edge_id = analysis
        .column_version(price_v1)
        .unwrap()
        .provenance_edge
        .unwrap();
    let new_price_edge = analysis.provenance().get_edge(new_price_edge_id).unwrap();
    assert!(new_price_edge.input_columns.contains(&quantity_v0));
}

#[test]
fn test_15_rebinding_same_symbol() {
    // TEST 15 — Rebinding Same Symbol
    // Input:
    // df <- df %>% mutate(total = price * quantity)
    // df2 <- df %>% mutate(tax = total * 0.18)
    let code = r#"
        df <- df %>% mutate(total = price * quantity)
        df2 <- df %>% mutate(tax = total * 0.18)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df_state = analysis.active_state_for_symbol("df").unwrap();
    assert_eq!(df_state.version, 1);
    let total_id = *df_state.schema.get("total").unwrap();

    let df2_state = analysis.active_state_for_symbol("df2").unwrap();
    assert_eq!(df2_state.version, 2);
    let tax_id = *df2_state.schema.get("tax").unwrap();

    let tax_edge_id = analysis
        .column_version(tax_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let tax_edge = analysis.provenance().get_edge(tax_edge_id).unwrap();
    assert!(tax_edge.input_columns.contains(&total_id));

    // The original df@v0 remains preserved in the registry
    let df0 = analysis.state(DataFrameId(0)).unwrap();
    assert_eq!(df0.version, 0);
    assert!(!df0.schema.contains_key("total"));
}

#[test]
fn test_16_independent_output_symbol() {
    // TEST 16 — Independent Output Symbol
    // Input: df2 <- df %>% mutate(total = price * quantity)
    let code = r#"
        df2 <- df %>% mutate(total = price * quantity)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df_state = analysis.active_state_for_symbol("df").unwrap();
    assert_eq!(df_state.version, 0);
    assert!(!df_state.schema.contains_key("total"));

    let df2_state = analysis.active_state_for_symbol("df2").unwrap();
    assert_eq!(df2_state.version, 1);
    assert!(df2_state.schema.contains_key("total"));
}

#[test]
fn test_transmute_semantics() {
    // Transmute semantics: left-to-right transformation, but final schema
    // contains ONLY explicitly produced columns.
    let code = r#"
        df2 <- df %>%
            transmute(
                total = price * quantity,
                tax = total * 0.18
            )
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let analysis = run_analysis(code, &seeds);

    let df2 = analysis.active_state_for_symbol("df2").unwrap();
    // Only total and tax in output schema!
    assert_eq!(df2.schema.len(), 2);
    assert!(df2.schema.contains_key("total"));
    assert!(df2.schema.contains_key("tax"));
    assert!(!df2.schema.contains_key("price"));
    assert!(!df2.schema.contains_key("quantity"));

    let total_id = *df2.schema.get("total").unwrap();
    let tax_id = *df2.schema.get("tax").unwrap();

    let tax_edge_id = analysis
        .column_version(tax_id)
        .unwrap()
        .provenance_edge
        .unwrap();
    let tax_edge = analysis.provenance().get_edge(tax_edge_id).unwrap();
    assert!(tax_edge.input_columns.contains(&total_id));
}
