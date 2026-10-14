//! Stable, machine-readable JSON formatters for CLI commands.

use std::collections::BTreeMap;

use rparse_runtime::json::JsonValue;
use rparse_slicing::{RetentionState, SliceCriterion, SliceResult};

use crate::args::RPARSE_VERSION;
use crate::error::CliError;
use crate::format_text::format_criterion;
use crate::pipeline::AnalysisContext;
use crate::provenance::{ColumnProvenanceResult, ColumnRef};

/// Formats the result of `rparse analyze` as a JSON string.
pub fn format_analyze_json(ctx: &AnalysisContext) -> String {
    let mut root = BTreeMap::new();
    root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
    root.insert(
        "version".to_string(),
        JsonValue::String(RPARSE_VERSION.to_string()),
    );
    root.insert(
        "command".to_string(),
        JsonValue::String("analyze".to_string()),
    );
    let status_str = if ctx.has_errors() { "error" } else { "success" };
    root.insert(
        "status".to_string(),
        JsonValue::String(status_str.to_string()),
    );
    root.insert(
        "file".to_string(),
        JsonValue::String(ctx.file_path.to_string_lossy().to_string()),
    );
    root.insert("mode".to_string(), JsonValue::String(ctx.mode.to_string()));

    let col_port_count = ctx
        .pdg
        .nodes()
        .iter()
        .filter(|n| n.is_column_version())
        .count();
    let mut summary = BTreeMap::new();
    summary.insert(
        "statement_count".to_string(),
        JsonValue::Number(ctx.program.stmts.len() as f64),
    );
    summary.insert(
        "diagnostic_count".to_string(),
        JsonValue::Number(ctx.diagnostics.len() as f64),
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
        "column_port_count".to_string(),
        JsonValue::Number(col_port_count as f64),
    );
    root.insert("summary".to_string(), JsonValue::Object(summary));

    let mut diags_arr = Vec::new();
    for d in &ctx.diagnostics {
        let mut d_map = BTreeMap::new();
        d_map.insert(
            "severity".to_string(),
            JsonValue::String(d.severity.to_string()),
        );
        d_map.insert("message".to_string(), JsonValue::String(d.message.clone()));
        diags_arr.push(JsonValue::Object(d_map));
    }
    root.insert("diagnostics".to_string(), JsonValue::Array(diags_arr));

    let mut dfs_arr = Vec::new();
    for state in ctx.df_analysis.all_states() {
        let mut df_map = BTreeMap::new();
        df_map.insert(
            "symbol".to_string(),
            JsonValue::String(state.symbol_name.clone()),
        );
        df_map.insert(
            "version".to_string(),
            JsonValue::Number(state.version as f64),
        );
        df_map.insert(
            "knowledge".to_string(),
            JsonValue::String(state.knowledge.to_string()),
        );
        let cols_arr: Vec<JsonValue> = state
            .schema
            .keys()
            .map(|k| JsonValue::String(k.clone()))
            .collect();
        df_map.insert("columns".to_string(), JsonValue::Array(cols_arr));
        dfs_arr.push(JsonValue::Object(df_map));
    }
    root.insert("dataframes".to_string(), JsonValue::Array(dfs_arr));

    JsonValue::Object(root).to_json_string()
}

/// Formats the result of `rparse provenance` as a JSON string.
pub fn format_provenance_json(res: &ColumnProvenanceResult, ctx: &AnalysisContext) -> String {
    let mut root = BTreeMap::new();
    root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
    root.insert(
        "version".to_string(),
        JsonValue::String(RPARSE_VERSION.to_string()),
    );
    root.insert(
        "command".to_string(),
        JsonValue::String("provenance".to_string()),
    );
    root.insert(
        "status".to_string(),
        JsonValue::String("success".to_string()),
    );
    root.insert(
        "file".to_string(),
        JsonValue::String(ctx.file_path.to_string_lossy().to_string()),
    );

    root.insert("target".to_string(), format_col_ref_json(&res.target));
    root.insert("is_unknown".to_string(), JsonValue::Bool(res.is_unknown));

    let direct_arr: Vec<JsonValue> = res.direct_inputs.iter().map(format_col_ref_json).collect();
    root.insert("direct_inputs".to_string(), JsonValue::Array(direct_arr));

    let trans_arr: Vec<JsonValue> = res
        .transitive_inputs
        .iter()
        .map(format_col_ref_json)
        .collect();
    root.insert("transitive_inputs".to_string(), JsonValue::Array(trans_arr));

    let mut steps_arr = Vec::new();
    for step in &res.derivation_steps {
        let mut step_map = BTreeMap::new();
        step_map.insert(
            "transformation".to_string(),
            JsonValue::String(step.transformation.clone()),
        );
        step_map.insert("output".to_string(), format_col_ref_json(&step.output));
        let in_arr: Vec<JsonValue> = step.inputs.iter().map(format_col_ref_json).collect();
        step_map.insert("inputs".to_string(), JsonValue::Array(in_arr));
        steps_arr.push(JsonValue::Object(step_map));
    }
    root.insert("derivation_steps".to_string(), JsonValue::Array(steps_arr));

    JsonValue::Object(root).to_json_string()
}

