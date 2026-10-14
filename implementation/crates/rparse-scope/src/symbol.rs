//! Symbol, Definition, and Reference data structures for R-PARSE.
//!
//! Maintains strict separation between:
//! - [`Symbol`]: A declared or synthesized named entity associated with a scope.
//! - [`Definition`]: A specific site in source code where a symbol is introduced or assigned.
//! - [`Reference`]: A specific site in source code where an identifier is read or used.

use rparse_core::{NodeId, Span};
use rparse_ir::AssignmentKind;
use std::fmt;

/// Unique identifier for a symbol across the scope hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolId(pub usize);

impl fmt::Display for SymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sym#{}", self.0)
    }
}

/// Unique identifier for a definition site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefId(pub usize);

impl fmt::Display for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def#{}", self.0)
    }
}

/// Unique identifier for a reference site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefId(pub usize);

impl fmt::Display for RefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ref#{}", self.0)
    }
}

/// Unique identifier for a lexical or data-mask scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(pub usize);

impl fmt::Display for ScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scope#{}", self.0)
    }
}

/// A named entity within a specific scope.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: String,
    pub scope_id: ScopeId,
    pub span: Span,
    pub defs: Vec<DefId>,
    pub refs: Vec<RefId>,
}

/// The semantic classification of a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    /// Variable assignment (`<-`, `->`, `=`, `<<-`).
    Variable(AssignmentKind),
    /// Formal function parameter.
    Parameter,
    /// Loop iteration variable (e.g. `for (x in seq)`).
    LoopIterator,
}

/// A specific source code location where a symbol is defined or assigned.
#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub id: DefId,
    pub symbol_id: SymbolId,
    pub name: String,
    pub node_id: NodeId,
    pub scope_id: ScopeId,
    pub span: Span,
    pub kind: DefKind,
}

/// The semantic classification of a reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefKind {
    /// Ordinary lexical variable read.
    Read,
    /// Identifier in write/assignment context.
    Write,
    /// Invoked function symbol in a call expression.
    CallTarget,
    /// Bare identifier inside a data-masking transformation context
    /// (may represent either a dataframe column or a lexical variable).
    DataMaskBare,
    /// Explicit dataframe column pronoun reference (`.data$col` or `.data[["col"]]`).
    DataMaskExplicitData { column: String },
    /// Explicit lexical environment pronoun reference (`.env$var`).
    DataMaskExplicitEnv { variable: String },
}

/// A specific source code location where a symbol or column is referenced.
#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub id: RefId,
    pub name: String,
    pub node_id: NodeId,
    pub symbol_id: Option<SymbolId>,
    pub scope_id: ScopeId,
    pub span: Span,
    pub kind: RefKind,
}
