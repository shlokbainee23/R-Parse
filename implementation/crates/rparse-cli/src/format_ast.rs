//! AST & CST Explorer data serialization for the Scientific Analysis Studio.
//!
//! Provides structured, truthful hierarchical representations of:
//! 1. Concrete Syntax Tree (CST) from Tree-sitter with error nodes and tokens.
//! 2. Canonical Semantic IR / AST from the normalized R-PARSE pipeline.

use std::collections::BTreeMap;

use rparse_core::{JsonValue, Span};
use rparse_ir::{
    Argument, AssignmentKind, Block, Expr, ExtractKind, LValue, LiteralKind, PipeKind, Program,
    Stmt,
};
use rparse_parser::CstNode;

use crate::pipeline::AnalysisContext;

/// Builds the comprehensive AST explorer payload containing both CST and Normalized IR trees.
pub fn build_ast_explorer_payload(ctx: &AnalysisContext) -> JsonValue {
    let source = ctx.source_file.content();
    let mut cst_counter = 0usize;
    let mut cst_errors = 0usize;
    let cst_json = serialize_cst_node(
        ctx.cst.root_node(),
        source,
        None,
        &mut cst_counter,
        &mut cst_errors,
        0,
    );

    let mut ir_counter = 0usize;
    let ir_json = serialize_ir_program(&ctx.program, source, &mut ir_counter);

    let mut summary = BTreeMap::new();
    summary.insert(
        "cst_node_count".to_string(),
        JsonValue::Number(cst_counter as f64),
    );
    summary.insert(
        "ir_node_count".to_string(),
        JsonValue::Number(ir_counter as f64),
    );
    summary.insert(
        "has_syntax_errors".to_string(),
        JsonValue::Bool(ctx.has_errors() || cst_errors > 0),
    );
    summary.insert(
        "error_node_count".to_string(),
        JsonValue::Number(cst_errors as f64),
    );

    let mut root = BTreeMap::new();
    root.insert("summary".to_string(), JsonValue::Object(summary));
    root.insert("cst".to_string(), cst_json);
    root.insert("ir".to_string(), ir_json);

    JsonValue::Object(root)
}

/// Builds AST explorer payload when the pipeline encounters a syntax error or analysis failure,
/// still providing the error-tolerant CST from Tree-sitter.
pub fn build_ast_explorer_payload_for_error_source(source: &str) -> JsonValue {
    if source.trim().is_empty() {
        return build_empty_ast_explorer_payload();
    }

    if let Ok(mut parser) = rparse_parser::RParser::new() {
        if let Ok(cst_tree) = parser.parse(source) {
            let mut cst_counter = 0usize;
            let mut cst_errors = 0usize;
            let cst_json = serialize_cst_node(
                cst_tree.root_node(),
                source,
                None,
                &mut cst_counter,
                &mut cst_errors,
                0,
            );

            // Attempt IR normalization if possible
            let program = rparse_ir::normalize_cst(&cst_tree);
            let mut ir_counter = 0usize;
            let ir_json = serialize_ir_program(&program, source, &mut ir_counter);

            let mut summary = BTreeMap::new();
            summary.insert(
                "cst_node_count".to_string(),
                JsonValue::Number(cst_counter as f64),
            );
            summary.insert(
                "ir_node_count".to_string(),
                JsonValue::Number(ir_counter as f64),
            );
            summary.insert("has_syntax_errors".to_string(), JsonValue::Bool(true));
            summary.insert(
                "error_node_count".to_string(),
                JsonValue::Number(cst_errors as f64),
            );

            let mut root = BTreeMap::new();
            root.insert("summary".to_string(), JsonValue::Object(summary));
            root.insert("cst".to_string(), cst_json);
            root.insert("ir".to_string(), ir_json);
            return JsonValue::Object(root);
        }
    }

    build_empty_ast_explorer_payload()
}

