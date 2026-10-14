//! Deterministic construction of the Unified Program Dependence Graph.

use crate::edge::{PdgEdge, PdgEdgeKind};
use crate::graph::ProgramDependenceGraph;
use crate::ids::{PdgEdgeId, PdgNodeId};
use crate::node::{PdgNode, PdgNodeKind};
use rparse_cfg::{BlockId, Cfg};
use rparse_core::{NodeId, Span};
use rparse_dataflow::{ControlDependencies, DefUseInfo, ReachingDefinitions, VarDefId, VarUseId};
use rparse_dataframe::{ColumnVersionId, DataFrameAnalysis, StaticKnowledge};
use rparse_ir::{AssignmentStmt, Expr, ExtractKind, ForStmt, LValue, Program, Stmt};
use rparse_scope::ScopeModel;
use std::collections::{BTreeMap, BTreeSet};

/// Inputs required to deterministically construct the Unified PDG.
#[derive(Clone, Copy)]
pub struct PdgBuildInputs<'a> {
    pub program: &'a Program,
    pub scope_model: &'a ScopeModel,
    pub cfg: &'a Cfg,
    pub def_use: &'a DefUseInfo,
    pub reaching_defs: &'a ReachingDefinitions,
    pub control_deps: &'a ControlDependencies,
    pub df_analysis: &'a DataFrameAnalysis,
}

/// Constructs the Unified Program Dependence Graph from Phase 1–4 analysis results.
pub fn build_pdg(
    program: &Program,
    scope_model: &ScopeModel,
    cfg: &Cfg,
    def_use: &DefUseInfo,
    reaching_defs: &ReachingDefinitions,
    control_deps: &ControlDependencies,
    df_analysis: &DataFrameAnalysis,
) -> ProgramDependenceGraph {
    let inputs = PdgBuildInputs {
        program,
        scope_model,
        cfg,
        def_use,
        reaching_defs,
        control_deps,
        df_analysis,
    };
    PdgBuilder::new(inputs).build()
}

struct PdgBuilder<'a> {
    inputs: PdgBuildInputs<'a>,
    nodes: Vec<PdgNode>,
    edges: Vec<PdgEdge>,
    stmt_nodes: BTreeMap<NodeId, PdgNodeId>,
    expr_nodes: BTreeMap<NodeId, PdgNodeId>,
    var_def_nodes: BTreeMap<VarDefId, PdgNodeId>,
    var_use_nodes: BTreeMap<VarUseId, PdgNodeId>,
    column_ports: BTreeMap<ColumnVersionId, PdgNodeId>,
    row_selection_nodes: BTreeMap<NodeId, PdgNodeId>,
    unknown_nodes: Vec<PdgNodeId>,
    ast_to_stmt: BTreeMap<NodeId, NodeId>,
    incoming: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    outgoing: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    existing_edges: BTreeSet<(PdgNodeId, PdgNodeId, String)>,
}

impl<'a> PdgBuilder<'a> {
    fn new(inputs: PdgBuildInputs<'a>) -> Self {
        Self {
            inputs,
            nodes: Vec::new(),
            edges: Vec::new(),
            stmt_nodes: BTreeMap::new(),
            expr_nodes: BTreeMap::new(),
            var_def_nodes: BTreeMap::new(),
            var_use_nodes: BTreeMap::new(),
            column_ports: BTreeMap::new(),
            row_selection_nodes: BTreeMap::new(),
            unknown_nodes: Vec::new(),
            ast_to_stmt: BTreeMap::new(),
            incoming: BTreeMap::new(),
            outgoing: BTreeMap::new(),
            existing_edges: BTreeSet::new(),
        }
    }

    fn alloc_node(&mut self, kind: PdgNodeKind, span: Span) -> PdgNodeId {
        let id = PdgNodeId(self.nodes.len() as u32);
        self.nodes.push(PdgNode::new(id, kind, span));
        id
    }

