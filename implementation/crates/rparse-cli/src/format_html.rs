//! Self-contained interactive HTML analysis workbench generator for R-PARSE.
//!
//! Generates a single, offline, self-contained HTML visualizer containing:
//! - View A: Column Provenance Explorer (interactive bipartite graph with grouped derivations)
//! - View B: Program Dependence Graph (PDG) Explorer (with RowSelection vs Control distinction)
//! - View C: Backward Slice Inspector (criterion selection, [KEEP]/[DROP] sub-statement attribution)
//! - View D: Dataframe Schema Evolution Inspector (transformation timeline and column diffs)
//! - Synchronized Source Code Viewer with character-accurate span highlighting
//! - Metadata Inspector with property details and cross-view navigation
//!
//! Security & Architecture:
//! - Zero external CDN or network dependencies (bundled vanilla CSS and JS).
//! - Input R source code is treated as untrusted text and safely escaped.
//! - Serialized analysis data is safely encoded into `<script type="application/json">`.
//! - Pure static presentation layer preserving 100% of underlying Rust engine semantics.

use std::collections::{BTreeMap, BTreeSet};

use rparse_core::JsonValue;
use rparse_slicing::{compute_backward_slice_with_source, RetentionState};

use crate::args::{GraphType, RPARSE_VERSION};
use crate::format_ast::build_ast_explorer_payload;
use crate::format_html_assets::{CSS_STYLES, HTML_BODY_MARKUP, JS_SCRIPTS};
use crate::parse_slice_criterion;
use crate::pipeline::AnalysisContext;

/// Formats the complete analysis context into a single self-contained HTML workbench.
pub fn format_html_report(
    ctx: &AnalysisContext,
    initial_criterion: Option<&str>,
    initial_column: Option<&str>,
    initial_graph: Option<GraphType>,
) -> String {
    let report_data = build_report_data(ctx, initial_criterion, initial_column, initial_graph);
    let json_str = report_data.to_json_string();
    let safe_json = safe_json_for_script(&json_str);

    render_html_template(
        &ctx.file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
        &safe_json,
    )
}

