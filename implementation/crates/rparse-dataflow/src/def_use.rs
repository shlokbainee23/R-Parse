//! Variable definitions and uses extraction for R-PARSE dataflow analysis.

use rparse_cfg::{BlockId, Cfg};
use rparse_core::{NodeId, Span};
use rparse_ir::Program;
use rparse_scope::{RefKind, ScopeModel, SymbolId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Unique identifier for a variable definition site in dataflow analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarDefId(pub u32);

impl fmt::Display for VarDefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vdef#{}", self.0)
    }
}

/// Unique identifier for a variable use site in dataflow analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarUseId(pub u32);

impl fmt::Display for VarUseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vuse#{}", self.0)
    }
}

/// A specific variable definition point where a symbol receives a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDef {
    pub id: VarDefId,
    pub symbol_id: SymbolId,
    pub name: String,
    pub node_id: NodeId,
    pub block_id: BlockId,
    pub span: Span,
}

/// A specific variable use point where a symbol is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarUse {
    pub id: VarUseId,
    /// The resolved symbol ID, or `None` if statically unresolved/uncertain.
    pub symbol_id: Option<SymbolId>,
    pub name: String,
    pub node_id: NodeId,
    pub block_id: BlockId,
    pub span: Span,
}

/// Collection of all variable definitions and uses across the program and CFG blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct DefUseInfo {
    defs: Vec<VarDef>,
    uses: Vec<VarUse>,
    block_defs: BTreeMap<BlockId, Vec<VarDefId>>,
    block_uses: BTreeMap<BlockId, Vec<VarUseId>>,
    sym_defs: BTreeMap<SymbolId, Vec<VarDefId>>,
    sym_uses: BTreeMap<SymbolId, Vec<VarUseId>>,
}

impl DefUseInfo {
    pub fn new() -> Self {
        Self {
            defs: Vec::new(),
            uses: Vec::new(),
            block_defs: BTreeMap::new(),
            block_uses: BTreeMap::new(),
            sym_defs: BTreeMap::new(),
            sym_uses: BTreeMap::new(),
        }
    }

    pub fn defs(&self) -> &[VarDef] {
        &self.defs
    }

    pub fn uses(&self) -> &[VarUse] {
        &self.uses
    }

    pub fn get_def(&self, id: VarDefId) -> Option<&VarDef> {
        self.defs.get(id.0 as usize)
    }

    pub fn get_use(&self, id: VarUseId) -> Option<&VarUse> {
        self.uses.get(id.0 as usize)
    }

    pub fn defs_in_block(&self, block_id: BlockId) -> &[VarDefId] {
        self.block_defs
            .get(&block_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn uses_in_block(&self, block_id: BlockId) -> &[VarUseId] {
        self.block_uses
            .get(&block_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn defs_for_symbol(&self, sym_id: SymbolId) -> &[VarDefId] {
        self.sym_defs
            .get(&sym_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn uses_for_symbol(&self, sym_id: SymbolId) -> &[VarUseId] {
        self.sym_uses
            .get(&sym_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn all_symbols_defined(&self) -> BTreeSet<SymbolId> {
        self.sym_defs.keys().copied().collect()
    }
}

impl Default for DefUseInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Collects variable definitions and uses by combining Semantic IR, resolved [`ScopeModel`], and the [`Cfg`].
pub fn collect_def_use(_program: &Program, scope_model: &ScopeModel, cfg: &Cfg) -> DefUseInfo {
    let mut info = DefUseInfo::new();

    // 1. Extract definitions from ScopeModel and map to CFG basic blocks
    for def in scope_model.definitions() {
        if let Some(block_id) = cfg.block_for_node(def.node_id) {
            let var_def_id = VarDefId(info.defs.len() as u32);
            let var_def = VarDef {
                id: var_def_id,
                symbol_id: def.symbol_id,
                name: def.name.clone(),
                node_id: def.node_id,
                block_id,
                span: def.span,
            };
            info.defs.push(var_def);
            info.block_defs
                .entry(block_id)
                .or_default()
                .push(var_def_id);
            info.sym_defs
                .entry(def.symbol_id)
                .or_default()
                .push(var_def_id);
        }
    }

    // 2. Extract uses from ScopeModel references that represent reads
    for r in scope_model.references() {
        // Skip assignment writes (LValue symbol targets)
        if r.kind == RefKind::Write {
            continue;
        }

        if let Some(block_id) = cfg.block_for_node(r.node_id) {
            let var_use_id = VarUseId(info.uses.len() as u32);
            let var_use = VarUse {
                id: var_use_id,
                symbol_id: r.symbol_id,
                name: r.name.clone(),
                node_id: r.node_id,
                block_id,
                span: r.span,
            };
            info.uses.push(var_use);
            info.block_uses
                .entry(block_id)
                .or_default()
                .push(var_use_id);
            if let Some(sym_id) = r.symbol_id {
                info.sym_uses.entry(sym_id).or_default().push(var_use_id);
            }
        }
    }

    info
}
