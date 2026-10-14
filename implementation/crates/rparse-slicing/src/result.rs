//! Slice result model, statement slice fragments, and confidence metadata.

use crate::criterion::SliceCriterion;
use crate::element::{RetentionState, SliceElement, SubElementSlice};
use rparse_core::{NodeId, Span};
use rparse_dataframe::ColumnVersionId;
use rparse_pdg::{PdgNodeId, ProgramDependenceGraph};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Semantic confidence status of the computed slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SliceConfidence {
    /// Exact static slice with zero dynamic/unknown uncertainty in ancestry.
    Complete,
    /// Slice completed conservatively with UNKNOWN_STATIC dependencies encountered in closure.
    PartialContainsUnknowns,
}

impl fmt::Display for SliceConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => write!(f, "COMPLETE"),
            Self::PartialContainsUnknowns => write!(f, "PARTIAL_CONTAINS_UNKNOWNS"),
        }
    }
}

/// Slice analysis details for a single statement, capturing partial retention of sub-nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementSlice {
    /// Statement AST node identifier.
    pub stmt_id: NodeId,
    /// Source coordinates of the statement.
    pub span: Span,
    /// Overall retention status for this statement.
    pub retention: RetentionState,
    /// Retained semantic sub-nodes belonging to this statement.
    pub retained_subnodes: Vec<PdgNodeId>,
    /// Excluded semantic sub-nodes belonging to this statement.
    pub excluded_subnodes: Vec<PdgNodeId>,
    /// Retained sub-statement source regions.
    pub retained_sub_regions: Vec<Span>,
    /// Excluded sub-statement source regions.
    pub excluded_sub_regions: Vec<Span>,
    /// Attributed sub-statement elements (e.g. named arguments).
    pub sub_elements: Vec<SubElementSlice>,
}

/// First-class slice result capturing retained nodes, partial statement retention,
/// and confidence metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceResult {
    /// The slicing criterion analyzed.
    pub criterion: SliceCriterion,
    /// Retained PDG node identifiers in deterministic order.
    pub retained_nodes: BTreeSet<PdgNodeId>,
    /// Retained fine-grained slice elements.
    pub retained_elements: Vec<SliceElement>,
    /// Statement-level retention mapping (`StmtId -> StatementSlice`).
    pub statement_slices: BTreeMap<NodeId, StatementSlice>,
    /// DataFrame column versions explicitly retained in the slice.
    pub retained_column_versions: BTreeSet<ColumnVersionId>,
    /// DataFrame column versions in participating dataframes explicitly excluded.
    pub excluded_column_versions: BTreeSet<ColumnVersionId>,
    /// Whether any static uncertainty (`UNKNOWN_STATIC`) participates in the slice.
    pub contains_unknowns: bool,
    /// Semantic confidence rating.
    pub confidence: SliceConfidence,
}

impl SliceResult {
    pub fn is_node_retained(&self, id: PdgNodeId) -> bool {
        self.retained_nodes.contains(&id)
    }

    pub fn is_column_retained(&self, col_id: ColumnVersionId) -> bool {
        self.retained_column_versions.contains(&col_id)
    }

    pub fn is_column_excluded(&self, col_id: ColumnVersionId) -> bool {
        self.excluded_column_versions.contains(&col_id)
    }

    pub fn is_stmt_retained(&self, stmt_id: NodeId) -> bool {
        self.statement_slices
            .get(&stmt_id)
            .map(|s| s.retention != RetentionState::NotRetained)
            .unwrap_or(false)
    }

    pub fn is_stmt_partially_retained(&self, stmt_id: NodeId) -> bool {
        self.statement_slices
            .get(&stmt_id)
            .map(|s| s.retention == RetentionState::PartiallyRetained)
            .unwrap_or(false)
    }

    pub fn stmt_retention(&self, stmt_id: NodeId) -> RetentionState {
        self.statement_slices
            .get(&stmt_id)
            .map(|s| s.retention)
            .unwrap_or(RetentionState::NotRetained)
    }

    /// Generates a deterministic textual summary of the slice result.
    pub fn format_report(&self, pdg: &ProgramDependenceGraph) -> String {
        let mut out = String::new();
        out.push_str(&format!("Slice Criterion: {:?}\n", self.criterion));
        out.push_str(&format!("Confidence: {}\n", self.confidence));
        out.push_str(&format!("Contains Unknowns: {}\n", self.contains_unknowns));
        out.push_str(&format!(
            "Retained Nodes Count: {}\n",
            self.retained_nodes.len()
        ));
        out.push_str(&format!(
            "Retained Columns Count: {}\n",
            self.retained_column_versions.len()
        ));
        out.push_str(&format!(
            "Excluded Columns Count: {}\n",
            self.excluded_column_versions.len()
        ));

        out.push_str("\n--- Retained Columns ---\n");
        for &cid in &self.retained_column_versions {
            if let Some(port) = pdg.column_port(cid) {
                if let Some(node) = pdg.node(port) {
                    out.push_str(&format!("  + {}\n", node));
                }
            }
        }

        out.push_str("\n--- Excluded Columns ---\n");
        for &cid in &self.excluded_column_versions {
            if let Some(port) = pdg.column_port(cid) {
                if let Some(node) = pdg.node(port) {
                    out.push_str(&format!("  - {}\n", node));
                }
            }
        }

        out.push_str("\n--- Statement Retention ---\n");
        for (stmt_id, st_slice) in &self.statement_slices {
            out.push_str(&format!(
                "  Stmt {} [{}]: retained_subnodes={}, excluded_subnodes={}, retained_sub_regions={}, excluded_sub_regions={}\n",
                stmt_id,
                st_slice.retention,
                st_slice.retained_subnodes.len(),
                st_slice.excluded_subnodes.len(),
                st_slice.retained_sub_regions.len(),
                st_slice.excluded_sub_regions.len(),
            ));
        }

        out
    }
}
