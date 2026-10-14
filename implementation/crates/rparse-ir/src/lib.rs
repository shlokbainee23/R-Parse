//! # rparse-ir
//!
//! Normalized Semantic Intermediate Representation (IR) for R-PARSE.
//!
//! Provides a strongly-typed, desugared representation of R programs stripped
//! of lexical trivia while maintaining exact source spans on every node.

pub mod ast;
pub mod golden;
pub mod normalize;

pub use ast::*;
pub use golden::to_golden_string;
pub use normalize::normalize_cst;

/// Parses an R source code string into a normalized Semantic IR [`Program`].
pub fn parse_source(source: &str) -> Result<Program, rparse_core::RParseError> {
    let mut parser = rparse_parser::RParser::new()?;
    let cst = parser.parse(source)?;
    Ok(normalize_cst(&cst))
}
