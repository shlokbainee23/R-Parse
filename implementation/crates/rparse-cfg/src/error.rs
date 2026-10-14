//! Error types for control-flow graph construction and invariant checks.

use crate::block::BlockId;
use std::fmt;

/// An error encountered during CFG construction or invariant validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgError {
    /// Invariant failure with a descriptive message.
    InvariantViolation(String),
    /// Referenced block ID does not exist in the graph.
    BlockNotFound(BlockId),
    /// Malformed or unexpected semantic IR structure.
    MalformedIr(String),
}

impl fmt::Display for CfgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvariantViolation(msg) => write!(f, "CFG invariant violation: {msg}"),
            Self::BlockNotFound(id) => write!(f, "Block not found in CFG: {id}"),
            Self::MalformedIr(msg) => write!(f, "Malformed IR for CFG: {msg}"),
        }
    }
}

impl std::error::Error for CfgError {}
