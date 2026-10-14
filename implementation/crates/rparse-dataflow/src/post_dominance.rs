//! Graph-based post-dominator analysis over the CFG.
//!
//! A node $P$ post-dominates node $N$ ($P \in PDOM(N)$) when every directed path
//! from $N$ to the CFG exit node passes through $P$.
//!
//! # Algorithm
//! Standard iterative dataflow analysis on the CFG:
//! 1. Identify all blocks from which the exit node is reachable via backward traversal.
//! 2. Initialize $PDOM[Exit] = \{ Exit \}$ and $PDOM[N] = \text{ReachableBlocks}$ for $N \neq Exit$.
//! 3. Repeatedly compute:
//!    $$PDOM[N] = \{ N \} \cup \left( \bigcap_{S \in succ(N) \cap Reachable} PDOM[S] \right)$$
//!    until a fixed point is reached.
//! 4. Compute Immediate Post-Dominators ($IPDOM[N]$) as the unique minimal strict post-dominator.

use rparse_cfg::{BlockId, Cfg};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Results of post-dominance analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct PostDominators {
    pdom: BTreeMap<BlockId, BTreeSet<BlockId>>,
    ipdom: BTreeMap<BlockId, BlockId>,
}

impl PostDominators {
    /// Returns the set of all blocks that post-dominate `block_id`.
    pub fn pdom(&self, block_id: BlockId) -> Option<&BTreeSet<BlockId>> {
        self.pdom.get(&block_id)
    }

    /// Returns the immediate post-dominator of `block_id`, if one exists.
    pub fn ipdom(&self, block_id: BlockId) -> Option<BlockId> {
        self.ipdom.get(&block_id).copied()
    }

    /// Returns whether `p` post-dominates `n` (i.e. `p ∈ PDOM(n)`).
    pub fn post_dominates(&self, p: BlockId, n: BlockId) -> bool {
        self.pdom
            .get(&n)
            .map(|set| set.contains(&p))
            .unwrap_or(false)
    }

    /// Returns whether `p` strictly post-dominates `n` (i.e. `p ∈ PDOM(n) \ {n}`).
    pub fn strictly_post_dominates(&self, p: BlockId, n: BlockId) -> bool {
        p != n && self.post_dominates(p, n)
    }
}

/// Computes post-dominators and immediate post-dominators for all CFG blocks.
pub fn compute_post_dominators(cfg: &Cfg) -> PostDominators {
    let exit_id = cfg.exit_id;

    // 1. Find all blocks that can reach the exit node (backward reachability from exit)
    let mut can_reach_exit = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(exit_id);
    can_reach_exit.insert(exit_id);

    while let Some(curr) = queue.pop_front() {
        for pred in cfg.predecessors(curr) {
            if can_reach_exit.insert(pred) {
                queue.push_back(pred);
            }
        }
    }

    // 2. Initialize PDOM sets
    let mut pdom: BTreeMap<BlockId, BTreeSet<BlockId>> = BTreeMap::new();
    for &b in &can_reach_exit {
        if b == exit_id {
            let mut set = BTreeSet::new();
            set.insert(exit_id);
            pdom.insert(exit_id, set);
        } else {
            pdom.insert(b, can_reach_exit.clone());
        }
    }

    // 3. Iterative fixed-point computation
    let mut changed = true;
    while changed {
        changed = false;

        for &n in &can_reach_exit {
            if n == exit_id {
                continue;
            }

            // Find successors that can reach exit
            let valid_succs: Vec<BlockId> = cfg
                .successors(n)
                .into_iter()
                .filter(|s| can_reach_exit.contains(s))
                .collect();

            let mut intersection: Option<BTreeSet<BlockId>> = None;
            for s in valid_succs {
                if let Some(s_pdom) = pdom.get(&s) {
                    match &mut intersection {
                        None => intersection = Some(s_pdom.clone()),
                        Some(acc) => {
                            acc.retain(|elem| s_pdom.contains(elem));
                        }
                    }
                }
            }

            let mut new_pdom = BTreeSet::new();
            new_pdom.insert(n);
            if let Some(inter) = intersection {
                for elem in inter {
                    new_pdom.insert(elem);
                }
            }

            if pdom.get(&n) != Some(&new_pdom) {
                pdom.insert(n, new_pdom);
                changed = true;
            }
        }
    }

    // 4. Compute Immediate Post-Dominator (IPDOM) for each block
    let mut ipdom = BTreeMap::new();
    for (&n, set) in &pdom {
        if n == exit_id {
            continue;
        }

        // Strict post-dominators of n: PDOM(n) \ {n}
        let strict_pdoms: Vec<BlockId> = set.iter().copied().filter(|&p| p != n).collect();

        // IPDOM(n) is the unique strict post-dominator p such that every other
        // strict post-dominator q of n post-dominates p (q ∈ PDOM(p)).
        let mut ipdom_node: Option<BlockId> = None;
        for &p in &strict_pdoms {
            let is_ipdom = strict_pdoms.iter().all(|&q| {
                q == p
                    || pdom
                        .get(&p)
                        .map(|p_set| p_set.contains(&q))
                        .unwrap_or(false)
            });
            if is_ipdom {
                ipdom_node = Some(p);
                break;
            }
        }

        if let Some(ip) = ipdom_node {
            ipdom.insert(n, ip);
        }
    }

    PostDominators { pdom, ipdom }
}
