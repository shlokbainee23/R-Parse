//! Slicing criteria model and resolution against analysis models.

use rparse_dataframe::{ColumnVersionId, DataFrameAnalysis, DataFrameId};
use rparse_pdg::{PdgNodeId, ProgramDependenceGraph};
use rparse_scope::{ScopeId, ScopeModel, SymbolId};
use std::fmt;

/// Strongly typed slicing criterion specifying what result to analyze.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SliceCriterion {
    /// Backward slice for a scalar variable by resolved [`SymbolId`].
    ScalarVariable(SymbolId),
    /// Backward slice for an entire dataframe object at a specific [`DataFrameId`].
    WholeDataFrame(DataFrameId),
    /// Backward slice for a specific column version by [`ColumnVersionId`].
    Column(ColumnVersionId),
    /// Backward slice for an individual column specified by dataframe symbol name and column name.
    ColumnByName {
        dataframe_symbol: String,
        column_name: String,
    },
    /// Backward slice for a whole dataframe specified by symbol name.
    WholeDataFrameByName { dataframe_symbol: String },
    /// Backward slice for a scalar variable specified by name.
    ScalarByName { symbol_name: String },
}

/// Error returned when a slicing criterion cannot be resolved against analysis models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CriterionResolutionError {
    SymbolNotFound(String),
    DataFrameNotFound(String),
    DataFrameIdNotFound(DataFrameId),
    ColumnNotFound { df_name: String, col_name: String },
    ColumnVersionNotFound(ColumnVersionId),
    NoMatchingPdgNode(String),
}

impl fmt::Display for CriterionResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolNotFound(name) => write!(f, "symbol '{}' not found in scope model", name),
            Self::DataFrameNotFound(name) => {
                write!(f, "dataframe symbol '{}' not active in analysis", name)
            }
            Self::DataFrameIdNotFound(id) => write!(f, "dataframe ID {} not found", id),
            Self::ColumnNotFound { df_name, col_name } => {
                write!(
                    f,
                    "column '{}' not found in dataframe '{}'",
                    col_name, df_name
                )
            }
            Self::ColumnVersionNotFound(id) => write!(f, "column version {} not found in PDG", id),
            Self::NoMatchingPdgNode(reason) => write!(f, "no matching PDG node: {}", reason),
        }
    }
}

impl std::error::Error for CriterionResolutionError {}

/// Resolves a [`SliceCriterion`] into initial seed [`PdgNodeId`]s in the Unified PDG.
pub fn resolve_criterion(
    criterion: &SliceCriterion,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
) -> Result<Vec<PdgNodeId>, CriterionResolutionError> {
    match criterion {
        SliceCriterion::Column(col_id) => {
            let port = pdg
                .column_port(*col_id)
                .ok_or(CriterionResolutionError::ColumnVersionNotFound(*col_id))?;
            Ok(vec![port])
        }

        SliceCriterion::ColumnByName {
            dataframe_symbol,
            column_name,
        } => {
            let st =
                df_analysis
                    .active_state_for_symbol(dataframe_symbol)
                    .or_else(|| {
                        df_analysis.all_states().into_iter().rev().find(|s| {
                            s.symbol_name == *dataframe_symbol && s.has_column(column_name)
                        })
                    })
                    .ok_or_else(|| {
                        CriterionResolutionError::DataFrameNotFound(dataframe_symbol.clone())
                    })?;

            let col_id =
                st.column(column_name)
                    .ok_or_else(|| CriterionResolutionError::ColumnNotFound {
                        df_name: dataframe_symbol.clone(),
                        col_name: column_name.clone(),
                    })?;

            let port = pdg
                .column_port(col_id)
                .ok_or(CriterionResolutionError::ColumnVersionNotFound(col_id))?;

            let mut seeds = vec![port];

            // If the dataframe state was created by an assignment or transformation,
            // include the creation statement and its defined variable in seeds so the defining/aliasing statement is retained.
            if let Some(stmt_node) = pdg.stmt_node(st.creation_node) {
                seeds.push(stmt_node);
                for &edge_id in pdg.outgoing_edges(stmt_node) {
                    if let Some(edge) = pdg.edge(edge_id) {
                        if let Some(node) = pdg.node(edge.to) {
                            if node.is_variable_def() {
                                seeds.push(edge.to);
                            }
                        }
                    }
                }
            }

            // Also check for statements that read/extract this column version directly (e.g. res <- df2$target)
            for &edge_id in pdg.outgoing_edges(port) {
                if let Some(edge) = pdg.edge(edge_id) {
                    if edge.is_column_statement_binding() {
                        seeds.push(edge.to);
                    }
                }
            }

            Ok(seeds)
        }

        SliceCriterion::WholeDataFrame(df_id) => {
            let st = df_analysis
                .state(*df_id)
                .ok_or(CriterionResolutionError::DataFrameIdNotFound(*df_id))?;

            let mut seeds = Vec::new();
            for &col_id in st.schema.values() {
                if let Some(port) = pdg.column_port(col_id) {
                    seeds.push(port);
                }
            }
            if let Some(stmt_node) = pdg.stmt_node(st.creation_node) {
                seeds.push(stmt_node);
            }

            if seeds.is_empty() {
                return Err(CriterionResolutionError::NoMatchingPdgNode(format!(
                    "empty dataframe {}",
                    df_id
                )));
            }
            Ok(seeds)
        }

        SliceCriterion::WholeDataFrameByName { dataframe_symbol } => {
            let st = df_analysis
                .active_state_for_symbol(dataframe_symbol)
                .or_else(|| {
                    df_analysis
                        .all_states()
                        .into_iter()
                        .rev()
                        .find(|s| s.symbol_name == *dataframe_symbol)
                })
                .ok_or_else(|| {
                    CriterionResolutionError::DataFrameNotFound(dataframe_symbol.clone())
                })?;

            let mut seeds = Vec::new();
            for &col_id in st.schema.values() {
                if let Some(port) = pdg.column_port(col_id) {
                    seeds.push(port);
                }
            }
            if let Some(stmt_node) = pdg.stmt_node(st.creation_node) {
                seeds.push(stmt_node);
            }

            if seeds.is_empty() {
                return Err(CriterionResolutionError::NoMatchingPdgNode(format!(
                    "empty dataframe '{}'",
                    dataframe_symbol
                )));
            }
            Ok(seeds)
        }

        SliceCriterion::ScalarVariable(sym_id) => {
            let mut seeds = Vec::new();
            for node in pdg.nodes() {
                if node.symbol_id() == Some(*sym_id) {
                    seeds.push(node.id);
                }
            }

            if seeds.is_empty() {
                return Err(CriterionResolutionError::NoMatchingPdgNode(format!(
                    "no nodes for symbol {}",
                    sym_id
                )));
            }
            Ok(seeds)
        }

        SliceCriterion::ScalarByName { symbol_name } => {
            let sym_id_opt = scope_model.resolve_symbol(ScopeId(0), symbol_name);
            let mut seeds = Vec::new();

            for node in pdg.nodes() {
                if node.name() == Some(symbol_name.as_str()) {
                    seeds.push(node.id);
                } else if let Some(sym_id) = sym_id_opt {
                    if node.symbol_id() == Some(sym_id) {
                        seeds.push(node.id);
                    }
                }
            }

            if seeds.is_empty() {
                return Err(CriterionResolutionError::SymbolNotFound(
                    symbol_name.clone(),
                ));
            }
            Ok(seeds)
        }
    }
}