/// Fallback builder for empty or unparsed source.
pub fn build_empty_ast_explorer_payload() -> JsonValue {
    let mut summary = BTreeMap::new();
    summary.insert("cst_node_count".to_string(), JsonValue::Number(0.0));
    summary.insert("ir_node_count".to_string(), JsonValue::Number(0.0));
    summary.insert("has_syntax_errors".to_string(), JsonValue::Bool(false));
    summary.insert("error_node_count".to_string(), JsonValue::Number(0.0));

    let mut empty_node = BTreeMap::new();
    empty_node.insert("id".to_string(), JsonValue::String("empty".to_string()));
    empty_node.insert("kind".to_string(), JsonValue::String("empty".to_string()));
    empty_node.insert(
        "label".to_string(),
        JsonValue::String("No source code".to_string()),
    );
    empty_node.insert("detail".to_string(), JsonValue::String("".to_string()));
    empty_node.insert("children".to_string(), JsonValue::Array(vec![]));
    empty_node.insert("child_count".to_string(), JsonValue::Number(0.0));
    empty_node.insert("span".to_string(), Span::empty().to_json());
    empty_node.insert("text".to_string(), JsonValue::String("".to_string()));

    let mut root = BTreeMap::new();
    root.insert("summary".to_string(), JsonValue::Object(summary));
    root.insert("cst".to_string(), JsonValue::Object(empty_node.clone()));
    root.insert("ir".to_string(), JsonValue::Object(empty_node));

    JsonValue::Object(root)
}

// ─────────────────────────────────────────────────────────────────────────────
// CST (Tree-sitter) Serializer
// ─────────────────────────────────────────────────────────────────────────────

fn serialize_cst_node(
    node: CstNode<'_>,
    source: &str,
    field_name: Option<&str>,
    counter: &mut usize,
    error_counter: &mut usize,
    depth: usize,
) -> JsonValue {
    let id_num = *counter;
    *counter += 1;
    let id = format!("cst-{}", id_num);

    let kind = node.kind().to_string();
    let span = node.span();
    let is_named = node.is_named();
    let is_error = node.is_error();
    let is_missing = node.is_missing();
    let has_error = node.has_error();

    if is_error || is_missing {
        *error_counter += 1;
    }

    let raw_snippet = span.slice(source).unwrap_or("").trim();
    let snippet = if raw_snippet.chars().count() > 80 {
        let truncated: String = raw_snippet.chars().take(80).collect();
        format!("{}…", truncated)
    } else {
        raw_snippet.to_string()
    };

    let mut label = String::new();
    if let Some(f) = field_name {
        label.push_str(f);
        label.push_str(": ");
    }

    if is_error {
        label.push_str(&format!("ERROR near '{}'", snippet));
    } else if is_missing {
        label.push_str(&format!("MISSING '{}'", kind));
    } else if kind == "identifier" || kind == "string" || kind == "float" || kind == "integer" {
        label.push_str(&format!("{} ({})", kind, snippet));
    } else if !snippet.is_empty() && snippet.len() <= 16 && !snippet.contains('\n') {
        label.push_str(&format!("{} [{}]", kind, snippet));
    } else {
        label.push_str(&kind);
    }

    let mut children_json = Vec::new();
    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            let child_field = node.field_name_for_child(i);
            children_json.push(serialize_cst_node(
                child,
                source,
                child_field,
                counter,
                error_counter,
                depth + 1,
            ));
        }
    }

    let mut map = BTreeMap::new();
    map.insert("id".to_string(), JsonValue::String(id));
    map.insert("depth".to_string(), JsonValue::Number(depth as f64));
    map.insert("kind".to_string(), JsonValue::String(kind));
    map.insert("label".to_string(), JsonValue::String(label));
    map.insert("detail".to_string(), JsonValue::String(snippet.clone()));
    if let Some(f) = field_name {
        map.insert("field_name".to_string(), JsonValue::String(f.to_string()));
    }
    map.insert("is_named".to_string(), JsonValue::Bool(is_named));
    map.insert("is_error".to_string(), JsonValue::Bool(is_error));
    map.insert("is_missing".to_string(), JsonValue::Bool(is_missing));
    map.insert("has_error".to_string(), JsonValue::Bool(has_error));
    map.insert("span".to_string(), span.to_json());
    map.insert("text".to_string(), JsonValue::String(snippet));
    map.insert(
        "child_count".to_string(),
        JsonValue::Number(children_json.len() as f64),
    );
    map.insert("children".to_string(), JsonValue::Array(children_json));

    JsonValue::Object(map)
}

