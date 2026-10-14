//! Control-Flow Graph builder constructing CFGs from Semantic IR.

use crate::block::{BasicBlock, BlockId, BlockTerminator, CfgEdge, EdgeKind};
use crate::error::CfgError;
use crate::graph::Cfg;
use rparse_core::{NodeId, Span};
use rparse_ir::{Block, Expr, IfStmt, LValue, Program, Stmt};
use std::collections::BTreeMap;

/// Builds a deterministic [`Cfg`] from a normalized [`Program`].
pub fn build_cfg(program: &Program) -> Result<Cfg, CfgError> {
    let mut builder = CfgBuilder::new(program.span);
    builder.build_program(program)?;
    let cfg = builder.finish()?;
    cfg.validate_invariants()?;
    Ok(cfg)
}

/// Builds a deterministic [`Cfg`] from a statement [`Block`] (e.g. function body).
pub fn build_cfg_for_block(block: &Block) -> Result<Cfg, CfgError> {
    let mut builder = CfgBuilder::new(block.span);
    builder.build_stmts(&block.stmts)?;
    let cfg = builder.finish()?;
    cfg.validate_invariants()?;
    Ok(cfg)
}

struct CfgBuilder {
    blocks: Vec<BasicBlock>,
    entry_id: BlockId,
    exit_id: BlockId,
    current_block: Option<BlockId>,
    next_id: u32,
    node_to_block: BTreeMap<NodeId, BlockId>,
}

impl CfgBuilder {
    fn new(span: Span) -> Self {
        // bb0 = Entry, bb1 = Exit
        let entry_id = BlockId(0);
        let exit_id = BlockId(1);
        let entry = BasicBlock::new(entry_id, span);
        let exit = BasicBlock::new(exit_id, span);

        Self {
            blocks: vec![entry, exit],
            entry_id,
            exit_id,
            current_block: Some(entry_id),
            next_id: 2,
            node_to_block: BTreeMap::new(),
        }
    }

    fn new_block(&mut self, span: Span) -> BlockId {
        let id = BlockId(self.next_id);
        self.next_id += 1;
        let block = BasicBlock::new(id, span);
        self.blocks.push(block);
        id
    }

    fn add_edge(&mut self, from: BlockId, to: BlockId, kind: EdgeKind) {
        let edge = CfgEdge::new(from, to, kind);
        // Add to outgoing of `from`
        if let Some(from_block) = self.blocks.iter_mut().find(|b| b.id == from) {
            from_block.outgoing.push(edge.clone());
        }
        // Add to incoming of `to`
        if let Some(to_block) = self.blocks.iter_mut().find(|b| b.id == to) {
            to_block.incoming.push(edge);
        }
    }

    fn map_node(&mut self, node_id: NodeId, block_id: BlockId) {
        self.node_to_block.insert(node_id, block_id);
    }

    fn map_expr(&mut self, expr: &Expr, block_id: BlockId) {
        self.map_node(expr.id(), block_id);
        match expr {
            Expr::Literal(_) | Expr::Identifier(_) | Expr::Unknown(_) => {}
            Expr::BinaryOp(b) => {
                self.map_expr(&b.left, block_id);
                self.map_expr(&b.right, block_id);
            }
            Expr::UnaryOp(u) => {
                self.map_expr(&u.operand, block_id);
            }
            Expr::Call(c) => {
                self.map_expr(&c.callee, block_id);
                for arg in &c.args {
                    self.map_node(arg.id(), block_id);
                    self.map_expr(arg.value(), block_id);
                }
            }
            Expr::Extract(e) => {
                self.map_expr(&e.target, block_id);
                self.map_expr(&e.index, block_id);
            }
            Expr::Pipe(p) => {
                self.map_expr(&p.left, block_id);
                self.map_expr(&p.right, block_id);
            }
            Expr::Lambda(l) => {
                self.map_node(l.id, block_id);
            }
            Expr::FunctionDef(f) => {
                self.map_node(f.id, block_id);
            }
        }
    }

    fn build_program(&mut self, program: &Program) -> Result<(), CfgError> {
        self.build_stmts(&program.stmts)
    }

    fn build_stmts(&mut self, stmts: &[Stmt]) -> Result<(), CfgError> {
        for stmt in stmts {
            self.build_stmt(stmt)?;
        }
        Ok(())
    }

