//! Variable dataflow, reaching definitions, post-dominance, and control dependencies for R-PARSE.
//!
//! Provides Phase 3 dataflow foundations:
//! - Definition and Use analysis over Semantic IR and resolved scopes.
//! - Standard forward reaching-definitions analysis with fixed-point iteration.
//! - Graph-based post-dominator analysis over CFGs.
//! - Control dependency computation based on reverse dominance frontiers.

pub mod control_dependence;
pub mod def_use;
pub mod post_dominance;
pub mod reaching_defs;

pub use control_dependence::{
    compute_control_dependencies, ControlDependencies, ControlDependency,
};
pub use def_use::{collect_def_use, DefUseInfo, VarDef, VarDefId, VarUse, VarUseId};
pub use post_dominance::{compute_post_dominators, PostDominators};
pub use reaching_defs::{compute_reaching_definitions, ReachingDefinitions};
