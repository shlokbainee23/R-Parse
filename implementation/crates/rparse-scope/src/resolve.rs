//! Semantic scope resolver walking Semantic IR to build symbol and scope models.

use crate::scope::{Scope, ScopeKind, ScopeModel};
use crate::symbol::{
    DefId, DefKind, Definition, RefId, RefKind, Reference, ScopeId, Symbol, SymbolId,
};
use rparse_core::{NodeId, Span};
use rparse_ir::*;

/// Resolves scopes, symbols, definitions, and references for a given [`Program`].
pub fn resolve_scopes(program: &Program) -> ScopeModel {
    let mut resolver = Resolver::new();
    resolver.resolve_program(program);
    resolver.model
}

struct Resolver {
    model: ScopeModel,
}

impl Resolver {
    fn new() -> Self {
        let mut model = ScopeModel::new();
        // Root scope is always Global (ScopeId(0))
        let global = Scope::new(ScopeId(0), None, ScopeKind::Global);
        model.scopes.push(global);
        Self { model }
    }

    fn create_scope(&mut self, parent: Option<ScopeId>, kind: ScopeKind) -> ScopeId {
        let id = ScopeId(self.model.scopes.len());
        let scope = Scope::new(id, parent, kind);
        self.model.scopes.push(scope);
        if let Some(pid) = parent {
            self.model.scope_mut(pid).children.push(id);
        }
        id
    }

    fn add_symbol(&mut self, scope_id: ScopeId, name: String, span: Span) -> SymbolId {
        let id = SymbolId(self.model.symbols.len());
        let sym = Symbol {
            id,
            name: name.clone(),
            scope_id,
            span,
            defs: Vec::new(),
            refs: Vec::new(),
        };
        self.model.symbols.push(sym);
        self.model.scope_mut(scope_id).symbols.insert(name, id);
        id
    }

    fn add_definition(
        &mut self,
        scope_id: ScopeId,
        symbol_id: SymbolId,
        name: String,
        node_id: NodeId,
        span: Span,
        kind: DefKind,
    ) -> DefId {
        let id = DefId(self.model.definitions.len());
        let def = Definition {
            id,
            symbol_id,
            name,
            node_id,
            scope_id,
            span,
            kind,
        };
        self.model.definitions.push(def);
        self.model.scope_mut(scope_id).definitions.push(id);
        self.model.symbol_mut(symbol_id).defs.push(id);
        id
    }

    fn add_reference(
        &mut self,
        scope_id: ScopeId,
        name: String,
        node_id: NodeId,
        symbol_id: Option<SymbolId>,
        span: Span,
        kind: RefKind,
    ) -> RefId {
        let id = RefId(self.model.references.len());
        let reference = Reference {
            id,
            name,
            node_id,
            symbol_id,
            scope_id,
            span,
            kind,
        };
        self.model.references.push(reference);
        self.model.scope_mut(scope_id).references.push(id);
        if let Some(sid) = symbol_id {
            self.model.symbol_mut(sid).refs.push(id);
        }
        id
    }

