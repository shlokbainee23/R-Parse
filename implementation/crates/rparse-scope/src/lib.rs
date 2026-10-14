//! # rparse-scope
//!
//! Lexical scope and symbol analysis for R-PARSE.
//!
//! Provides:
//! - [`Symbol`], [`Definition`], and [`Reference`] data structures.
//! - Scope trees modeling [`ScopeKind::Global`], [`ScopeKind::Function`], and [`ScopeKind::DataMask`].
//! - Local → Enclosing → Global lexical symbol resolution.
//! - Identification of data-masking transformation contexts (`mutate`, `filter`, etc.)
//!   and explicit pronouns (`.data`, `.env`).

pub mod resolve;
pub mod scope;
pub mod symbol;

pub use resolve::resolve_scopes;
pub use scope::{Scope, ScopeKind, ScopeModel};
pub use symbol::{
    DefId, DefKind, Definition, RefId, RefKind, Reference, ScopeId, Symbol, SymbolId,
};