// ─────────────────────────────────────────────────────────────────────────────
// Canonical Semantic IR / AST Serializer
// ─────────────────────────────────────────────────────────────────────────────

fn snippet_preview(span: Span, source: &str) -> String {
    let raw = span.slice(source).unwrap_or("").trim();
    if raw.chars().count() > 80 {
        let truncated: String = raw.chars().take(80).collect();
        format!("{}…", truncated)
    } else {
        raw.to_string()
    }
}

fn make_ir_node(
    id_num: usize,
    kind: &str,
    label: String,
    detail: String,
    span: Span,
    source: &str,
    children: Vec<JsonValue>,
) -> JsonValue {
    let mut map = BTreeMap::new();
    map.insert(
        "id".to_string(),
        JsonValue::String(format!("ir-{}", id_num)),
    );
    map.insert("kind".to_string(), JsonValue::String(kind.to_string()));
    map.insert("label".to_string(), JsonValue::String(label));
    map.insert("detail".to_string(), JsonValue::String(detail));
    map.insert("span".to_string(), span.to_json());
    map.insert(
        "text".to_string(),
        JsonValue::String(snippet_preview(span, source)),
    );
    map.insert(
        "child_count".to_string(),
        JsonValue::Number(children.len() as f64),
    );
    map.insert("children".to_string(), JsonValue::Array(children));
    JsonValue::Object(map)
}

fn serialize_ir_program(prog: &Program, source: &str, counter: &mut usize) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    let mut children = Vec::new();
    for stmt in &prog.stmts {
        children.push(serialize_ir_stmt(stmt, source, counter));
    }

    let label = format!("Program ({} statements)", prog.stmts.len());
    make_ir_node(
        id_num,
        "Program",
        label,
        format!("{} stmts", prog.stmts.len()),
        prog.span,
        source,
        children,
    )
}

fn serialize_ir_stmt(stmt: &Stmt, source: &str, counter: &mut usize) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    match stmt {
        Stmt::Assignment(a) => {
            let children = vec![
                serialize_ir_lvalue(&a.target, source, counter),
                serialize_ir_expr(&a.value, source, counter),
            ];

            let op_symbol = match a.kind {
                AssignmentKind::Standard => "<-",
                AssignmentKind::Super => "<<-",
            };
            let label = format!("Assignment ({})", op_symbol);
            let detail = format!("kind: {:?}", a.kind);
            make_ir_node(
                id_num,
                "AssignmentStmt",
                label,
                detail,
                a.span,
                source,
                children,
            )
        }
        Stmt::Expr(e) => {
            let children = vec![serialize_ir_expr(&e.expr, source, counter)];
            make_ir_node(
                id_num,
                "ExprStmt",
                "ExprStmt".to_string(),
                "".to_string(),
                e.span,
                source,
                children,
            )
        }
        Stmt::If(i) => {
            let mut children = Vec::new();
            children.push(serialize_ir_expr(&i.condition, source, counter));
            children.push(serialize_ir_block(
                &i.then_branch,
                "ThenBranch",
                source,
                counter,
            ));
            if let Some(eb) = &i.else_branch {
                children.push(serialize_ir_block(eb, "ElseBranch", source, counter));
            }
            make_ir_node(
                id_num,
                "IfStmt",
                "If Condition".to_string(),
                "".to_string(),
                i.span,
                source,
                children,
            )
        }
        Stmt::For(f) => {
            let children = vec![
                serialize_ir_expr(&f.iterable, source, counter),
                serialize_ir_block(&f.body, "ForBody", source, counter),
            ];
            let label = format!("For Loop (var: {})", f.iterator);
            make_ir_node(
                id_num,
                "ForStmt",
                label,
                f.iterator.clone(),
                f.span,
                source,
                children,
            )
        }
        Stmt::While(w) => {
            let children = vec![
                serialize_ir_expr(&w.condition, source, counter),
                serialize_ir_block(&w.body, "WhileBody", source, counter),
            ];
            make_ir_node(
                id_num,
                "WhileStmt",
                "While Loop".to_string(),
                "".to_string(),
                w.span,
                source,
                children,
            )
        }
        Stmt::Return(r) => {
            let mut children = Vec::new();
            if let Some(v) = &r.value {
                children.push(serialize_ir_expr(v, source, counter));
            }
            make_ir_node(
                id_num,
                "ReturnStmt",
                "Return".to_string(),
                "".to_string(),
                r.span,
                source,
                children,
            )
        }
        Stmt::Block(b) => serialize_ir_block(b, "Block", source, counter),
    }
}

