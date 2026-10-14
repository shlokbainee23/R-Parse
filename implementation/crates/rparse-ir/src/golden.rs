//! Deterministic golden serialization for Normalized Semantic IR.
//!
//! Emits an indented, address-free, human-readable S-expression / AST string
//! suitable for golden snapshot testing across parser versions.

use crate::ast::*;
use std::fmt::Write;

/// Serializes a [`Program`] into a deterministic golden representation.
pub fn to_golden_string(program: &Program) -> String {
    let mut out = String::new();
    writeln!(out, "Program [{}]", program.span).unwrap();
    for stmt in &program.stmts {
        write_stmt(&mut out, stmt, 1);
    }
    out
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn write_stmt(out: &mut String, stmt: &Stmt, depth: usize) {
    indent(out, depth);
    match stmt {
        Stmt::Assignment(a) => {
            let kind_str = match a.kind {
                AssignmentKind::Standard => "<-",
                AssignmentKind::Super => "<<-",
            };
            writeln!(out, "AssignmentStmt ({kind_str}) [{}]", a.span).unwrap();
            write_lvalue(out, &a.target, depth + 1);
            write_expr(out, &a.value, depth + 1);
        }
        Stmt::Expr(e) => {
            writeln!(out, "ExprStmt [{}]", e.span).unwrap();
            write_expr(out, &e.expr, depth + 1);
        }
        Stmt::If(i) => {
            writeln!(out, "IfStmt [{}]", i.span).unwrap();
            indent(out, depth + 1);
            writeln!(out, "Condition:").unwrap();
            write_expr(out, &i.condition, depth + 2);
            indent(out, depth + 1);
            writeln!(out, "Then:").unwrap();
            write_block(out, &i.then_branch, depth + 2);
            if let Some(eb) = &i.else_branch {
                indent(out, depth + 1);
                writeln!(out, "Else:").unwrap();
                write_block(out, eb, depth + 2);
            }
        }
        Stmt::For(f) => {
            writeln!(out, "ForStmt (var: {}) [{}]", f.iterator, f.span).unwrap();
            indent(out, depth + 1);
            writeln!(out, "Iterable:").unwrap();
            write_expr(out, &f.iterable, depth + 2);
            indent(out, depth + 1);
            writeln!(out, "Body:").unwrap();
            write_block(out, &f.body, depth + 2);
        }
        Stmt::While(w) => {
            writeln!(out, "WhileStmt [{}]", w.span).unwrap();
            indent(out, depth + 1);
            writeln!(out, "Condition:").unwrap();
            write_expr(out, &w.condition, depth + 2);
            indent(out, depth + 1);
            writeln!(out, "Body:").unwrap();
            write_block(out, &w.body, depth + 2);
        }
        Stmt::Return(r) => {
            writeln!(out, "ReturnStmt [{}]", r.span).unwrap();
            if let Some(v) = &r.value {
                write_expr(out, v, depth + 1);
            }
        }
        Stmt::Block(b) => {
            writeln!(out, "Block [{}]", b.span).unwrap();
            for s in &b.stmts {
                write_stmt(out, s, depth + 1);
            }
        }
    }
}

fn write_block(out: &mut String, block: &Block, depth: usize) {
    indent(out, depth);
    writeln!(out, "Block [{}]", block.span).unwrap();
    for s in &block.stmts {
        write_stmt(out, s, depth + 1);
    }
}

fn write_lvalue(out: &mut String, lval: &LValue, depth: usize) {
    indent(out, depth);
    match lval {
        LValue::Symbol { name, span, .. } => {
            writeln!(out, "TargetSymbol ({name}) [{span}]").unwrap();
        }
        LValue::Extraction {
            target,
            index,
            kind,
            span,
            ..
        } => {
            let k = match kind {
                ExtractKind::Dollar => "$",
                ExtractKind::SingleBracket => "[]",
                ExtractKind::DoubleBracket => "[[]]",
            };
            writeln!(out, "TargetExtraction ({k}) [{span}]").unwrap();
            write_expr(out, target, depth + 1);
            write_expr(out, index, depth + 1);
        }
    }
}

fn write_expr(out: &mut String, expr: &Expr, depth: usize) {
    indent(out, depth);
    match expr {
        Expr::Literal(l) => {
            let lit_desc = match &l.kind {
                LiteralKind::Integer(i) => format!("Integer({i})"),
                LiteralKind::Numeric(n) => format!("Numeric({n})"),
                LiteralKind::String(s) => format!("String(\"{s}\")"),
                LiteralKind::Boolean(b) => format!("Boolean({b})"),
                LiteralKind::Null => "Null".to_string(),
                LiteralKind::Na => "NA".to_string(),
            };
            writeln!(out, "Literal ({lit_desc}) [{}]", l.span).unwrap();
        }
        Expr::Identifier(i) => {
            writeln!(out, "Identifier ({}) [{}]", i.name, i.span).unwrap();
        }
        Expr::BinaryOp(b) => {
            writeln!(out, "BinaryOp ({}) [{}]", b.op_str, b.span).unwrap();
            write_expr(out, &b.left, depth + 1);
            write_expr(out, &b.right, depth + 1);
        }
        Expr::UnaryOp(u) => {
            writeln!(out, "UnaryOp ({:?}) [{}]", u.op, u.span).unwrap();
            write_expr(out, &u.operand, depth + 1);
        }
        Expr::Call(c) => {
            writeln!(out, "CallExpr [{}]", c.span).unwrap();
            indent(out, depth + 1);
            writeln!(out, "Callee:").unwrap();
            write_expr(out, &c.callee, depth + 2);
            if !c.args.is_empty() {
                indent(out, depth + 1);
                writeln!(out, "Arguments:").unwrap();
                for arg in &c.args {
                    match arg {
                        Argument::Positional(p) => {
                            indent(out, depth + 2);
                            writeln!(out, "Positional [{}]", p.span).unwrap();
                            write_expr(out, &p.value, depth + 3);
                        }
                        Argument::Named(n) => {
                            indent(out, depth + 2);
                            writeln!(out, "Named ({} = ) [{}]", n.name, n.span).unwrap();
                            write_expr(out, &n.value, depth + 3);
                        }
                    }
                }
            }
        }
        Expr::Extract(e) => {
            let k = match e.kind {
                ExtractKind::Dollar => "$",
                ExtractKind::SingleBracket => "[]",
                ExtractKind::DoubleBracket => "[[]]",
            };
            writeln!(out, "ExtractExpr ({k}) [{}]", e.span).unwrap();
            write_expr(out, &e.target, depth + 1);
            write_expr(out, &e.index, depth + 1);
        }
        Expr::Pipe(p) => {
            let pk = match p.kind {
                PipeKind::Native => "|>",
                PipeKind::Magrittr => "%>%",
            };
            writeln!(out, "PipeExpr ({pk}) [{}]", p.span).unwrap();
            write_expr(out, &p.left, depth + 1);
            write_expr(out, &p.right, depth + 1);
        }
        Expr::Lambda(l) => {
            let param_names: Vec<_> = l.params.iter().map(|p| p.name.as_str()).collect();
            writeln!(
                out,
                "LambdaExpr (\\({})) [{}]",
                param_names.join(", "),
                l.span
            )
            .unwrap();
            write_block(out, &l.body, depth + 1);
        }
        Expr::FunctionDef(f) => {
            let param_names: Vec<_> = f.params.iter().map(|p| p.name.as_str()).collect();
            writeln!(
                out,
                "FunctionDefExpr (function({})) [{}]",
                param_names.join(", "),
                f.span
            )
            .unwrap();
            write_block(out, &f.body, depth + 1);
        }
        Expr::Unknown(u) => {
            writeln!(out, "UnknownExpr ({}) [{}]", u.reason, u.span).unwrap();
        }
    }
}