    fn build_stmt(&mut self, stmt: &Stmt) -> Result<(), CfgError> {
        match stmt {
            Stmt::Assignment(assign) => {
                let blk_id = self.new_block(stmt.span());
                if let Some(b) = self.blocks.iter_mut().find(|b| b.id == blk_id) {
                    b.stmts.push(stmt.clone());
                }

                self.map_node(assign.id, blk_id);
                self.map_node(assign.target.id(), blk_id);
                if let LValue::Extraction { target, index, .. } = &assign.target {
                    self.map_expr(target, blk_id);
                    self.map_expr(index, blk_id);
                }
                self.map_expr(&assign.value, blk_id);

                if let Some(curr) = self.current_block {
                    self.add_edge(curr, blk_id, EdgeKind::Normal);
                }
                self.current_block = Some(blk_id);
            }
            Stmt::Expr(expr_stmt) => {
                let blk_id = self.new_block(stmt.span());
                if let Some(b) = self.blocks.iter_mut().find(|b| b.id == blk_id) {
                    b.stmts.push(stmt.clone());
                }

                self.map_node(expr_stmt.id, blk_id);
                self.map_expr(&expr_stmt.expr, blk_id);

                if let Some(curr) = self.current_block {
                    self.add_edge(curr, blk_id, EdgeKind::Normal);
                }
                self.current_block = Some(blk_id);
            }
            Stmt::If(if_stmt) => {
                self.build_if(if_stmt)?;
            }
            Stmt::For(for_stmt) => {
                let header_id = self.new_block(for_stmt.span);
                if let Some(h) = self.blocks.iter_mut().find(|b| b.id == header_id) {
                    h.terminator = BlockTerminator::LoopHeader {
                        condition_node: for_stmt.id,
                        condition_span: for_stmt.span,
                    };
                }

                self.map_node(for_stmt.id, header_id);
                self.map_node(for_stmt.iterator_id, header_id);
                self.map_expr(&for_stmt.iterable, header_id);

                if let Some(curr) = self.current_block {
                    self.add_edge(curr, header_id, EdgeKind::Normal);
                }

                let loop_exit_id = self.new_block(for_stmt.span);

                if for_stmt.body.stmts.is_empty() {
                    let empty_body_id = self.new_block(for_stmt.span);
                    self.add_edge(header_id, empty_body_id, EdgeKind::TrueBranch);
                    self.add_edge(empty_body_id, header_id, EdgeKind::LoopBack);
                } else {
                    let body_entry_id = self.new_block(for_stmt.body.span);
                    self.add_edge(header_id, body_entry_id, EdgeKind::TrueBranch);
                    self.current_block = Some(body_entry_id);

                    self.build_stmts(&for_stmt.body.stmts)?;

                    if let Some(body_exit) = self.current_block {
                        self.add_edge(body_exit, header_id, EdgeKind::LoopBack);
                    } else {
                        // Body returned early; add structural latch to preserve loop back invariant
                        let latch_id = self.new_block(for_stmt.span);
                        self.add_edge(latch_id, header_id, EdgeKind::LoopBack);
                    }
                }

                self.add_edge(header_id, loop_exit_id, EdgeKind::FalseBranch);
                self.current_block = Some(loop_exit_id);
            }
            Stmt::While(while_stmt) => {
                let header_id = self.new_block(while_stmt.span);
                if let Some(h) = self.blocks.iter_mut().find(|b| b.id == header_id) {
                    h.terminator = BlockTerminator::LoopHeader {
                        condition_node: while_stmt.condition.id(),
                        condition_span: while_stmt.condition.span(),
                    };
                }

                self.map_node(while_stmt.id, header_id);
                self.map_expr(&while_stmt.condition, header_id);

                if let Some(curr) = self.current_block {
                    self.add_edge(curr, header_id, EdgeKind::Normal);
                }

                let loop_exit_id = self.new_block(while_stmt.span);

                if while_stmt.body.stmts.is_empty() {
                    let empty_body_id = self.new_block(while_stmt.span);
                    self.add_edge(header_id, empty_body_id, EdgeKind::TrueBranch);
                    self.add_edge(empty_body_id, header_id, EdgeKind::LoopBack);
                } else {
                    let body_entry_id = self.new_block(while_stmt.body.span);
                    self.add_edge(header_id, body_entry_id, EdgeKind::TrueBranch);
                    self.current_block = Some(body_entry_id);

                    self.build_stmts(&while_stmt.body.stmts)?;

                    if let Some(body_exit) = self.current_block {
                        self.add_edge(body_exit, header_id, EdgeKind::LoopBack);
                    } else {
                        let latch_id = self.new_block(while_stmt.span);
                        self.add_edge(latch_id, header_id, EdgeKind::LoopBack);
                    }
                }

                self.add_edge(header_id, loop_exit_id, EdgeKind::FalseBranch);
                self.current_block = Some(loop_exit_id);
            }
            Stmt::Return(ret_stmt) => {
                let ret_block_id = self.new_block(ret_stmt.span);
                if let Some(b) = self.blocks.iter_mut().find(|b| b.id == ret_block_id) {
                    b.stmts.push(stmt.clone());
                    b.terminator = BlockTerminator::Return {
                        return_node: ret_stmt.id,
                        return_span: ret_stmt.span,
                    };
                }

                self.map_node(ret_stmt.id, ret_block_id);
                if let Some(val) = &ret_stmt.value {
                    self.map_expr(val, ret_block_id);
                }

                if let Some(curr) = self.current_block {
                    self.add_edge(curr, ret_block_id, EdgeKind::Normal);
                }
                // Return transfers directly to exit block
                self.add_edge(ret_block_id, self.exit_id, EdgeKind::Return);

                // Control terminates here: no fall-through
                self.current_block = None;
            }
            Stmt::Block(block) => {
                self.build_stmts(&block.stmts)?;
            }
        }
        Ok(())
    }

