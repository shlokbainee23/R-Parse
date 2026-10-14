//! Standard forward reaching-definitions dataflow analysis over the CFG.
//!
//! Computes for each basic block $B$:
//! - $IN[B] = \bigcup_{P \in pred(B)} OUT[P]$
//! - $OUT[B] = GEN[B] \cup (IN[B] \setminus KILL[B])$
//!
//! Solved to fixed point using a worklist algorithm.

use crate::def_use::{DefUseInfo, VarDefId, VarUseId};
use rparse_cfg::{BlockId, Cfg};
use rparse_scope::SymbolId;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Results of forward reaching-definitions analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct ReachingDefinitions {
    in_sets: BTreeMap<BlockId, BTreeSet<VarDefId>>,
    out_sets: BTreeMap<BlockId, BTreeSet<VarDefId>>,
    gen_sets: BTreeMap<BlockId, BTreeSet<VarDefId>>,
    kill_sets: BTreeMap<BlockId, BTreeSet<VarDefId>>,
    use_reaching_defs: BTreeMap<VarUseId, BTreeSet<VarDefId>>,
}

impl ReachingDefinitions {
    /// Reaching definitions at the entry of basic block `block_id`.
    pub fn in_set(&self, block_id: BlockId) -> &BTreeSet<VarDefId> {
        static EMPTY: BTreeSet<VarDefId> = BTreeSet::new();
        self.in_sets.get(&block_id).unwrap_or(&EMPTY)
    }

    /// Reaching definitions at the exit of basic block `block_id`.
    pub fn out_set(&self, block_id: BlockId) -> &BTreeSet<VarDefId> {
        static EMPTY: BTreeSet<VarDefId> = BTreeSet::new();
        self.out_sets.get(&block_id).unwrap_or(&EMPTY)
    }

    /// Definitions generated in basic block `block_id` that reach the end of the block.
    pub fn gen_set(&self, block_id: BlockId) -> &BTreeSet<VarDefId> {
        static EMPTY: BTreeSet<VarDefId> = BTreeSet::new();
        self.gen_sets.get(&block_id).unwrap_or(&EMPTY)
    }

    /// Definitions killed by basic block `block_id`.
    pub fn kill_set(&self, block_id: BlockId) -> &BTreeSet<VarDefId> {
        static EMPTY: BTreeSet<VarDefId> = BTreeSet::new();
        self.kill_sets.get(&block_id).unwrap_or(&EMPTY)
    }

    /// Definitions reaching a specific variable use site.
    pub fn reaching_defs_for_use(&self, use_id: VarUseId) -> &BTreeSet<VarDefId> {
        static EMPTY: BTreeSet<VarDefId> = BTreeSet::new();
        self.use_reaching_defs.get(&use_id).unwrap_or(&EMPTY)
    }

    /// Returns whether the symbol read by `use_id` is guaranteed to have a definition along
    /// all execution paths from CFG entry reaching its use site (i.e. not potentially undefined).
    pub fn is_fully_defined(&self, use_id: VarUseId, cfg: &Cfg, def_use: &DefUseInfo) -> bool {
        let u = match def_use.get_use(use_id) {
            Some(u) => u,
            None => return false,
        };
        let sym_id = match u.symbol_id {
            Some(s) => s,
            None => return false,
        };

        // All blocks that define this symbol
        let def_blocks: BTreeSet<BlockId> = def_use
            .defs_for_symbol(sym_id)
            .iter()
            .map(|&d| def_use.get_def(d).unwrap().block_id)
            .collect();

        if def_blocks.is_empty() {
            return false;
        }

        // Test if entry can reach u.block_id without passing through any defining block
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        if !def_blocks.contains(&cfg.entry_id) {
            queue.push_back(cfg.entry_id);
            visited.insert(cfg.entry_id);
        }

        while let Some(curr) = queue.pop_front() {
            if curr == u.block_id {
                // An undefined path reaches the use block!
                return false;
            }

            for succ in cfg.successors(curr) {
                // If successor defines the symbol, that path is covered
                if !def_blocks.contains(&succ) && visited.insert(succ) {
                    queue.push_back(succ);
                }
            }
        }

        true
    }
}

