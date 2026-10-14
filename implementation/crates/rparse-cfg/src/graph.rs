//! Control-Flow Graph representation and invariant validation.

use crate::block::{BasicBlock, BlockId, BlockTerminator, CfgEdge, EdgeKind};
use crate::error::CfgError;
use rparse_core::NodeId;
use std::collections::{BTreeMap, HashSet, VecDeque};

/// A directed control-flow graph for an R program or function.
#[derive(Debug, Clone, PartialEq)]
pub struct Cfg {
    pub entry_id: BlockId,
    pub exit_id: BlockId,
    pub(crate) blocks: Vec<BasicBlock>,
    pub(crate) node_to_block: BTreeMap<NodeId, BlockId>,
}

impl Cfg {
    pub fn new(
        entry_id: BlockId,
        exit_id: BlockId,
        blocks: Vec<BasicBlock>,
        node_to_block: BTreeMap<NodeId, BlockId>,
    ) -> Self {
        Self {
            entry_id,
            exit_id,
            blocks,
            node_to_block,
        }
    }

    /// Finds the [`BlockId`] associated with a semantic IR node, if any.
    pub fn block_for_node(&self, id: NodeId) -> Option<BlockId> {
        self.node_to_block.get(&id).copied()
    }

    /// Access the mapping from IR [`NodeId`] to [`BlockId`].
    pub fn node_to_block(&self) -> &BTreeMap<NodeId, BlockId> {
        &self.node_to_block
    }

    /// Access the entry block of the CFG.
    pub fn entry_block(&self) -> &BasicBlock {
        self.block(self.entry_id)
            .expect("CFG entry block must exist")
    }

    /// Access the exit block of the CFG.
    pub fn exit_block(&self) -> &BasicBlock {
        self.block(self.exit_id).expect("CFG exit block must exist")
    }

    /// Access a block by its [`BlockId`].
    pub fn block(&self, id: BlockId) -> Option<&BasicBlock> {
        let idx = id.0 as usize;
        if idx < self.blocks.len() && self.blocks[idx].id == id {
            Some(&self.blocks[idx])
        } else {
            self.blocks.iter().find(|b| b.id == id)
        }
    }

    /// Mutable access to a block by its [`BlockId`].
    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut BasicBlock> {
        let idx = id.0 as usize;
        if idx < self.blocks.len() && self.blocks[idx].id == id {
            Some(&mut self.blocks[idx])
        } else {
            self.blocks.iter_mut().find(|b| b.id == id)
        }
    }

    /// Returns a slice of all basic blocks in deterministic index order.
    pub fn blocks(&self) -> &[BasicBlock] {
        &self.blocks
    }

    /// Returns the total number of blocks in the CFG.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Returns the successor block IDs of a block in deterministic order.
    pub fn successors(&self, id: BlockId) -> Vec<BlockId> {
        self.block(id)
            .map(|b| b.outgoing.iter().map(|e| e.to).collect())
            .unwrap_or_default()
    }

    /// Returns the predecessor block IDs of a block in deterministic order.
    pub fn predecessors(&self, id: BlockId) -> Vec<BlockId> {
        self.block(id)
            .map(|b| b.incoming.iter().map(|e| e.from).collect())
            .unwrap_or_default()
    }

    /// Returns all edges in the CFG in deterministic order.
    pub fn edges(&self) -> Vec<CfgEdge> {
        let mut edges = Vec::new();
        for block in &self.blocks {
            for edge in &block.outgoing {
                edges.push(edge.clone());
            }
        }
        edges
    }

