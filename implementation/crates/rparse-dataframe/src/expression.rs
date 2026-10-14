use crate::dependency::ExpressionDependencies;
use crate::ids::ColumnVersionId;
use crate::state::DataFrameState;
use rparse_ir::{BinaryOpExpr, CallExpr, Expr, ExtractExpr, LiteralKind, UnaryOpExpr};
use rparse_scope::{ScopeId, ScopeModel};
use std::collections::BTreeMap;

/// Standard modeled R helper functions whose semantics pass arguments without dynamic column effects.
const MODELED_BUILTIN_FUNCTIONS: &[&str] = &[
    "c",
    "paste",
    "paste0",
    "sprintf",
    "as.character",
    "as.numeric",
    "as.integer",
    "as.factor",
    "as.logical",
    "as.Date",
    "sum",
    "mean",
    "median",
    "min",
    "max",
    "abs",
    "sqrt",
    "log",
    "log10",
    "exp",
    "round",
    "floor",
    "ceiling",
    "nchar",
    "tolower",
    "toupper",
    "trimws",
    "substr",
    "is.na",
    "is.null",
    "coalesce",
];

/// Extracts fine-grained expression dependencies against an active dataframe schema.
pub fn extract_expression_dependencies(
    expr: &Expr,
    active_schema: &BTreeMap<String, ColumnVersionId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
    known_dataframes: &BTreeMap<String, DataFrameState>,
) -> ExpressionDependencies {
    let mut deps = ExpressionDependencies::new();
    extract_recursive(
        expr,
        active_schema,
        scope_model,
        current_scope,
        known_dataframes,
        &mut deps,
    );
    deps
}

fn extract_recursive(
    expr: &Expr,
    active_schema: &BTreeMap<String, ColumnVersionId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
    known_dataframes: &BTreeMap<String, DataFrameState>,
    deps: &mut ExpressionDependencies,
) {
    match expr {
        // RULE A: Literals produce no dependencies
        Expr::Literal(_) => {}

        // RULE B: Binary Operators -> union of operands
        Expr::BinaryOp(BinaryOpExpr { left, right, .. }) => {
            extract_recursive(
                left,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
            extract_recursive(
                right,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
        }

        // RULE C: Unary Operators -> operand dependencies
        Expr::UnaryOp(UnaryOpExpr { operand, .. }) => {
            extract_recursive(
                operand,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
        }

        // RULE D: Bare Identifier inside data-masking context
        Expr::Identifier(id) => {
            if id.name == ".data" || id.name == ".env" {
                // Handled in extraction expressions below; bare pronoun without member is uncertain
                return;
            }

            // Priority 1: If column exists in active dataframe schema -> Column dependency
            if let Some(&col_ver) = active_schema.get(&id.name) {
                deps.add_column(col_ver);
            }
            // Priority 2: If resolves to lexical symbol -> Lexical variable dependency
            else if let Some(sym_id) = scope_model.resolve_lexical(current_scope, &id.name) {
                deps.add_variable(sym_id);
            }
            // Priority 3: Otherwise UNKNOWN
            else {
                deps.set_unknown();
            }
        }

        // Extraction expressions (.data$col, .data[["col"]], .env$var, df$col, dynamic indexing)
        Expr::Extract(extract) => {
            handle_extract(
                extract,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
        }

        // Function Calls
        Expr::Call(call) => {
            handle_call(
                call,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
        }

        Expr::Pipe(pipe) => {
            extract_recursive(
                &pipe.left,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
            extract_recursive(
                &pipe.right,
                active_schema,
                scope_model,
                current_scope,
                known_dataframes,
                deps,
            );
        }

        Expr::Lambda(lambda) => {
            // Lambda body expression
            for stmt in &lambda.body.stmts {
                if let rparse_ir::Stmt::Expr(expr_stmt) = stmt {
                    extract_recursive(
                        &expr_stmt.expr,
                        active_schema,
                        scope_model,
                        current_scope,
                        known_dataframes,
                        deps,
                    );
                }
            }
        }

        Expr::FunctionDef(_) => {}

        Expr::Unknown(_) => {
            deps.set_unknown();
        }
    }
}

fn handle_extract(
    extract: &ExtractExpr,
    active_schema: &BTreeMap<String, ColumnVersionId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
    known_dataframes: &BTreeMap<String, DataFrameState>,
    deps: &mut ExpressionDependencies,
) {
    // 1. RULE E: .data$col or .data[["col"]]
    if is_ident(&extract.target, ".data") {
        if let Some(col_name) = extract_static_name(&extract.index) {
            if let Some(&col_ver) = active_schema.get(&col_name) {
                deps.add_column(col_ver);
            } else {
                deps.set_unknown();
            }
        } else {
            // Dynamic indexing .data[[get_col_name()]] -> RULE H
            deps.set_unknown();
        }
        return;
    }

    // 2. RULE F: .env$rate
    if is_ident(&extract.target, ".env") {
        if let Some(var_name) = extract_static_name(&extract.index) {
            if let Some(sym_id) = scope_model.resolve_lexical(current_scope, &var_name) {
                deps.add_variable(sym_id);
            } else {
                deps.set_unknown();
            }
        } else {
            deps.set_unknown();
        }
        return;
    }

    // 3. RULE G: df$col where df is a known dataframe
    if let Expr::Identifier(target_id) = extract.target.as_ref() {
        if let Some(df_state) = known_dataframes.get(&target_id.name) {
            if let Some(col_name) = extract_static_name(&extract.index) {
                if let Some(&col_ver) = df_state.schema.get(&col_name) {
                    deps.add_column(col_ver);
                    return;
                }
            } else {
                // Dynamic indexing on known dataframe -> RULE H
                deps.set_unknown();
                return;
            }
        }
    }

    // Fallback: general extraction
    extract_recursive(
        &extract.target,
        active_schema,
        scope_model,
        current_scope,
        known_dataframes,
        deps,
    );
    extract_recursive(
        &extract.index,
        active_schema,
        scope_model,
        current_scope,
        known_dataframes,
        deps,
    );
}

fn handle_call(
    call: &CallExpr,
    active_schema: &BTreeMap<String, ColumnVersionId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
    known_dataframes: &BTreeMap<String, DataFrameState>,
    deps: &mut ExpressionDependencies,
) {
    let fn_name = match call.callee.as_ref() {
        Expr::Identifier(id) => Some(id.name.as_str()),
        _ => None,
    };

    // Recursively extract from all arguments
    for arg in &call.args {
        extract_recursive(
            arg.value(),
            active_schema,
            scope_model,
            current_scope,
            known_dataframes,
            deps,
        );
    }

    // If the function is not a recognized built-in helper, conservatively preserve unknown
    // while keeping all statically identified argument dependencies!
    if let Some(name) = fn_name {
        if !MODELED_BUILTIN_FUNCTIONS.contains(&name) {
            deps.set_unknown();
        }
    } else {
        // Dynamic or computed callee -> mark unknown
        deps.set_unknown();
    }
}

fn is_ident(expr: &Expr, expected: &str) -> bool {
    matches!(expr, Expr::Identifier(id) if id.name == expected)
}

fn extract_static_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(id) => Some(id.name.clone()),
        Expr::Literal(lit) => match &lit.kind {
            LiteralKind::String(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}
