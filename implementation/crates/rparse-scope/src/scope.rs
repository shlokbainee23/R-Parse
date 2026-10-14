//! Lexical and data-mask scope models for R-PARSE.
//!
//! Provides hierarchical scope trees supporting:
//! - Local → Enclosing → Global lexical name resolution.
//! - Distinctions between Global, Function, and DataMask scopes.
//! - Collection of symbols, definitions, and references across all scopes.

use crate::symbol::{DefId, Definition, RefId, Reference, ScopeId, Symbol, SymbolId};
use rparse_core::NodeId;
use std::collections::HashMap;

/// The kind of scope in the hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    /// The root global environment.
    Global,
    /// A lexical function or closure scope.
    Function { node_id: NodeId },
    /// A data-masking transformation scope created by verbs like `mutate`, `filter`, etc.
    DataMask { verb: String, call_node_id: NodeId },
}

/// A scope node in the environment tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
    pub kind: ScopeKind,
    pub symbols: HashMap<String, SymbolId>,
    pub definitions: Vec<DefId>,
    pub references: Vec<RefId>,
}

impl Scope {
    pub fn new(id: ScopeId, parent: Option<ScopeId>, kind: ScopeKind) -> Self {
        Self {
            id,
            parent,
            children: Vec::new(),
            kind,
            symbols: HashMap::new(),
            definitions: Vec::new(),
            references: Vec::new(),
        }
    }

    pub fn is_global(&self) -> bool {
        matches!(self.kind, ScopeKind::Global)
    }

    pub fn is_function(&self) -> bool {
        matches!(self.kind, ScopeKind::Function { .. })
    }

    pub fn is_data_mask(&self) -> bool {
        matches!(self.kind, ScopeKind::DataMask { .. })
    }
}

/// The complete resolved symbol and scope model for a program.
#[derive(Debug, Clone, PartialEq)]
pub struct ScopeModel {
    pub(crate) scopes: Vec<Scope>,
    pub(crate) symbols: Vec<Symbol>,
    pub(crate) definitions: Vec<Definition>,
    pub(crate) references: Vec<Reference>,
}

impl ScopeModel {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            symbols: Vec::new(),
            definitions: Vec::new(),
            references: Vec::new(),
        }
    }

    /// Returns the root global scope ID (always `ScopeId(0)`).
    pub fn root_scope(&self) -> ScopeId {
        ScopeId(0)
    }

    /// Access a scope by its [`ScopeId`].
    pub fn scope(&self, id: ScopeId) -> &Scope {
        &self.scopes[id.0]
    }

    /// Mutable access to a scope by its [`ScopeId`].
    pub(crate) fn scope_mut(&mut self, id: ScopeId) -> &mut Scope {
        &mut self.scopes[id.0]
    }

    /// Access a symbol by its [`SymbolId`].
    pub fn symbol(&self, id: SymbolId) -> &Symbol {
        &self.symbols[id.0]
    }

    /// Mutable access to a symbol by its [`SymbolId`].
    pub(crate) fn symbol_mut(&mut self, id: SymbolId) -> &mut Symbol {
        &mut self.symbols[id.0]
    }

    /// Access a definition by its [`DefId`].
    pub fn definition(&self, id: DefId) -> &Definition {
        &self.definitions[id.0]
    }

    /// Access a reference by its [`RefId`].
    pub fn reference(&self, id: RefId) -> &Reference {
        &self.references[id.0]
    }

    /// Returns all scopes in the model.
    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }

    /// Returns all symbols in the model.
    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    /// Returns all definitions in the model.
    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    /// Returns all references in the model.
    pub fn references(&self) -> &[Reference] {
        &self.references
    }

    /// Resolves a symbol name by ascending the lexical scope chain:
    /// Local → Enclosing → Global.
    pub fn resolve_symbol(&self, start_scope: ScopeId, name: &str) -> Option<SymbolId> {
        let mut curr = Some(start_scope);
        while let Some(sid) = curr {
            let sc = self.scope(sid);
            if let Some(&sym_id) = sc.symbols.get(name) {
                return Some(sym_id);
            }
            curr = sc.parent;
        }
        None
    }

    /// Resolves a symbol strictly in enclosing lexical scopes (skipping any immediate DataMask scope).
    pub fn resolve_lexical(&self, start_scope: ScopeId, name: &str) -> Option<SymbolId> {
        let mut curr = Some(start_scope);
        while let Some(sid) = curr {
            let sc = self.scope(sid);
            if !sc.is_data_mask() {
                if let Some(&sym_id) = sc.symbols.get(name) {
                    return Some(sym_id);
                }
            }
            curr = sc.parent;
        }
        None
    }

    /// Returns all definitions associated with a specific symbol.
    pub fn get_definitions_for_symbol(&self, symbol_id: SymbolId) -> Vec<&Definition> {
        let sym = self.symbol(symbol_id);
        sym.defs.iter().map(|&d| self.definition(d)).collect()
    }

    /// Returns all references associated with a specific symbol.
    pub fn get_references_for_symbol(&self, symbol_id: SymbolId) -> Vec<&Reference> {
        let sym = self.symbol(symbol_id);
        sym.refs.iter().map(|&r| self.reference(r)).collect()
    }

    /// Returns all data-masking scopes identified in the program.
    pub fn find_data_mask_scopes(&self) -> Vec<&Scope> {
        self.scopes.iter().filter(|s| s.is_data_mask()).collect()
    }
}

impl Default for ScopeModel {
    fn default() -> Self {
        Self::new()
    }
}
