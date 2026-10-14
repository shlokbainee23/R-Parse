//! Unified Program Dependence Graph (PDG) for R-PARSE.
//!
//! Provides Phase 5 PDG representations and deterministic builder:
//! - [`ProgramDependenceGraph`]: Integrated graph of scalar dataflow, column provenance,
//!   control dependencies, row selections, and explicit uncertainties.
//! - Strongly typed [`PdgNode`] hierarchy exposing column ports (`ColumnVersion`),
//!   statements, expressions, definitions, uses, row selections, and unknown nodes.
//! - Directed dependency [`PdgEdge`] model (`SOURCE -> DEPENDENT`).
//! - Comprehensive invariant validation (`validate_pdg`).

pub mod builder;
pub mod edge;
pub mod graph;
pub mod ids;
pub mod node;

pub use builder::{build_pdg, PdgBuildInputs};
pub use edge::{PdgEdge, PdgEdgeKind};
pub use graph::{validate_pdg, PdgGraphInit, PdgValidationError, ProgramDependenceGraph};
pub use ids::{PdgEdgeId, PdgNodeId};
pub use node::{PdgNode, PdgNodeKind};
