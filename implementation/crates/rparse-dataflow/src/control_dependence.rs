//! Graph-based control dependence analysis using CFG and post-dominator trees.
//!
//! A node $Y$ is control-dependent on node $A$ along edge $(A \to B)$ if:
//! 1. $Y$ post-dominates $B$ in the CFG.
//! 2. $Y$ does not strictly post-dominate $A$.
//!
//! # Algorithm
//! Standard Cytron/Ferrante reverse-dominance frontier algorithm:
//! For each CFG edge $A \to B$:
//! If $B$ does not strictly post-dominate $A$:
//! Traverse the post-dominator tree from $B$ up to $IPDOM(A)$, marking each visited
//! node as control-dependent on $A$.

use crate::post_dominance::PostDominators;
use rparse_cfg::{BlockId, BlockTerminator, Cfg, EdgeKind};
use rparse_core::NodeId;
use std::collections::BTreeMap;

/// A control dependency relationship between a controlled block and a controlling block.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlDependency {
    /// The block containing the branch or loop condition.
    pub controlling_block: BlockId,
    /// The semantic AST node representing the condition expression, if any.
    pub condition_node: Option<NodeId>,
    /// The edge kind taken from the controlling block (e.g. `TrueBranch`, `FalseBranch`).
    pub edge_kind: EdgeKind,
}

/// Results of control dependency analysis across the CFG.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlDependencies {
    block_deps: BTreeMap<BlockId, Vec<ControlDependency>>,
}

impl ControlDependencies {
    /// Returns all control dependencies for a given basic block.
    pub fn dependencies_for_block(&self, block_id: BlockId) -> &[ControlDependency] {
        self.block_deps
            .get(&block_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns whether `block_id` is control-dependent on `controlling_block`.
    pub fn is_dependent_on(&self, block_id: BlockId, controlling_block: BlockId) -> bool {
        self.dependencies_for_block(block_id)
            .iter()
            .any(|d| d.controlling_block == controlling_block)
    }

    /// Returns all blocks that are control-dependent on `controlling_block`.
    pub fn dependent_blocks(&self, controlling_block: BlockId) -> Vec<BlockId> {
        self.block_deps
            .iter()
            .filter(|(_, deps)| {
                deps.iter()
                    .any(|d| d.controlling_block == controlling_block)
            })
            .map(|(&b, _)| b)
            .collect()
    }
}

/// Computes control dependencies from CFG and post-dominator analysis.
pub fn compute_control_dependencies(
    cfg: &Cfg,
    post_dominators: &PostDominators,
) -> ControlDependencies {
    let mut block_deps: BTreeMap<BlockId, Vec<ControlDependency>> = BTreeMap::new();

    // Iterate through all edges A -> B in the CFG
    for edge in cfg.edges() {
        let a = edge.from;
        let b = edge.to;

        // Skip edges where B strictly post-dominates A (straight-line flow or normal merge)
        if post_dominators.strictly_post_dominates(b, a) {
            continue;
        }

        // Determine condition node from block terminator
        let condition_node = match cfg.block(a).map(|blk| &blk.terminator) {
            Some(BlockTerminator::Branch { condition_node, .. }) => Some(*condition_node),
            Some(BlockTerminator::LoopHeader { condition_node, .. }) => Some(*condition_node),
            _ => None,
        };

        let ipdom_a = post_dominators.ipdom(a);

        // Traverse post-dominator tree from B up to IPDOM(A)
        let mut curr = Some(b);
        while let Some(node) = curr {
            if Some(node) == ipdom_a {
                break;
            }

            let dep = ControlDependency {
                controlling_block: a,
                condition_node,
                edge_kind: edge.kind,
            };

            let list = block_deps.entry(node).or_default();
            if !list.contains(&dep) {
                list.push(dep);
            }

            curr = post_dominators.ipdom(node);
        }
    }

    ControlDependencies { block_deps }
}
