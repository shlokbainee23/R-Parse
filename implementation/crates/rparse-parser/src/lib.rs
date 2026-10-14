//! # rparse-parser
//!
//! Production R parser and Concrete Syntax Tree (CST) wrapper for R-PARSE.
//!
//! Integrates the Tree-sitter R grammar with safe, idiomatic Rust wrappers.
//! The parser output is strictly CST-oriented and guarantees:
//! - Zero panics on valid or malformed R input.
//! - Accurate source spans across single-line, multiline, and UTF-8 constructs.
//! - Rich error recovery reporting via diagnostics.

pub mod cst;
pub mod parser;
pub mod span_ext;

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_r() -> *const ();
}

/// The Tree-sitter [`LanguageFn`] for R.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_r) };

// Re-exports for consumers
pub use cst::{CstNode, CstTree};
pub use parser::RParser;
pub use span_ext::{position_from_ts_point, span_from_ts_node};
