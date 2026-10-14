//! Backward slicing engine over the Unified Program Dependence Graph.

use crate::criterion::{resolve_criterion, CriterionResolutionError, SliceCriterion};
use crate::element::{RetentionState, SliceElement, SubElementSlice};
use crate::result::{SliceConfidence, SliceResult, StatementSlice};
use rparse_core::NodeId;
use rparse_dataframe::{ColumnVersionId, DataFrameAnalysis, StaticKnowledge};
use rparse_ir::{Argument, Expr, NamedArg, Program, Stmt};
use rparse_pdg::{PdgNodeId, PdgNodeKind, ProgramDependenceGraph};
use rparse_scope::ScopeModel;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// Error returned when slice computation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlicingError {
    CriterionResolution(CriterionResolutionError),
    NoInitialNodes,
}

impl fmt::Display for SlicingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CriterionResolution(e) => write!(f, "failed to resolve criterion: {}", e),
            Self::NoInitialNodes => write!(f, "resolved criterion produced zero seed PDG nodes"),
        }
    }
}

impl std::error::Error for SlicingError {}

impl From<CriterionResolutionError> for SlicingError {
    fn from(e: CriterionResolutionError) -> Self {
        Self::CriterionResolution(e)
    }
}

/// Computes the column-aware static backward slice for a given criterion.
pub fn compute_backward_slice(
    criterion: &SliceCriterion,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
) -> Result<SliceResult, SlicingError> {
    compute_backward_slice_with_source(criterion, pdg, df_analysis, scope_model, None, None)
}