    fn resolve_program(&mut self, program: &Program) {
        let global_scope = ScopeId(0);
        for stmt in &program.stmts {
            self.resolve_stmt(stmt, global_scope);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, current_scope: ScopeId) {
        match stmt {
            Stmt::Assignment(assign) => self.resolve_assignment(assign, current_scope),
            Stmt::Expr(expr_stmt) => self.resolve_expr(&expr_stmt.expr, current_scope),
            Stmt::If(if_stmt) => {
                self.resolve_expr(&if_stmt.condition, current_scope);
                for s in &if_stmt.then_branch.stmts {
                    self.resolve_stmt(s, current_scope);
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for s in &else_branch.stmts {
                        self.resolve_stmt(s, current_scope);
                    }
                }
            }
            Stmt::For(for_stmt) => {
                // Loop iterator variable is defined in the current scope
                let sym_id = if let Some(&existing) = self
                    .model
                    .scope(current_scope)
                    .symbols
                    .get(&for_stmt.iterator)
                {
                    existing
                } else {
                    self.add_symbol(
                        current_scope,
                        for_stmt.iterator.clone(),
                        for_stmt.iterator_span,
                    )
                };

                self.add_definition(
                    current_scope,
                    sym_id,
                    for_stmt.iterator.clone(),
                    for_stmt.iterator_id,
                    for_stmt.iterator_span,
                    DefKind::LoopIterator,
                );

                self.resolve_expr(&for_stmt.iterable, current_scope);
                for s in &for_stmt.body.stmts {
                    self.resolve_stmt(s, current_scope);
                }
            }
            Stmt::While(while_stmt) => {
                self.resolve_expr(&while_stmt.condition, current_scope);
                for s in &while_stmt.body.stmts {
                    self.resolve_stmt(s, current_scope);
                }
            }
            Stmt::Return(ret) => {
                if let Some(val) = &ret.value {
                    self.resolve_expr(val, current_scope);
                }
            }
            Stmt::Block(block) => {
                for s in &block.stmts {
                    self.resolve_stmt(s, current_scope);
                }
            }
        }
    }

    fn resolve_assignment(&mut self, assign: &AssignmentStmt, current_scope: ScopeId) {
        match assign.kind {
            AssignmentKind::Standard => match &assign.target {
                LValue::Symbol { id, name, span } => {
                    let sym_id = if let Some(&existing) =
                        self.model.scope(current_scope).symbols.get(name)
                    {
                        existing
                    } else {
                        self.add_symbol(current_scope, name.clone(), *span)
                    };

                    self.add_definition(
                        current_scope,
                        sym_id,
                        name.clone(),
                        *id,
                        *span,
                        DefKind::Variable(AssignmentKind::Standard),
                    );

                    self.add_reference(
                        current_scope,
                        name.clone(),
                        *id,
                        Some(sym_id),
                        *span,
                        RefKind::Write,
                    );
                }
                LValue::Extraction { target, index, .. } => {
                    self.resolve_expr(target, current_scope);
                    self.resolve_expr(index, current_scope);
                }
            },
            AssignmentKind::Super => {
                match &assign.target {
                    LValue::Symbol { id, name, span } => {
                        // Super-assignment searches enclosing scopes starting from parent
                        let parent_scope = self.model.scope(current_scope).parent;
                        let resolved_sym =
                            parent_scope.and_then(|p| self.model.resolve_symbol(p, name));

                        let (target_scope, sym_id) = if let Some(sid) = resolved_sym {
                            let sym_scope = self.model.symbol(sid).scope_id;
                            (sym_scope, sid)
                        } else {
                            // If not found in any enclosing scope, created in Global scope
                            let root = ScopeId(0);
                            let sid =
                                if let Some(&existing) = self.model.scope(root).symbols.get(name) {
                                    existing
                                } else {
                                    self.add_symbol(root, name.clone(), *span)
                                };
                            (root, sid)
                        };

                        self.add_definition(
                            current_scope,
                            sym_id,
                            name.clone(),
                            *id,
                            *span,
                            DefKind::Variable(AssignmentKind::Super),
                        );

                        self.add_reference(
                            current_scope,
                            name.clone(),
                            *id,
                            Some(sym_id),
                            *span,
                            RefKind::Write,
                        );
                        let _ = target_scope;
                    }
                    LValue::Extraction { target, index, .. } => {
                        self.resolve_expr(target, current_scope);
                        self.resolve_expr(index, current_scope);
                    }
                }
            }
        }

        self.resolve_expr(&assign.value, current_scope);
    }

    fn resolve_expr(&mut self, expr: &Expr, current_scope: ScopeId) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Identifier(ident) => {
                let sc = self.model.scope(current_scope);
                if sc.is_data_mask() {
                    // In a data-masking context, bare identifiers may represent
                    // either dataframe columns or lexical variables.
                    let lexical_sym = self.model.resolve_lexical(current_scope, &ident.name);
                    self.add_reference(
                        current_scope,
                        ident.name.clone(),
                        ident.id,
                        lexical_sym,
                        ident.span,
                        RefKind::DataMaskBare,
                    );
                } else {
                    // Ordinary lexical reference
                    let sym_id = self.model.resolve_symbol(current_scope, &ident.name);
                    self.add_reference(
                        current_scope,
                        ident.name.clone(),
                        ident.id,
                        sym_id,
                        ident.span,
                        RefKind::Read,
                    );
                }
            }
            Expr::BinaryOp(bin) => {
                self.resolve_expr(&bin.left, current_scope);
                self.resolve_expr(&bin.right, current_scope);
            }
            Expr::UnaryOp(un) => {
                self.resolve_expr(&un.operand, current_scope);
            }
            Expr::Call(call) => {
                self.resolve_call(call, current_scope);
            }
            Expr::Extract(ext) => {
                // Check for explicit .data$col, .data[["col"]], .env$var
                if self
                    .check_explicit_data_mask_extraction(ext, current_scope)
                    .is_none()
                {
                    self.resolve_expr(&ext.target, current_scope);
                    self.resolve_expr(&ext.index, current_scope);
                }
            }
            Expr::Pipe(pipe) => {
                self.resolve_expr(&pipe.left, current_scope);
                self.resolve_expr(&pipe.right, current_scope);
            }
            Expr::FunctionDef(fn_def) => {
                let fn_scope = self.create_scope(
                    Some(current_scope),
                    ScopeKind::Function { node_id: fn_def.id },
                );
                for param in &fn_def.params {
                    let sym_id = self.add_symbol(fn_scope, param.name.clone(), param.span);
                    self.add_definition(
                        fn_scope,
                        sym_id,
                        param.name.clone(),
                        param.id,
                        param.span,
                        DefKind::Parameter,
                    );
                    if let Some(default_val) = &param.default_value {
                        self.resolve_expr(default_val, fn_scope);
                    }
                }
                for s in &fn_def.body.stmts {
                    self.resolve_stmt(s, fn_scope);
                }
            }
            Expr::Lambda(lambda) => {
                let fn_scope = self.create_scope(
                    Some(current_scope),
                    ScopeKind::Function { node_id: lambda.id },
                );
                for param in &lambda.params {
                    let sym_id = self.add_symbol(fn_scope, param.name.clone(), param.span);
                    self.add_definition(
                        fn_scope,
                        sym_id,
                        param.name.clone(),
                        param.id,
                        param.span,
                        DefKind::Parameter,
                    );
                    if let Some(default_val) = &param.default_value {
                        self.resolve_expr(default_val, fn_scope);
                    }
                }
                for s in &lambda.body.stmts {
                    self.resolve_stmt(s, fn_scope);
                }
            }
            Expr::Unknown(_) => {}
        }
    }

    fn check_explicit_data_mask_extraction(
        &mut self,
        ext: &ExtractExpr,
        current_scope: ScopeId,
    ) -> Option<()> {
        if let Expr::Identifier(target_id) = ext.target.as_ref() {
            if target_id.name == ".data" {
                let col_name = match ext.index.as_ref() {
                    Expr::Identifier(col_id) => col_id.name.clone(),
                    Expr::Literal(lit) => {
                        if let LiteralKind::String(s) = &lit.kind {
                            s.clone()
                        } else {
                            lit.raw.clone()
                        }
                    }
                    _ => return None,
                };

                self.add_reference(
                    current_scope,
                    format!(".data${col_name}"),
                    ext.id,
                    None,
                    ext.span,
                    RefKind::DataMaskExplicitData { column: col_name },
                );
                return Some(());
            } else if target_id.name == ".env" {
                let var_name = match ext.index.as_ref() {
                    Expr::Identifier(var_id) => var_id.name.clone(),
                    Expr::Literal(lit) => {
                        if let LiteralKind::String(s) = &lit.kind {
                            s.clone()
                        } else {
                            lit.raw.clone()
                        }
                    }
                    _ => return None,
                };

                let lexical_sym = self.model.resolve_lexical(current_scope, &var_name);
                self.add_reference(
                    current_scope,
                    format!(".env${var_name}"),
                    ext.id,
                    lexical_sym,
                    ext.span,
                    RefKind::DataMaskExplicitEnv { variable: var_name },
                );
                return Some(());
            }
        }
        None
    }

    fn resolve_call(&mut self, call: &CallExpr, current_scope: ScopeId) {
        // Check if callee is a known data-masking transformation verb
        let verb_name = match call.callee.as_ref() {
            Expr::Identifier(id) if is_data_mask_verb(&id.name) => Some(id.name.clone()),
            _ => None,
        };

        if let Some(verb) = verb_name {
            // Record call callee reference
            let callee_sym = self.model.resolve_symbol(current_scope, &verb);
            self.add_reference(
                current_scope,
                verb.clone(),
                call.id,
                callee_sym,
                call.callee.span(),
                RefKind::CallTarget,
            );

            // Create a DataMask scope
            let mask_scope = self.create_scope(
                Some(current_scope),
                ScopeKind::DataMask {
                    verb,
                    call_node_id: call.id,
                },
            );

            // In verbs like `mutate(df, total = price * quantity)`,
            // the first argument if positional is the dataset evaluated in current_scope.
            // Subsequent arguments (or all arguments if piped / named) are evaluated in mask_scope.
            let (data_arg, mask_args) =
                if call.args.len() >= 2 && matches!(call.args[0], Argument::Positional(_)) {
                    (Some(&call.args[0]), &call.args[1..])
                } else {
                    (None, &call.args[..])
                };

            if let Some(arg) = data_arg {
                self.resolve_expr(arg.value(), current_scope);
            }

            for arg in mask_args {
                match arg {
                    Argument::Positional(pos) => {
                        self.resolve_expr(&pos.value, mask_scope);
                    }
                    Argument::Named(named) => {
                        // In mutate(df, total = price * quantity), `total` is a column assignment
                        self.add_reference(
                            mask_scope,
                            named.name.clone(),
                            named.id,
                            None,
                            named.name_span,
                            RefKind::Write,
                        );
                        self.resolve_expr(&named.value, mask_scope);
                    }
                }
            }
        } else {
            // Normal function call
            if let Expr::Identifier(callee_id) = call.callee.as_ref() {
                let sym_id = self.model.resolve_symbol(current_scope, &callee_id.name);
                self.add_reference(
                    current_scope,
                    callee_id.name.clone(),
                    callee_id.id,
                    sym_id,
                    callee_id.span,
                    RefKind::CallTarget,
                );
            } else {
                self.resolve_expr(&call.callee, current_scope);
            }

            for arg in &call.args {
                match arg {
                    Argument::Positional(pos) => {
                        self.resolve_expr(&pos.value, current_scope);
                    }
                    Argument::Named(named) => {
                        // Rule: Named function arguments like foo(x = 10) do NOT create variable definitions
                        self.resolve_expr(&named.value, current_scope);
                    }
                }
            }
        }
    }
}

fn is_data_mask_verb(name: &str) -> bool {
    matches!(
        name,
        "mutate" | "filter" | "select" | "rename" | "transmute"
    )
}
