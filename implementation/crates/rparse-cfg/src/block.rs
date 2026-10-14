//! Basic block, edge, and terminator models for R-PARSE CFG.

use rparse_core::{NodeId, Span};
use rparse_ir::Stmt;
use std::fmt;

/// Unique, stable identifier for a basic block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub u32);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}

/// The semantic classification of a control-flow graph edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EdgeKind {
    /// Straight-line sequential control flow or unconditional jump.
    Normal,
    /// Edge taken when a branch/loop condition evaluates to true.
    TrueBranch,
    /// Edge taken when a branch/loop condition evaluates to false (or bypasses branch).
    FalseBranch,
    /// Back edge from a loop body exit returning to the loop header.
    LoopBack,
    /// Control transfer directly to the CFG exit point from a `return(...)` statement.
    Return,
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::TrueBranch => write!(f, "true"),
            Self::FalseBranch => write!(f, "false"),
            Self::LoopBack => write!(f, "loop_back"),
            Self::Return => write!(f, "return"),
        }
    }
}

/// A directed edge connecting two basic blocks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CfgEdge {
    pub from: BlockId,
    pub to: BlockId,
    pub kind: EdgeKind,
}

impl CfgEdge {
    pub fn new(from: BlockId, to: BlockId, kind: EdgeKind) -> Self {
        Self { from, to, kind }
    }
}

/// The terminator construct that determines how a basic block branches.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockTerminator {
    /// Normal fall-through or jump.
    None,
    /// Two-way conditional branch (`if`).
    Branch {
        condition_node: NodeId,
        condition_span: Span,
    },
    /// Loop iteration condition / header (`while`, `for`).
    LoopHeader {
        condition_node: NodeId,
        condition_span: Span,
    },
    /// Function exit transfer (`return`).
    Return {
        return_node: NodeId,
        return_span: Span,
    },
}

/// A maximal sequence of straight-line statements with single entry and exit.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub id: BlockId,
    pub stmts: Vec<Stmt>,
    pub terminator: BlockTerminator,
    pub incoming: Vec<CfgEdge>,
    pub outgoing: Vec<CfgEdge>,
    pub span: Span,
}

impl BasicBlock {
    pub fn new(id: BlockId, span: Span) -> Self {
        Self {
            id,
            stmts: Vec::new(),
            terminator: BlockTerminator::None,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            span,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }

    pub fn stmt_count(&self) -> usize {
        self.stmts.len()
    }

    pub fn statement_ids(&self) -> Vec<NodeId> {
        self.stmts.iter().map(|s| s.id()).collect()
    }
}