    fn add_edge(&mut self, from: PdgNodeId, to: PdgNodeId, kind: PdgEdgeKind) {
        let key = (from, to, format!("{:?}", kind));
        if self.existing_edges.insert(key) {
            let id = PdgEdgeId(self.edges.len() as u32);
            self.edges.push(PdgEdge::new(id, from, to, kind));
            self.incoming.entry(to).or_default().push(id);
            self.outgoing.entry(from).or_default().push(id);
        }
    }

    fn build(mut self) -> ProgramDependenceGraph {
        // 1. Walk program statements and register Statement nodes
        self.collect_statements(&self.inputs.program.stmts);

        // 2. Register Expression nodes from Provenance edges & control conditions
        self.collect_expressions();

        // 3. Register Variable Definition nodes
        for def in self.inputs.def_use.defs() {
            let id = self.alloc_node(
                PdgNodeKind::VariableDef {
                    def_id: def.id,
                    symbol_id: def.symbol_id,
                    name: def.name.clone(),
                },
                def.span,
            );
            self.var_def_nodes.insert(def.id, id);
        }

        // 4. Register Variable Use nodes
        for u in self.inputs.def_use.uses() {
            let id = self.alloc_node(
                PdgNodeKind::VariableUse {
                    use_id: u.id,
                    symbol_id: u.symbol_id,
                    name: u.name.clone(),
                },
                u.span,
            );
            self.var_use_nodes.insert(u.id, id);
        }

        // 5. Register Column Version nodes (Column Ports)
        for col in &self.inputs.df_analysis.registry.columns {
            let span = if let Some(pedge_id) = col.provenance_edge {
                self.inputs
                    .df_analysis
                    .provenance()
                    .get_edge(pedge_id)
                    .map(|e| e.span)
                    .unwrap_or(Span::new(0, 0, 1, 0, 1, 0))
            } else if let Some(st) = self.inputs.df_analysis.state(col.dataframe_id) {
                st.span
            } else {
                Span::new(0, 0, 1, 0, 1, 0)
            };

            let id = self.alloc_node(
                PdgNodeKind::ColumnVersion {
                    col_id: col.id,
                    df_id: col.dataframe_id,
                    col_name: col.name.clone(),
                    version: col.version,
                    knowledge: col.knowledge,
                },
                span,
            );
            self.column_ports.insert(col.id, id);
        }

        // 6. Register Row Selection nodes
        for st in self.inputs.df_analysis.all_states() {
            for row_dep in st.row_selections() {
                if !self
                    .row_selection_nodes
                    .contains_key(&row_dep.predicate_node)
                {
                    let id = self.alloc_node(
                        PdgNodeKind::RowSelection {
                            df_id: row_dep.dataframe_id,
                            predicate_node: row_dep.predicate_node,
                            knowledge: row_dep.knowledge,
                        },
                        row_dep.span,
                    );
                    self.row_selection_nodes.insert(row_dep.predicate_node, id);
                }
            }
        }

        // 7. Register Unknown nodes
        self.collect_unknowns();

        // 8. Connect Scalar Data Dependencies
        self.connect_scalar_dependencies();

        // 9. Connect Column Derivation Dependencies
        self.connect_column_derivations();

        // 10. Connect Column-to-Statement Bindings
        self.connect_column_statement_bindings();

        // 11. Connect Control Dependencies
        self.connect_control_dependencies();

        // 12. Connect Row Selection Dependencies
        self.connect_row_selection_dependencies();

        // 13. Connect Unknown Dependencies
        self.connect_unknown_dependencies();

        // 14. Connect Direct Extraction Subsetting ($ access)
        self.connect_extractions(&self.inputs.program.stmts);

        ProgramDependenceGraph::new(crate::graph::PdgGraphInit {
            nodes: self.nodes,
            edges: self.edges,
            incoming: self.incoming,
            outgoing: self.outgoing,
            column_ports: self.column_ports,
            stmt_nodes: self.stmt_nodes,
            expr_nodes: self.expr_nodes,
            var_def_nodes: self.var_def_nodes,
            var_use_nodes: self.var_use_nodes,
            row_selection_nodes: self.row_selection_nodes,
            unknown_nodes: self.unknown_nodes,
        })
    }