/// Computes forward reaching definitions for all blocks and uses.
pub fn compute_reaching_definitions(cfg: &Cfg, def_use: &DefUseInfo) -> ReachingDefinitions {
    let mut gen_sets: BTreeMap<BlockId, BTreeSet<VarDefId>> = BTreeMap::new();
    let mut kill_sets: BTreeMap<BlockId, BTreeSet<VarDefId>> = BTreeMap::new();

    // 1. Compute GEN and KILL for each basic block
    for block in cfg.blocks() {
        let def_ids = def_use.defs_in_block(block.id);
        let mut gen = BTreeSet::new();
        let mut kill = BTreeSet::new();

        // Find the last definition of each symbol in this block
        let mut last_defs_in_block: BTreeMap<SymbolId, VarDefId> = BTreeMap::new();
        for &def_id in def_ids {
            let def = def_use.get_def(def_id).expect("def must exist");
            last_defs_in_block.insert(def.symbol_id, def_id);
        }

        for (&sym_id, &def_id) in &last_defs_in_block {
            gen.insert(def_id);

            // Kill all other definitions of this symbol across the program
            for &other_def_id in def_use.defs_for_symbol(sym_id) {
                if other_def_id != def_id {
                    kill.insert(other_def_id);
                }
            }
        }

        gen_sets.insert(block.id, gen);
        kill_sets.insert(block.id, kill);
    }

    // 2. Iterative fixed-point worklist algorithm
    let mut in_sets: BTreeMap<BlockId, BTreeSet<VarDefId>> = BTreeMap::new();
    let mut out_sets: BTreeMap<BlockId, BTreeSet<VarDefId>> = BTreeMap::new();

    for block in cfg.blocks() {
        in_sets.insert(block.id, BTreeSet::new());
        out_sets.insert(block.id, gen_sets[&block.id].clone());
    }

    let mut worklist: VecDeque<BlockId> = VecDeque::new();
    for block in cfg.blocks() {
        worklist.push_back(block.id);
    }

    while let Some(block_id) = worklist.pop_front() {
        // IN[B] = union of OUT[P] for every predecessor P
        let mut new_in = BTreeSet::new();
        for pred in cfg.predecessors(block_id) {
            if let Some(pred_out) = out_sets.get(&pred) {
                for &d in pred_out {
                    new_in.insert(d);
                }
            }
        }

        // OUT[B] = GEN[B] union (IN[B] - KILL[B])
        let gen = &gen_sets[&block_id];
        let kill = &kill_sets[&block_id];

        let mut new_out = gen.clone();
        for &d in &new_in {
            if !kill.contains(&d) {
                new_out.insert(d);
            }
        }

        let out_changed = out_sets.get(&block_id) != Some(&new_out);
        in_sets.insert(block_id, new_in);

        if out_changed {
            out_sets.insert(block_id, new_out);
            for succ in cfg.successors(block_id) {
                if !worklist.contains(&succ) {
                    worklist.push_back(succ);
                }
            }
        }
    }

    // 3. Compute reaching definitions for each variable use
    // Uses inside a statement block see the definitions reaching the entry of that block (IN set).
    let mut use_reaching_defs: BTreeMap<VarUseId, BTreeSet<VarDefId>> = BTreeMap::new();

    for u in def_use.uses() {
        let mut reaching = BTreeSet::new();
        if let Some(sym_id) = u.symbol_id {
            if let Some(in_set) = in_sets.get(&u.block_id) {
                for &d in in_set {
                    let def = def_use.get_def(d).unwrap();
                    if def.symbol_id == sym_id {
                        reaching.insert(d);
                    }
                }
            }
        }

        use_reaching_defs.insert(u.id, reaching);
    }

    ReachingDefinitions {
        in_sets,
        out_sets,
        gen_sets,
        kill_sets,
        use_reaching_defs,
    }
}
