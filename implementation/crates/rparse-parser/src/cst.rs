//! Concrete Syntax Tree (CST) wrapper abstraction.
//!
//! Encapsulates Tree-sitter AST/CST types (`Tree`, `Node`, `TreeCursor`) behind
//! safe, idiomatic Rust types to avoid leaking raw Tree-sitter types across the system.

use crate::span_ext::span_from_ts_node;
use rparse_core::{Diagnostic, DiagnosticSeverity, Span};
use std::fmt;

/// An immutable wrapper around a Tree-sitter CST node.
#[derive(Clone, Copy)]
pub struct CstNode<'a> {
    inner: tree_sitter::Node<'a>,
}

impl<'a> CstNode<'a> {
    /// Creates a new `CstNode` wrapping a raw Tree-sitter node.
    pub(crate) const fn new(inner: tree_sitter::Node<'a>) -> Self {
        Self { inner }
    }

    /// Returns the node type/kind name (e.g. "program", "call", "identifier", "ERROR").
    #[inline]
    pub fn kind(&self) -> &'static str {
        self.inner.kind()
    }

    /// Returns the source [`Span`] corresponding to this node.
    #[inline]
    pub fn span(&self) -> Span {
        span_from_ts_node(&self.inner)
    }

    /// Returns true if this is a named syntax rule node (as opposed to anonymous syntax tokens like commas or parens).
    #[inline]
    pub fn is_named(&self) -> bool {
        self.inner.is_named()
    }

    /// Returns true if this node represents a syntax error.
    #[inline]
    pub fn is_error(&self) -> bool {
        self.inner.is_error()
    }

    /// Returns true if this node was inserted by error recovery (a missing token).
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.inner.is_missing()
    }

    /// Returns true if this node or any of its descendants contains a syntax error.
    #[inline]
    pub fn has_error(&self) -> bool {
        self.inner.has_error()
    }

    /// Returns the total number of child nodes (both named and anonymous tokens).
    #[inline]
    pub fn child_count(&self) -> usize {
        self.inner.child_count()
    }

    /// Returns the number of named child nodes.
    #[inline]
    pub fn named_child_count(&self) -> usize {
        self.inner.named_child_count()
    }

    /// Returns the child node at index `index`.
    #[inline]
    pub fn child(&self, index: usize) -> Option<CstNode<'a>> {
        self.inner.child(index).map(CstNode::new)
    }

    /// Returns the named child node at index `index`.
    #[inline]
    pub fn named_child(&self, index: usize) -> Option<CstNode<'a>> {
        self.inner.named_child(index).map(CstNode::new)
    }

    /// Returns the child node associated with the given field name (e.g. "name", "value", "lhs").
    #[inline]
    pub fn child_by_field_name(&self, field_name: &str) -> Option<CstNode<'a>> {
        self.inner.child_by_field_name(field_name).map(CstNode::new)
    }

    /// Returns the field name of the child at the specified index, if one was defined in the grammar.
    #[inline]
    pub fn field_name_for_child(&self, index: usize) -> Option<&'static str> {
        self.inner.field_name_for_child(index as u32)
    }

    /// Returns an iterator over all child nodes.
    pub fn children(&self) -> impl Iterator<Item = CstNode<'a>> {
        let count = self.inner.child_count();
        let node = self.inner;
        (0..count).filter_map(move |i| node.child(i).map(CstNode::new))
    }

    /// Returns an iterator over named child nodes.
    pub fn named_children(&self) -> impl Iterator<Item = CstNode<'a>> {
        let count = self.inner.named_child_count();
        let node = self.inner;
        (0..count).filter_map(move |i| node.named_child(i).map(CstNode::new))
    }

    /// Returns the parent node if one exists.
    #[inline]
    pub fn parent(&self) -> Option<CstNode<'a>> {
        self.inner.parent().map(CstNode::new)
    }

    /// Returns the next sibling node.
    #[inline]
    pub fn next_sibling(&self) -> Option<CstNode<'a>> {
        self.inner.next_sibling().map(CstNode::new)
    }

    /// Returns the previous sibling node.
    #[inline]
    pub fn prev_sibling(&self) -> Option<CstNode<'a>> {
        self.inner.prev_sibling().map(CstNode::new)
    }

    /// Returns the next named sibling node.
    #[inline]
    pub fn next_named_sibling(&self) -> Option<CstNode<'a>> {
        self.inner.next_named_sibling().map(CstNode::new)
    }

    /// Returns the previous named sibling node.
    #[inline]
    pub fn prev_named_sibling(&self) -> Option<CstNode<'a>> {
        self.inner.prev_named_sibling().map(CstNode::new)
    }

    /// Extracts the slice of source text covered by this node.
    #[inline]
    pub fn text<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.span().slice(source)
    }

    /// Returns an S-expression representation of the syntax node for debugging and inspection.
    pub fn to_sexp(&self) -> String {
        self.inner.to_sexp()
    }
}

