//! Tests for PDG column ports, column derivations, and column statement bindings.

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameAnalysis, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg, ProgramDependenceGraph};
use rparse_scope::resolve_scopes;

fn setup_analysis(
    source: &str,
    seeds: &[DataFrameSeed],
) -> (DataFrameAnalysis, ProgramDependenceGraph) {
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
    (df_analysis, pdg)
}

#[test]
fn test_column_ports_individual_granularity() {
    let code = r#"
        df2 <- df %>%
            mutate(
                target = price * quantity,
                unused = age + 10
            )
        res <- df2$target
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity", "age"])];
    let (analysis, pdg) = setup_analysis(code, &seeds);

    let df2_st = analysis.active_state_for_symbol("df2").unwrap();
    let target_col_id = df2_st.column("target").unwrap();
    let unused_col_id = df2_st.column("unused").unwrap();

    let target_port = pdg.column_port(target_col_id).expect("target port exists");
    let unused_port = pdg.column_port(unused_col_id).expect("unused port exists");

    assert_ne!(target_port, unused_port);

    // Verify incoming dependencies to target port:
    // It must depend on price and quantity, but NOT age!
    let target_preds = pdg.predecessors(target_port);
    let df0_st = analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_col_id = df0_st.column("price").unwrap();
    let quantity_col_id = df0_st.column("quantity").unwrap();
    let age_col_id = df0_st.column("age").unwrap();

    let price_port = pdg.column_port(price_col_id).unwrap();
    let quantity_port = pdg.column_port(quantity_col_id).unwrap();
    let age_port = pdg.column_port(age_col_id).unwrap();

    assert!(target_preds.contains(&price_port));
    assert!(target_preds.contains(&quantity_port));
    assert!(!target_preds.contains(&age_port));

    // Verify that unused port depends on age, but NOT price or quantity!
    let unused_preds = pdg.predecessors(unused_port);
    assert!(unused_preds.contains(&age_port));
    assert!(!unused_preds.contains(&price_port));
    assert!(!unused_preds.contains(&quantity_port));
}

#[test]
fn test_multi_step_column_chaining() {
    let code = r#"
        df2 <- df %>% mutate(total = price * quantity)
        df3 <- df2 %>% mutate(tax = total * 0.18)
    "#;
    let seeds = vec![DataFrameSeed::new("df", vec!["price", "quantity"])];
    let (analysis, pdg) = setup_analysis(code, &seeds);

    let df3_st = analysis.active_state_for_symbol("df3").unwrap();
    let tax_col_id = df3_st.column("tax").unwrap();
    let tax_port = pdg.column_port(tax_col_id).unwrap();

    let df2_st = analysis.active_state_for_symbol("df2").unwrap();
    let total_col_id = df2_st.column("total").unwrap();
    let total_port = pdg.column_port(total_col_id).unwrap();

    // Immediate predecessor of tax is total!
    let tax_preds = pdg.predecessors(tax_port);
    assert!(tax_preds.contains(&total_port));

    // Predecessors of total are price and quantity
    let total_preds = pdg.predecessors(total_port);
    let df0_st = analysis.state(rparse_dataframe::DataFrameId(0)).unwrap();
    let price_port = pdg.column_port(df0_st.column("price").unwrap()).unwrap();
    let qty_port = pdg.column_port(df0_st.column("quantity").unwrap()).unwrap();

    assert!(total_preds.contains(&price_port));
    assert!(total_preds.contains(&qty_port));
}
