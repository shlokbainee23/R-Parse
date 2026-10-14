use rparse_ir::*;
use rparse_parser::RParser;

fn parse_to_ir(source: &str) -> Program {
    let mut parser = RParser::new().expect("parser creation should succeed");
    let cst = parser.parse(source).expect("parsing should succeed");
    normalize_cst(&cst)
}

#[test]
fn test_assignment_normalization() {
    // 1. <- assignment
    let p1 = parse_to_ir("x <- 1");
    assert_eq!(p1.stmts.len(), 1);
    match &p1.stmts[0] {
        Stmt::Assignment(a) => {
            assert_eq!(a.kind, AssignmentKind::Standard);
            match &a.target {
                LValue::Symbol { name, .. } => assert_eq!(name, "x"),
                _ => panic!("Expected symbol target"),
            }
            match a.value.as_ref() {
                Expr::Literal(l) => assert_eq!(l.kind, LiteralKind::Numeric(1.0)),
                _ => panic!("Expected literal value"),
            }
        }
        _ => panic!("Expected assignment stmt"),
    }

    // 2. -> right assignment
    let p2 = parse_to_ir("1 -> x");
    assert_eq!(p2.stmts.len(), 1);
    match &p2.stmts[0] {
        Stmt::Assignment(a) => {
            assert_eq!(a.kind, AssignmentKind::Standard);
            match &a.target {
                LValue::Symbol { name, .. } => assert_eq!(name, "x"),
                _ => panic!("Expected symbol target"),
            }
            match a.value.as_ref() {
                Expr::Literal(l) => assert_eq!(l.kind, LiteralKind::Numeric(1.0)),
                _ => panic!("Expected literal value"),
            }
        }
        _ => panic!("Expected assignment stmt"),
    }

    // 3. = assignment
    let p3 = parse_to_ir("x = 1");
    assert_eq!(p3.stmts.len(), 1);
    match &p3.stmts[0] {
        Stmt::Assignment(a) => {
            assert_eq!(a.kind, AssignmentKind::Standard);
            match &a.target {
                LValue::Symbol { name, .. } => assert_eq!(name, "x"),
                _ => panic!("Expected symbol target"),
            }
        }
        _ => panic!("Expected assignment stmt"),
    }
}

#[test]
fn test_named_arguments_in_calls() {
    let prog = parse_to_ir("foo(x = 1)");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => match e.expr.as_ref() {
            Expr::Call(c) => {
                assert_eq!(c.args.len(), 1);
                match &c.args[0] {
                    Argument::Named(n) => {
                        assert_eq!(n.name, "x");
                        match n.value.as_ref() {
                            Expr::Literal(l) => assert_eq!(l.kind, LiteralKind::Numeric(1.0)),
                            _ => panic!("Expected literal value"),
                        }
                    }
                    _ => panic!("Expected named arg"),
                }
            }
            _ => panic!("Expected CallExpr"),
        },
        _ => panic!("Expected ExprStmt"),
    }
}

#[test]
fn test_super_assignment() {
    let prog = parse_to_ir("x <<- 1");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Assignment(a) => {
            assert_eq!(a.kind, AssignmentKind::Super);
            match &a.target {
                LValue::Symbol { name, .. } => assert_eq!(name, "x"),
                _ => panic!("Expected symbol target"),
            }
        }
        _ => panic!("Expected assignment stmt"),
    }
}

#[test]
fn test_unsupported_assignment_operators_remain_expressions() {
    // := and ->> are not approved Phase 2 assignments and must remain expressions
    let prog_colon = parse_to_ir("x := 1");
    assert_eq!(prog_colon.stmts.len(), 1);
    assert!(matches!(&prog_colon.stmts[0], Stmt::Expr(_)));

    let prog_right_super = parse_to_ir("1 ->> x");
    assert_eq!(prog_right_super.stmts.len(), 1);
    assert!(matches!(&prog_right_super.stmts[0], Stmt::Expr(_)));
}

#[test]
fn test_if_else_normalization() {
    let prog = parse_to_ir("if (x > 0) { y <- 1 } else { y <- 2 }");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::If(i) => {
            assert!(matches!(i.condition.as_ref(), Expr::BinaryOp(_)));
            assert_eq!(i.then_branch.stmts.len(), 1);
            assert!(i.else_branch.is_some());
            assert_eq!(i.else_branch.as_ref().unwrap().stmts.len(), 1);
        }
        _ => panic!("Expected IfStmt"),
    }
}

