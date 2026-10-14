//! The primary R parser interface wrapping Tree-sitter.

use crate::cst::CstTree;
use crate::LANGUAGE;
use rparse_core::{RParseError, Result, SourceFile};

/// The primary R parser for R-PARSE.
///
/// Wraps the underlying Tree-sitter R grammar and provides safe, panic-free parsing
/// into a [`CstTree`].
pub struct RParser {
    inner: tree_sitter::Parser,
}

impl RParser {
    /// Initializes a new R parser instance configured with the Tree-sitter R grammar.
    ///
    /// # Errors
    /// Returns an [`RParseError::Parser`] if loading the Tree-sitter grammar fails.
    pub fn new() -> Result<Self> {
        let mut inner = tree_sitter::Parser::new();
        inner.set_language(&LANGUAGE.into()).map_err(|e| {
            RParseError::parser(format!("Failed to initialize Tree-sitter R grammar: {e}"))
        })?;

        Ok(Self { inner })
    }

    /// Parses UTF-8 R source code into a [`CstTree`].
    ///
    /// # Invariants
    /// - Never panics on malformed input.
    /// - Never panics on valid R syntax.
    /// - Returns a valid [`CstTree`] with syntax recovery nodes if errors are present.
    pub fn parse(&mut self, source: impl Into<String>) -> Result<CstTree> {
        let source_str = source.into();
        let tree = self.inner.parse(&source_str, None).ok_or_else(|| {
            RParseError::parser("Tree-sitter parser failed to produce a parse tree")
        })?;

        Ok(CstTree::new(tree, source_str))
    }

    /// Parses a [`SourceFile`] into a [`CstTree`].
    pub fn parse_source_file(&mut self, file: &SourceFile) -> Result<CstTree> {
        self.parse(file.content())
    }
}

impl Default for RParser {
    fn default() -> Self {
        Self::new().expect("Tree-sitter R grammar must initialize successfully")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = RParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_simple_parse() {
        let mut parser = RParser::new().unwrap();
        let cst = parser.parse("x <- 10").unwrap();
        assert!(!cst.has_errors());
        assert_eq!(cst.root_node().kind(), "program");
    }
}