fn serialize_ir_block(
    block: &Block,
    label_prefix: &str,
    source: &str,
    counter: &mut usize,
) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    let mut children = Vec::new();
    for s in &block.stmts {
        children.push(serialize_ir_stmt(s, source, counter));
    }
    let label = format!("{} ({} stmts)", label_prefix, block.stmts.len());
    make_ir_node(
        id_num,
        "Block",
        label,
        format!("{} stmts", block.stmts.len()),
        block.span,
        source,
        children,
    )
}

fn serialize_ir_lvalue(lval: &LValue, source: &str, counter: &mut usize) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    match lval {
        LValue::Symbol { name, span, .. } => {
            let label = format!("TargetSymbol: {}", name);
            make_ir_node(
                id_num,
                "LValue::Symbol",
                label,
                name.clone(),
                *span,
                source,
                vec![],
            )
        }
        LValue::Extraction {
            target,
            index,
            kind,
            span,
            ..
        } => {
            let children = vec![
                serialize_ir_expr(target, source, counter),
                serialize_ir_expr(index, source, counter),
            ];
            let op_str = match kind {
                ExtractKind::Dollar => "$",
                ExtractKind::SingleBracket => "[]",
                ExtractKind::DoubleBracket => "[[]]",
            };
            let label = format!("TargetExtraction ({})", op_str);
            make_ir_node(
                id_num,
                "LValue::Extraction",
                label,
                format!("{:?}", kind),
                *span,
                source,
                children,
            )
        }
    }
}