    fn build_if(&mut self, if_stmt: &IfStmt) -> Result<(), CfgError> {
        // Condition evaluation block
        let cond_id = self.new_block(if_stmt.condition.span());
        if let Some(b) = self.blocks.iter_mut().find(|b| b.id == cond_id) {
            b.terminator = BlockTerminator::Branch {
                condition_node: if_stmt.condition.id(),
                condition_span: if_stmt.condition.span(),
            };
        }

        self.map_node(if_stmt.id, cond_id);
        self.map_expr(&if_stmt.condition, cond_id);

        if let Some(curr) = self.current_block {
            self.add_edge(curr, cond_id, EdgeKind::Normal);
        }

        // Merge block where branches rejoin
        let merge_id = self.new_block(if_stmt.span);

        // 1. Then branch
        if if_stmt.then_branch.stmts.is_empty() {
            let empty_then = self.new_block(if_stmt.then_branch.span);
            self.add_edge(cond_id, empty_then, EdgeKind::TrueBranch);
            self.add_edge(empty_then, merge_id, EdgeKind::Normal);
        } else {
            let then_entry = self.new_block(if_stmt.then_branch.span);
            self.add_edge(cond_id, then_entry, EdgeKind::TrueBranch);
            self.current_block = Some(then_entry);

            self.build_stmts(&if_stmt.then_branch.stmts)?;

            if let Some(then_exit) = self.current_block {
                self.add_edge(then_exit, merge_id, EdgeKind::Normal);
            }
        }

        // 2. Else branch
        if let Some(else_branch) = &if_stmt.else_branch {
            if else_branch.stmts.is_empty() {
                let empty_else = self.new_block(else_branch.span);
                self.add_edge(cond_id, empty_else, EdgeKind::FalseBranch);
                self.add_edge(empty_else, merge_id, EdgeKind::Normal);
            } else {
                let else_entry = self.new_block(else_branch.span);
                self.add_edge(cond_id, else_entry, EdgeKind::FalseBranch);
                self.current_block = Some(else_entry);

                self.build_stmts(&else_branch.stmts)?;

                if let Some(else_exit) = self.current_block {
                    self.add_edge(else_exit, merge_id, EdgeKind::Normal);
                }
            }
        } else {
            // No else branch: false branch bypasses directly to merge block
            self.add_edge(cond_id, merge_id, EdgeKind::FalseBranch);
        }

        self.current_block = Some(merge_id);
        Ok(())
    }

    fn finish(mut self) -> Result<Cfg, CfgError> {
        // Connect the final block to the exit block with a Normal edge
        if let Some(curr) = self.current_block {
            self.add_edge(curr, self.exit_id, EdgeKind::Normal);
        }

        Ok(Cfg::new(
            self.entry_id,
            self.exit_id,
            self.blocks,
            self.node_to_block,
        ))
    }
}
