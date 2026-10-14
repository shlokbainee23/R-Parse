//! DataFrame analysis engine executing static schema transitions and column provenance.

use crate::ids::{ColumnVersionId, DataFrameId};
use crate::operations::{
    apply_base_r_column_assignment, apply_base_r_projection, apply_filter, apply_mutate,
    apply_rename, apply_select, apply_transmute, ColumnAssignmentTarget, StateRegistry,
    TransformationContext,
};
use crate::provenance::ProvenanceGraph;
use crate::state::{DataFrameState, DataFrameStateInit};
use crate::unknown::StaticKnowledge;
use rparse_core::{NodeId, Span};
use rparse_ir::{
    AssignmentStmt, CallExpr, Expr, ExtractKind, LValue, LiteralKind, PipeExpr, Program, Stmt,
};
use rparse_scope::{ScopeId, ScopeModel};
use std::collections::BTreeMap;

/// Seed definition for initializing a static dataframe schema for analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFrameSeed {
    /// The variable name bound to the dataframe.
    pub symbol_name: String,
    /// The column names present in the initial schema.
    pub columns: Vec<String>,
}

impl DataFrameSeed {
    pub fn new(symbol_name: impl Into<String>, columns: Vec<impl Into<String>>) -> Self {
        Self {
            symbol_name: symbol_name.into(),
            columns: columns.into_iter().map(|c| c.into()).collect(),
        }
    }
}

/// The complete static DataFrame analysis results.
#[derive(Debug, Clone, PartialEq)]
pub struct DataFrameAnalysis {
    pub registry: StateRegistry,
    pub active_symbol_states: BTreeMap<String, DataFrameId>,
}

impl DataFrameAnalysis {
    /// Looks up a dataframe state by its [`DataFrameId`].
    pub fn state(&self, id: DataFrameId) -> Option<&DataFrameState> {
        self.registry.get_state(id)
    }

    /// Looks up the current active dataframe state for a variable symbol name.
    pub fn active_state_for_symbol(&self, symbol_name: &str) -> Option<&DataFrameState> {
        self.active_symbol_states
            .get(symbol_name)
            .and_then(|&id| self.state(id))
    }

    /// Access the global column provenance graph.
    pub fn provenance(&self) -> &ProvenanceGraph {
        &self.registry.provenance
    }

    /// Access a column version by its [`ColumnVersionId`].
    pub fn column_version(&self, id: ColumnVersionId) -> Option<&crate::column::ColumnVersion> {
        self.registry.get_column_version(id)
    }

    /// Returns all dataframe states created during analysis in deterministic order.
    pub fn all_states(&self) -> Vec<&DataFrameState> {
        self.registry.states.values().collect()
    }

    /// Builds a map of ColumnVersionId -> human-readable label (e.g. "df$total@v1").
    pub fn build_column_label_map(&self) -> BTreeMap<ColumnVersionId, String> {
        let mut map = BTreeMap::new();
        for col in &self.registry.columns {
            let df_name = self
                .state(col.dataframe_id)
                .map(|s| s.symbol_name.as_str())
                .unwrap_or("df");
            map.insert(col.id, format!("{}${}@v{}", df_name, col.name, col.version));
        }
        map
    }

    /// Emits a deterministic Graphviz DOT string of the column provenance graph annotated with full column metadata.
    pub fn provenance_to_dot(&self) -> String {
        let label_map = self.build_column_label_map();
        self.provenance().to_dot_with_labels(&label_map)
    }

    /// Emits a deterministic structured JSON string of the column provenance graph annotated with full column metadata.
    pub fn provenance_to_json(&self) -> String {
        let label_map = self.build_column_label_map();
        self.provenance().to_json_with_labels(&label_map)
    }

    /// Emits a structured JSON value of the column provenance graph annotated with full column metadata.
    pub fn provenance_to_json_value(&self) -> rparse_core::JsonValue {
        let label_map = self.build_column_label_map();
        self.provenance().to_json_value_with_labels(&label_map)
    }
}

