//! Semantic transformation handlers for the frozen MVP verbs.
//!
//! Tidyverse: mutate, transmute, select, rename, filter.
//! Base R: df$col <- expr, df[, c(...)], transform(df, ...).

use crate::dependency::RowSelectionDependency;
use crate::expression::extract_expression_dependencies;
use crate::ids::{ColumnVersionId, DataFrameId, ProvenanceEdgeId};
use crate::provenance::{ProvenanceEdge, ProvenanceEdgeInit, ProvenanceGraph, TransformationType};
use crate::state::{DataFrameState, DataFrameStateInit};
use crate::unknown::StaticKnowledge;
use rparse_core::{NodeId, Span};
use rparse_ir::{Argument, CallExpr, Expr, ExtractExpr, LiteralKind};
use rparse_scope::{ScopeId, ScopeModel};
use std::collections::BTreeMap;

/// Context for resolving symbols and extracting dependencies during dataframe transformations.
#[derive(Clone, Copy)]
pub struct TransformationContext<'a> {
    pub scope_model: &'a ScopeModel,
    pub current_scope: ScopeId,
    pub known_dataframes: &'a BTreeMap<String, DataFrameState>,
}

/// Target specification for Base R direct column assignment (`df$col <- expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnAssignmentTarget<'a> {
    pub col_name: &'a str,
    pub output_symbol_name: String,
    pub stmt_id: NodeId,
    pub span: Span,
}

/// Shared registry for allocating IDs and storing column versions and provenance edges.
#[derive(Debug, Clone, PartialEq)]
pub struct StateRegistry {
    next_df_id: u32,
    next_col_ver_id: u32,
    next_pedge_id: u32,
    pub columns: Vec<crate::column::ColumnVersion>,
    pub provenance: ProvenanceGraph,
    pub states: BTreeMap<DataFrameId, DataFrameState>,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            next_df_id: 0,
            next_col_ver_id: 0,
            next_pedge_id: 0,
            columns: Vec::new(),
            provenance: ProvenanceGraph::new(),
            states: BTreeMap::new(),
        }
    }

    pub fn next_df_id(&mut self) -> DataFrameId {
        let id = DataFrameId(self.next_df_id);
        self.next_df_id += 1;
        id
    }

    pub fn next_col_ver_id(&mut self) -> ColumnVersionId {
        let id = ColumnVersionId(self.next_col_ver_id);
        self.next_col_ver_id += 1;
        id
    }

    pub fn next_pedge_id(&mut self) -> ProvenanceEdgeId {
        let id = ProvenanceEdgeId(self.next_pedge_id);
        self.next_pedge_id += 1;
        id
    }

    pub fn add_column_version(&mut self, col: crate::column::ColumnVersion) {
        self.columns.push(col);
    }

    pub fn get_column_version(&self, id: ColumnVersionId) -> Option<&crate::column::ColumnVersion> {
        self.columns.get(id.0 as usize)
    }

    pub fn add_state(&mut self, state: DataFrameState) {
        self.states.insert(state.id, state);
    }

    pub fn get_state(&self, id: DataFrameId) -> Option<&DataFrameState> {
        self.states.get(&id)
    }
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies `mutate(...)` or `transform(...)` semantics.
///
/// Evaluates arguments strictly LEFT-TO-RIGHT.
pub fn apply_mutate(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    ctx: TransformationContext<'_>,
    registry: &mut StateRegistry,
    is_transform: bool,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut working_schema = input_state.schema.clone();
    let mut overall_knowledge = input_state.knowledge;

    for arg in call.args.iter().skip(args_start_index) {
        if let Argument::Named(named) = arg {
            let col_name = named.name.clone();
            let expr = &named.value;

            // Extract dependencies using current working schema
            let deps = extract_expression_dependencies(
                expr,
                &working_schema,
                ctx.scope_model,
                ctx.current_scope,
                ctx.known_dataframes,
            );

            if deps.unknown {
                overall_knowledge = StaticKnowledge::UnknownStatic;
            }

            // Determine version number
            let version = if let Some(&prev_id) = working_schema.get(&col_name) {
                let prev_ver = registry
                    .get_column_version(prev_id)
                    .map(|c| c.version)
                    .unwrap_or(0);
                prev_ver + 1
            } else {
                0
            };

            let new_col_id = registry.next_col_ver_id();
            let edge_id = registry.next_pedge_id();

            let transform_type = if is_transform {
                TransformationType::Transform
            } else {
                TransformationType::Mutate
            };

            let edge_knowledge = if deps.unknown {
                StaticKnowledge::UnknownStatic
            } else {
                StaticKnowledge::KnownStatic
            };

            let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
                id: edge_id,
                input_columns: deps.columns,
                input_variables: deps.variables,
                output_column: new_col_id,
                expression_node: Some(expr.id()),
                span: expr.span(),
                transform_type,
                knowledge: edge_knowledge,
            });
            registry.provenance.add_edge(edge);

            let col_ver = crate::column::ColumnVersion::new(
                new_col_id,
                output_df_id,
                col_name.clone(),
                version,
                Some(edge_id),
                edge_knowledge,
            );
            registry.add_column_version(col_ver);

            // Update active working schema immediately for subsequent arguments
            working_schema.insert(col_name, new_col_id);
        }
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: working_schema,
        creation_node: call.id,
        span: call.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge: overall_knowledge,
    });
    registry.add_state(state.clone());
    state
}

