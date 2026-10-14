//! Semantic normalizer lowering Tree-sitter CST to Normalized Semantic IR.

use crate::ast::*;
use rparse_core::{NodeIdGenerator, Span};
use rparse_parser::{CstNode, CstTree};

/// Normalizes a parsed [`CstTree`] into a [`Program`].
pub fn normalize_cst(cst: &CstTree) -> Program {
    let mut normalizer = Normalizer::new(cst.source());
    normalizer.normalize_program(cst.root_node())
}

struct Normalizer<'s> {
    source: &'s str,
    id_gen: NodeIdGenerator,
}

impl<'s> Normalizer<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            source,
            id_gen: NodeIdGenerator::new(),
        }
    }

    fn next_id(&mut self) -> rparse_core::NodeId {
        self.id_gen.next_id()
    }

    fn normalize_program(&mut self, root: CstNode<'_>) -> Program {
        let mut stmts = Vec::new();
        for child in root.children() {
            if child.kind() == "comment" || child.kind() == ";" {
                continue;
            }
            if child.is_named() {
                stmts.push(self.normalize_stmt(child));
            }
        }

        Program {
            id: self.next_id(),
            stmts,
            span: root.span(),
        }
    }

    fn normalize_stmt(&mut self, node: CstNode<'_>) -> Stmt {
        let span = node.span();

        match node.kind() {
            "binary_operator" => {
                if let Some(op_node) = node.child_by_field_name("operator") {
                    let op_str = op_node.text(self.source).unwrap_or(op_node.kind());
                    match op_str {
                        "<-" => {
                            let lhs = node.child_by_field_name("lhs");
                            let rhs = node.child_by_field_name("rhs");
                            if let (Some(target_node), Some(val_node)) = (lhs, rhs) {
                                let target = self.normalize_lvalue(target_node);
                                let value = Box::new(self.normalize_expr(val_node));
                                return Stmt::Assignment(AssignmentStmt {
                                    id: self.next_id(),
                                    target,
                                    value,
                                    kind: AssignmentKind::Standard,
                                    span,
                                });
                            }
                        }
                        "->" => {
                            let lhs = node.child_by_field_name("lhs");
                            let rhs = node.child_by_field_name("rhs");
                            // In 10 -> x, lhs is value (10), rhs is target (x)
                            if let (Some(val_node), Some(target_node)) = (lhs, rhs) {
                                let target = self.normalize_lvalue(target_node);
                                let value = Box::new(self.normalize_expr(val_node));
                                return Stmt::Assignment(AssignmentStmt {
                                    id: self.next_id(),
                                    target,
                                    value,
                                    kind: AssignmentKind::Standard,
                                    span,
                                });
                            }
                        }
                        "=" => {
                            // Context-sensitive: In statement context, '=' is standard assignment
                            let lhs = node.child_by_field_name("lhs");
                            let rhs = node.child_by_field_name("rhs");
                            if let (Some(target_node), Some(val_node)) = (lhs, rhs) {
                                let target = self.normalize_lvalue(target_node);
                                let value = Box::new(self.normalize_expr(val_node));
                                return Stmt::Assignment(AssignmentStmt {
                                    id: self.next_id(),
                                    target,
                                    value,
                                    kind: AssignmentKind::Standard,
                                    span,
                                });
                            }
                        }
                        "<<-" => {
                            let lhs = node.child_by_field_name("lhs");
                            let rhs = node.child_by_field_name("rhs");
                            if let (Some(target_node), Some(val_node)) = (lhs, rhs) {
                                let target = self.normalize_lvalue(target_node);
                                let value = Box::new(self.normalize_expr(val_node));
                                return Stmt::Assignment(AssignmentStmt {
                                    id: self.next_id(),
                                    target,
                                    value,
                                    kind: AssignmentKind::Super,
                                    span,
                                });
                            }
                        }
                        _ => {}
                    }
                }
                // Other binary operators are expressions
                Stmt::Expr(ExprStmt {
                    id: self.next_id(),
                    expr: Box::new(self.normalize_expr(node)),
                    span,
                })
            }
            "if_statement" => {
                let cond_node = node
                    .child_by_field_name("condition")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing if condition", span));

                let then_node = node.child_by_field_name("consequence");
                let then_branch = self.normalize_to_block(then_node, span);

                let else_branch = node
                    .child_by_field_name("alternative")
                    .map(|alt| self.normalize_to_block(Some(alt), alt.span()));

                Stmt::If(IfStmt {
                    id: self.next_id(),
                    condition: Box::new(cond_node),
                    then_branch,
                    else_branch,
                    span,
                })
            }
            "for_statement" => {
                let var_node = node
                    .child_by_field_name("variable")
                    .or_else(|| node.child_by_field_name("name"));
                let (iterator, iterator_span, iterator_id) = if let Some(vn) = var_node {
                    (
                        vn.text(self.source).unwrap_or("?").to_string(),
                        vn.span(),
                        self.next_id(),
                    )
                } else {
                    ("?".to_string(), span, self.next_id())
                };

                let iter_node = node
                    .child_by_field_name("sequence")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing for sequence", span));

                let body = self.normalize_to_block(node.child_by_field_name("body"), span);

                Stmt::For(ForStmt {
                    id: self.next_id(),
                    iterator,
                    iterator_span,
                    iterator_id,
                    iterable: Box::new(iter_node),
                    body,
                    span,
                })
            }
            "while_statement" => {
                let cond_node = node
                    .child_by_field_name("condition")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing while condition", span));

                let body = self.normalize_to_block(node.child_by_field_name("body"), span);

                Stmt::While(WhileStmt {
                    id: self.next_id(),
                    condition: Box::new(cond_node),
                    body,
                    span,
                })
            }
            "braced_expression" => Stmt::Block(self.normalize_block(node)),
            "call" => {
                // Check if this is an explicit `return(...)`
                if let Some(fn_node) = node.child_by_field_name("function") {
                    if fn_node.text(self.source) == Some("return") {
                        let ret_val = if let Some(args_node) = node.child_by_field_name("arguments")
                        {
                            args_node.named_children().next().map(|arg| {
                                let val_node = if arg.kind() == "argument" {
                                    arg.child_by_field_name("value").unwrap_or(arg)
                                } else {
                                    arg
                                };
                                Box::new(self.normalize_expr(val_node))
                            })
                        } else {
                            None
                        };

                        return Stmt::Return(ReturnStmt {
                            id: self.next_id(),
                            value: ret_val,
                            span,
                        });
                    }
                }
                Stmt::Expr(ExprStmt {
                    id: self.next_id(),
                    expr: Box::new(self.normalize_expr(node)),
                    span,
                })
            }
            _ => Stmt::Expr(ExprStmt {
                id: self.next_id(),
                expr: Box::new(self.normalize_expr(node)),
                span,
            }),
        }
    }

    fn normalize_block(&mut self, node: CstNode<'_>) -> Block {
        let mut stmts = Vec::new();
        for child in node.children() {
            if child.kind() == "comment"
                || child.kind() == "{"
                || child.kind() == "}"
                || child.kind() == ";"
            {
                continue;
            }
            if child.is_named() {
                stmts.push(self.normalize_stmt(child));
            }
        }
        Block {
            id: self.next_id(),
            stmts,
            span: node.span(),
        }
    }

    fn normalize_to_block(&mut self, node: Option<CstNode<'_>>, fallback_span: Span) -> Block {
        match node {
            Some(n) if n.kind() == "braced_expression" => self.normalize_block(n),
            Some(n) => {
                let stmt = self.normalize_stmt(n);
                Block {
                    id: self.next_id(),
                    stmts: vec![stmt],
                    span: n.span(),
                }
            }
            None => Block {
                id: self.next_id(),
                stmts: Vec::new(),
                span: fallback_span,
            },
        }
    }

    fn normalize_lvalue(&mut self, node: CstNode<'_>) -> LValue {
        let span = node.span();
        match node.kind() {
            "identifier" => LValue::Symbol {
                id: self.next_id(),
                name: node.text(self.source).unwrap_or("?").to_string(),
                span,
            },
            "extract_operator" => {
                let target = node
                    .child(0)
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing extract target", span));
                let index = node
                    .child(2)
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing extract index", span));
                LValue::Extraction {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::Dollar,
                    span,
                }
            }
            "subset2" => {
                let target = node
                    .child_by_field_name("function")
                    .or_else(|| node.child(0))
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset2 target", span));
                let index = node
                    .child_by_field_name("arguments")
                    .and_then(|args| args.named_children().next())
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset2 index", span));
                LValue::Extraction {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::DoubleBracket,
                    span,
                }
            }
            "subset" => {
                let target = node
                    .child_by_field_name("function")
                    .or_else(|| node.child(0))
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset target", span));
                let index = node
                    .child_by_field_name("arguments")
                    .map(|args| self.normalize_expr(args))
                    .unwrap_or_else(|| self.unknown_expr("missing subset index", span));
                LValue::Extraction {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::SingleBracket,
                    span,
                }
            }
            _ => LValue::Symbol {
                id: self.next_id(),
                name: node.text(self.source).unwrap_or("?").to_string(),
                span,
            },
        }
    }

    pub fn normalize_expr(&mut self, node: CstNode<'_>) -> Expr {
        let span = node.span();

        match node.kind() {
            "identifier" => Expr::Identifier(IdentifierExpr {
                id: self.next_id(),
                name: node.text(self.source).unwrap_or("?").to_string(),
                span,
            }),
            "integer" => {
                let raw = node.text(self.source).unwrap_or("0").to_string();
                let clean = raw.trim_end_matches('L');
                let val = if clean.starts_with("0x") || clean.starts_with("0X") {
                    i64::from_str_radix(&clean[2..], 16).unwrap_or(0)
                } else {
                    clean.parse::<i64>().unwrap_or(0)
                };
                Expr::Literal(LiteralExpr {
                    id: self.next_id(),
                    kind: LiteralKind::Integer(val),
                    raw,
                    span,
                })
            }
            "float" => {
                let raw = node.text(self.source).unwrap_or("0.0").to_string();
                let val = raw.parse::<f64>().unwrap_or(0.0);
                Expr::Literal(LiteralExpr {
                    id: self.next_id(),
                    kind: LiteralKind::Numeric(val),
                    raw,
                    span,
                })
            }
            "string" => {
                let raw = node.text(self.source).unwrap_or("\"\"").to_string();
                let content = if ((raw.starts_with('"') && raw.ends_with('"'))
                    || (raw.starts_with('\'') && raw.ends_with('\'')))
                    && raw.len() >= 2
                {
                    raw[1..raw.len() - 1].to_string()
                } else {
                    raw.clone()
                };
                Expr::Literal(LiteralExpr {
                    id: self.next_id(),
                    kind: LiteralKind::String(content),
                    raw,
                    span,
                })
            }
            "true" => Expr::Literal(LiteralExpr {
                id: self.next_id(),
                kind: LiteralKind::Boolean(true),
                raw: "TRUE".to_string(),
                span,
            }),
            "false" => Expr::Literal(LiteralExpr {
                id: self.next_id(),
                kind: LiteralKind::Boolean(false),
                raw: "FALSE".to_string(),
                span,
            }),
            "null" => Expr::Literal(LiteralExpr {
                id: self.next_id(),
                kind: LiteralKind::Null,
                raw: "NULL".to_string(),
                span,
            }),
            "na" => Expr::Literal(LiteralExpr {
                id: self.next_id(),
                kind: LiteralKind::Na,
                raw: node.text(self.source).unwrap_or("NA").to_string(),
                span,
            }),
            "binary_operator" => {
                let op_node = node.child_by_field_name("operator");
                let op_text = op_node.and_then(|o| o.text(self.source)).unwrap_or("");

                // Check pipes
                if op_text == "|>" {
                    let lhs = node
                        .child_by_field_name("lhs")
                        .map(|n| self.normalize_expr(n))
                        .unwrap_or_else(|| self.unknown_expr("missing pipe lhs", span));
                    let rhs = node
                        .child_by_field_name("rhs")
                        .map(|n| self.normalize_expr(n))
                        .unwrap_or_else(|| self.unknown_expr("missing pipe rhs", span));
                    return Expr::Pipe(PipeExpr {
                        id: self.next_id(),
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        kind: PipeKind::Native,
                        span,
                    });
                }
                if op_text == "%>%" {
                    let lhs = node
                        .child_by_field_name("lhs")
                        .map(|n| self.normalize_expr(n))
                        .unwrap_or_else(|| self.unknown_expr("missing pipe lhs", span));
                    let rhs = node
                        .child_by_field_name("rhs")
                        .map(|n| self.normalize_expr(n))
                        .unwrap_or_else(|| self.unknown_expr("missing pipe rhs", span));
                    return Expr::Pipe(PipeExpr {
                        id: self.next_id(),
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        kind: PipeKind::Magrittr,
                        span,
                    });
                }

                // General binary operators
                let op = match op_text {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Sub,
                    "*" => BinaryOp::Mul,
                    "/" => BinaryOp::Div,
                    "%%" => BinaryOp::Mod,
                    "^" | "**" => BinaryOp::Pow,
                    "==" => BinaryOp::Eq,
                    "!=" => BinaryOp::NotEq,
                    "<" => BinaryOp::Lt,
                    "<=" => BinaryOp::LtEq,
                    ">" => BinaryOp::Gt,
                    ">=" => BinaryOp::GtEq,
                    "&" | "&&" => BinaryOp::And,
                    "|" | "||" => BinaryOp::Or,
                    ":" => BinaryOp::Colon,
                    _ => BinaryOp::Special,
                };

                let left = node
                    .child_by_field_name("lhs")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing binary lhs", span));
                let right = node
                    .child_by_field_name("rhs")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing binary rhs", span));

                Expr::BinaryOp(BinaryOpExpr {
                    id: self.next_id(),
                    op,
                    op_str: op_text.to_string(),
                    left: Box::new(left),
                    right: Box::new(right),
                    span,
                })
            }
            "unary_operator" => {
                let op_node = node.child_by_field_name("operator");
                let op_text = op_node.and_then(|o| o.text(self.source)).unwrap_or("");
                let op = match op_text {
                    "+" => UnaryOp::Plus,
                    "-" => UnaryOp::Minus,
                    "!" => UnaryOp::Not,
                    "~" => UnaryOp::Tilde,
                    "?" => UnaryOp::Help,
                    _ => UnaryOp::Plus,
                };
                let operand = node
                    .child_by_field_name("operand")
                    .or_else(|| node.named_children().next())
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing unary operand", span));

                Expr::UnaryOp(UnaryOpExpr {
                    id: self.next_id(),
                    op,
                    operand: Box::new(operand),
                    span,
                })
            }
            "call" => {
                let callee_node = node
                    .child_by_field_name("function")
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing call callee", span));

                let mut args = Vec::new();
                if let Some(args_node) = node.child_by_field_name("arguments") {
                    for arg_child in args_node.children() {
                        if arg_child.kind() == "("
                            || arg_child.kind() == ")"
                            || arg_child.kind() == ","
                            || arg_child.kind() == "comma"
                            || arg_child.kind() == "comment"
                        {
                            continue;
                        }
                        if arg_child.is_named() {
                            args.push(self.normalize_argument(arg_child));
                        }
                    }
                }

                Expr::Call(CallExpr {
                    id: self.next_id(),
                    callee: Box::new(callee_node),
                    args,
                    span,
                })
            }
            "extract_operator" => {
                let target = node
                    .child(0)
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing extract target", span));
                let index = node
                    .child(2)
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing extract index", span));
                Expr::Extract(ExtractExpr {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::Dollar,
                    span,
                })
            }
            "subset2" => {
                let target = node
                    .child_by_field_name("function")
                    .or_else(|| node.child(0))
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset2 target", span));
                let index = node
                    .child_by_field_name("arguments")
                    .and_then(|args| args.named_children().next())
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset2 index", span));
                Expr::Extract(ExtractExpr {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::DoubleBracket,
                    span,
                })
            }
            "subset" => {
                let target = node
                    .child_by_field_name("function")
                    .or_else(|| node.child(0))
                    .map(|n| self.normalize_expr(n))
                    .unwrap_or_else(|| self.unknown_expr("missing subset target", span));
                let index = node
                    .child_by_field_name("arguments")
                    .map(|args| self.normalize_expr(args))
                    .unwrap_or_else(|| self.unknown_expr("missing subset index", span));
                Expr::Extract(ExtractExpr {
                    id: self.next_id(),
                    target: Box::new(target),
                    index: Box::new(index),
                    kind: ExtractKind::SingleBracket,
                    span,
                })
            }
            "function_definition" => {
                let is_lambda = node
                    .text(self.source)
                    .map(|t| t.starts_with('\\'))
                    .unwrap_or(false);
                let params = self.normalize_parameters(node.child_by_field_name("parameters"));
                let body = self.normalize_to_block(node.child_by_field_name("body"), span);

                if is_lambda {
                    Expr::Lambda(LambdaExpr {
                        id: self.next_id(),
                        params,
                        body,
                        span,
                    })
                } else {
                    Expr::FunctionDef(FunctionDefExpr {
                        id: self.next_id(),
                        params,
                        body,
                        span,
                    })
                }
            }
            "parenthesized_expression" => {
                // Unwrap parenthesized expression to inner expression
                if let Some(inner) = node.named_children().next() {
                    self.normalize_expr(inner)
                } else {
                    self.unknown_expr("empty parens", span)
                }
            }
            "braced_expression" => {
                // Braced expression in expr context
                let block = self.normalize_block(node);
                // If single statement expr, unwrap or represent
                if block.stmts.len() == 1 {
                    if let Stmt::Expr(e) = &block.stmts[0] {
                        return *e.expr.clone();
                    }
                }
                self.unknown_expr("compound braced expr in expression context", span)
            }
            "argument" => {
                if let Some(val) = node.child_by_field_name("value") {
                    self.normalize_expr(val)
                } else if let Some(first) = node.named_children().next() {
                    self.normalize_expr(first)
                } else {
                    self.unknown_expr("empty argument", span)
                }
            }
            _ => self.unknown_expr(node.kind(), span),
        }
    }

    fn normalize_parameters(&mut self, params_node: Option<CstNode<'_>>) -> Vec<Param> {
        let mut params = Vec::new();
        if let Some(pn) = params_node {
            for child in pn.children() {
                if child.kind() == "("
                    || child.kind() == ")"
                    || child.kind() == ","
                    || child.kind() == "comma"
                    || child.kind() == "comment"
                {
                    continue;
                }
                if child.is_named() {
                    let span = child.span();
                    let name = child
                        .child_by_field_name("name")
                        .and_then(|n| n.text(self.source))
                        .or_else(|| child.text(self.source))
                        .unwrap_or("?")
                        .to_string();

                    let default_value = child
                        .child_by_field_name("value")
                        .map(|v| Box::new(self.normalize_expr(v)));

                    params.push(Param {
                        id: self.next_id(),
                        name,
                        default_value,
                        span,
                    });
                }
            }
        }
        params
    }

    fn normalize_argument(&mut self, node: CstNode<'_>) -> Argument {
        let span = node.span();
        if node.kind() == "argument" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = name_node.text(self.source).unwrap_or("?").to_string();
                let name_span = name_node.span();
                let val_expr = node
                    .child_by_field_name("value")
                    .map(|v| self.normalize_expr(v))
                    .unwrap_or_else(|| self.unknown_expr("missing named argument value", span));

                return Argument::Named(NamedArg {
                    id: self.next_id(),
                    name,
                    name_span,
                    value: Box::new(val_expr),
                    span,
                });
            } else if let Some(val_node) = node.child_by_field_name("value") {
                let val_expr = self.normalize_expr(val_node);
                return Argument::Positional(PositionalArg {
                    id: self.next_id(),
                    value: Box::new(val_expr),
                    span,
                });
            }
        }

        // Positional argument
        let val_expr = self.normalize_expr(node);
        Argument::Positional(PositionalArg {
            id: self.next_id(),
            value: Box::new(val_expr),
            span,
        })
    }

    fn unknown_expr(&mut self, reason: &str, span: Span) -> Expr {
        Expr::Unknown(UnknownExpr {
            id: self.next_id(),
            reason: reason.to_string(),
            raw: span.slice(self.source).map(|s| s.to_string()),
            span,
        })
    }
}
