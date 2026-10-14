//! Human-readable text formatters for CLI commands.

use rparse_slicing::{RetentionState, SliceCriterion, SliceResult};

use crate::pipeline::AnalysisContext;
use crate::provenance::ColumnProvenanceResult;

/// Formats the result of `rparse analyze` as clean text.
pub fn format_analyze_text(ctx: &AnalysisContext) -> String {
    let mut out = String::new();
    out.push_str("R-PARSE STATIC ANALYSIS\n");
    out.push_str("───────────────────────\n");
    out.push_str(&format!("File:         {}\n", ctx.file_path.display()));
    let status_str = if ctx.has_errors() { "ERROR" } else { "SUCCESS" };
    out.push_str(&format!("Status:       {}\n", status_str));
    out.push_str(&format!("Mode:         {}\n", ctx.mode));
    out.push('\n');

    out.push_str("Program Structure:\n");
    out.push_str(&format!("  Statements:    {}\n", ctx.program.stmts.len()));
    out.push_str(&format!("  Diagnostics:   {}\n", ctx.diagnostics.len()));
    for diag in &ctx.diagnostics {
        out.push_str(&format!("    [{}] {}\n", diag.severity, diag.message));
    }
    out.push('\n');

    out.push_str("Dataframe Schema Lineage:\n");
    let all_states = ctx.df_analysis.all_states();
    if all_states.is_empty() {
        out.push_str("  (No dataframes discovered)\n");
    } else {
        for state in all_states {
            let cols: Vec<&String> = state.schema.keys().collect();
            out.push_str(&format!(
                "  {} (v{}) [{}]: [{}]\n",
                state.symbol_name,
                state.version,
                state.knowledge,
                cols.iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    out.push('\n');

    let col_port_count = ctx
        .pdg
        .nodes()
        .iter()
        .filter(|n| n.is_column_version())
        .count();
    out.push_str("Program Dependence Graph:\n");
    out.push_str(&format!("  Nodes:         {}\n", ctx.pdg.nodes().len()));
    out.push_str(&format!("  Edges:         {}\n", ctx.pdg.edges().len()));
    out.push_str(&format!("  Column Ports:  {}\n", col_port_count));

    if !ctx.runtime_evidence.is_empty() {
        out.push('\n');
        out.push_str("Runtime Evidence:\n");
        for ev in &ctx.runtime_evidence {
            if let Some(rec) = ev.as_observed() {
                out.push_str(&format!(
                    "  Observed '{}' ({} cols, {:?} rows, {} ms)\n",
                    rec.target_symbol,
                    rec.observed_columns.len(),
                    rec.row_count_estimate,
                    rec.execution_time_ms
                ));
            }
        }
    }

    out
}

/// Formats the result of `rparse provenance` as clean text.
pub fn format_provenance_text(res: &ColumnProvenanceResult) -> String {
    let mut out = String::new();
    out.push_str("R-PARSE COLUMN PROVENANCE\n");
    out.push_str("─────────────────────────\n");
    out.push_str(&format!(
        "Target Column:     {} (v{})\n",
        res.target.display_name(),
        res.target.version
    ));
    out.push_str(&format!(
        "Certainty State:   {}\n",
        if res.is_unknown {
            "UNKNOWN_STATIC"
        } else {
            "KNOWN_STATIC"
        }
    ));
    out.push('\n');

    out.push_str("Direct Input Columns:\n");
    if res.direct_inputs.is_empty() {
        out.push_str("  (None — root or unmodeled column)\n");
    } else {
        for input in &res.direct_inputs {
            out.push_str(&format!(
                "  ← {} (v{})\n",
                input.display_name(),
                input.version
            ));
        }
    }
    out.push('\n');

    out.push_str("Transitive Source Columns:\n");
    if res.transitive_inputs.is_empty() {
        out.push_str("  (None)\n");
    } else {
        for input in &res.transitive_inputs {
            out.push_str(&format!(
                "  ← {} (v{})\n",
                input.display_name(),
                input.version
            ));
        }
    }
    out.push('\n');

    out.push_str("Derivation Chain:\n");
    if res.derivation_steps.is_empty() {
        out.push_str("  (Direct declaration)\n");
    } else {
        for step in &res.derivation_steps {
            let in_names: Vec<String> = step.inputs.iter().map(|i| i.display_name()).collect();
            out.push_str(&format!(
                "  [{}] {} → {}\n",
                step.transformation,
                in_names.join(", "),
                step.output.display_name()
            ));
        }
    }

    out
}

/// Formats the result of `rparse slice` as clean text.
pub fn format_slice_text(
    res: &SliceResult,
    criterion: &SliceCriterion,
    ctx: &AnalysisContext,
) -> String {
    let mut out = String::new();
    out.push_str("R-PARSE BACKWARD STATIC SLICE\n");
    out.push_str("─────────────────────────────\n");
    out.push_str(&format!(
        "Criterion:         {}\n",
        format_criterion(criterion)
    ));
    out.push_str(&format!("Confidence:        {}\n", res.confidence));
    out.push_str(&format!(
        "Contains Unknowns: {}\n",
        if res.contains_unknowns { "YES" } else { "NO" }
    ));
    out.push('\n');

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

    out.push_str("Slice Metrics:\n");
    out.push_str(&format!(
        "  Retained Semantic Elements:  {}\n",
        res.retained_elements.len()
    ));
    out.push_str(&format!(
        "  Fully Retained Statements:    {}\n",
        fully_retained
    ));
    out.push_str(&format!(
        "  Partially Retained Statements:{}\n",
        partially_retained
    ));
    out.push('\n');

    out.push_str("Retained Program Statements:\n");
    let content = ctx.source_file.content();
    for stmt_slice in res.statement_slices.values() {
        let span = stmt_slice.span;
        let line_info = if span.start_line > 0 {
            format!("Line {}:", span.start_line)
        } else {
            "Line ?:".to_string()
        };

        let snippet = if (span.start_byte as usize) < content.len()
            && (span.end_byte as usize) <= content.len()
            && span.start_byte < span.end_byte
        {
            content[span.start_byte as usize..span.end_byte as usize]
                .lines()
                .next()
                .unwrap_or("")
                .trim()
        } else {
            ""
        };

        let state_badge = match stmt_slice.retention {
            RetentionState::FullyRetained => "[FULL]",
            RetentionState::PartiallyRetained => "[PARTIAL]",
            RetentionState::NotRetained => "[NONE]",
        };

        out.push_str(&format!("  {} {:<9} {}\n", state_badge, line_info, snippet));

        if stmt_slice.retention == RetentionState::PartiallyRetained {
            for sub in &stmt_slice.sub_elements {
                let sub_badge = match sub.retention {
                    RetentionState::FullyRetained => "[KEEP]",
                    RetentionState::PartiallyRetained => "[PARTIAL]",
                    RetentionState::NotRetained => "[DROP]",
                };
                out.push_str(&format!("    {} {}\n", sub_badge, sub.label));
            }
        }
    }

    out
}

pub fn format_criterion(crit: &SliceCriterion) -> String {
    match crit {
        SliceCriterion::ColumnByName {
            dataframe_symbol,
            column_name,
        } => format!("{}${}", dataframe_symbol, column_name),
        SliceCriterion::WholeDataFrameByName { dataframe_symbol } => dataframe_symbol.clone(),
        SliceCriterion::ScalarByName { symbol_name } => symbol_name.clone(),
        _ => format!("{:?}", crit),
    }
}