impl<'a> fmt::Debug for CstNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CstNode")
            .field("kind", &self.kind())
            .field("is_named", &self.is_named())
            .field("span", &self.span())
            .field("has_error", &self.has_error())
            .finish()
    }
}

/// A parsed Concrete Syntax Tree representing an R source file or snippet.
pub struct CstTree {
    tree: tree_sitter::Tree,
    source: String,
}

impl CstTree {
    /// Creates a new `CstTree` from a Tree-sitter tree and source code.
    pub(crate) fn new(tree: tree_sitter::Tree, source: String) -> Self {
        Self { tree, source }
    }

    /// Returns the root node of the CST.
    #[inline]
    pub fn root_node(&self) -> CstNode<'_> {
        CstNode::new(self.tree.root_node())
    }

    /// Returns the full UTF-8 source text.
    #[inline]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns true if any syntax errors or missing tokens exist in the CST.
    #[inline]
    pub fn has_errors(&self) -> bool {
        self.tree.root_node().has_error()
    }

    /// Scans the CST and gathers syntax error diagnostics.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        collect_diagnostics(self.root_node(), &self.source, &mut diags);
        diags
    }

    /// Finds the smallest named node enclosing the specified byte range.
    pub fn named_descendant_for_byte_range(&self, start: u32, end: u32) -> Option<CstNode<'_>> {
        self.tree
            .root_node()
            .named_descendant_for_byte_range(start as usize, end as usize)
            .map(CstNode::new)
    }

    /// Finds the smallest node enclosing the specified byte range.
    pub fn descendant_for_byte_range(&self, start: u32, end: u32) -> Option<CstNode<'_>> {
        self.tree
            .root_node()
            .descendant_for_byte_range(start as usize, end as usize)
            .map(CstNode::new)
    }
}

impl fmt::Debug for CstTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CstTree")
            .field("root", &self.root_node())
            .field("has_errors", &self.has_errors())
            .field("source_bytes", &self.source.len())
            .finish()
    }
}

fn collect_diagnostics(node: CstNode<'_>, source: &str, out: &mut Vec<Diagnostic>) {
    if node.is_missing() {
        let span = node.span();
        let message = format!("Syntax error: missing expected token '{}'", node.kind());
        out.push(
            Diagnostic::new(DiagnosticSeverity::Error, message, Some(span))
                .with_code("MISSING_TOKEN"),
        );
    } else if node.is_error() {
        let span = node.span();
        let snippet = node.text(source).unwrap_or("<unknown>");
        let message = format!("Syntax error near '{}'", snippet);
        out.push(
            Diagnostic::new(DiagnosticSeverity::Error, message, Some(span))
                .with_code("SYNTAX_ERROR"),
        );
    }

    for child in node.children() {
        if child.has_error() {
            collect_diagnostics(child, source, out);
        }
    }
}
