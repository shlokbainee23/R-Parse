//! Expression and row-selection dependencies.

use crate::ids::{ColumnVersionId, DataFrameId};
use crate::unknown::StaticKnowledge;
use rparse_core::{NodeId, Span};
use rparse_scope::SymbolId;
use std::collections::BTreeSet;

/// Dependencies extracted from an expression evaluated against an active dataframe schema.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExpressionDependencies {
    /// Columns in the active dataframe schema that this expression references.
    pub columns: BTreeSet<ColumnVersionId>,
    /// Lexical variables (scalar symbols) that this expression references.
    pub variables: BTreeSet<SymbolId>,
    /// Whether any part of the expression contains static uncertainty.
    pub unknown: bool,
}

impl ExpressionDependencies {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_column(&mut self, col: ColumnVersionId) {
        self.columns.insert(col);
    }

    pub fn add_variable(&mut self, var: SymbolId) {
        self.variables.insert(var);
    }

    pub fn set_unknown(&mut self) {
        self.unknown = true;
    }

    pub fn union_with(&mut self, other: &Self) {
        self.columns.extend(other.columns.iter().copied());
        self.variables.extend(other.variables.iter().copied());
        self.unknown |= other.unknown;
    }
}

/// A first-class row-selection dependency introduced by dataframe filtering (`filter`).
///
/// NOTE: This is NOT a CFG control dependency and does NOT represent standard column derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowSelectionDependency {
    /// The dataframe state produced by or subject to this filter.
    pub dataframe_id: DataFrameId,
    /// Columns referenced in the filtering predicate.
    pub predicate_columns: BTreeSet<ColumnVersionId>,
    /// Lexical variables referenced in the filtering predicate.
    pub predicate_variables: BTreeSet<SymbolId>,
    /// The semantic AST node of the filter predicate expression.
    pub predicate_node: NodeId,
    /// Source coordinates of the filter predicate.
    pub span: Span,
    /// Static certainty status of the predicate.
    pub knowledge: StaticKnowledge,
}
