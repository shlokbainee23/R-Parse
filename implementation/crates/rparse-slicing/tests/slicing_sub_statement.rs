//! Tests for sub-statement slice attribution (Phase C Milestone 2).

use rparse_cfg::build_cfg;
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_dataframe::{analyze_dataframes, DataFrameSeed};
use rparse_ir::parse_source;
use rparse_pdg::{build_pdg, validate_pdg};
use rparse_scope::resolve_scopes;
use rparse_slicing::{compute_column_slice_with_source, RetentionState};

#[test]
fn test_sub_statement_attribution_spans_and_labels() {
    let code = r#"df2 <- df %>% mutate(total = price * quantity, unused = age * 2)"#;
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

    let slice = compute_column_slice_with_source(
        "df2",
        "total",
        &pdg,
        &df_analysis,
        &scope_model,
        Some(&program),
        Some(code),
    )
    .expect("slice computation should succeed");

    let mut found_partial = false;
    for stmt_slice in slice.statement_slices.values() {
        if stmt_slice.retention == RetentionState::PartiallyRetained {
            found_partial = true;
            assert_eq!(stmt_slice.sub_elements.len(), 2);

            let kept = &stmt_slice.sub_elements[0];
            assert_eq!(kept.label, "total = price * quantity");
            assert_eq!(kept.retention, RetentionState::FullyRetained);
            assert!(kept.span.is_some());
            let kept_span = kept.span.unwrap();
            let kept_snippet = &code[kept_span.start_byte as usize..kept_span.end_byte as usize];
            assert_eq!(kept_snippet, "total = price * quantity");

            let dropped = &stmt_slice.sub_elements[1];
            assert_eq!(dropped.label, "unused = age * 2");
            assert_eq!(dropped.retention, RetentionState::NotRetained);
            assert!(dropped.span.is_some());
            let dropped_span = dropped.span.unwrap();
            let dropped_snippet =
                &code[dropped_span.start_byte as usize..dropped_span.end_byte as usize];
            assert_eq!(dropped_snippet, "unused = age * 2");

            assert_eq!(stmt_slice.retained_sub_regions.len(), 1);
            assert_eq!(stmt_slice.retained_sub_regions[0], kept_span);

            assert_eq!(stmt_slice.excluded_sub_regions.len(), 1);
            assert_eq!(stmt_slice.excluded_sub_regions[0], dropped_span);
        }
    }
    assert!(found_partial, "PartiallyRetained statement must be present");
}

#[test]
fn test_sub_statement_backward_compatibility_without_source() {
    let code = r#"df2 <- df %>% mutate(total = price * quantity, unused = age * 2)"#;
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

    // Call without source or program (legacy backward compatibility)
    let slice = compute_column_slice_with_source(
        "df2",
        "total",
        &pdg,
        &df_analysis,
        &scope_model,
        None,
        None,
    )
    .expect("slice computation should succeed");

    for stmt_slice in slice.statement_slices.values() {
        if stmt_slice.retention == RetentionState::PartiallyRetained {
            // Slicing succeeds and attributes via PDG subnodes
            assert!(!stmt_slice.sub_elements.is_empty());
            assert!(!stmt_slice.retained_subnodes.is_empty());
            assert!(!stmt_slice.excluded_subnodes.is_empty());
            for elem in &stmt_slice.sub_elements {
                assert!(!elem.label.is_empty());
            }
        }
    }
}
