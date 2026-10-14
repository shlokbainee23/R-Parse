//! Edge representations in the Unified Program Dependence Graph.

use crate::ids::{PdgEdgeId, PdgNodeId};
use rparse_cfg::EdgeKind;
use rparse_dataframe::{ProvenanceEdgeId, StaticKnowledge, TransformationType};
use std::fmt;

/// Semantic classification of a directed dependency edge in the PDG.
/// Direction is strictly: `SOURCE -> DEPENDENT` (backward slicing traverses incoming edges).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdgEdgeKind {
    /// Scalar variable def-use chain or use-to-statement dependency.
    VariableDataDependency,
    /// Fine-grained column derivation event linking input column(s) to output column.
    ColumnDerivation {
        provenance_edge_id: Option<ProvenanceEdgeId>,
        transform_type: TransformationType,
        knowledge: StaticKnowledge,
    },
    /// Statement-to-column binding linking the defining statement/expression to the column version.
    ColumnStatementBinding,
    /// Classical control dependency linking branch/loop predicate to controlled statement.
    ControlDependency { edge_kind: EdgeKind },
    /// Row-selection dependency linking predicate column/statement -> RowSelection -> output columns.
    RowSelectionDependency { knowledge: StaticKnowledge },
    /// Explicit static uncertainty edge linking an unknown origin to an affected node.
    UnknownDependency { reason: String },
}

/// A directed dependency edge in the Unified Program Dependence Graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdgEdge {
    /// Deterministic identifier for this edge.
    pub id: PdgEdgeId,
    /// Source of the dependency.
    pub from: PdgNodeId,
    /// Dependent target of the dependency.
    pub to: PdgNodeId,
    /// Detailed classification and metadata.
    pub kind: PdgEdgeKind,
}

impl PdgEdge {
    pub fn new(id: PdgEdgeId, from: PdgNodeId, to: PdgNodeId, kind: PdgEdgeKind) -> Self {
        Self { id, from, to, kind }
    }

    pub fn is_variable_data(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::VariableDataDependency)
    }

    pub fn is_column_derivation(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::ColumnDerivation { .. })
    }

    pub fn is_column_statement_binding(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::ColumnStatementBinding)
    }

    pub fn is_control(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::ControlDependency { .. })
    }

    pub fn is_row_selection(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::RowSelectionDependency { .. })
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self.kind, PdgEdgeKind::UnknownDependency { .. })
    }
}

impl fmt::Display for PdgEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} -> {} [{:?}]",
            self.id, self.from, self.to, self.kind
        )
    }
}
