//! Comprehensive tests for control-flow graph (CFG) construction and invariant validation.

use rparse_cfg::{build_cfg, BlockId, BlockTerminator, EdgeKind};
use rparse_ir::parse_source;

#[test]
fn test_sequential_flow() {
    // Section 3.A: Straight-line sequential code
    // entry -> block(x) -> block(y) -> block(z) -> exit
    let code = r#"
x <- 1
y <- x + 1
z <- y + 1
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    assert_eq!(cfg.entry_id, BlockId(0));
    assert_eq!(cfg.exit_id, BlockId(1));
    // bb0 (entry), bb1 (exit), bb2 (x <- 1), bb3 (y <- x + 1), bb4 (z <- y + 1)
    assert_eq!(cfg.block_count(), 5);

    // Verify chain of normal edges
    let succ_0 = cfg.successors(BlockId(0));
    assert_eq!(succ_0, vec![BlockId(2)]);

    let succ_2 = cfg.successors(BlockId(2));
    assert_eq!(succ_2, vec![BlockId(3)]);

    let succ_3 = cfg.successors(BlockId(3));
    assert_eq!(succ_3, vec![BlockId(4)]);

    let succ_4 = cfg.successors(BlockId(4));
    assert_eq!(succ_4, vec![BlockId(1)]);

    // Check invariants
    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_if_else() {
    // Section 3.B: if / else
    let code = r#"
if (cond) {
    x <- 1
} else {
    x <- 2
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    // Condition block has two outgoing edges: TrueBranch and FalseBranch
    let cond_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::Branch { .. }))
        .collect();
    assert_eq!(cond_blocks.len(), 1);
    let cond_block = cond_blocks[0];

    let true_edge = cond_block
        .outgoing
        .iter()
        .find(|e| e.kind == EdgeKind::TrueBranch)
        .expect("must have true branch");
    let false_edge = cond_block
        .outgoing
        .iter()
        .find(|e| e.kind == EdgeKind::FalseBranch)
        .expect("must have false branch");
    assert_ne!(true_edge.to, false_edge.to);

    // Both branches merge before y <- x
    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_if_without_else() {
    // Section 3.C: if without else
    let code = r#"
if (cond) {
    x <- 1
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    let cond_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::Branch { .. }))
        .collect();
    assert_eq!(cond_blocks.len(), 1);
    let cond_block = cond_blocks[0];

    // True branch enters body, false branch bypasses directly to merge block
    let false_edge = cond_block
        .outgoing
        .iter()
        .find(|e| e.kind == EdgeKind::FalseBranch)
        .expect("must have false bypass branch");

    // Check that false edge goes to a block that has multiple predecessors (merge block)
    let merge_block_id = false_edge.to;
    let preds = cfg.predecessors(merge_block_id);
    assert_eq!(preds.len(), 2); // from then branch and from cond bypass

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_for_loop() {
    // Section 3.D: for loops
    let code = r#"
for (i in xs) {
    x <- i
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    let header_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::LoopHeader { .. }))
        .collect();
    assert_eq!(header_blocks.len(), 1);
    let header = header_blocks[0];

    // Header has TrueBranch (body), FalseBranch (exit), and incoming LoopBack
    assert!(header
        .outgoing
        .iter()
        .any(|e| e.kind == EdgeKind::TrueBranch));
    assert!(header
        .outgoing
        .iter()
        .any(|e| e.kind == EdgeKind::FalseBranch));
    assert!(header.incoming.iter().any(|e| e.kind == EdgeKind::LoopBack));

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_while_loop() {
    // Section 3.E: while loops
    let code = r#"
while (cond) {
    x <- compute()
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    let header_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::LoopHeader { .. }))
        .collect();
    assert_eq!(header_blocks.len(), 1);
    let header = header_blocks[0];

    assert!(header
        .outgoing
        .iter()
        .any(|e| e.kind == EdgeKind::TrueBranch));
    assert!(header
        .outgoing
        .iter()
        .any(|e| e.kind == EdgeKind::FalseBranch));
    assert!(header.incoming.iter().any(|e| e.kind == EdgeKind::LoopBack));

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_return_no_fallthrough() {
    // Section 3.F: return
    let code = r#"
y <- 1
return(y)
z <- 99
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    // Find return block
    let ret_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::Return { .. }))
        .collect();
    assert_eq!(ret_blocks.len(), 1);
    let ret_block = ret_blocks[0];

    // Outgoing edge must be EdgeKind::Return directly to exit_id
    assert_eq!(ret_block.outgoing.len(), 1);
    assert_eq!(ret_block.outgoing[0].kind, EdgeKind::Return);
    assert_eq!(ret_block.outgoing[0].to, cfg.exit_id);

    // Subsequent dead code block (z <- 99) must NOT have incoming edge from return
    for block in cfg.blocks() {
        if block.id != ret_block.id && block.id != cfg.exit_id {
            assert!(
                !block.incoming.iter().any(|e| e.from == ret_block.id),
                "No block except exit should be reached from return"
            );
        }
    }

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_nested_control_flow() {
    // Section 3.G: Nested control flow
    let code = r#"
if (a > 0) {
    while (b > 0) {
        x <- x + 1
    }
} else {
    for (i in xs) {
        x <- x - 1
    }
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    // Must have 1 branch terminator, 2 loop header terminators
    let branch_count = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::Branch { .. }))
        .count();
    let loop_count = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::LoopHeader { .. }))
        .count();

    assert_eq!(branch_count, 1);
    assert_eq!(loop_count, 2);

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_empty_blocks() {
    // Section 3.H: Empty blocks
    let code = r#"
if (cond) {
}
while (cond) {
}
for (i in xs) {
}
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    cfg.validate_invariants().expect("invariants should hold");
}

