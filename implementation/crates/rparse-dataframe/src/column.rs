//! First-class immutable column version representation.

use crate::ids::{ColumnVersionId, DataFrameId, ProvenanceEdgeId};
use crate::unknown::StaticKnowledge;
use std::fmt;

/// A specific, immutable semantic version of a dataframe column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnVersion {
    /// Unique identifier for this column version.
    pub id: ColumnVersionId,
    /// The dataframe state in which this column version was introduced or assigned.
    pub dataframe_id: DataFrameId,
    /// Column name in the schema.
    pub name: String,
    /// Deterministic, monotonic version number for this column name in its lineage.
    pub version: u32,
    /// The provenance edge that produced this column version, if derived.
    pub provenance_edge: Option<ProvenanceEdgeId>,
    /// Static certainty status of this column.
    pub knowledge: StaticKnowledge,
}

impl ColumnVersion {
    pub fn new(
        id: ColumnVersionId,
        dataframe_id: DataFrameId,
        name: String,
        version: u32,
        provenance_edge: Option<ProvenanceEdgeId>,
        knowledge: StaticKnowledge,
    ) -> Self {
        Self {
            id,
            dataframe_id,
            name,
            version,
            provenance_edge,
            knowledge,
        }
    }
}

impl fmt::Display for ColumnVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@v{} ({})", self.name, self.version, self.id)
    }
}