#[test]
fn test_for_loop_normalization() {
    let prog = parse_to_ir("for (i in 1:10) { print(i) }");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::For(f) => {
            assert_eq!(f.iterator, "i");
            assert!(matches!(f.iterable.as_ref(), Expr::BinaryOp(_)));
            assert_eq!(f.body.stmts.len(), 1);
        }
        _ => panic!("Expected ForStmt"),
    }
}

#[test]
fn test_while_loop_normalization() {
    let prog = parse_to_ir("while (count < 10) { count <- count + 1 }");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::While(w) => {
            assert!(matches!(w.condition.as_ref(), Expr::BinaryOp(_)));
            assert_eq!(w.body.stmts.len(), 1);
        }
        _ => panic!("Expected WhileStmt"),
    }
}

#[test]
fn test_return_normalization() {
    let prog = parse_to_ir("return(42)");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Return(r) => {
            assert!(r.value.is_some());
            match r.value.as_ref().unwrap().as_ref() {
                Expr::Literal(l) => assert_eq!(l.kind, LiteralKind::Numeric(42.0)),
                _ => panic!("Expected numeric literal"),
            }
        }
        _ => panic!("Expected ReturnStmt"),
    }
}

#[test]
fn test_lambda_normalization() {
    let prog = parse_to_ir("\\(x) x + 1");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => match e.expr.as_ref() {
            Expr::Lambda(l) => {
                assert_eq!(l.params.len(), 1);
                assert_eq!(l.params[0].name, "x");
                assert_eq!(l.body.stmts.len(), 1);
            }
            _ => panic!("Expected LambdaExpr"),
        },
        _ => panic!("Expected ExprStmt"),
    }
}

#[test]
fn test_native_pipe() {
    let prog = parse_to_ir("x |> f()");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => match e.expr.as_ref() {
            Expr::Pipe(p) => {
                assert_eq!(p.kind, PipeKind::Native);
                match p.left.as_ref() {
                    Expr::Identifier(i) => assert_eq!(i.name, "x"),
                    _ => panic!("Expected identifier x"),
                }
                match p.right.as_ref() {
                    Expr::Call(c) => match c.callee.as_ref() {
                        Expr::Identifier(i) => assert_eq!(i.name, "f"),
                        _ => panic!("Expected callee f"),
                    },
                    _ => panic!("Expected call"),
                }
            }
            _ => panic!("Expected PipeExpr"),
        },
        _ => panic!("Expected ExprStmt"),
    }
}

#[test]
fn test_native_pipe_placeholder() {
    let prog = parse_to_ir("x |> f(y = _)");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => match e.expr.as_ref() {
            Expr::Pipe(p) => {
                assert_eq!(p.kind, PipeKind::Native);
                match p.right.as_ref() {
                    Expr::Call(c) => {
                        assert_eq!(c.args.len(), 1);
                        match &c.args[0] {
                            Argument::Named(n) => {
                                assert_eq!(n.name, "y");
                                // The placeholder _ should be represented
                                match n.value.as_ref() {
                                    Expr::Identifier(id) => assert_eq!(id.name, "_"),
                                    Expr::Unknown(u) => assert_eq!(u.raw.as_deref(), Some("_")),
                                    _ => {} // preserved representation
                                }
                            }
                            _ => panic!("Expected named arg"),
                        }
                    }
                    _ => panic!("Expected call"),
                }
            }
            _ => panic!("Expected PipeExpr"),
        },
        _ => panic!("Expected ExprStmt"),
    }
}

#[test]
fn test_magrittr_pipe() {
    let prog = parse_to_ir("x %>% f()");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => match e.expr.as_ref() {
            Expr::Pipe(p) => {
                assert_eq!(p.kind, PipeKind::Magrittr);
            }
            _ => panic!("Expected PipeExpr"),
        },
        _ => panic!("Expected ExprStmt"),
    }
}

