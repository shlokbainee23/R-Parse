//! Control-Flow Graph (CFG) construction and invariant validation for R-PARSE.
//!
//! Models execution flow across basic blocks for normalized Semantic IR.

pub mod block;
pub mod builder;
pub mod error;
pub mod graph;

pub use block::{BasicBlock, BlockId, BlockTerminator, CfgEdge, EdgeKind};
pub use builder::{build_cfg, build_cfg_for_block};
pub use error::CfgError;
pub use graph::Cfg;
