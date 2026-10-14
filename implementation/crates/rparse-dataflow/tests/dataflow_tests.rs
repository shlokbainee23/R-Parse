//! Comprehensive integration tests for variable dataflow, reaching definitions,
//! post-dominance, and control dependencies.

use rparse_cfg::{build_cfg, BlockTerminator};
use rparse_dataflow::{
    collect_def_use, compute_control_dependencies, compute_post_dominators,
    compute_reaching_definitions,
};
use rparse_ir::parse_source;
use rparse_scope::resolve_scopes;

#[test]
fn test_def_use_extraction() {
    // Group 1: Definition / Use extraction
    // Verifies symbols defined and used are correctly extracted with symbols and spans
    let code = r#"
x <- 1
y <- x + 2
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);

    // Two definitions: x and y
    assert_eq!(def_use.defs().len(), 2);
    let def_x = &def_use.defs()[0];
    let def_y = &def_use.defs()[1];
    assert_eq!(def_x.name, "x");
    assert_eq!(def_y.name, "y");
    assert_ne!(def_x.symbol_id, def_y.symbol_id);

    // One use: x in x + 2
    assert_eq!(def_use.uses().len(), 1);
    let use_x = &def_use.uses()[0];
    assert_eq!(use_x.name, "x");
    assert_eq!(use_x.symbol_id, Some(def_x.symbol_id));
}

#[test]
fn test_straight_line_reaching_definitions() {
    // Group 2: Straight-line reaching definitions
    // Verifies definition of x reaches the subsequent read of x
    let code = r#"
x <- 1
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let def_x = def_use.defs().iter().find(|d| d.name == "x").unwrap();
    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();

    let reaching = rd.reaching_defs_for_use(use_x.id);
    assert_eq!(reaching.len(), 1);
    assert!(reaching.contains(&def_x.id));
    assert!(rd.is_fully_defined(use_x.id, &cfg, &def_use));
}

#[test]
fn test_reassignment_kills_previous_definition() {
    // Group 3: CASE A — Sequential reassignment
    // x <- 1; x <- 2; y <- x
    // Expected: The definition x <- 2 reaches y, while x <- 1 is killed.
    let code = r#"
x <- 1
x <- 2
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    // Must distinguish x <- 1 and x <- 2 as two distinct definitions of the same symbol
    let x_defs: Vec<_> = def_use.defs().iter().filter(|d| d.name == "x").collect();
    assert_eq!(x_defs.len(), 2);
    let def1 = x_defs[0];
    let def2 = x_defs[1];
    assert_ne!(def1.id, def2.id);
    assert_eq!(def1.symbol_id, def2.symbol_id);

    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();
    let reaching = rd.reaching_defs_for_use(use_x.id);

    // Only def2 reaches use_x; def1 must NOT be in reaching set
    assert_eq!(reaching.len(), 1);
    assert!(reaching.contains(&def2.id));
    assert!(!reaching.contains(&def1.id));
}

