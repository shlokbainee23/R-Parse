//! DataFrame semantic layer, column versioning, and fine-grained column provenance for R-PARSE.
//!
//! Provides Phase 4 foundations:
//! - Immutable versioned [`DataFrameState`].
//! - Fine-grained [`ColumnVersion`] model with explicit [`StaticKnowledge`].
//! - Expression dependency extraction distinguishing column vs lexical variable dependencies.
//! - First-class [`ProvenanceEdge`] derivation events and [`ProvenanceGraph`].
//! - Row-selection dependency tracking for `filter` distinct from CFG control flow.
//! - Frozen MVP transformation handlers (`mutate`, `transmute`, `select`, `rename`, `filter`,
//!   `df$col <-`, `df[, c(...)]`, `transform`).

pub mod analysis;
pub mod column;
pub mod dependency;
pub mod expression;
pub mod ids;
pub mod operations;
pub mod provenance;
pub mod state;
pub mod unknown;

pub use analysis::{analyze_dataframes, DataFrameAnalysis, DataFrameSeed};
pub use column::ColumnVersion;
pub use dependency::{ExpressionDependencies, RowSelectionDependency};
pub use expression::extract_expression_dependencies;
pub use ids::{ColumnVersionId, DataFrameId, ProvenanceEdgeId};
pub use provenance::{
    ColumnLabelProvider, ProvenanceEdge, ProvenanceEdgeInit, ProvenanceGraph, TransformationType,
};
pub use state::DataFrameState;
pub use unknown::StaticKnowledge;