#[test]
fn test_source_span_preservation() {
    let prog = parse_to_ir("x <- 10 + 20");
    assert_eq!(prog.stmts.len(), 1);
    let s = &prog.stmts[0];
    assert_eq!(s.span().start_line, 1);
    assert_eq!(s.span().start_col, 0);
    assert_eq!(s.span().end_line, 1);
    assert_eq!(s.span().end_col, 12);
    match s {
        Stmt::Assignment(a) => {
            assert_eq!(a.target.span().start_col, 0);
            assert_eq!(a.target.span().end_col, 1);
            assert_eq!(a.value.span().start_col, 5);
            assert_eq!(a.value.span().end_col, 12);
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_extraction_normalization() {
    let prog_dollar = parse_to_ir("df$x <- 10");
    match &prog_dollar.stmts[0] {
        Stmt::Assignment(a) => match &a.target {
            LValue::Extraction { kind, .. } => assert_eq!(*kind, ExtractKind::Dollar),
            _ => panic!("Expected extraction"),
        },
        _ => panic!("Expected assignment"),
    }

    let prog_bracket = parse_to_ir("df[[\"col\"]] <- 10");
    match &prog_bracket.stmts[0] {
        Stmt::Assignment(a) => match &a.target {
            LValue::Extraction { kind, .. } => assert_eq!(*kind, ExtractKind::DoubleBracket),
            _ => panic!("Expected extraction"),
        },
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_unknown_unsupported_construct_representation() {
    // Unrecognized or unsupported syntax construct produces UnknownExpr
    let prog = parse_to_ir("@");
    assert_eq!(prog.stmts.len(), 1);
    match &prog.stmts[0] {
        Stmt::Expr(e) => {
            assert!(matches!(e.expr.as_ref(), Expr::Unknown(_)));
            if let Expr::Unknown(u) = e.expr.as_ref() {
                assert!(u.raw.is_some() || !u.reason.is_empty());
            }
        }
        _ => panic!("Expected ExprStmt with UnknownExpr"),
    }
}

// =========================================================================
// Golden IR Snapshot Tests (Deterministic Serialization)
// =========================================================================

fn golden_output(source: &str) -> String {
    let ir = parse_to_ir(source);
    to_golden_string(&ir)
}

#[test]
fn test_golden_assignments() {
    let src = "x <- 10\ny = 20\n30 -> z\nw <<- 40\n";
    let actual = golden_output(src);
    let expected = "\
Program [1:0-5:0]
  AssignmentStmt (<-) [1:0-1:7]
    TargetSymbol (x) [1:0-1:1]
    Literal (Numeric(10)) [1:5-1:7]
  AssignmentStmt (<-) [2:0-2:6]
    TargetSymbol (y) [2:0-2:1]
    Literal (Numeric(20)) [2:4-2:6]
  AssignmentStmt (<-) [3:0-3:7]
    TargetSymbol (z) [3:6-3:7]
    Literal (Numeric(30)) [3:0-3:2]
  AssignmentStmt (<<-) [4:0-4:8]
    TargetSymbol (w) [4:0-4:1]
    Literal (Numeric(40)) [4:6-4:8]
";
    assert_eq!(actual, expected);
}

#[test]
fn test_golden_function_and_closure() {
    let src = "f <- function(x) {\n  y <- x + 1\n  \\(z) y + z\n}\n";
    let actual = golden_output(src);
    let expected = "\
Program [1:0-5:0]
  AssignmentStmt (<-) [1:0-4:1]
    TargetSymbol (f) [1:0-1:1]
    FunctionDefExpr (function(x)) [1:5-4:1]
      Block [1:17-4:1]
        AssignmentStmt (<-) [2:2-2:12]
          TargetSymbol (y) [2:2-2:3]
          BinaryOp (+) [2:7-2:12]
            Identifier (x) [2:7-2:8]
            Literal (Numeric(1)) [2:11-2:12]
        ExprStmt [3:2-3:12]
          LambdaExpr (\\(z)) [3:2-3:12]
            Block [3:7-3:12]
              ExprStmt [3:7-3:12]
                BinaryOp (+) [3:7-3:12]
                  Identifier (y) [3:7-3:8]
                  Identifier (z) [3:11-3:12]
";
    assert_eq!(actual, expected);
}

#[test]
fn test_golden_pipes_and_calls() {
    let src = "df |> mutate(col = price * .env$rate)\n";
    let actual = golden_output(src);
    let expected = "\
Program [1:0-2:0]
  ExprStmt [1:0-1:37]
    PipeExpr (|>) [1:0-1:37]
      Identifier (df) [1:0-1:2]
      CallExpr [1:6-1:37]
        Callee:
          Identifier (mutate) [1:6-1:12]
        Arguments:
          Named (col = ) [1:13-1:36]
            BinaryOp (*) [1:19-1:36]
              Identifier (price) [1:19-1:24]
              ExtractExpr ($) [1:27-1:36]
                Identifier (.env) [1:27-1:31]
                Identifier (rate) [1:32-1:36]
";
    assert_eq!(actual, expected);
}