/// Computes the column-aware static backward slice with optional AST and source text for sub-statement attribution.
pub fn compute_backward_slice_with_source(
    criterion: &SliceCriterion,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
    program: Option<&Program>,
    source_text: Option<&str>,
) -> Result<SliceResult, SlicingError> {
    let seeds = resolve_criterion(criterion, pdg, df_analysis, scope_model)?;
    if seeds.is_empty() {
        return Err(SlicingError::NoInitialNodes);
    }

    // 1. Graph-based fixed-point backward traversal
    let mut visited: BTreeSet<PdgNodeId> = BTreeSet::new();
    let mut worklist: VecDeque<PdgNodeId> = VecDeque::new();

    for seed in seeds {
        if visited.insert(seed) {
            worklist.push_back(seed);
        }
    }

    while let Some(curr) = worklist.pop_front() {
        // Follow all incoming dependency edges (from source to curr)
        for &edge_id in pdg.incoming_edges(curr) {
            if let Some(edge) = pdg.edge(edge_id) {
                let src = edge.from;
                if visited.insert(src) {
                    worklist.push_back(src);
                }
            }
        }
    }

    // 2. Identify uncertainty metadata
    let mut contains_unknowns = false;
    for &node_id in &visited {
        if let Some(node) = pdg.node(node_id) {
            if node.is_unknown() {
                contains_unknowns = true;
            } else if let PdgNodeKind::ColumnVersion { knowledge, .. } = &node.kind {
                if *knowledge == StaticKnowledge::UnknownStatic {
                    contains_unknowns = true;
                }
            } else if let PdgNodeKind::RowSelection { knowledge, .. } = &node.kind {
                if *knowledge == StaticKnowledge::UnknownStatic {
                    contains_unknowns = true;
                }
            }
        }
    }

    // 3. Identify retained vs excluded dataframe column versions
    let mut retained_column_versions: BTreeSet<ColumnVersionId> = BTreeSet::new();
    let mut participating_dfs = BTreeSet::new();

    for &node_id in &visited {
        if let Some(node) = pdg.node(node_id) {
            if let PdgNodeKind::ColumnVersion { col_id, df_id, .. } = &node.kind {
                retained_column_versions.insert(*col_id);
                participating_dfs.insert(*df_id);
            }
        }
    }

    let mut excluded_column_versions: BTreeSet<ColumnVersionId> = BTreeSet::new();
    for &df_id in &participating_dfs {
        if let Some(st) = df_analysis.state(df_id) {
            for &cid in st.schema.values() {
                if !retained_column_versions.contains(&cid) {
                    excluded_column_versions.insert(cid);
                }
            }
        }
    }

    // 4. Compute fine-grained statement retention
    let mut statement_slices: BTreeMap<NodeId, StatementSlice> = BTreeMap::new();
    let mut stmt_subnodes: BTreeMap<NodeId, Vec<PdgNodeId>> = BTreeMap::new();

    // Group PDG nodes by statement
    for node in pdg.nodes() {
        match &node.kind {
            PdgNodeKind::Statement { stmt_id, .. } => {
                stmt_subnodes.entry(*stmt_id).or_default().push(node.id);
            }
            PdgNodeKind::Expression { .. } => {
                // Find statement containing this expression
                for stmt_node in pdg.nodes() {
                    if let PdgNodeKind::Statement { stmt_id, .. } = &stmt_node.kind {
                        if stmt_node.span.contains_span(&node.span) {
                            stmt_subnodes.entry(*stmt_id).or_default().push(node.id);
                            break;
                        }
                    }
                }
            }
            PdgNodeKind::VariableDef { .. } | PdgNodeKind::VariableUse { .. } => {
                for stmt_node in pdg.nodes() {
                    if let PdgNodeKind::Statement { stmt_id, .. } = &stmt_node.kind {
                        if stmt_node.span.contains_span(&node.span) {
                            stmt_subnodes.entry(*stmt_id).or_default().push(node.id);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    for node in pdg.nodes() {
        if let PdgNodeKind::Statement { stmt_id, .. } = &node.kind {
            let subnodes = stmt_subnodes.get(stmt_id).cloned().unwrap_or_default();
            let mut retained_sub = Vec::new();
            let mut excluded_sub = Vec::new();

            for &sn in &subnodes {
                if visited.contains(&sn) {
                    retained_sub.push(sn);
                } else {
                    excluded_sub.push(sn);
                }
            }

            // Check if AST has named arguments in this statement (e.g. mutate(total = ..., unused = ...))
            let named_args = program
                .and_then(|p| find_statement_in_program(p, *stmt_id))
                .map(collect_named_args_in_stmt)
                .unwrap_or_default();

            let mut retained_sub_regions = Vec::new();
            let mut excluded_sub_regions = Vec::new();
            let mut sub_elements = Vec::new();

            if !named_args.is_empty() {
                let target_sym = program
                    .and_then(|p| find_statement_in_program(p, *stmt_id))
                    .and_then(get_stmt_target_name);

                for named in named_args {
                    let is_retained = if let Some(expr_node_id) = pdg.expr_node(named.value.id()) {
                        visited.contains(&expr_node_id)
                    } else if let Some(pedge) = df_analysis
                        .provenance()
                        .edges()
                        .iter()
                        .find(|e| e.expression_node == Some(named.value.id()))
                    {
                        retained_column_versions.contains(&pedge.output_column)
                    } else if let Some(target_name) = &target_sym {
                        let all_states = df_analysis.all_states();
                        if let Some(col_id) = all_states
                            .iter()
                            .find(|s| s.creation_node == *stmt_id && &s.symbol_name == target_name)
                            .or_else(|| {
                                all_states
                                    .iter()
                                    .rev()
                                    .find(|s| &s.symbol_name == target_name)
                            })
                            .and_then(|s| s.column(&named.name))
                        {
                            retained_column_versions.contains(&col_id)
                        } else {
                            retained_sub.iter().any(|&sn| {
                                if let Some(n) = pdg.node(sn) {
                                    named.span.contains_span(&n.span)
                                } else {
                                    false
                                }
                            })
                        }
                    } else {
                        retained_sub.iter().any(|&sn| {
                            if let Some(n) = pdg.node(sn) {
                                named.span.contains_span(&n.span)
                            } else {
                                false
                            }
                        })
                    };

                    let label = if let Some(src) = source_text {
                        if (named.span.start_byte as usize) < src.len()
                            && (named.span.end_byte as usize) <= src.len()
                            && named.span.start_byte < named.span.end_byte
                        {
                            src[named.span.start_byte as usize..named.span.end_byte as usize]
                                .trim()
                                .to_string()
                        } else {
                            format!("{} = <expr>", named.name)
                        }
                    } else {
                        format!("{} = <expr>", named.name)
                    };

                    if is_retained {
                        retained_sub_regions.push(named.span);
                        sub_elements.push(SubElementSlice::new(
                            label,
                            RetentionState::FullyRetained,
                            Some(named.span),
                        ));
                    } else {
                        excluded_sub_regions.push(named.span);
                        sub_elements.push(SubElementSlice::new(
                            label,
                            RetentionState::NotRetained,
                            Some(named.span),
                        ));
                    }
                }
            } else {
                // Attribution based on PDG sub-nodes
                for &sn in &retained_sub {
                    if sn != node.id {
                        if let Some(n) = pdg.node(sn) {
                            if n.is_expression() && !retained_sub_regions.contains(&n.span) {
                                retained_sub_regions.push(n.span);
                                let label = if let Some(src) = source_text {
                                    if (n.span.start_byte as usize) < src.len()
                                        && (n.span.end_byte as usize) <= src.len()
                                        && n.span.start_byte < n.span.end_byte
                                    {
                                        src[n.span.start_byte as usize..n.span.end_byte as usize]
                                            .trim()
                                            .to_string()
                                    } else {
                                        n.kind.to_string()
                                    }
                                } else {
                                    n.kind.to_string()
                                };
                                sub_elements.push(SubElementSlice::new(
                                    label,
                                    RetentionState::FullyRetained,
                                    Some(n.span),
                                ));
                            }
                        }
                    }
                }

                for &sn in &excluded_sub {
                    if sn != node.id {
                        if let Some(n) = pdg.node(sn) {
                            if n.is_expression() && !excluded_sub_regions.contains(&n.span) {
                                excluded_sub_regions.push(n.span);
                                let label = if let Some(src) = source_text {
                                    if (n.span.start_byte as usize) < src.len()
                                        && (n.span.end_byte as usize) <= src.len()
                                        && n.span.start_byte < n.span.end_byte
                                    {
                                        src[n.span.start_byte as usize..n.span.end_byte as usize]
                                            .trim()
                                            .to_string()
                                    } else {
                                        n.kind.to_string()
                                    }
                                } else {
                                    n.kind.to_string()
                                };
                                sub_elements.push(SubElementSlice::new(
                                    label,
                                    RetentionState::NotRetained,
                                    Some(n.span),
                                ));
                            }
                        }
                    }
                }
            }

            let retention = if retained_sub.is_empty() && !visited.contains(&node.id) {
                RetentionState::NotRetained
            } else if excluded_sub.is_empty() && excluded_sub_regions.is_empty() {
                RetentionState::FullyRetained
            } else {
                RetentionState::PartiallyRetained
            };

            statement_slices.insert(
                *stmt_id,
                StatementSlice {
                    stmt_id: *stmt_id,
                    span: node.span,
                    retention,
                    retained_subnodes: retained_sub,
                    excluded_subnodes: excluded_sub,
                    retained_sub_regions,
                    excluded_sub_regions,
                    sub_elements,
                },
            );
        }
    }

    // 5. Build retained elements list in deterministic PdgNodeId order
    let mut retained_elements = Vec::new();
    for &node_id in &visited {
        if let Some(node) = pdg.node(node_id) {
            let elem = SliceElement::new(
                node_id,
                node.kind.clone(),
                node.span,
                RetentionState::FullyRetained,
            );
            retained_elements.push(elem);
        }
    }

    let confidence = if contains_unknowns {
        SliceConfidence::PartialContainsUnknowns
    } else {
        SliceConfidence::Complete
    };

    Ok(SliceResult {
        criterion: criterion.clone(),
        retained_nodes: visited,
        retained_elements,
        statement_slices,
        retained_column_versions,
        excluded_column_versions,
        contains_unknowns,
        confidence,
    })
}

/// Convenience API to compute a backward slice for an individual dataframe column.
pub fn compute_column_slice(
    dataframe_symbol: impl Into<String>,
    column_name: impl Into<String>,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
) -> Result<SliceResult, SlicingError> {
    let criterion = SliceCriterion::ColumnByName {
        dataframe_symbol: dataframe_symbol.into(),
        column_name: column_name.into(),
    };
    compute_backward_slice(&criterion, pdg, df_analysis, scope_model)
}

/// Convenience API to compute a backward slice for an individual column with AST and source attribution.
pub fn compute_column_slice_with_source(
    dataframe_symbol: impl Into<String>,
    column_name: impl Into<String>,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
    program: Option<&Program>,
    source_text: Option<&str>,
) -> Result<SliceResult, SlicingError> {
    let criterion = SliceCriterion::ColumnByName {
        dataframe_symbol: dataframe_symbol.into(),
        column_name: column_name.into(),
    };
    compute_backward_slice_with_source(
        &criterion,
        pdg,
        df_analysis,
        scope_model,
        program,
        source_text,
    )
}

/// Convenience API to compute a whole-object backward slice for a dataframe variable.
pub fn compute_object_slice(
    dataframe_symbol: impl Into<String>,
    pdg: &ProgramDependenceGraph,
    df_analysis: &DataFrameAnalysis,
    scope_model: &ScopeModel,
) -> Result<SliceResult, SlicingError> {
    let criterion = SliceCriterion::WholeDataFrameByName {
        dataframe_symbol: dataframe_symbol.into(),
    };
    compute_backward_slice(&criterion, pdg, df_analysis, scope_model)
}

fn find_statement_in_program(program: &Program, stmt_id: NodeId) -> Option<&Stmt> {
    find_statement_in_slice(&program.stmts, stmt_id)
}

fn find_statement_in_slice(stmts: &[Stmt], stmt_id: NodeId) -> Option<&Stmt> {
    for s in stmts {
        if s.id() == stmt_id {
            return Some(s);
        }
        match s {
            Stmt::If(if_stmt) => {
                if let Some(found) = find_statement_in_slice(&if_stmt.then_branch.stmts, stmt_id) {
                    return Some(found);
                }
                if let Some(eb) = &if_stmt.else_branch {
                    if let Some(found) = find_statement_in_slice(&eb.stmts, stmt_id) {
                        return Some(found);
                    }
                }
            }
            Stmt::For(for_stmt) => {
                if let Some(found) = find_statement_in_slice(&for_stmt.body.stmts, stmt_id) {
                    return Some(found);
                }
            }
            Stmt::While(while_stmt) => {
                if let Some(found) = find_statement_in_slice(&while_stmt.body.stmts, stmt_id) {
                    return Some(found);
                }
            }
            Stmt::Block(b) => {
                if let Some(found) = find_statement_in_slice(&b.stmts, stmt_id) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn collect_named_args_in_stmt(stmt: &Stmt) -> Vec<&NamedArg> {
    let mut args = Vec::new();
    match stmt {
        Stmt::Assignment(assign) => {
            collect_named_args_in_expr(&assign.value, &mut args);
        }
        Stmt::Expr(expr_stmt) => {
            collect_named_args_in_expr(&expr_stmt.expr, &mut args);
        }
        _ => {}
    }
    args
}

fn collect_named_args_in_expr<'a>(expr: &'a Expr, out: &mut Vec<&'a NamedArg>) {
    match expr {
        Expr::Call(call) => {
            for arg in &call.args {
                if let Argument::Named(named) = arg {
                    out.push(named);
                }
            }
            collect_named_args_in_expr(&call.callee, out);
        }
        Expr::Pipe(pipe) => {
            collect_named_args_in_expr(&pipe.left, out);
            collect_named_args_in_expr(&pipe.right, out);
        }
        Expr::BinaryOp(b) => {
            collect_named_args_in_expr(&b.left, out);
            collect_named_args_in_expr(&b.right, out);
        }
        Expr::UnaryOp(u) => {
            collect_named_args_in_expr(&u.operand, out);
        }
        _ => {}
    }
}

fn get_stmt_target_name(stmt: &Stmt) -> Option<String> {
    if let Stmt::Assignment(assign) = stmt {
        match &assign.target {
            rparse_ir::LValue::Symbol { name, .. } => Some(name.clone()),
            _ => None,
        }
    } else {
        None
    }
}
