//! Integration tests for control and nested control dependency backward slicing (Benchmarks 27 and 28).

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
fn test_benchmark_27_control_dependency() {
    // BENCHMARK 27 — Control Dependency Benchmark
    // Input:
    // if (enabled) {
    //     df2 <- df %>%
    //         mutate(
    //             target = price * quantity
    //         )
    // }
    // res <- df2$target
    // Criterion: df2$target
    // Expected: enabled condition retained.
    let code = r#"
        if (enabled) {
            df2 <- df %>%
                mutate(
                    target = price * quantity
                )
        }
        res <- df2$target
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

    let slice = compute_column_slice("df2", "target", &pdg, &df_analysis, &scope_model)
        .expect("slice should compute");

    // The controlling condition expression or if statement must be in the slice
    let mut cond_found = false;
    for elem in &slice.retained_elements {
        if elem.kind.to_string().contains("if_condition")
            || elem.kind.to_string().contains("if_stmt")
            || elem.kind.to_string().contains("enabled")
        {
            cond_found = true;
            break;
        }
    }
    assert!(cond_found, "enabled condition must be retained in slice");
}

#[test]
fn test_benchmark_28_nested_control_dependencies() {
    // BENCHMARK 28 — Nested Control Dependency Benchmark
    // Input:
    // if (outer) {
    //     if (inner) {
    //         x <- 1
    //     }
    // }
    // result <- x
    // Criterion: result
    // Expected relevant slice includes: outer, inner, x definition, result use.
    let code = r#"
        if (outer) {
            if (inner) {
                x <- 1
            }
        }
        result <- x
    "#;

    let program = parse_source(code).expect("parse source");
    let scope_model = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("cfg build");
    let def_use = collect_def_use(&program, &scope_model, &cfg);
    let reaching_defs = compute_reaching_definitions(&cfg, &def_use);
    let post_dominators = compute_post_dominators(&cfg);
    let control_deps = compute_control_dependencies(&cfg, &post_dominators);
    let df_analysis = analyze_dataframes(&program, &scope_model, &[]);

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

    let criterion = SliceCriterion::ScalarByName {
        symbol_name: "result".to_string(),
    };
    let slice = compute_backward_slice(&criterion, &pdg, &df_analysis, &scope_model)
        .expect("slice should compute");

    let retained_text = slice
        .retained_elements
        .iter()
        .map(|e| e.kind.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(retained_text.contains("x"), "x must be retained");
    assert!(retained_text.contains("result"), "result must be retained");
    assert!(
        retained_text.contains("outer")
            || retained_text.contains("if_stmt")
            || retained_text.contains("if_condition"),
        "outer control dependency must be retained"
    );
    assert!(
        retained_text.contains("inner") || retained_text.contains("if_condition"),
        "inner control dependency must be retained"
    );
}
