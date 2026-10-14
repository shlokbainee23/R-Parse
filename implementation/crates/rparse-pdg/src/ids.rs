//! Strongly typed identifiers for the Unified Program Dependence Graph.

use std::fmt;

/// Unique identifier for a node in the Unified Program Dependence Graph (PDG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdgNodeId(pub u32);

impl fmt::Display for PdgNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pdg_node#{}", self.0)
    }
}

/// Unique identifier for a directed dependency edge in the Unified PDG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdgEdgeId(pub u32);

impl fmt::Display for PdgEdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pdg_edge#{}", self.0)
    }
}