#[test]
fn test_branch_merge_reaching_definitions() {
    // Group 4: CASE B — Branching definitions
    // if (cond) { x <- 1 } else { x <- 2 }
    // y <- x
    // Expected: Both branch definitions of x may reach y. The analysis must NOT arbitrarily choose one.
    let code = r#"
if (cond) {
    x <- 1
} else {
    x <- 2
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let x_defs: Vec<_> = def_use.defs().iter().filter(|d| d.name == "x").collect();
    assert_eq!(x_defs.len(), 2);
    let d1 = x_defs[0].id;
    let d2 = x_defs[1].id;

    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();
    let reaching = rd.reaching_defs_for_use(use_x.id);

    // Both branch definitions MUST reach y
    assert_eq!(reaching.len(), 2);
    assert!(reaching.contains(&d1));
    assert!(reaching.contains(&d2));
    assert!(rd.is_fully_defined(use_x.id, &cfg, &def_use));
}

#[test]
fn test_partially_defined_symbol() {
    // Group 5: CASE C — Definition on only one branch
    // if (cond) { x <- 1 }
    // y <- x
    // Expected: Preserves the possibility that x has no local definition along one path.
    let code = r#"
if (cond) {
    x <- 1
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let x_def = def_use.defs().iter().find(|d| d.name == "x").unwrap();
    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();

    let reaching = rd.reaching_defs_for_use(use_x.id);
    assert_eq!(reaching.len(), 1);
    assert!(reaching.contains(&x_def.id));

    // Must detect that x is NOT guaranteed to be defined along all reaching paths
    assert!(!rd.is_fully_defined(use_x.id, &cfg, &def_use));
}

#[test]
fn test_loop_carried_definitions() {
    // Group 6: CASE D — Loop-carried definition
    // x <- 0
    // while (cond) { x <- x + 1 }
    // y <- x
    // Expected: The loop definition of x participates in reaching-definition set after iterations.
    let code = r#"
x <- 0
while (cond) {
    x <- x + 1
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let x_defs: Vec<_> = def_use.defs().iter().filter(|d| d.name == "x").collect();
    assert_eq!(x_defs.len(), 2);
    let init_def = x_defs[0].id;
    let loop_def = x_defs[1].id;

    // Use of x inside loop body (x + 1): sees both initial def and loop def due to back edge
    let body_use_x = def_use
        .uses()
        .iter()
        .find(|u| u.name == "x" && u.block_id != def_use.uses().last().unwrap().block_id)
        .unwrap();
    let body_reaching = rd.reaching_defs_for_use(body_use_x.id);
    assert!(body_reaching.contains(&init_def));
    assert!(body_reaching.contains(&loop_def));

    // Use of x after loop (y <- x): sees both initial def and loop def
    let after_use_x = def_use.uses().last().unwrap();
    let after_reaching = rd.reaching_defs_for_use(after_use_x.id);
    assert!(after_reaching.contains(&init_def));
    assert!(after_reaching.contains(&loop_def));
}

#[test]
fn test_independent_variables() {
    // Group 7: CASE E — Independent variables
    // x <- 1
    // y <- 2
    // z <- x
    // Expected: Definition of y must not interfere with x's reaching definitions.
    let code = r#"
x <- 1
y <- 2
z <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let def_x = def_use.defs().iter().find(|d| d.name == "x").unwrap();
    let def_y = def_use.defs().iter().find(|d| d.name == "y").unwrap();
    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();

    let reaching = rd.reaching_defs_for_use(use_x.id);
    assert_eq!(reaching.len(), 1);
    assert!(reaching.contains(&def_x.id));
    assert!(!reaching.contains(&def_y.id));
}

#[test]
fn test_nested_control_flow_reaching_definitions() {
    // Group 8: CASE F — Nested control flow
    let code = r#"
x <- 0
if (a > 0) {
    while (b > 0) {
        x <- x + 1
    }
} else {
    x <- 99
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let rd = compute_reaching_definitions(&cfg, &def_use);

    let use_x = def_use
        .uses()
        .iter()
        .find(|u| u.name == "x" && u.id == def_use.uses().last().unwrap().id)
        .unwrap();
    let reaching = rd.reaching_defs_for_use(use_x.id);

    // Definitions from initial, while-loop body, and else-branch can reach y
    assert!(reaching.len() >= 2);
}

#[test]
fn test_post_dominators() {
    // Group 9: Post-dominators computation
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

    let pdom = compute_post_dominators(&cfg);

    // Exit block post-dominates every block that reaches it
    for block in cfg.blocks() {
        assert!(pdom.post_dominates(cfg.exit_id, block.id));
    }

    // Every block post-dominates itself
    for block in cfg.blocks() {
        assert!(pdom.post_dominates(block.id, block.id));
    }
}

#[test]
fn test_control_dependencies() {
    // Group 10: Control dependencies computation
    let code = r#"
if (cond) {
    x <- 1
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let pdom = compute_post_dominators(&cfg);
    let cd = compute_control_dependencies(&cfg, &pdom);

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let def_x = def_use.defs().iter().find(|d| d.name == "x").unwrap();
    let use_x = def_use.uses().iter().find(|u| u.name == "x").unwrap();

    // Find condition block
    let cond_block = cfg
        .blocks()
        .iter()
        .find(|b| matches!(b.terminator, BlockTerminator::Branch { .. }))
        .unwrap();

    // Body block (x <- 1) must be control-dependent on condition block
    assert!(cd.is_dependent_on(def_x.block_id, cond_block.id));

    // Merge/subsequent block (y <- x) must NOT be control-dependent on condition block
    assert!(!cd.is_dependent_on(use_x.block_id, cond_block.id));
}

#[test]
fn test_filter_not_treated_as_cfg_control_dependency() {
    // Section 11 requirement:
    // filter(...) predicates must NOT be treated as CFG control dependencies!
    let code = r#"
df <- filter(df, valid == TRUE)
y <- df
"#;
    let program = parse_source(code).expect("should parse");
    let cfg = build_cfg(&program).expect("should build CFG");

    let pdom = compute_post_dominators(&cfg);
    let cd = compute_control_dependencies(&cfg, &pdom);

    // Straight line code: no block should have any control dependencies
    for block in cfg.blocks() {
        assert!(
            cd.dependencies_for_block(block.id).is_empty(),
            "Filter expression must not create CFG control dependencies"
        );
    }
}

#[test]
fn test_deterministic_analysis_results() {
    // Group 11: Deterministic analysis results
    let code = r#"
x <- 10
if (x > 5) {
    x <- x + 1
} else {
    x <- x - 1
}
while (x > 0) {
    x <- x - 2
}
y <- x
"#;
    let p1 = parse_source(code).expect("parse 1");
    let p2 = parse_source(code).expect("parse 2");

    let s1 = resolve_scopes(&p1);
    let s2 = resolve_scopes(&p2);

    let cfg1 = build_cfg(&p1).expect("cfg 1");
    let cfg2 = build_cfg(&p2).expect("cfg 2");

    let du1 = collect_def_use(&p1, &s1, &cfg1);
    let du2 = collect_def_use(&p2, &s2, &cfg2);

    let rd1 = compute_reaching_definitions(&cfg1, &du1);
    let rd2 = compute_reaching_definitions(&cfg2, &du2);

    let pd1 = compute_post_dominators(&cfg1);
    let pd2 = compute_post_dominators(&cfg2);

    let cd1 = compute_control_dependencies(&cfg1, &pd1);
    let cd2 = compute_control_dependencies(&cfg2, &pd2);

    assert_eq!(du1.defs().len(), du2.defs().len());
    assert_eq!(du1.uses().len(), du2.uses().len());
    assert_eq!(rd1, rd2);
    assert_eq!(pd1, pd2);
    assert_eq!(cd1, cd2);
}

#[test]
fn test_branch_join_post_dominator_structure() {
    // Contract Audit 3: The post-dominator of the two branch bodies should resolve
    // to the actual join structure, and the immediate post-dominator relationships must be correct.
    let code = r#"
if (cond) {
    x <- 1
} else {
    x <- 2
}
y <- x
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let pdom = compute_post_dominators(&cfg);

    let x_defs: Vec<_> = def_use.defs().iter().filter(|d| d.name == "x").collect();
    assert_eq!(x_defs.len(), 2);
    let then_body = x_defs[0].block_id;
    let else_body = x_defs[1].block_id;

    let use_y = def_use.uses().iter().find(|u| u.name == "x").unwrap();
    let y_block = use_y.block_id;

    // Both branch bodies must have their immediate post-dominator resolving to the join structure
    let ipdom_then = pdom.ipdom(then_body).expect("then body must have ipdom");
    let ipdom_else = pdom.ipdom(else_body).expect("else body must have ipdom");

    // Both branch bodies converge at the join block
    assert_eq!(
        ipdom_then, ipdom_else,
        "both branch bodies must share the same join block as IPDOM"
    );
    let join_block = ipdom_then;

    // The join block post-dominates both branches
    assert!(pdom.post_dominates(join_block, then_body));
    assert!(pdom.post_dominates(join_block, else_body));

    // The statement block y <- x is strictly post-dominated by exit, and join_block's IPDOM leads to y_block
    assert!(pdom.post_dominates(y_block, join_block));
    assert_eq!(pdom.ipdom(join_block), Some(y_block));
    assert_eq!(pdom.ipdom(y_block), Some(cfg.exit_id));
}

#[test]
fn test_nested_if_control_dependencies() {
    // Contract Audit 4: Nested control flow control dependencies
    // if (a) {
    //     if (b) {
    //         x <- 1
    //     }
    // }
    let code = r#"
if (a) {
    if (b) {
        x <- 1
    }
}
"#;
    let program = parse_source(code).expect("should parse");
    let scopes = resolve_scopes(&program);
    let cfg = build_cfg(&program).expect("should build CFG");

    let def_use = collect_def_use(&program, &scopes, &cfg);
    let pdom = compute_post_dominators(&cfg);
    let cd = compute_control_dependencies(&cfg, &pdom);

    // Find condition blocks for a and b
    let cond_blocks: Vec<_> = cfg
        .blocks()
        .iter()
        .filter(|b| matches!(b.terminator, BlockTerminator::Branch { .. }))
        .collect();
    assert_eq!(cond_blocks.len(), 2);
    let cond_a = cond_blocks[0];
    let cond_b = cond_blocks[1];

    let def_x = def_use.defs().iter().find(|d| d.name == "x").unwrap();
    let x_block = def_x.block_id;

    // Outer controlled region (cond_b) is control-dependent on cond_a
    assert!(cd.is_dependent_on(cond_b.id, cond_a.id));

    // Inner controlled region (x <- 1) is control-dependent on cond_b
    assert!(cd.is_dependent_on(x_block, cond_b.id));

    // Exit block is NOT control-dependent on cond_a or cond_b
    assert!(!cd.is_dependent_on(cfg.exit_id, cond_a.id));
    assert!(!cd.is_dependent_on(cfg.exit_id, cond_b.id));
}
