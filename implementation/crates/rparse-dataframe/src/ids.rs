//! Deterministic identifiers for DataFrame states, column versions, and provenance edges.

use std::fmt;

/// Unique, deterministic identifier for a DataFrame state in the program analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataFrameId(pub u32);

impl fmt::Display for DataFrameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "df#{}", self.0)
    }
}

/// Unique, deterministic identifier for a specific immutable version of a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnVersionId(pub u32);

impl fmt::Display for ColumnVersionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "col_ver#{}", self.0)
    }
}

/// Unique, deterministic identifier for a provenance derivation edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceEdgeId(pub u32);

impl fmt::Display for ProvenanceEdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pedge#{}", self.0)
    }
}