fn serialize_ir_expr(expr: &Expr, source: &str, counter: &mut usize) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    match expr {
        Expr::Literal(l) => {
            let lit_type = match &l.kind {
                LiteralKind::Numeric(n) => format!("Numeric ({})", n),
                LiteralKind::Integer(i) => format!("Integer ({})", i),
                LiteralKind::String(s) => format!("String (\"{}\")", s),
                LiteralKind::Boolean(b) => format!("Boolean ({})", b),
                LiteralKind::Null => "Null".to_string(),
                LiteralKind::Na => "NA".to_string(),
            };
            let label = format!("Literal: {}", l.raw);
            make_ir_node(
                id_num,
                "LiteralExpr",
                label,
                lit_type,
                l.span,
                source,
                vec![],
            )
        }
        Expr::Identifier(i) => {
            let label = format!("Identifier: {}", i.name);
            make_ir_node(
                id_num,
                "IdentifierExpr",
                label,
                i.name.clone(),
                i.span,
                source,
                vec![],
            )
        }
        Expr::BinaryOp(b) => {
            let children = vec![
                serialize_ir_expr(&b.left, source, counter),
                serialize_ir_expr(&b.right, source, counter),
            ];
            let label = format!("BinaryOp: {}", b.op_str);
            let detail = format!("{:?}", b.op);
            make_ir_node(
                id_num,
                "BinaryOpExpr",
                label,
                detail,
                b.span,
                source,
                children,
            )
        }
        Expr::UnaryOp(u) => {
            let children = vec![serialize_ir_expr(&u.operand, source, counter)];
            let label = format!("UnaryOp ({:?})", u.op);
            make_ir_node(
                id_num,
                "UnaryOpExpr",
                label,
                format!("{:?}", u.op),
                u.span,
                source,
                children,
            )
        }
        Expr::Call(c) => {
            let mut children = Vec::new();
            children.push(serialize_ir_expr(&c.callee, source, counter));
            for arg in &c.args {
                children.push(serialize_ir_arg(arg, source, counter));
            }
            let label = format!("Call ({} args)", c.args.len());
            make_ir_node(
                id_num,
                "CallExpr",
                label,
                format!("{} args", c.args.len()),
                c.span,
                source,
                children,
            )
        }
        Expr::Extract(e) => {
            let children = vec![
                serialize_ir_expr(&e.target, source, counter),
                serialize_ir_expr(&e.index, source, counter),
            ];
            let op_str = match e.kind {
                ExtractKind::Dollar => "$",
                ExtractKind::SingleBracket => "[]",
                ExtractKind::DoubleBracket => "[[]]",
            };
            let label = format!("Extraction ({})", op_str);
            make_ir_node(
                id_num,
                "ExtractExpr",
                label,
                format!("{:?}", e.kind),
                e.span,
                source,
                children,
            )
        }
        Expr::Pipe(p) => {
            let children = vec![
                serialize_ir_expr(&p.left, source, counter),
                serialize_ir_expr(&p.right, source, counter),
            ];
            let op_str = match p.kind {
                PipeKind::Native => "|>",
                PipeKind::Magrittr => "%>%",
            };
            let label = format!("Pipe ({})", op_str);
            make_ir_node(
                id_num,
                "PipeExpr",
                label,
                format!("{:?}", p.kind),
                p.span,
                source,
                children,
            )
        }
        Expr::FunctionDef(f) => {
            let mut children = Vec::new();
            for p in &f.params {
                let p_id = *counter;
                *counter += 1;
                let mut p_children = Vec::new();
                if let Some(def_val) = &p.default_value {
                    p_children.push(serialize_ir_expr(def_val, source, counter));
                }
                children.push(make_ir_node(
                    p_id,
                    "Param",
                    format!("Param: {}", p.name),
                    p.name.clone(),
                    p.span,
                    source,
                    p_children,
                ));
            }
            children.push(serialize_ir_block(&f.body, "FunctionBody", source, counter));
            make_ir_node(
                id_num,
                "FunctionDefExpr",
                format!("FunctionDef ({} params)", f.params.len()),
                format!("{} params", f.params.len()),
                f.span,
                source,
                children,
            )
        }
        Expr::Lambda(l) => {
            let mut children = Vec::new();
            for p in &l.params {
                let p_id = *counter;
                *counter += 1;
                children.push(make_ir_node(
                    p_id,
                    "Param",
                    format!("Param: {}", p.name),
                    p.name.clone(),
                    p.span,
                    source,
                    vec![],
                ));
            }
            children.push(serialize_ir_block(&l.body, "LambdaBody", source, counter));
            make_ir_node(
                id_num,
                "LambdaExpr",
                format!("Lambda ({} params)", l.params.len()),
                format!("{} params", l.params.len()),
                l.span,
                source,
                children,
            )
        }
        Expr::Unknown(u) => {
            let label = format!("Unknown: {}", u.reason);
            make_ir_node(
                id_num,
                "UnknownExpr",
                label,
                u.reason.clone(),
                u.span,
                source,
                vec![],
            )
        }
    }
}

fn serialize_ir_arg(arg: &Argument, source: &str, counter: &mut usize) -> JsonValue {
    let id_num = *counter;
    *counter += 1;

    match arg {
        Argument::Positional(p) => {
            let children = vec![serialize_ir_expr(&p.value, source, counter)];
            make_ir_node(
                id_num,
                "PositionalArg",
                "PositionalArg".to_string(),
                "".to_string(),
                p.span,
                source,
                children,
            )
        }
        Argument::Named(n) => {
            let children = vec![serialize_ir_expr(&n.value, source, counter)];
            let label = format!("NamedArg: {}", n.name);
            make_ir_node(
                id_num,
                "NamedArg",
                label,
                n.name.clone(),
                n.span,
                source,
                children,
            )
        }
    }
}