    /// Computes the set of all blocks reachable from the entry point.
    pub fn reachable_blocks(&self) -> HashSet<BlockId> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.entry_id);
        reachable.insert(self.entry_id);

        while let Some(curr) = queue.pop_front() {
            for succ in self.successors(curr) {
                if reachable.insert(succ) {
                    queue.push_back(succ);
                }
            }
        }
        reachable
    }

    /// Validates that the CFG satisfies all mandatory invariants:
    /// 1. Exactly one entry point exists.
    /// 2. Exactly one exit point exists.
    /// 3. Every referenced block in edges exists.
    /// 4. No dangling edges (incoming and outgoing edges are consistent).
    /// 5. Every reachable non-entry block has at least one incoming edge.
    /// 6. Branching blocks preserve both true and false paths.
    /// 7. Return does not fall through to later statements.
    /// 8. Loops contain an explicit back edge.
    pub fn validate_invariants(&self) -> Result<(), CfgError> {
        // Invariant 1: Entry block exists
        if self.block(self.entry_id).is_none() {
            return Err(CfgError::InvariantViolation(format!(
                "Entry block {} does not exist",
                self.entry_id
            )));
        }

        // Invariant 2: Exit block exists
        if self.block(self.exit_id).is_none() {
            return Err(CfgError::InvariantViolation(format!(
                "Exit block {} does not exist",
                self.exit_id
            )));
        }

        let reachable = self.reachable_blocks();

        for block in &self.blocks {
            // Invariant 3 & 4: Outgoing and incoming consistency
            for out_edge in &block.outgoing {
                if self.block(out_edge.to).is_none() {
                    return Err(CfgError::InvariantViolation(format!(
                        "Outgoing edge from {} targets non-existent block {}",
                        block.id, out_edge.to
                    )));
                }
                // Verify target block has matching incoming edge
                let target = self.block(out_edge.to).unwrap();
                let has_incoming = target
                    .incoming
                    .iter()
                    .any(|in_edge| in_edge.from == block.id && in_edge.kind == out_edge.kind);
                if !has_incoming {
                    return Err(CfgError::InvariantViolation(format!(
                        "Dangling edge: {} -> {} ({:?}) missing in target incoming list",
                        block.id, out_edge.to, out_edge.kind
                    )));
                }
            }

            for in_edge in &block.incoming {
                if self.block(in_edge.from).is_none() {
                    return Err(CfgError::InvariantViolation(format!(
                        "Incoming edge to {} originates from non-existent block {}",
                        block.id, in_edge.from
                    )));
                }
            }

            // Invariant 5: Reachable non-entry block has incoming edge
            if reachable.contains(&block.id)
                && block.id != self.entry_id
                && block.incoming.is_empty()
            {
                return Err(CfgError::InvariantViolation(format!(
                    "Reachable non-entry block {} has no incoming edges",
                    block.id
                )));
            }

            // Invariant 6: Branching constructs preserve true and false paths
            if matches!(block.terminator, BlockTerminator::Branch { .. }) {
                let has_true = block
                    .outgoing
                    .iter()
                    .any(|e| e.kind == EdgeKind::TrueBranch);
                let has_false = block
                    .outgoing
                    .iter()
                    .any(|e| e.kind == EdgeKind::FalseBranch);
                if !has_true || !has_false {
                    return Err(CfgError::InvariantViolation(format!(
                        "Branch block {} missing true or false branch (has_true={}, has_false={})",
                        block.id, has_true, has_false
                    )));
                }
            }

            // Invariant 7: Return does not fall through
            if matches!(block.terminator, BlockTerminator::Return { .. }) {
                let has_normal = block.outgoing.iter().any(|e| e.kind == EdgeKind::Normal);
                if has_normal {
                    return Err(CfgError::InvariantViolation(format!(
                        "Return block {} has illegal normal fall-through edge",
                        block.id
                    )));
                }
            }

            // Invariant 8: Loop header must have a LoopBack incoming edge if reachable
            if matches!(block.terminator, BlockTerminator::LoopHeader { .. })
                && reachable.contains(&block.id)
            {
                let has_back = block.incoming.iter().any(|e| e.kind == EdgeKind::LoopBack);
                if !has_back {
                    return Err(CfgError::InvariantViolation(format!(
                        "Loop header block {} has no LoopBack incoming edge",
                        block.id
                    )));
                }
            }
        }

        Ok(())
    }

    /// Emits a deterministic Graphviz DOT string of the CFG.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph CFG {\n");
        dot.push_str("  node [shape=box, fontname=\"Courier\"];\n");

        for block in &self.blocks {
            let label = format!(
                "{}\\nstmts: {}\\nterm: {:?}",
                block.id,
                block.stmts.len(),
                block.terminator
            );
            dot.push_str(&format!("  {} [label=\"{}\"];\n", block.id, label));
        }

        for edge in self.edges() {
            let style = match edge.kind {
                EdgeKind::Normal => "",
                EdgeKind::TrueBranch => " [color=green, label=\"true\"]",
                EdgeKind::FalseBranch => " [color=red, label=\"false\"]",
                EdgeKind::LoopBack => " [color=blue, label=\"loop_back\", style=dashed]",
                EdgeKind::Return => " [color=purple, label=\"return\"]",
            };
            dot.push_str(&format!("  {} -> {}{};\n", edge.from, edge.to, style));
        }

        dot.push_str("}\n");
        dot
    }

    /// Emits a structured JSON value of the CFG topology.
    pub fn to_json_value(&self) -> rparse_core::JsonValue {
        use rparse_core::JsonValue;

        let mut root = BTreeMap::new();
        root.insert(
            "graph_type".to_string(),
            JsonValue::String("cfg".to_string()),
        );
        root.insert(
            "entry_id".to_string(),
            JsonValue::Number(self.entry_id.0 as f64),
        );
        root.insert(
            "exit_id".to_string(),
            JsonValue::Number(self.exit_id.0 as f64),
        );

        let mut nodes_arr = Vec::new();
        for block in &self.blocks {
            let mut b_map = BTreeMap::new();
            b_map.insert("id".to_string(), JsonValue::Number(block.id.0 as f64));
            b_map.insert(
                "label".to_string(),
                JsonValue::String(format!("{}", block.id)),
            );
            b_map.insert(
                "statement_count".to_string(),
                JsonValue::Number(block.stmts.len() as f64),
            );
            let stmts: Vec<JsonValue> = block
                .stmts
                .iter()
                .map(|s| JsonValue::Number(s.id().raw() as f64))
                .collect();
            b_map.insert("statements".to_string(), JsonValue::Array(stmts));
            b_map.insert(
                "terminator".to_string(),
                JsonValue::String(format!("{:?}", block.terminator)),
            );
            nodes_arr.push(JsonValue::Object(b_map));
        }
        root.insert("nodes".to_string(), JsonValue::Array(nodes_arr));

        let mut edges_arr = Vec::new();
        for edge in self.edges() {
            let mut e_map = BTreeMap::new();
            e_map.insert("from".to_string(), JsonValue::Number(edge.from.0 as f64));
            e_map.insert("to".to_string(), JsonValue::Number(edge.to.0 as f64));
            e_map.insert(
                "kind".to_string(),
                JsonValue::String(format!("{:?}", edge.kind)),
            );
            edges_arr.push(JsonValue::Object(e_map));
        }
        root.insert("edges".to_string(), JsonValue::Array(edges_arr));

        let mut summary = BTreeMap::new();
        summary.insert(
            "block_count".to_string(),
            JsonValue::Number(self.blocks.len() as f64),
        );
        summary.insert(
            "edge_count".to_string(),
            JsonValue::Number(self.edges().len() as f64),
        );
        root.insert("summary".to_string(), JsonValue::Object(summary));

        JsonValue::Object(root)
    }

    /// Emits a deterministic compact JSON string of the CFG topology.
    pub fn to_json(&self) -> String {
        self.to_json_value().to_json_string()
    }
}