/// Runs Phase 4 DataFrame semantic analysis on a program given initial schema seeds.
pub fn analyze_dataframes(
    program: &Program,
    scope_model: &ScopeModel,
    seeds: &[DataFrameSeed],
) -> DataFrameAnalysis {
    let mut registry = StateRegistry::new();
    let mut active_symbol_states: BTreeMap<String, DataFrameId> = BTreeMap::new();

    // 1. Initialize seeded dataframe states
    for seed in seeds {
        let df_id = registry.next_df_id();
        let mut schema = BTreeMap::new();

        let (creation_node, span) = find_seed_definition(program, &seed.symbol_name)
            .unwrap_or((NodeId::new(0), Span::new(0, 0, 1, 0, 1, 0)));

        for col_name in &seed.columns {
            let col_id = registry.next_col_ver_id();
            let col_ver = crate::column::ColumnVersion::new(
                col_id,
                df_id,
                col_name.clone(),
                0,    // initial seed version 0
                None, // root seed has no parent provenance edge
                StaticKnowledge::KnownStatic,
            );
            registry.add_column_version(col_ver);
            schema.insert(col_name.clone(), col_id);
        }

        let symbol_id = scope_model.resolve_symbol(ScopeId(0), &seed.symbol_name);
        let state = DataFrameState::new(DataFrameStateInit {
            id: df_id,
            version: 0,
            symbol_id,
            symbol_name: seed.symbol_name.clone(),
            schema,
            creation_node,
            span,
            parent_state: None,
            row_selections: Vec::new(),
            knowledge: StaticKnowledge::KnownStatic,
        });
        registry.add_state(state);
        active_symbol_states.insert(seed.symbol_name.clone(), df_id);
    }

    // 2. Walk program statements in execution order
    let root_scope = ScopeId(0);
    process_stmts(
        &program.stmts,
        &mut registry,
        &mut active_symbol_states,
        scope_model,
        root_scope,
    );

    DataFrameAnalysis {
        registry,
        active_symbol_states,
    }
}

fn process_stmts(
    stmts: &[Stmt],
    registry: &mut StateRegistry,
    active_symbol_states: &mut BTreeMap<String, DataFrameId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
) {
    for stmt in stmts {
        process_stmt(
            stmt,
            registry,
            active_symbol_states,
            scope_model,
            current_scope,
        );
    }
}

fn process_stmt(
    stmt: &Stmt,
    registry: &mut StateRegistry,
    active_symbol_states: &mut BTreeMap<String, DataFrameId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
) {
    match stmt {
        Stmt::Assignment(assign) => {
            process_assignment(
                assign,
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            );
        }
        Stmt::If(if_stmt) => {
            // Branch-aware conservative handling (Section 27)
            let mut then_states = active_symbol_states.clone();
            process_stmts(
                &if_stmt.then_branch.stmts,
                registry,
                &mut then_states,
                scope_model,
                current_scope,
            );

            let mut else_states = active_symbol_states.clone();
            if let Some(else_branch) = &if_stmt.else_branch {
                process_stmts(
                    &else_branch.stmts,
                    registry,
                    &mut else_states,
                    scope_model,
                    current_scope,
                );
            }

            // Reconcile branch states at merge site
            for (sym, then_id) in then_states {
                if let Some(&else_id) = else_states.get(&sym) {
                    if then_id == else_id {
                        active_symbol_states.insert(sym, then_id);
                    } else {
                        // Differing states along branches: inspect schema agreement
                        let s_then = registry.get_state(then_id).cloned();
                        let s_else = registry.get_state(else_id).cloned();

                        if let (Some(st_then), Some(st_else)) = (s_then, s_else) {
                            if st_then.schema == st_else.schema {
                                active_symbol_states.insert(sym, then_id);
                            } else {
                                // Differing column schema -> conservative merge with UNKNOWN_STATIC (Section 27)
                                let merged_df_id = registry.next_df_id();
                                let mut merged_schema = BTreeMap::new();

                                for (c, id) in &st_then.schema {
                                    if st_else.schema.get(c) == Some(id) {
                                        merged_schema.insert(c.clone(), *id);
                                    }
                                }

                                let merged_state = DataFrameState::new(DataFrameStateInit {
                                    id: merged_df_id,
                                    version: st_then.version.max(st_else.version) + 1,
                                    symbol_id: st_then.symbol_id,
                                    symbol_name: sym.clone(),
                                    schema: merged_schema,
                                    creation_node: if_stmt.id,
                                    span: if_stmt.span,
                                    parent_state: Some(then_id),
                                    row_selections: st_then.row_selections,
                                    knowledge: StaticKnowledge::UnknownStatic,
                                });
                                registry.add_state(merged_state);
                                active_symbol_states.insert(sym, merged_df_id);
                            }
                        }
                    }
                }
            }
        }
        Stmt::For(for_stmt) => {
            process_stmts(
                &for_stmt.body.stmts,
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            );
        }
        Stmt::While(while_stmt) => {
            process_stmts(
                &while_stmt.body.stmts,
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            );
        }
        Stmt::Block(block) => {
            process_stmts(
                &block.stmts,
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            );
        }
        Stmt::Expr(_) | Stmt::Return(_) => {}
    }
}