    fn collect_statements(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            let stmt_id = stmt.id();
            let span = stmt.span();
            let desc = format_stmt_desc(stmt);
            let node_id = self.alloc_node(
                PdgNodeKind::Statement {
                    stmt_id,
                    description: desc,
                },
                span,
            );
            self.stmt_nodes.insert(stmt_id, node_id);
            self.record_ast_to_stmt(stmt, stmt_id);

            match stmt {
                Stmt::If(if_stmt) => {
                    self.collect_statements(&if_stmt.then_branch.stmts);
                    if let Some(eb) = &if_stmt.else_branch {
                        self.collect_statements(&eb.stmts);
                    }
                }
                Stmt::For(for_stmt) => {
                    self.collect_statements(&for_stmt.body.stmts);
                }
                Stmt::While(while_stmt) => {
                    self.collect_statements(&while_stmt.body.stmts);
                }
                Stmt::Block(block) => {
                    self.collect_statements(&block.stmts);
                }
                _ => {}
            }
        }
    }

    fn record_ast_to_stmt(&mut self, stmt: &Stmt, stmt_id: NodeId) {
        self.ast_to_stmt.insert(stmt.id(), stmt_id);
        match stmt {
            Stmt::Assignment(assign) => {
                self.record_lvalue_to_stmt(&assign.target, stmt_id);
                self.record_expr_to_stmt(&assign.value, stmt_id);
            }
            Stmt::Expr(expr_stmt) => {
                self.record_expr_to_stmt(&expr_stmt.expr, stmt_id);
            }
            Stmt::If(if_stmt) => {
                self.record_expr_to_stmt(&if_stmt.condition, stmt_id);
            }
            Stmt::For(for_stmt) => {
                self.ast_to_stmt.insert(for_stmt.iterator_id, stmt_id);
                self.record_expr_to_stmt(&for_stmt.iterable, stmt_id);
            }
            Stmt::While(while_stmt) => {
                self.record_expr_to_stmt(&while_stmt.condition, stmt_id);
            }
            Stmt::Return(ret) => {
                if let Some(val) = &ret.value {
                    self.record_expr_to_stmt(val, stmt_id);
                }
            }
            Stmt::Block(b) => {
                self.ast_to_stmt.insert(b.id, stmt_id);
            }
        }
    }

    fn record_lvalue_to_stmt(&mut self, lval: &LValue, stmt_id: NodeId) {
        match lval {
            LValue::Symbol { id, .. } => {
                self.ast_to_stmt.insert(*id, stmt_id);
            }
            LValue::Extraction {
                id, target, index, ..
            } => {
                self.ast_to_stmt.insert(*id, stmt_id);
                self.record_expr_to_stmt(target, stmt_id);
                self.record_expr_to_stmt(index, stmt_id);
            }
        }
    }

    fn record_expr_to_stmt(&mut self, expr: &Expr, stmt_id: NodeId) {
        self.ast_to_stmt.insert(expr.id(), stmt_id);
        match expr {
            Expr::BinaryOp(b) => {
                self.record_expr_to_stmt(&b.left, stmt_id);
                self.record_expr_to_stmt(&b.right, stmt_id);
            }
            Expr::UnaryOp(u) => {
                self.record_expr_to_stmt(&u.operand, stmt_id);
            }
            Expr::Call(c) => {
                self.record_expr_to_stmt(&c.callee, stmt_id);
                for arg in &c.args {
                    self.ast_to_stmt.insert(arg.id(), stmt_id);
                    self.record_expr_to_stmt(arg.value(), stmt_id);
                }
            }
            Expr::Extract(e) => {
                self.record_expr_to_stmt(&e.target, stmt_id);
                self.record_expr_to_stmt(&e.index, stmt_id);
            }
            Expr::Pipe(p) => {
                self.record_expr_to_stmt(&p.left, stmt_id);
                self.record_expr_to_stmt(&p.right, stmt_id);
            }
            _ => {}
        }
    }

    fn collect_expressions(&mut self) {
        // Collect transformation expressions from Provenance edges
        for pedge in self.inputs.df_analysis.provenance().edges() {
            if let Some(expr_id) = pedge.expression_node {
                if !self.expr_nodes.contains_key(&expr_id) {
                    let desc = format!("{:?} derivation expr", pedge.transform_type);
                    let node_id = self.alloc_node(
                        PdgNodeKind::Expression {
                            expr_id,
                            description: desc,
                        },
                        pedge.span,
                    );
                    self.expr_nodes.insert(expr_id, node_id);
                }
            }
        }

        // Collect condition expressions from If & While statements
        self.collect_condition_expressions(&self.inputs.program.stmts);
    }

    fn collect_condition_expressions(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::If(if_stmt) => {
                    let cond_id = if_stmt.condition.id();
                    if !self.expr_nodes.contains_key(&cond_id) {
                        let node_id = self.alloc_node(
                            PdgNodeKind::Expression {
                                expr_id: cond_id,
                                description: "if_condition".to_string(),
                            },
                            if_stmt.condition.span(),
                        );
                        self.expr_nodes.insert(cond_id, node_id);
                    }
                    self.collect_condition_expressions(&if_stmt.then_branch.stmts);
                    if let Some(eb) = &if_stmt.else_branch {
                        self.collect_condition_expressions(&eb.stmts);
                    }
                }
                Stmt::While(while_stmt) => {
                    let cond_id = while_stmt.condition.id();
                    if !self.expr_nodes.contains_key(&cond_id) {
                        let node_id = self.alloc_node(
                            PdgNodeKind::Expression {
                                expr_id: cond_id,
                                description: "while_condition".to_string(),
                            },
                            while_stmt.condition.span(),
                        );
                        self.expr_nodes.insert(cond_id, node_id);
                    }
                    self.collect_condition_expressions(&while_stmt.body.stmts);
                }
                Stmt::For(for_stmt) => {
                    self.collect_condition_expressions(&for_stmt.body.stmts);
                }
                Stmt::Block(block) => {
                    self.collect_condition_expressions(&block.stmts);
                }
                _ => {}
            }
        }
    }

    fn collect_unknowns(&mut self) {
        // Unknowns from provenance edges
        for pedge in self.inputs.df_analysis.provenance().edges() {
            if pedge.knowledge == StaticKnowledge::UnknownStatic {
                let id = self.alloc_node(
                    PdgNodeKind::Unknown {
                        origin_node: pedge.expression_node,
                        reason: "dynamic_column_derivation".to_string(),
                    },
                    pedge.span,
                );
                self.unknown_nodes.push(id);
            }
        }

        // Unknowns from column versions without edge
        for col in &self.inputs.df_analysis.registry.columns {
            if col.knowledge == StaticKnowledge::UnknownStatic && col.provenance_edge.is_none() {
                let span = self
                    .inputs
                    .df_analysis
                    .state(col.dataframe_id)
                    .map(|s| s.span)
                    .unwrap_or(Span::new(0, 0, 1, 0, 1, 0));
                let id = self.alloc_node(
                    PdgNodeKind::Unknown {
                        origin_node: None,
                        reason: format!("unknown_column_{}", col.name),
                    },
                    span,
                );
                self.unknown_nodes.push(id);
            }
        }
    }

    fn connect_scalar_dependencies(&mut self) {
        // A. Reaching definitions to uses
        for u in self.inputs.def_use.uses() {
            let u_node = self.var_use_nodes[&u.id];
            for &def_id in self.inputs.reaching_defs.reaching_defs_for_use(u.id) {
                if let Some(&def_node) = self.var_def_nodes.get(&def_id) {
                    self.add_edge(def_node, u_node, PdgEdgeKind::VariableDataDependency);
                }
            }
        }

        // B. Statement defining variable -> VarDef
        for def in self.inputs.def_use.defs() {
            let def_node = self.var_def_nodes[&def.id];
            if let Some(&stmt_id) = self.ast_to_stmt.get(&def.node_id) {
                if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                    self.add_edge(stmt_node, def_node, PdgEdgeKind::VariableDataDependency);
                }
            }
        }

        // C. VarUse -> containing statement or expression
        for u in self.inputs.def_use.uses() {
            let u_node = self.var_use_nodes[&u.id];
            // Check if use belongs to an explicit expression node
            let mut connected_to_expr = false;
            for (&expr_id, &expr_node) in &self.expr_nodes {
                if self.node_contains(expr_id, u.node_id) {
                    self.add_edge(u_node, expr_node, PdgEdgeKind::VariableDataDependency);
                    connected_to_expr = true;
                    break;
                }
            }

            if !connected_to_expr {
                if let Some(&stmt_id) = self.ast_to_stmt.get(&u.node_id) {
                    if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                        self.add_edge(u_node, stmt_node, PdgEdgeKind::VariableDataDependency);
                    }
                }
            }
        }
    }

    fn connect_column_derivations(&mut self) {
        for pedge in self.inputs.df_analysis.provenance().edges() {
            let out_col_node = self.column_ports[&pedge.output_column];

            // 1. Input columns -> output column
            for &cin in &pedge.input_columns {
                if let Some(&in_col_node) = self.column_ports.get(&cin) {
                    self.add_edge(
                        in_col_node,
                        out_col_node,
                        PdgEdgeKind::ColumnDerivation {
                            provenance_edge_id: Some(pedge.id),
                            transform_type: pedge.transform_type.clone(),
                            knowledge: pedge.knowledge,
                        },
                    );
                }
            }

            // 2. Lexical scalar variables participating in derivation -> output column & expression
            for &var_sym in &pedge.input_variables {
                for u in self.inputs.def_use.uses() {
                    if u.symbol_id == Some(var_sym) && pedge.span.contains_span(&u.span) {
                        let u_node = self.var_use_nodes[&u.id];
                        self.add_edge(u_node, out_col_node, PdgEdgeKind::VariableDataDependency);

                        if let Some(expr_id) = pedge.expression_node {
                            if let Some(&expr_node) = self.expr_nodes.get(&expr_id) {
                                self.add_edge(
                                    u_node,
                                    expr_node,
                                    PdgEdgeKind::VariableDataDependency,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn connect_column_statement_bindings(&mut self) {
        // A. Statement creating dataframe state -> newly introduced column versions
        for st in self.inputs.df_analysis.all_states() {
            let stmt_id_opt = self
                .ast_to_stmt
                .get(&st.creation_node)
                .copied()
                .or_else(|| {
                    self.stmt_nodes
                        .get(&st.creation_node)
                        .map(|_| st.creation_node)
                });

            if let Some(stmt_id) = stmt_id_opt {
                if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                    for (col_name, &col_id) in &st.schema {
                        let is_new = if let Some(pid) = st.parent_state {
                            self.inputs
                                .df_analysis
                                .state(pid)
                                .map(|p| p.column(col_name) != Some(col_id))
                                .unwrap_or(true)
                        } else {
                            true
                        };

                        if is_new {
                            let col_node = self.column_ports[&col_id];
                            self.add_edge(stmt_node, col_node, PdgEdgeKind::ColumnStatementBinding);
                        }
                    }
                }
            }
        }

        // B. Mutate/transmute argument expressions -> column versions and statement -> expr
        for pedge in self.inputs.df_analysis.provenance().edges() {
            if let Some(expr_id) = pedge.expression_node {
                if let Some(&expr_node) = self.expr_nodes.get(&expr_id) {
                    let out_col_node = self.column_ports[&pedge.output_column];
                    self.add_edge(expr_node, out_col_node, PdgEdgeKind::ColumnStatementBinding);

                    if let Some(&stmt_id) = self.ast_to_stmt.get(&expr_id) {
                        if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                            self.add_edge(
                                stmt_node,
                                expr_node,
                                PdgEdgeKind::ColumnStatementBinding,
                            );
                        }
                    }
                }
            }
        }
    }

    fn connect_control_dependencies(&mut self) {
        for block in self.inputs.cfg.blocks() {
            for dep in self.inputs.control_deps.dependencies_for_block(block.id) {
                if let Some(cond_node) = dep.condition_node {
                    let cond_pdg_node = self
                        .expr_nodes
                        .get(&cond_node)
                        .copied()
                        .or_else(|| self.stmt_nodes.get(&cond_node).copied())
                        .or_else(|| {
                            self.ast_to_stmt
                                .get(&cond_node)
                                .and_then(|s| self.stmt_nodes.get(s).copied())
                        });

                    if let Some(ctrl_node) = cond_pdg_node {
                        // Find all statement nodes belonging to this CFG basic block
                        for stmt in &self.inputs.program.stmts {
                            self.connect_block_statements(block.id, ctrl_node, stmt, dep.edge_kind);
                        }
                    }
                }
            }
        }
    }

    fn connect_block_statements(
        &mut self,
        block_id: BlockId,
        ctrl_node: PdgNodeId,
        stmt: &Stmt,
        edge_kind: rparse_cfg::EdgeKind,
    ) {
        if self.inputs.cfg.block_for_node(stmt.id()) == Some(block_id) {
            if let Some(&stmt_pdg_node) = self.stmt_nodes.get(&stmt.id()) {
                if stmt_pdg_node != ctrl_node {
                    self.add_edge(
                        ctrl_node,
                        stmt_pdg_node,
                        PdgEdgeKind::ControlDependency { edge_kind },
                    );
                }
            }
        }

        match stmt {
            Stmt::If(if_stmt) => {
                for s in &if_stmt.then_branch.stmts {
                    self.connect_block_statements(block_id, ctrl_node, s, edge_kind);
                }
                if let Some(eb) = &if_stmt.else_branch {
                    for s in &eb.stmts {
                        self.connect_block_statements(block_id, ctrl_node, s, edge_kind);
                    }
                }
            }
            Stmt::For(for_stmt) => {
                for s in &for_stmt.body.stmts {
                    self.connect_block_statements(block_id, ctrl_node, s, edge_kind);
                }
            }
            Stmt::While(while_stmt) => {
                for s in &while_stmt.body.stmts {
                    self.connect_block_statements(block_id, ctrl_node, s, edge_kind);
                }
            }
            Stmt::Block(b) => {
                for s in &b.stmts {
                    self.connect_block_statements(block_id, ctrl_node, s, edge_kind);
                }
            }
            _ => {}
        }
    }

    fn connect_row_selection_dependencies(&mut self) {
        for st in self.inputs.df_analysis.all_states() {
            for row_dep in st.row_selections() {
                if let Some(&rsel_node) = self.row_selection_nodes.get(&row_dep.predicate_node) {
                    // 1. Filter statement -> RowSelectionNode
                    if let Some(&stmt_id) = self.ast_to_stmt.get(&row_dep.predicate_node) {
                        if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                            self.add_edge(
                                stmt_node,
                                rsel_node,
                                PdgEdgeKind::RowSelectionDependency {
                                    knowledge: row_dep.knowledge,
                                },
                            );
                        }
                    }

                    // 2. Predicate columns -> RowSelectionNode
                    for &pcol in &row_dep.predicate_columns {
                        if let Some(&pcol_node) = self.column_ports.get(&pcol) {
                            self.add_edge(
                                pcol_node,
                                rsel_node,
                                PdgEdgeKind::RowSelectionDependency {
                                    knowledge: row_dep.knowledge,
                                },
                            );
                        }
                    }

                    // 3. Predicate variables -> RowSelectionNode
                    for &pvar in &row_dep.predicate_variables {
                        for u in self.inputs.def_use.uses() {
                            if u.symbol_id == Some(pvar) && row_dep.span.contains_span(&u.span) {
                                let u_node = self.var_use_nodes[&u.id];
                                self.add_edge(
                                    u_node,
                                    rsel_node,
                                    PdgEdgeKind::RowSelectionDependency {
                                        knowledge: row_dep.knowledge,
                                    },
                                );
                            }
                        }
                    }

                    // 4. RowSelectionNode -> filtered dataframe output columns
                    for &col_id in st.schema.values() {
                        if let Some(&col_node) = self.column_ports.get(&col_id) {
                            self.add_edge(
                                rsel_node,
                                col_node,
                                PdgEdgeKind::RowSelectionDependency {
                                    knowledge: row_dep.knowledge,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    fn connect_unknown_dependencies(&mut self) {
        let mut edges_to_add: Vec<(PdgNodeId, PdgNodeId, String)> = Vec::new();

        for &unk_node_id in &self.unknown_nodes {
            if let Some(unk_node) = self.nodes.get(unk_node_id.0 as usize) {
                if let PdgNodeKind::Unknown {
                    origin_node: Some(origin_id),
                    reason,
                } = &unk_node.kind
                {
                    if let Some(&stmt_id) = self.ast_to_stmt.get(origin_id) {
                        if let Some(&stmt_node) = self.stmt_nodes.get(&stmt_id) {
                            edges_to_add.push((stmt_node, unk_node_id, reason.clone()));
                        }
                    }
                }
            }
        }

        for (from, to, reason) in edges_to_add {
            self.add_edge(from, to, PdgEdgeKind::UnknownDependency { reason });
        }

        // Connect unknown nodes to output column versions that carry UnknownStatic
        let unk_ids = self.unknown_nodes.clone();
        for col in &self.inputs.df_analysis.registry.columns {
            if col.knowledge == StaticKnowledge::UnknownStatic {
                let col_node = self.column_ports[&col.id];
                for &unk_id in &unk_ids {
                    self.add_edge(
                        unk_id,
                        col_node,
                        PdgEdgeKind::UnknownDependency {
                            reason: "column_uncertainty".to_string(),
                        },
                    );
                }
            }
        }
    }

    fn connect_extractions(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Stmt::Assignment(assign) = stmt {
                if let Expr::Extract(extract) = assign.value.as_ref() {
                    if extract.kind == ExtractKind::Dollar {
                        if let Expr::Identifier(target_id) = extract.target.as_ref() {
                            if let Some(col_name) = extract_static_name(&extract.index) {
                                if let Some(df_st) = self
                                    .inputs
                                    .df_analysis
                                    .active_state_for_symbol(&target_id.name)
                                    .or_else(|| {
                                        self.inputs.df_analysis.all_states().into_iter().rev().find(
                                            |s| {
                                                s.symbol_name == target_id.name
                                                    && s.has_column(&col_name)
                                            },
                                        )
                                    })
                                {
                                    if let Some(col_id) = df_st.column(&col_name) {
                                        if let Some(&col_node) = self.column_ports.get(&col_id) {
                                            if let Some(&stmt_node) =
                                                self.stmt_nodes.get(&stmt.id())
                                            {
                                                self.add_edge(
                                                    col_node,
                                                    stmt_node,
                                                    PdgEdgeKind::ColumnStatementBinding,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn node_contains(&self, parent: NodeId, child: NodeId) -> bool {
        self.ast_to_stmt.get(&parent) == self.ast_to_stmt.get(&child)
    }
}

fn format_stmt_desc(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Assignment(AssignmentStmt { target, .. }) => match target {
            LValue::Symbol { name, .. } => format!("assign_{}", name),
            LValue::Extraction { .. } => "dollar_assign".to_string(),
        },
        Stmt::Expr(_) => "expr_stmt".to_string(),
        Stmt::If(_) => "if_stmt".to_string(),
        Stmt::For(ForStmt { iterator, .. }) => format!("for_loop_{}", iterator),
        Stmt::While(_) => "while_loop".to_string(),
        Stmt::Return(_) => "return_stmt".to_string(),
        Stmt::Block(_) => "block_stmt".to_string(),
    }
}

fn extract_static_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Identifier(id) => Some(id.name.clone()),
        Expr::Literal(lit) => match &lit.kind {
            rparse_ir::LiteralKind::String(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}