/// Applies `transmute(...)` semantics.
///
/// Evaluates arguments left-to-right, but final output schema contains ONLY explicitly produced columns.
pub fn apply_transmute(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    ctx: TransformationContext<'_>,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut working_schema = input_state.schema.clone();
    let mut final_schema = BTreeMap::new();
    let mut overall_knowledge = input_state.knowledge;

    for arg in call.args.iter().skip(args_start_index) {
        if let Argument::Named(named) = arg {
            let col_name = named.name.clone();
            let expr = &named.value;

            let deps = extract_expression_dependencies(
                expr,
                &working_schema,
                ctx.scope_model,
                ctx.current_scope,
                ctx.known_dataframes,
            );

            if deps.unknown {
                overall_knowledge = StaticKnowledge::UnknownStatic;
            }

            let version = if let Some(&prev_id) = working_schema.get(&col_name) {
                let prev_ver = registry
                    .get_column_version(prev_id)
                    .map(|c| c.version)
                    .unwrap_or(0);
                prev_ver + 1
            } else {
                0
            };

            let new_col_id = registry.next_col_ver_id();
            let edge_id = registry.next_pedge_id();

            let edge_knowledge = if deps.unknown {
                StaticKnowledge::UnknownStatic
            } else {
                StaticKnowledge::KnownStatic
            };

            let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
                id: edge_id,
                input_columns: deps.columns,
                input_variables: deps.variables,
                output_column: new_col_id,
                expression_node: Some(expr.id()),
                span: expr.span(),
                transform_type: TransformationType::Transmute,
                knowledge: edge_knowledge,
            });
            registry.provenance.add_edge(edge);

            let col_ver = crate::column::ColumnVersion::new(
                new_col_id,
                output_df_id,
                col_name.clone(),
                version,
                Some(edge_id),
                edge_knowledge,
            );
            registry.add_column_version(col_ver);

            working_schema.insert(col_name.clone(), new_col_id);
            final_schema.insert(col_name, new_col_id);
        }
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: final_schema,
        creation_node: call.id,
        span: call.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge: overall_knowledge,
    });
    registry.add_state(state.clone());
    state
}

/// Applies `select(...)` semantics.
///
/// Output schema contains only selected columns; retained columns preserve existing ColumnVersionIds.
pub fn apply_select(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut output_schema = BTreeMap::new();
    let mut overall_knowledge = input_state.knowledge;

    for arg in call.args.iter().skip(args_start_index) {
        let expr = arg.value();
        let selected_names = extract_selected_names(expr);

        if selected_names.is_empty() {
            // Dynamic selector helper not statically resolved -> emit UNKNOWN_STATIC
            overall_knowledge = StaticKnowledge::UnknownStatic;
        } else {
            for name in selected_names {
                if let Some(&col_ver) = input_state.schema.get(&name) {
                    output_schema.insert(name, col_ver);
                }
            }
        }
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: output_schema,
        creation_node: call.id,
        span: call.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge: overall_knowledge,
    });
    registry.add_state(state.clone());
    state
}