/// Formats the result of `rparse slice` as a JSON string.
pub fn format_slice_json(
    res: &SliceResult,
    criterion: &SliceCriterion,
    ctx: &AnalysisContext,
) -> String {
    let mut root = BTreeMap::new();
    root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
    root.insert(
        "version".to_string(),
        JsonValue::String(RPARSE_VERSION.to_string()),
    );
    root.insert(
        "command".to_string(),
        JsonValue::String("slice".to_string()),
    );
    root.insert(
        "status".to_string(),
        JsonValue::String("success".to_string()),
    );
    root.insert(
        "file".to_string(),
        JsonValue::String(ctx.file_path.to_string_lossy().to_string()),
    );
    root.insert(
        "criterion".to_string(),
        JsonValue::String(format_criterion(criterion)),
    );
    root.insert(
        "confidence".to_string(),
        JsonValue::String(res.confidence.to_string()),
    );
    root.insert(
        "contains_unknowns".to_string(),
        JsonValue::Bool(res.contains_unknowns),
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

    root.insert(
        "retained_element_count".to_string(),
        JsonValue::Number(res.retained_elements.len() as f64),
    );
    root.insert(
        "fully_retained_statement_count".to_string(),
        JsonValue::Number(fully_retained as f64),
    );
    root.insert(
        "partially_retained_statement_count".to_string(),
        JsonValue::Number(partially_retained as f64),
    );

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
        let state_str = match stmt_slice.retention {
            RetentionState::FullyRetained => "fully_retained",
            RetentionState::PartiallyRetained => "partially_retained",
            RetentionState::NotRetained => "not_retained",
        };
        s_map.insert(
            "state".to_string(),
            JsonValue::String(state_str.to_string()),
        );
        s_map.insert("span".to_string(), stmt_slice.span.to_json());

        if !stmt_slice.sub_elements.is_empty() {
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
        }

        stmts_arr.push(JsonValue::Object(s_map));
    }
    root.insert("statements".to_string(), JsonValue::Array(stmts_arr));

    JsonValue::Object(root).to_json_string()
}

/// Formats a CLI error as a JSON string.
pub fn format_error_json(command: &str, file: Option<&str>, err: &CliError) -> String {
    let mut root = BTreeMap::new();
    root.insert("tool".to_string(), JsonValue::String("rparse".to_string()));
    root.insert(
        "version".to_string(),
        JsonValue::String(RPARSE_VERSION.to_string()),
    );
    root.insert(
        "command".to_string(),
        JsonValue::String(command.to_string()),
    );
    root.insert("status".to_string(), JsonValue::String("error".to_string()));
    if let Some(f) = file {
        root.insert("file".to_string(), JsonValue::String(f.to_string()));
    }

    let mut err_map = BTreeMap::new();
    err_map.insert("message".to_string(), JsonValue::String(err.to_string()));
    err_map.insert(
        "exit_code".to_string(),
        JsonValue::Number(err.exit_code() as f64),
    );
    root.insert("error".to_string(), JsonValue::Object(err_map));

    JsonValue::Object(root).to_json_string()
}

fn format_col_ref_json(col: &ColumnRef) -> JsonValue {
    let mut map = BTreeMap::new();
    map.insert(
        "dataframe".to_string(),
        JsonValue::String(col.dataframe.clone()),
    );
    map.insert("column".to_string(), JsonValue::String(col.column.clone()));
    map.insert("version".to_string(), JsonValue::Number(col.version as f64));
    JsonValue::Object(map)
}