fn process_assignment(
    assign: &AssignmentStmt,
    registry: &mut StateRegistry,
    active_symbol_states: &mut BTreeMap<String, DataFrameId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
) {
    match &assign.target {
        LValue::Symbol { name, .. } => {
            // RHS might be a dataframe transformation or pipeline
            if let Some(new_state) = evaluate_transformation_expr(
                &assign.value,
                name.clone(),
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            ) {
                active_symbol_states.insert(name.clone(), new_state.id);
            } else if let Expr::Identifier(src_id) = assign.value.as_ref() {
                // Simple dataframe aliasing: target <- source_df
                if let Some(&src_df_id) = active_symbol_states.get(&src_id.name) {
                    if let Some(src_state) = registry.get_state(src_df_id).cloned() {
                        let alias_df_id = registry.next_df_id();
                        let symbol_id = scope_model.resolve_symbol(current_scope, name);
                        let alias_state = DataFrameState::new(DataFrameStateInit {
                            id: alias_df_id,
                            version: src_state.version,
                            symbol_id,
                            symbol_name: name.clone(),
                            schema: src_state.schema.clone(),
                            creation_node: assign.id,
                            span: assign.span,
                            parent_state: Some(src_state.id),
                            row_selections: src_state.row_selections.clone(),
                            knowledge: src_state.knowledge,
                        });
                        registry.add_state(alias_state);
                        active_symbol_states.insert(name.clone(), alias_df_id);
                    }
                }
            }
        }
        LValue::Extraction {
            target,
            index,
            kind: ExtractKind::Dollar,
            span,
            id,
        } => {
            // Base R direct column assignment: df$col <- expr
            if let Expr::Identifier(target_id) = target.as_ref() {
                if let Some(&df_id) = active_symbol_states.get(&target_id.name) {
                    if let Some(df_state) = registry.get_state(df_id).cloned() {
                        if let Some(col_name) = extract_static_col_name(index) {
                            let known_dfs = build_known_dfs(registry, active_symbol_states);
                            let target = ColumnAssignmentTarget {
                                col_name: &col_name,
                                output_symbol_name: target_id.name.clone(),
                                stmt_id: *id,
                                span: *span,
                            };
                            let ctx = TransformationContext {
                                scope_model,
                                current_scope,
                                known_dataframes: &known_dfs,
                            };
                            let new_state = apply_base_r_column_assignment(
                                &df_state,
                                target,
                                &assign.value,
                                ctx,
                                registry,
                            );
                            active_symbol_states.insert(target_id.name.clone(), new_state.id);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn evaluate_transformation_expr(
    expr: &Expr,
    output_symbol_name: String,
    registry: &mut StateRegistry,
    active_symbol_states: &BTreeMap<String, DataFrameId>,
    scope_model: &ScopeModel,
    current_scope: ScopeId,
) -> Option<DataFrameState> {
    match expr {
        // Pipeline: left %>% right or left |> right
        Expr::Pipe(PipeExpr { left, right, .. }) => {
            let input_state = match evaluate_transformation_expr(
                left,
                output_symbol_name.clone(),
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            ) {
                Some(s) => s,
                None => match left.as_ref() {
                    Expr::Identifier(id) => {
                        let df_id = *active_symbol_states.get(&id.name)?;
                        registry.get_state(df_id)?.clone()
                    }
                    _ => return None,
                },
            };

            // Evaluate right with piped input
            if let Expr::Call(call) = right.as_ref() {
                let mut ctx = CallVerbContext {
                    registry,
                    active_symbol_states,
                    scope_model,
                    current_scope,
                };
                apply_call_verb(
                    &input_state,
                    call,
                    0, // piped call: dataframe is not in args
                    output_symbol_name,
                    &mut ctx,
                )
            } else {
                None
            }
        }

        // Direct function call: mutate(df, ...) or transform(df, ...)
        Expr::Call(call) => {
            if !matches!(call.callee.as_ref(), Expr::Identifier(_)) {
                return None;
            }

            if call.args.is_empty() {
                return None;
            }

            // First argument is the input dataframe
            let first_arg = call.args[0].value();
            let input_state = match first_arg {
                Expr::Identifier(id) => {
                    let df_id = *active_symbol_states.get(&id.name)?;
                    registry.get_state(df_id)?.clone()
                }
                _ => return None,
            };

            let mut ctx = CallVerbContext {
                registry,
                active_symbol_states,
                scope_model,
                current_scope,
            };
            apply_call_verb(
                &input_state,
                call,
                1, // direct call: skip first dataframe arg
                output_symbol_name,
                &mut ctx,
            )
        }

        // Base R projection: df[, c("a", "b")]
        Expr::Extract(extract) => {
            if let Expr::Identifier(id) = extract.target.as_ref() {
                if let Some(&df_id) = active_symbol_states.get(&id.name) {
                    if let Some(df_state) = registry.get_state(df_id).cloned() {
                        return Some(apply_base_r_projection(
                            &df_state,
                            extract,
                            extract.id,
                            output_symbol_name,
                            registry,
                        ));
                    }
                }
            }
            None
        }

        _ => None,
    }
}

struct CallVerbContext<'a> {
    pub registry: &'a mut StateRegistry,
    pub active_symbol_states: &'a BTreeMap<String, DataFrameId>,
    pub scope_model: &'a ScopeModel,
    pub current_scope: ScopeId,
}

fn apply_call_verb(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    ctx: &mut CallVerbContext<'_>,
) -> Option<DataFrameState> {
    let fn_name = match call.callee.as_ref() {
        Expr::Identifier(id) => id.name.as_str(),
        _ => return None,
    };

    let known_dfs = build_known_dfs(ctx.registry, ctx.active_symbol_states);
    let trans_ctx = TransformationContext {
        scope_model: ctx.scope_model,
        current_scope: ctx.current_scope,
        known_dataframes: &known_dfs,
    };

    match fn_name {
        "mutate" => Some(apply_mutate(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            trans_ctx,
            ctx.registry,
            false,
        )),
        "transform" => Some(apply_mutate(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            trans_ctx,
            ctx.registry,
            true,
        )),
        "transmute" => Some(apply_transmute(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            trans_ctx,
            ctx.registry,
        )),
        "select" => Some(apply_select(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            ctx.registry,
        )),
        "rename" => Some(apply_rename(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            ctx.registry,
        )),
        "filter" => Some(apply_filter(
            input_state,
            call,
            args_start_index,
            output_symbol_name,
            trans_ctx,
            ctx.registry,
        )),
        _ => None,
    }
}

fn build_known_dfs(
    registry: &StateRegistry,
    active_symbol_states: &BTreeMap<String, DataFrameId>,
) -> BTreeMap<String, DataFrameState> {
    let mut map = BTreeMap::new();
    for (sym, &df_id) in active_symbol_states {
        if let Some(st) = registry.get_state(df_id) {
            map.insert(sym.clone(), st.clone());
        }
    }
    map
}

fn extract_static_col_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(id) => Some(id.name.clone()),
        Expr::Literal(lit) => match &lit.kind {
            LiteralKind::String(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn find_seed_definition(program: &Program, symbol_name: &str) -> Option<(NodeId, Span)> {
    for stmt in &program.stmts {
        if let Stmt::Assignment(assign) = stmt {
            if let LValue::Symbol { name, .. } = &assign.target {
                if name == symbol_name {
                    return Some((assign.id, assign.span));
                }
            }
        }
    }
    None
}