/// Applies `rename(...)` semantics.
///
/// Evaluates mappings against input schema; creates renamed column versions linked to old ancestry.
pub fn apply_rename(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut output_schema = input_state.schema.clone();
    let mut overall_knowledge = input_state.knowledge;

    for arg in call.args.iter().skip(args_start_index) {
        if let Argument::Named(named) = arg {
            let new_name = named.name.clone();
            let old_name = extract_static_col_name(&named.value);

            if let Some(old) = old_name {
                if let Some(&old_col_id) = input_state.schema.get(&old) {
                    let old_ver = registry
                        .get_column_version(old_col_id)
                        .map(|c| c.version)
                        .unwrap_or(0);
                    let new_col_id = registry.next_col_ver_id();
                    let edge_id = registry.next_pedge_id();

                    let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
                        id: edge_id,
                        input_columns: [old_col_id].into_iter().collect(),
                        input_variables: std::collections::BTreeSet::new(),
                        output_column: new_col_id,
                        expression_node: Some(named.value.id()),
                        span: named.value.span(),
                        transform_type: TransformationType::Rename {
                            old_name: old.clone(),
                        },
                        knowledge: StaticKnowledge::KnownStatic,
                    });
                    registry.provenance.add_edge(edge);

                    let col_ver = crate::column::ColumnVersion::new(
                        new_col_id,
                        output_df_id,
                        new_name.clone(),
                        old_ver + 1,
                        Some(edge_id),
                        StaticKnowledge::KnownStatic,
                    );
                    registry.add_column_version(col_ver);

                    output_schema.remove(&old);
                    output_schema.insert(new_name, new_col_id);
                } else {
                    overall_knowledge = StaticKnowledge::UnknownStatic;
                }
            } else {
                overall_knowledge = StaticKnowledge::UnknownStatic;
            }
        }
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: output_schema,
        creation_node: call.id,
        span: call.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge: overall_knowledge,
    });
    registry.add_state(state.clone());
    state
}

/// Applies `filter(...)` semantics.
///
/// Column schema carried forward without modification.
/// Row-selection dependencies are created and chained.
pub fn apply_filter(
    input_state: &DataFrameState,
    call: &CallExpr,
    args_start_index: usize,
    output_symbol_name: String,
    ctx: TransformationContext<'_>,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut row_selections = input_state.row_selections.clone();
    let mut overall_knowledge = input_state.knowledge;

    for arg in call.args.iter().skip(args_start_index) {
        let pred_expr = arg.value();
        let deps = extract_expression_dependencies(
            pred_expr,
            &input_state.schema,
            ctx.scope_model,
            ctx.current_scope,
            ctx.known_dataframes,
        );

        let pred_knowledge = if deps.unknown {
            overall_knowledge = StaticKnowledge::UnknownStatic;
            StaticKnowledge::UnknownStatic
        } else {
            StaticKnowledge::KnownStatic
        };

        let row_dep = RowSelectionDependency {
            dataframe_id: output_df_id,
            predicate_columns: deps.columns,
            predicate_variables: deps.variables,
            predicate_node: pred_expr.id(),
            span: pred_expr.span(),
            knowledge: pred_knowledge,
        };
        row_selections.push(row_dep);
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: input_state.schema.clone(),
        creation_node: call.id,
        span: call.span,
        parent_state: Some(input_state.id),
        row_selections,
        knowledge: overall_knowledge,
    });
    registry.add_state(state.clone());
    state
}

