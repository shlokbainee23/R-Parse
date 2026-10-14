//! Immutable, versioned dataframe semantic state model.

use crate::dependency::RowSelectionDependency;
use crate::ids::{ColumnVersionId, DataFrameId};
use crate::unknown::StaticKnowledge;
use rparse_core::{NodeId, Span};
use rparse_scope::SymbolId;
use std::collections::BTreeMap;
use std::fmt;

/// An immutable, versioned semantic state of a dataframe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFrameState {
    /// Unique identifier for this dataframe state.
    pub id: DataFrameId,
    /// Monotonic version index for this dataframe in its lineage.
    pub version: u32,
    /// The resolved symbol ID if bound to a variable symbol.
    pub symbol_id: Option<SymbolId>,
    /// The variable name bound to this dataframe.
    pub symbol_name: String,
    /// The active schema mapping `ColumnName -> ColumnVersionId`.
    pub schema: BTreeMap<String, ColumnVersionId>,
    /// The semantic IR node that created or transformed this state.
    pub creation_node: NodeId,
    /// Source coordinates of the creation or transformation.
    pub span: Span,
    /// Ancestor dataframe state ID, if derived from an earlier state.
    pub parent_state: Option<DataFrameId>,
    /// Accumulated row selection dependencies associated with this dataframe state.
    pub row_selections: Vec<RowSelectionDependency>,
    /// Static certainty status of this dataframe state.
    pub knowledge: StaticKnowledge,
}

/// Initialization parameters for constructing a [`DataFrameState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFrameStateInit {
    pub id: DataFrameId,
    pub version: u32,
    pub symbol_id: Option<SymbolId>,
    pub symbol_name: String,
    pub schema: BTreeMap<String, ColumnVersionId>,
    pub creation_node: NodeId,
    pub span: Span,
    pub parent_state: Option<DataFrameId>,
    pub row_selections: Vec<RowSelectionDependency>,
    pub knowledge: StaticKnowledge,
}

impl From<DataFrameStateInit> for DataFrameState {
    fn from(init: DataFrameStateInit) -> Self {
        Self {
            id: init.id,
            version: init.version,
            symbol_id: init.symbol_id,
            symbol_name: init.symbol_name,
            schema: init.schema,
            creation_node: init.creation_node,
            span: init.span,
            parent_state: init.parent_state,
            row_selections: init.row_selections,
            knowledge: init.knowledge,
        }
    }
}

impl DataFrameState {
    pub fn new(init: DataFrameStateInit) -> Self {
        init.into()
    }

    /// Looks up a column's current active version ID by name.
    pub fn column(&self, name: &str) -> Option<ColumnVersionId> {
        self.schema.get(name).copied()
    }

    /// Checks if a column exists in the current schema.
    pub fn has_column(&self, name: &str) -> bool {
        self.schema.contains_key(name)
    }

    /// Returns the active schema map.
    pub fn schema(&self) -> &BTreeMap<String, ColumnVersionId> {
        &self.schema
    }

    /// Returns the row-selection dependencies applied to this state.
    pub fn row_selections(&self) -> &[RowSelectionDependency] {
        &self.row_selections
    }
}

impl fmt::Display for DataFrameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@v{} ({}) cols={:?}",
            self.symbol_name,
            self.version,
            self.id,
            self.schema.keys().collect::<Vec<_>>()
        )
    }
}