#[test]
fn test_deterministic_generation() {
    // Section 15: Deterministic CFG generation
    let code = r#"
if (cond) {
    x <- 1
} else {
    x <- 2
}
while (x > 0) {
    x <- x - 1
}
"#;
    let p1 = parse_source(code).expect("should parse");
    let p2 = parse_source(code).expect("should parse");

    let cfg1 = build_cfg(&p1).expect("build 1");
    let cfg2 = build_cfg(&p2).expect("build 2");

    assert_eq!(cfg1.block_count(), cfg2.block_count());
    assert_eq!(cfg1.edges(), cfg2.edges());
    assert_eq!(cfg1.to_dot(), cfg2.to_dot());
}

#[test]
fn test_cfg_invariants_hold() {
    let code = r#"
x <- 10
if (x > 5) {
    x <- x + 1
}
while (x < 20) {
    x <- x + 2
}
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");
    assert!(cfg.validate_invariants().is_ok());
}

#[test]
fn test_unexpected_unknown_ir_handled_gracefully() {
    // Contract Audit B: Verify unsupported/unexpected IR produces a controlled result rather than panic
    use rparse_core::{NodeId, Span};
    use rparse_ir::{Expr, ExprStmt, Program, Stmt, UnknownExpr};

    let span = Span::new(0, 10, 1, 0, 1, 10);
    let unknown_expr = Expr::Unknown(UnknownExpr {
        id: NodeId::new(42),
        reason: "unsupported syntax construct".to_string(),
        raw: Some("`???`".to_string()),
        span,
    });
    let stmt = Stmt::Expr(ExprStmt {
        id: NodeId::new(43),
        expr: Box::new(unknown_expr),
        span,
    });
    let program = Program {
        id: NodeId::new(44),
        stmts: vec![stmt],
        span,
    };

    // Should build without panic and satisfy invariants
    let cfg_result = build_cfg(&program);
    assert!(cfg_result.is_ok());
    let cfg = cfg_result.unwrap();
    assert!(cfg.validate_invariants().is_ok());
}

#[test]
fn test_malformed_graph_fails_invariants_controlled() {
    // Contract Audit B: Invariant validation fails with controlled CfgError without panicking
    use rparse_cfg::{BasicBlock, Cfg, CfgEdge, CfgError};
    use rparse_core::Span;
    use std::collections::BTreeMap;

    let span = Span::new(0, 1, 1, 0, 1, 1);
    let mut bb0 = BasicBlock::new(BlockId(0), span);
    let bb1 = BasicBlock::new(BlockId(1), span);

    // Add dangling edge pointing to non-existent block bb99
    bb0.outgoing
        .push(CfgEdge::new(BlockId(0), BlockId(99), EdgeKind::Normal));

    let malformed_cfg = Cfg::new(BlockId(0), BlockId(1), vec![bb0, bb1], BTreeMap::new());
    let err = malformed_cfg.validate_invariants();
    assert!(matches!(err, Err(CfgError::InvariantViolation(_))));
}