/// Applies Base R direct column assignment: `df$col <- expr`.
pub fn apply_base_r_column_assignment(
    input_state: &DataFrameState,
    target: ColumnAssignmentTarget<'_>,
    expr: &Expr,
    ctx: TransformationContext<'_>,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut new_schema = input_state.schema.clone();

    let deps = extract_expression_dependencies(
        expr,
        &input_state.schema,
        ctx.scope_model,
        ctx.current_scope,
        ctx.known_dataframes,
    );

    let version = if let Some(&prev_id) = input_state.schema.get(target.col_name) {
        let prev_ver = registry
            .get_column_version(prev_id)
            .map(|c| c.version)
            .unwrap_or(0);
        prev_ver + 1
    } else {
        0
    };

    let new_col_id = registry.next_col_ver_id();
    let edge_id = registry.next_pedge_id();

    let edge_knowledge = if deps.unknown {
        StaticKnowledge::UnknownStatic
    } else {
        StaticKnowledge::KnownStatic
    };

    let edge = ProvenanceEdge::new(ProvenanceEdgeInit {
        id: edge_id,
        input_columns: deps.columns,
        input_variables: deps.variables,
        output_column: new_col_id,
        expression_node: Some(expr.id()),
        span: expr.span(),
        transform_type: TransformationType::BaseRDirectAssignment,
        knowledge: edge_knowledge,
    });
    registry.provenance.add_edge(edge);

    let col_ver = crate::column::ColumnVersion::new(
        new_col_id,
        output_df_id,
        target.col_name.to_string(),
        version,
        Some(edge_id),
        edge_knowledge,
    );
    registry.add_column_version(col_ver);

    new_schema.insert(target.col_name.to_string(), new_col_id);

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: target.output_symbol_name,
        schema: new_schema,
        creation_node: target.stmt_id,
        span: target.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge: input_state.knowledge.combine(edge_knowledge),
    });
    registry.add_state(state.clone());
    state
}

/// Applies Base R subsetting projection: `df[, c("a", "b")]`.
pub fn apply_base_r_projection(
    input_state: &DataFrameState,
    extract: &ExtractExpr,
    stmt_id: NodeId,
    output_symbol_name: String,
    registry: &mut StateRegistry,
) -> DataFrameState {
    let output_df_id = registry.next_df_id();
    let mut output_schema = BTreeMap::new();
    let mut knowledge = input_state.knowledge;

    let selected_names = extract_selected_names(&extract.index);
    if selected_names.is_empty() {
        knowledge = StaticKnowledge::UnknownStatic;
    } else {
        for name in selected_names {
            if let Some(&col_id) = input_state.schema.get(&name) {
                output_schema.insert(name, col_id);
            }
        }
    }

    let state = DataFrameState::new(DataFrameStateInit {
        id: output_df_id,
        version: input_state.version + 1,
        symbol_id: input_state.symbol_id,
        symbol_name: output_symbol_name,
        schema: output_schema,
        creation_node: stmt_id,
        span: extract.span,
        parent_state: Some(input_state.id),
        row_selections: input_state.row_selections.clone(),
        knowledge,
    });
    registry.add_state(state.clone());
    state
}

fn extract_selected_names(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Identifier(id) => vec![id.name.clone()],
        Expr::Literal(lit) => match &lit.kind {
            LiteralKind::String(s) => vec![s.clone()],
            _ => Vec::new(),
        },
        Expr::Call(call) => {
            let fn_name = match call.callee.as_ref() {
                Expr::Identifier(id) => id.name.as_str(),
                _ => "",
            };
            if fn_name == "c" || fn_name == "all_of" {
                let mut names = Vec::new();
                for arg in &call.args {
                    names.extend(extract_selected_names(arg.value()));
                }
                names
            } else {
                Vec::new()
            }
        }
        Expr::Unknown(u) => {
            if let Some(raw) = &u.raw {
                let mut names = Vec::new();
                let mut in_quote = false;
                let mut quote_char = '"';
                let mut current = String::new();
                for c in raw.chars() {
                    if in_quote {
                        if c == quote_char {
                            in_quote = false;
                            names.push(current.clone());
                            current.clear();
                        } else {
                            current.push(c);
                        }
                    } else if c == '"' || c == '\'' {
                        in_quote = true;
                        quote_char = c;
                    }
                }
                names
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
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
