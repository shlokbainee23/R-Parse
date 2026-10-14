//! # rparse-core
//!
//! Foundational types, identifiers, source spans, diagnostics, and errors for R-PARSE.
//!
//! This crate contains only pure foundational abstractions. It does NOT depend on Tree-sitter,
//! dataframes, CFG, provenance, slicing, or the GNU R runtime.

pub mod diagnostic;
pub mod error;
pub mod json;
pub mod node_id;
pub mod source;
pub mod span;

// Re-exports of key types
pub use diagnostic::{Diagnostic, DiagnosticSeverity};
pub use error::{RParseError, Result};
pub use json::JsonValue;
pub use node_id::{NodeId, NodeIdGenerator};
pub use source::{SourceFile, SourceId};
pub use span::{Position, Span};