/// Escapes JSON so it can be safely embedded inside an HTML `<script>` tag.
pub fn safe_json_for_script(json: &str) -> String {
    json.replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

/// Generates the default HTML studio page using the showcase pipeline fixture.
pub fn build_default_studio_html() -> String {
    let source = include_str!("../../../fixtures/showcase_pipeline.R").to_string();
    if let Ok(ctx) = AnalysisContext::run_from_source(
        std::path::PathBuf::from("showcase_pipeline.R"),
        source,
        rparse_runtime::config::AnalysisMode::Static,
        vec![],
    ) {
        format_html_report(&ctx, None, None, None)
    } else {
        render_html_template("R-PARSE Studio", "{}")
    }
}

/// Builds the structured JSON dataset containing all analysis models.
pub fn build_report_data(
    ctx: &AnalysisContext,
    initial_criterion: Option<&str>,
    initial_column: Option<&str>,
    initial_graph: Option<GraphType>,
) -> JsonValue {
    let mut root = BTreeMap::new();

    // 1. Tool & File Metadata
    root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
    root.insert(
        "version".to_string(),
        JsonValue::String(RPARSE_VERSION.to_string()),
    );
    let file_display = if ctx.file_path.as_os_str() == "-" {
        "<stdin>".to_string()
    } else {
        ctx.file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    };
    root.insert("file_name".to_string(), JsonValue::String(file_display));
    root.insert(
        "mode".to_string(),
        JsonValue::String(format!("{:?}", ctx.mode).to_uppercase()),
    );

    // 2. Untrusted Source Code (split into lines)
    let source_content = ctx.source_file.content();
    root.insert(
        "source_code".to_string(),
        JsonValue::String(source_content.to_string()),
    );
    let lines: Vec<JsonValue> = source_content
        .lines()
        .map(|l| JsonValue::String(l.to_string()))
        .collect();
    root.insert("source_lines".to_string(), JsonValue::Array(lines));

    // 3. Summary Metrics
    let mut summary = BTreeMap::new();
    summary.insert(
        "statement_count".to_string(),
        JsonValue::Number(ctx.program.stmts.len() as f64),
    );
    summary.insert(
        "dataframe_state_count".to_string(),
        JsonValue::Number(ctx.df_analysis.all_states().len() as f64),
    );
    summary.insert(
        "column_version_count".to_string(),
        JsonValue::Number(ctx.df_analysis.registry.columns.len() as f64),
    );
    summary.insert(
        "pdg_node_count".to_string(),
        JsonValue::Number(ctx.pdg.nodes().len() as f64),
    );
    summary.insert(
        "pdg_edge_count".to_string(),
        JsonValue::Number(ctx.pdg.edges().len() as f64),
    );
    summary.insert(
        "derivation_count".to_string(),
        JsonValue::Number(ctx.df_analysis.provenance().edges().len() as f64),
    );
    summary.insert(
        "contains_unknowns".to_string(),
        JsonValue::Bool(!ctx.pdg.unknown_nodes().is_empty()),
    );
    root.insert("summary".to_string(), JsonValue::Object(summary));

    // 4. Graph Topology: Provenance & PDG
    root.insert(
        "provenance".to_string(),
        ctx.df_analysis.provenance_to_json_value(),
    );
    root.insert("pdg".to_string(), ctx.pdg.to_json_value());

    // 5. Dataframe Schema Evolution Across Transformations
    let schema_evolution = build_schema_evolution(ctx);
    root.insert(
        "schema_evolution".to_string(),
        JsonValue::Array(schema_evolution),
    );

    // 6. Precomputed Backward Slices for Available Criteria
    let slices = build_precomputed_slices(ctx, initial_criterion);
    root.insert("slices".to_string(), JsonValue::Object(slices));

    // 7. Interactive AST & CST Explorer Trees
    root.insert("ast_explorer".to_string(), build_ast_explorer_payload(ctx));

    // 8. Initial Selection & Navigation
    let mut init_sel = BTreeMap::new();
    let default_view = match initial_graph {
        Some(GraphType::Provenance) => "provenance",
        Some(GraphType::Pdg) => "pdg",
        Some(GraphType::Cfg) => "pdg",
        Some(GraphType::Ast) => "ast",
        None => {
            if initial_criterion.is_some() {
                "slice"
            } else {
                "provenance"
            }
        }
    };
    init_sel.insert(
        "view".to_string(),
        JsonValue::String(default_view.to_string()),
    );
    if let Some(crit) = initial_criterion {
        init_sel.insert("criterion".to_string(), JsonValue::String(crit.to_string()));
    }
    if let Some(col) = initial_column {
        init_sel.insert("column".to_string(), JsonValue::String(col.to_string()));
    }
    root.insert("initial_selection".to_string(), JsonValue::Object(init_sel));

    // 8. Compiler & Parser Diagnostics
    let mut diags_arr = Vec::new();
    for diag in &ctx.diagnostics {
        let mut d_map = BTreeMap::new();
        d_map.insert(
            "severity".to_string(),
            JsonValue::String(diag.severity.as_str().to_string()),
        );
        d_map.insert(
            "message".to_string(),
            JsonValue::String(diag.message.clone()),
        );
        if let Some(sp) = diag.span {
            d_map.insert("span".to_string(), sp.to_json());
        }
        if let Some(ref c) = diag.code {
            d_map.insert("code".to_string(), JsonValue::String(c.clone()));
        }
        diags_arr.push(JsonValue::Object(d_map));
    }
    root.insert("diagnostics".to_string(), JsonValue::Array(diags_arr));

    // 9. Built-in Sample Fixtures for Entry Screen exploration
    let mut examples = BTreeMap::new();
    examples.insert(
        "showcase_pipeline".to_string(),
        JsonValue::String(include_str!("../../../fixtures/showcase_pipeline.R").to_string()),
    );
    examples.insert(
        "filter_provenance".to_string(),
        JsonValue::String(include_str!("../../../fixtures/filter_provenance.R").to_string()),
    );
    examples.insert(
        "partial_retention".to_string(),
        JsonValue::String(include_str!("../../../fixtures/partial_retention.R").to_string()),
    );
    examples.insert(
        "uncertainty".to_string(),
        JsonValue::String(include_str!("../../../fixtures/uncertainty.R").to_string()),
    );
    examples.insert(
        "canonical".to_string(),
        JsonValue::String(include_str!("../../../fixtures/canonical.R").to_string()),
    );
    root.insert("examples".to_string(), JsonValue::Object(examples));

    JsonValue::Object(root)
}

/// Builds the schema evolution records comparing each dataframe state with its predecessor.
fn build_schema_evolution(ctx: &AnalysisContext) -> Vec<JsonValue> {
    let mut list = Vec::new();

    for state in ctx.df_analysis.all_states() {
        let mut st_map = BTreeMap::new();
        st_map.insert("state_id".to_string(), JsonValue::Number(state.id.0 as f64));
        st_map.insert(
            "version".to_string(),
            JsonValue::Number(state.version as f64),
        );
        st_map.insert(
            "symbol_name".to_string(),
            JsonValue::String(state.symbol_name.clone()),
        );
        st_map.insert(
            "creation_node".to_string(),
            JsonValue::Number(state.creation_node.raw() as f64),
        );
        st_map.insert("span".to_string(), state.span.to_json());
        st_map.insert(
            "parent_state".to_string(),
            state
                .parent_state
                .map(|p| JsonValue::Number(p.0 as f64))
                .unwrap_or(JsonValue::Null),
        );
        st_map.insert(
            "knowledge".to_string(),
            JsonValue::String(state.knowledge.to_string()),
        );

        // Active schema columns
        let mut cols_arr = Vec::new();
        for (col_name, &col_id) in &state.schema {
            let mut col_obj = BTreeMap::new();
            col_obj.insert("name".to_string(), JsonValue::String(col_name.clone()));
            col_obj.insert("col_id".to_string(), JsonValue::Number(col_id.0 as f64));
            col_obj.insert(
                "col_ver_id".to_string(),
                JsonValue::String(col_id.to_string()),
            );
            cols_arr.push(JsonValue::Object(col_obj));
        }
        st_map.insert("columns".to_string(), JsonValue::Array(cols_arr));

        // Schema difference relative to parent
        let parent = state
            .parent_state
            .and_then(|pid| ctx.df_analysis.state(pid));
        let mut diff_obj = BTreeMap::new();
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut retained = Vec::new();

        if let Some(p) = parent {
            for (col_name, &col_id) in &state.schema {
                if let Some(p_cid) = p.column(col_name) {
                    if p_cid == col_id {
                        retained.push(JsonValue::String(col_name.clone()));
                    } else {
                        let mut mod_obj = BTreeMap::new();
                        mod_obj.insert("name".to_string(), JsonValue::String(col_name.clone()));
                        mod_obj.insert("old_col_id".to_string(), JsonValue::Number(p_cid.0 as f64));
                        mod_obj
                            .insert("new_col_id".to_string(), JsonValue::Number(col_id.0 as f64));
                        modified.push(JsonValue::Object(mod_obj));
                    }
                } else {
                    added.push(JsonValue::String(col_name.clone()));
                }
            }
            for col_name in p.schema.keys() {
                if !state.has_column(col_name) {
                    removed.push(JsonValue::String(col_name.clone()));
                }
            }
        } else {
            // Root seed state: all columns are added
            for col_name in state.schema.keys() {
                added.push(JsonValue::String(col_name.clone()));
            }
        }

        diff_obj.insert("added".to_string(), JsonValue::Array(added));
        diff_obj.insert("removed".to_string(), JsonValue::Array(removed));
        diff_obj.insert("modified".to_string(), JsonValue::Array(modified));
        diff_obj.insert("retained".to_string(), JsonValue::Array(retained));
        st_map.insert("diff".to_string(), JsonValue::Object(diff_obj));

        // Row selections
        let mut row_sels = Vec::new();
        for rs in &state.row_selections {
            let mut rs_obj = BTreeMap::new();
            rs_obj.insert(
                "df_id".to_string(),
                JsonValue::Number(rs.dataframe_id.0 as f64),
            );
            rs_obj.insert(
                "predicate_node".to_string(),
                JsonValue::Number(rs.predicate_node.raw() as f64),
            );
            rs_obj.insert(
                "knowledge".to_string(),
                JsonValue::String(rs.knowledge.to_string()),
            );
            row_sels.push(JsonValue::Object(rs_obj));
        }
        st_map.insert("row_selections".to_string(), JsonValue::Array(row_sels));

        list.push(JsonValue::Object(st_map));
    }

    list
}

/// Precomputes backward slices for all active columns and dataframes in the program.
fn build_precomputed_slices(
    ctx: &AnalysisContext,
    initial_criterion: Option<&str>,
) -> BTreeMap<String, JsonValue> {
    let mut criteria_set = BTreeSet::new();

    // Collect all candidate criteria from active dataframe states
    for state in ctx.df_analysis.all_states() {
        criteria_set.insert(state.symbol_name.clone());
        for col_name in state.schema.keys() {
            criteria_set.insert(format!("{}${}", state.symbol_name, col_name));
        }
    }

    if let Some(c) = initial_criterion {
        criteria_set.insert(c.to_string());
    }

    let mut slices_map = BTreeMap::new();

    for crit_str in criteria_set {
        let criterion = parse_slice_criterion(&crit_str, ctx);
        if let Ok(res) = compute_backward_slice_with_source(
            &criterion,
            &ctx.pdg,
            &ctx.df_analysis,
            &ctx.scope_model,
            Some(&ctx.program),
            Some(ctx.source_file.content()),
        ) {
            let mut sl_obj = BTreeMap::new();
            sl_obj.insert("criterion".to_string(), JsonValue::String(crit_str.clone()));
            sl_obj.insert(
                "confidence".to_string(),
                JsonValue::String(res.confidence.to_string()),
            );
            sl_obj.insert(
                "contains_unknowns".to_string(),
                JsonValue::Bool(res.contains_unknowns),
            );
            sl_obj.insert(
                "retained_element_count".to_string(),
                JsonValue::Number(res.retained_elements.len() as f64),
            );

            let fully_retained = res
                .statement_slices
                .values()
                .filter(|s| s.retention == RetentionState::FullyRetained)
                .count();
            let partially_retained = res
                .statement_slices
                .values()
                .filter(|s| s.retention == RetentionState::PartiallyRetained)
                .count();

            sl_obj.insert(
                "fully_retained_statement_count".to_string(),
                JsonValue::Number(fully_retained as f64),
            );
            sl_obj.insert(
                "partially_retained_statement_count".to_string(),
                JsonValue::Number(partially_retained as f64),
            );

            // Statements
            let mut stmts_arr = Vec::new();
            for stmt_slice in res.statement_slices.values() {
                let mut s_map = BTreeMap::new();
                s_map.insert(
                    "start_line".to_string(),
                    JsonValue::Number(stmt_slice.span.start_line as f64),
                );
                s_map.insert(
                    "end_line".to_string(),
                    JsonValue::Number(stmt_slice.span.end_line as f64),
                );
                s_map.insert("span".to_string(), stmt_slice.span.to_json());

                let state_str = match stmt_slice.retention {
                    RetentionState::FullyRetained => "fully_retained",
                    RetentionState::PartiallyRetained => "partially_retained",
                    RetentionState::NotRetained => "not_retained",
                };
                s_map.insert(
                    "state".to_string(),
                    JsonValue::String(state_str.to_string()),
                );

                let mut subs_arr = Vec::new();
                for sub in &stmt_slice.sub_elements {
                    let mut sub_map = BTreeMap::new();
                    sub_map.insert("label".to_string(), JsonValue::String(sub.label.clone()));
                    let sub_state_str = match sub.retention {
                        RetentionState::FullyRetained => "retained",
                        RetentionState::PartiallyRetained => "partially_retained",
                        RetentionState::NotRetained => "excluded",
                    };
                    sub_map.insert(
                        "state".to_string(),
                        JsonValue::String(sub_state_str.to_string()),
                    );
                    if let Some(sp) = sub.span {
                        sub_map.insert("span".to_string(), sp.to_json());
                    }
                    subs_arr.push(JsonValue::Object(sub_map));
                }
                s_map.insert("sub_elements".to_string(), JsonValue::Array(subs_arr));

                stmts_arr.push(JsonValue::Object(s_map));
            }
            sl_obj.insert("statements".to_string(), JsonValue::Array(stmts_arr));

            // Retained column versions
            let ret_cols: Vec<JsonValue> = res
                .retained_column_versions
                .iter()
                .map(|c| JsonValue::Number(c.0 as f64))
                .collect();
            sl_obj.insert(
                "retained_column_versions".to_string(),
                JsonValue::Array(ret_cols),
            );

            let excl_cols: Vec<JsonValue> = res
                .excluded_column_versions
                .iter()
                .map(|c| JsonValue::Number(c.0 as f64))
                .collect();
            sl_obj.insert(
                "excluded_column_versions".to_string(),
                JsonValue::Array(excl_cols),
            );

            slices_map.insert(crit_str, JsonValue::Object(sl_obj));
        }
    }

    slices_map
}

/// Renders the self-contained HTML page embedding CSS, JavaScript, and serialized JSON.
pub fn render_html_template(title_file: &str, safe_json_data: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>R-PARSE Workbench — {title_file}</title>
  <style>
{CSS_STYLES}
  </style>
</head>
<body>
{HTML_BODY_MARKUP}
  <!-- Safely Embedded Report Dataset -->
  <script type="application/json" id="rparse-report-data">
{safe_json_data}
  </script>
  <!-- Visualizer Application Logic -->
  <script>
{JS_SCRIPTS}
  </script>
</body>
</html>
"#
    )
}
