//! Fine-grained column provenance extraction and lineage resolution.

use std::collections::{BTreeSet, VecDeque};

use rparse_dataframe::{ColumnVersionId, StaticKnowledge};

use crate::error::CliError;
use crate::pipeline::AnalysisContext;

/// Reference to a versioned dataframe column.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnRef {
    pub dataframe: String,
    pub column: String,
    pub version: u32,
}

impl ColumnRef {
    pub fn display_name(&self) -> String {
        format!("{}${}", self.dataframe, self.column)
    }
}

/// A single column derivation step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationStep {
    pub output: ColumnRef,
    pub inputs: Vec<ColumnRef>,
    pub transformation: String,
}

/// Complete provenance lineage result for a queried column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnProvenanceResult {
    pub target: ColumnRef,
    pub direct_inputs: Vec<ColumnRef>,
    pub transitive_inputs: Vec<ColumnRef>,
    pub derivation_steps: Vec<DerivationStep>,
    pub is_unknown: bool,
}

/// Resolves and traces the column provenance for a target column string (e.g. "df$total" or "total").
pub fn trace_column_provenance(
    column_str: &str,
    ctx: &AnalysisContext,
) -> Result<ColumnProvenanceResult, CliError> {
    let (df_name, col_name) = parse_column_spec(column_str, ctx)?;

    // Locate the dataframe state and target ColumnVersionId
    let state = ctx
        .df_analysis
        .active_state_for_symbol(&df_name)
        .or_else(|| {
            ctx.df_analysis
                .all_states()
                .into_iter()
                .rev()
                .find(|s| s.symbol_name == df_name && s.has_column(&col_name))
        })
        .ok_or_else(|| {
            CliError::ResolutionFailed(format!("Dataframe '{}' not found in analysis", df_name))
        })?;

    let target_col_id = state.column(&col_name).ok_or_else(|| {
        CliError::ResolutionFailed(format!(
            "Column '{}' not found in dataframe '{}'",
            col_name, df_name
        ))
    })?;

    let prov_graph = ctx.df_analysis.provenance();
    let target_ver = ctx
        .df_analysis
        .column_version(target_col_id)
        .ok_or_else(|| CliError::ResolutionFailed("Target column version not found".to_string()))?;

    let target_ref = ColumnRef {
        dataframe: state.symbol_name.clone(),
        column: target_ver.name.clone(),
        version: state.version,
    };

    let mut direct_inputs = Vec::new();
    let mut derivation_steps = Vec::new();
    let mut is_unknown = target_ver.knowledge == StaticKnowledge::UnknownStatic;

    let mut visited_cols = BTreeSet::new();
    let mut leaf_inputs = BTreeSet::new();
    let mut worklist = VecDeque::new();

    // Check direct edge
    if let Some(edge_id) = target_ver.provenance_edge {
        if let Some(edge) = prov_graph.edge(edge_id) {
            if edge.knowledge == StaticKnowledge::UnknownStatic {
                is_unknown = true;
            }
            let mut inputs_for_step = Vec::new();
            for &in_id in &edge.input_columns {
                if let Some(in_ref) = resolve_col_ref(in_id, ctx) {
                    inputs_for_step.push(in_ref.clone());
                    direct_inputs.push(in_ref);
                    worklist.push_back(in_id);
                }
            }
            derivation_steps.push(DerivationStep {
                output: target_ref.clone(),
                inputs: inputs_for_step,
                transformation: format!("{:?}", edge.transform_type),
            });
        }
    }

    visited_cols.insert(target_col_id);

    // Transitive traversal
    while let Some(current_id) = worklist.pop_front() {
        if !visited_cols.insert(current_id) {
            continue;
        }

        if let Some(current_ver) = ctx.df_analysis.column_version(current_id) {
            if current_ver.knowledge == StaticKnowledge::UnknownStatic {
                is_unknown = true;
            }
            if let Some(edge_id) = current_ver.provenance_edge {
                if let Some(edge) = prov_graph.edge(edge_id) {
                    if edge.knowledge == StaticKnowledge::UnknownStatic {
                        is_unknown = true;
                    }
                    if let Some(curr_ref) = resolve_col_ref(current_id, ctx) {
                        let mut step_inputs = Vec::new();
                        for &in_id in &edge.input_columns {
                            if let Some(in_ref) = resolve_col_ref(in_id, ctx) {
                                step_inputs.push(in_ref);
                                worklist.push_back(in_id);
                            }
                        }
                        derivation_steps.push(DerivationStep {
                            output: curr_ref,
                            inputs: step_inputs,
                            transformation: format!("{:?}", edge.transform_type),
                        });
                    }
                }
            } else {
                // Leaf column version with no parents
                if let Some(leaf_ref) = resolve_col_ref(current_id, ctx) {
                    leaf_inputs.insert(leaf_ref);
                }
            }
        }
    }

    let transitive_inputs: Vec<ColumnRef> = leaf_inputs.into_iter().collect();

    Ok(ColumnProvenanceResult {
        target: target_ref,
        direct_inputs,
        transitive_inputs,
        derivation_steps,
        is_unknown,
    })
}

fn resolve_col_ref(col_id: ColumnVersionId, ctx: &AnalysisContext) -> Option<ColumnRef> {
    let ver = ctx.df_analysis.column_version(col_id)?;
    let state = ctx.df_analysis.state(ver.dataframe_id)?;
    Some(ColumnRef {
        dataframe: state.symbol_name.clone(),
        column: ver.name.clone(),
        version: state.version,
    })
}

fn parse_column_spec(spec: &str, ctx: &AnalysisContext) -> Result<(String, String), CliError> {
    if let Some((df, col)) = spec.split_once('$') {
        let df_trimmed = df.trim();
        let col_trimmed = col.trim();
        if df_trimmed.is_empty() || col_trimmed.is_empty() {
            return Err(CliError::Usage(format!(
                "Invalid column syntax: '{}'",
                spec
            )));
        }
        Ok((df_trimmed.to_string(), col_trimmed.to_string()))
    } else {
        let col_trimmed = spec.trim();
        // Search active states for dataframe containing this column
        for &state_id in ctx.df_analysis.active_symbol_states.values() {
            if let Some(state) = ctx.df_analysis.state(state_id) {
                if state.has_column(col_trimmed) {
                    return Ok((state.symbol_name.clone(), col_trimmed.to_string()));
                }
            }
        }
        // Fallback search all states
        for state in ctx.df_analysis.all_states().into_iter().rev() {
            if state.has_column(col_trimmed) {
                return Ok((state.symbol_name.clone(), col_trimmed.to_string()));
            }
        }
        Err(CliError::ResolutionFailed(format!(
            "Could not determine dataframe for column '{}'. Please specify as 'df$column'",
            col_trimmed
        )))
    }
}
