//! Unified Program Dependence Graph structure and invariant validation.

use crate::edge::PdgEdge;
use crate::ids::{PdgEdgeId, PdgNodeId};
use crate::node::{PdgNode, PdgNodeKind};
use rparse_core::NodeId;
use rparse_dataflow::{VarDefId, VarUseId};
use rparse_dataframe::ColumnVersionId;
use std::collections::BTreeMap;
use std::fmt;

/// Error returned when PDG invariant validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdgValidationError {
    /// An edge references a non-existent source node.
    InvalidEdgeSource {
        edge_id: PdgEdgeId,
        source: PdgNodeId,
    },
    /// An edge references a non-existent target node.
    InvalidEdgeTarget {
        edge_id: PdgEdgeId,
        target: PdgNodeId,
    },
    /// Node IDs are non-sequential or non-deterministic.
    NonSequentialNodeId { expected: u32, actual: u32 },
    /// Edge IDs are non-sequential or non-deterministic.
    NonSequentialEdgeId { expected: u32, actual: u32 },
    /// Duplicate semantic identifier encountered for a node category.
    DuplicateSemanticNode { description: String },
    /// Adjacency index mismatch between edges list and incoming/outgoing index.
    AdjacencyIndexMismatch {
        node_id: PdgNodeId,
        edge_id: PdgEdgeId,
    },
    /// Column port does not point to a ColumnVersion node.
    InvalidColumnPort {
        col_id: ColumnVersionId,
        node_id: PdgNodeId,
    },
    /// An unknown node does not retain reason information.
    UnknownNodeMissingReason { node_id: PdgNodeId },
}

impl fmt::Display for PdgValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEdgeSource { edge_id, source } => {
                write!(
                    f,
                    "edge {} references non-existent source node {}",
                    edge_id, source
                )
            }
            Self::InvalidEdgeTarget { edge_id, target } => {
                write!(
                    f,
                    "edge {} references non-existent target node {}",
                    edge_id, target
                )
            }
            Self::NonSequentialNodeId { expected, actual } => {
                write!(
                    f,
                    "non-sequential node ID: expected {}, got {}",
                    expected, actual
                )
            }
            Self::NonSequentialEdgeId { expected, actual } => {
                write!(
                    f,
                    "non-sequential edge ID: expected {}, got {}",
                    expected, actual
                )
            }
            Self::DuplicateSemanticNode { description } => {
                write!(f, "duplicate semantic node: {}", description)
            }
            Self::AdjacencyIndexMismatch { node_id, edge_id } => {
                write!(
                    f,
                    "adjacency mismatch at node {} for edge {}",
                    node_id, edge_id
                )
            }
            Self::InvalidColumnPort { col_id, node_id } => {
                write!(
                    f,
                    "column port {} mapped to non-column node {}",
                    col_id, node_id
                )
            }
            Self::UnknownNodeMissingReason { node_id } => {
                write!(f, "unknown node {} has empty reason", node_id)
            }
        }
    }
}

impl std::error::Error for PdgValidationError {}

/// The Unified Program Dependence Graph combining scalar dataflow, column provenance,
/// statement bindings, control dependencies, row-selection, and explicit uncertainties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramDependenceGraph {
    nodes: Vec<PdgNode>,
    edges: Vec<PdgEdge>,
    incoming: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    outgoing: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    column_ports: BTreeMap<ColumnVersionId, PdgNodeId>,
    stmt_nodes: BTreeMap<NodeId, PdgNodeId>,
    expr_nodes: BTreeMap<NodeId, PdgNodeId>,
    var_def_nodes: BTreeMap<VarDefId, PdgNodeId>,
    var_use_nodes: BTreeMap<VarUseId, PdgNodeId>,
    row_selection_nodes: BTreeMap<NodeId, PdgNodeId>,
    unknown_nodes: Vec<PdgNodeId>,
}

/// Container for initializing [`ProgramDependenceGraph`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdgGraphInit {
    pub nodes: Vec<PdgNode>,
    pub edges: Vec<PdgEdge>,
    pub incoming: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    pub outgoing: BTreeMap<PdgNodeId, Vec<PdgEdgeId>>,
    pub column_ports: BTreeMap<ColumnVersionId, PdgNodeId>,
    pub stmt_nodes: BTreeMap<NodeId, PdgNodeId>,
    pub expr_nodes: BTreeMap<NodeId, PdgNodeId>,
    pub var_def_nodes: BTreeMap<VarDefId, PdgNodeId>,
    pub var_use_nodes: BTreeMap<VarUseId, PdgNodeId>,
    pub row_selection_nodes: BTreeMap<NodeId, PdgNodeId>,
    pub unknown_nodes: Vec<PdgNodeId>,
}

impl ProgramDependenceGraph {
    pub fn new(init: PdgGraphInit) -> Self {
        Self {
            nodes: init.nodes,
            edges: init.edges,
            incoming: init.incoming,
            outgoing: init.outgoing,
            column_ports: init.column_ports,
            stmt_nodes: init.stmt_nodes,
            expr_nodes: init.expr_nodes,
            var_def_nodes: init.var_def_nodes,
            var_use_nodes: init.var_use_nodes,
            row_selection_nodes: init.row_selection_nodes,
            unknown_nodes: init.unknown_nodes,
        }
    }

    /// Returns a slice of all nodes in the PDG in deterministic generation order.
    pub fn nodes(&self) -> &[PdgNode] {
        &self.nodes
    }

    /// Returns a slice of all dependency edges in the PDG in deterministic generation order.
    pub fn edges(&self) -> &[PdgEdge] {
        &self.edges
    }

    /// Looks up a node by its [`PdgNodeId`].
    pub fn node(&self, id: PdgNodeId) -> Option<&PdgNode> {
        self.nodes.get(id.0 as usize)
    }

    /// Looks up an edge by its [`PdgEdgeId`].
    pub fn edge(&self, id: PdgEdgeId) -> Option<&PdgEdge> {
        self.edges.get(id.0 as usize)
    }

    /// Returns incoming edge IDs for `node_id` (pointing from sources to `node_id`).
    pub fn incoming_edges(&self, node_id: PdgNodeId) -> &[PdgEdgeId] {
        self.incoming
            .get(&node_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns outgoing edge IDs for `node_id` (pointing from `node_id` to dependents).
    pub fn outgoing_edges(&self, node_id: PdgNodeId) -> &[PdgEdgeId] {
        self.outgoing
            .get(&node_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns the immediate predecessor (source) node IDs that `node_id` depends on.
    pub fn predecessors(&self, node_id: PdgNodeId) -> Vec<PdgNodeId> {
        let mut preds = Vec::new();
        for &eid in self.incoming_edges(node_id) {
            if let Some(edge) = self.edge(eid) {
                preds.push(edge.from);
            }
        }
        preds
    }

    /// Returns the immediate successor (dependent) node IDs that depend on `node_id`.
    pub fn successors(&self, node_id: PdgNodeId) -> Vec<PdgNodeId> {
        let mut succs = Vec::new();
        for &eid in self.outgoing_edges(node_id) {
            if let Some(edge) = self.edge(eid) {
                succs.push(edge.to);
            }
        }
        succs
    }

    /// Looks up the specific column port [`PdgNodeId`] for a [`ColumnVersionId`].
    pub fn column_port(&self, col_id: ColumnVersionId) -> Option<PdgNodeId> {
        self.column_ports.get(&col_id).copied()
    }

    /// Looks up the [`PdgNodeId`] for a statement [`NodeId`].
    pub fn stmt_node(&self, stmt_id: NodeId) -> Option<PdgNodeId> {
        self.stmt_nodes.get(&stmt_id).copied()
    }

    /// Looks up the [`PdgNodeId`] for an expression [`NodeId`].
    pub fn expr_node(&self, expr_id: NodeId) -> Option<PdgNodeId> {
        self.expr_nodes.get(&expr_id).copied()
    }

    /// Looks up the [`PdgNodeId`] for a variable definition [`VarDefId`].
    pub fn var_def_node(&self, def_id: VarDefId) -> Option<PdgNodeId> {
        self.var_def_nodes.get(&def_id).copied()
    }

    /// Looks up the [`PdgNodeId`] for a variable use [`VarUseId`].
    pub fn var_use_node(&self, use_id: VarUseId) -> Option<PdgNodeId> {
        self.var_use_nodes.get(&use_id).copied()
    }

    /// Looks up the [`PdgNodeId`] for a row selection predicate [`NodeId`].
    pub fn row_selection_node(&self, pred_node: NodeId) -> Option<PdgNodeId> {
        self.row_selection_nodes.get(&pred_node).copied()
    }

    /// Returns all explicit unknown node IDs in the graph.
    pub fn unknown_nodes(&self) -> &[PdgNodeId] {
        &self.unknown_nodes
    }

    /// Validates all Section 33 PDG invariants.
    pub fn validate(&self) -> Result<(), PdgValidationError> {
        // 1. Verify monotonic, sequential node identities
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.id.0 != idx as u32 {
                return Err(PdgValidationError::NonSequentialNodeId {
                    expected: idx as u32,
                    actual: node.id.0,
                });
            }
            if let PdgNodeKind::Unknown { reason, .. } = &node.kind {
                if reason.is_empty() {
                    return Err(PdgValidationError::UnknownNodeMissingReason { node_id: node.id });
                }
            }
        }

        // 2. Verify monotonic, sequential edge identities
        for (idx, edge) in self.edges.iter().enumerate() {
            if edge.id.0 != idx as u32 {
                return Err(PdgValidationError::NonSequentialEdgeId {
                    expected: idx as u32,
                    actual: edge.id.0,
                });
            }

            // 3. Verify valid source and target nodes exist
            if edge.from.0 as usize >= self.nodes.len() {
                return Err(PdgValidationError::InvalidEdgeSource {
                    edge_id: edge.id,
                    source: edge.from,
                });
            }
            if edge.to.0 as usize >= self.nodes.len() {
                return Err(PdgValidationError::InvalidEdgeTarget {
                    edge_id: edge.id,
                    target: edge.to,
                });
            }
        }

        // 4. Verify adjacency indexes correspond to edges
        for edge in &self.edges {
            let in_list = self.incoming.get(&edge.to);
            if in_list.map(|l| !l.contains(&edge.id)).unwrap_or(true) {
                return Err(PdgValidationError::AdjacencyIndexMismatch {
                    node_id: edge.to,
                    edge_id: edge.id,
                });
            }
            let out_list = self.outgoing.get(&edge.from);
            if out_list.map(|l| !l.contains(&edge.id)).unwrap_or(true) {
                return Err(PdgValidationError::AdjacencyIndexMismatch {
                    node_id: edge.from,
                    edge_id: edge.id,
                });
            }
        }

        // 5. Verify column ports map to actual ColumnVersion nodes
        for (&col_id, &node_id) in &self.column_ports {
            if let Some(node) = self.node(node_id) {
                if !node.is_column_version() {
                    return Err(PdgValidationError::InvalidColumnPort { col_id, node_id });
                }
            } else {
                return Err(PdgValidationError::InvalidColumnPort { col_id, node_id });
            }
        }

        Ok(())
    }

    /// Emits a deterministic Graphviz DOT string of the Unified Program Dependence Graph.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph PDG {\n");
        dot.push_str("  node [fontname=\"Courier\"];\n");
        dot.push_str("  edge [fontname=\"Courier\", fontsize=10];\n");

        for node in &self.nodes {
            let (label_str, shape_attrs) = match &node.kind {
                PdgNodeKind::Statement {
                    stmt_id,
                    description,
                } => {
                    let desc = escape_dot(description);
                    (
                        format!(
                            "{}\\nStatement({})\\n{}\\n[{}]",
                            node.id, stmt_id, desc, node.span
                        ),
                        "shape=box, style=filled, fillcolor=\"#f5f5f5\", color=\"#616161\"",
                    )
                }
                PdgNodeKind::Expression {
                    expr_id,
                    description,
                } => {
                    let desc = escape_dot(description);
                    (
                        format!(
                            "{}\\nExpression({})\\n{}\\n[{}]",
                            node.id, expr_id, desc, node.span
                        ),
                        "shape=box, style=\"dashed,filled\", fillcolor=\"#fafafa\", color=\"#9e9e9e\"",
                    )
                }
                PdgNodeKind::VariableDef {
                    def_id,
                    symbol_id,
                    name,
                } => (
                    format!(
                        "{}\\nVarDef({} sym#{})\\n'{}'\\n[{}]",
                        node.id,
                        def_id,
                        symbol_id,
                        escape_dot(name),
                        node.span
                    ),
                    "shape=note, style=filled, fillcolor=\"#fff9c4\", color=\"#fbc02d\"",
                ),
                PdgNodeKind::VariableUse {
                    use_id,
                    symbol_id,
                    name,
                } => (
                    format!(
                        "{}\\nVarUse({} sym={:?})\\n'{}'\\n[{}]",
                        node.id,
                        use_id,
                        symbol_id,
                        escape_dot(name),
                        node.span
                    ),
                    "shape=cds, style=filled, fillcolor=\"#fffde7\", color=\"#fbc02d\"",
                ),
                PdgNodeKind::ColumnVersion {
                    col_id,
                    df_id,
                    col_name,
                    version,
                    knowledge,
                } => (
                    format!(
                        "{}\\nColumnPort({} {}@v{})\\n{}\\n[{}]",
                        node.id,
                        df_id,
                        escape_dot(col_name),
                        version,
                        col_id,
                        knowledge
                    ),
                    "shape=ellipse, style=filled, fillcolor=\"#e1f5fe\", color=\"#0288d1\"",
                ),
                PdgNodeKind::RowSelection {
                    df_id,
                    predicate_node,
                    knowledge,
                } => (
                    format!(
                        "{}\\nRowSelection({})\\npred={}\\n[{}]",
                        node.id, df_id, predicate_node, knowledge
                    ),
                    "shape=hexagon, style=filled, fillcolor=\"#f3e5f5\", color=\"#7b1fa2\"",
                ),
                PdgNodeKind::Unknown {
                    origin_node,
                    reason,
                } => (
                    format!(
                        "{}\\nUNKNOWN(origin={:?})\\n{}\\n[{}]",
                        node.id,
                        origin_node,
                        escape_dot(reason),
                        node.span
                    ),
                    "shape=octagon, style=\"bold,filled\", fillcolor=\"#ffebee\", color=\"#d32f2f\", fontcolor=\"#b71c1c\"",
                ),
            };

            dot.push_str(&format!(
                "  node_{} [label=\"{}\", {}];\n",
                node.id.0, label_str, shape_attrs
            ));
        }

        for edge in &self.edges {
            let edge_attrs = match &edge.kind {
                crate::edge::PdgEdgeKind::VariableDataDependency => {
                    "color=\"#1976d2\", label=\"var_data\""
                }
                crate::edge::PdgEdgeKind::ColumnDerivation {
                    transform_type,
                    knowledge,
                    ..
                } => {
                    &format!(
                        "color=\"#2e7d32\", style=bold, label=\"col_deriv({:?}, {})\"",
                        transform_type, knowledge
                    )
                }
                crate::edge::PdgEdgeKind::ColumnStatementBinding => {
                    "color=\"#0288d1\", style=dashed, label=\"stmt_col_binding\""
                }
                crate::edge::PdgEdgeKind::ControlDependency { edge_kind } => {
                    &format!("color=\"#c2185b\", style=dashed, label=\"control({:?})\"", edge_kind)
                }
                crate::edge::PdgEdgeKind::RowSelectionDependency { knowledge } => {
                    &format!("color=\"#7b1fa2\", label=\"row_select({})\"", knowledge)
                }
                crate::edge::PdgEdgeKind::UnknownDependency { reason } => {
                    &format!(
                        "color=\"#d50000\", style=\"dashed,bold\", fontcolor=\"#d50000\", label=\"unknown({})\"",
                        escape_dot(reason)
                    )
                }
            };

            dot.push_str(&format!(
                "  node_{} -> node_{} [{}];\n",
                edge.from.0, edge.to.0, edge_attrs
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Emits a structured JSON value of the PDG topology.
    pub fn to_json_value(&self) -> rparse_core::JsonValue {
        use rparse_core::JsonValue;

        let mut root = BTreeMap::new();
        root.insert(
            "graph_type".to_string(),
            JsonValue::String("pdg".to_string()),
        );

        let mut nodes_arr = Vec::new();
        let mut stmt_cnt = 0;
        let mut expr_cnt = 0;
        let mut vdef_cnt = 0;
        let mut vuse_cnt = 0;
        let mut col_cnt = 0;
        let mut row_cnt = 0;
        let mut unk_node_cnt = 0;

        for node in &self.nodes {
            let mut n_map = BTreeMap::new();
            n_map.insert("id".to_string(), JsonValue::Number(node.id.0 as f64));
            n_map.insert(
                "node_id".to_string(),
                JsonValue::String(node.id.to_string()),
            );
            n_map.insert("label".to_string(), JsonValue::String(node.to_string()));
            n_map.insert("span".to_string(), node.span.to_json());
            n_map.insert("is_unknown".to_string(), JsonValue::Bool(node.is_unknown()));
            n_map.insert(
                "is_column_version".to_string(),
                JsonValue::Bool(node.is_column_version()),
            );

            let mut details = BTreeMap::new();
            let kind_str = match &node.kind {
                PdgNodeKind::Statement {
                    stmt_id,
                    description,
                } => {
                    stmt_cnt += 1;
                    details.insert(
                        "stmt_id".to_string(),
                        JsonValue::Number(stmt_id.raw() as f64),
                    );
                    details.insert(
                        "description".to_string(),
                        JsonValue::String(description.clone()),
                    );
                    "Statement"
                }
                PdgNodeKind::Expression {
                    expr_id,
                    description,
                } => {
                    expr_cnt += 1;
                    details.insert(
                        "expr_id".to_string(),
                        JsonValue::Number(expr_id.raw() as f64),
                    );
                    details.insert(
                        "description".to_string(),
                        JsonValue::String(description.clone()),
                    );
                    "Expression"
                }
                PdgNodeKind::VariableDef {
                    def_id,
                    symbol_id,
                    name,
                } => {
                    vdef_cnt += 1;
                    details.insert("def_id".to_string(), JsonValue::Number(def_id.0 as f64));
                    details.insert(
                        "symbol_id".to_string(),
                        JsonValue::Number(symbol_id.0 as f64),
                    );
                    details.insert("name".to_string(), JsonValue::String(name.clone()));
                    "VariableDef"
                }
                PdgNodeKind::VariableUse {
                    use_id,
                    symbol_id,
                    name,
                } => {
                    vuse_cnt += 1;
                    details.insert("use_id".to_string(), JsonValue::Number(use_id.0 as f64));
                    details.insert(
                        "symbol_id".to_string(),
                        symbol_id
                            .map(|s| JsonValue::Number(s.0 as f64))
                            .unwrap_or(JsonValue::Null),
                    );
                    details.insert("name".to_string(), JsonValue::String(name.clone()));
                    "VariableUse"
                }
                PdgNodeKind::ColumnVersion {
                    col_id,
                    df_id,
                    col_name,
                    version,
                    knowledge,
                } => {
                    col_cnt += 1;
                    details.insert("col_id".to_string(), JsonValue::Number(col_id.0 as f64));
                    details.insert(
                        "col_ver_id".to_string(),
                        JsonValue::String(col_id.to_string()),
                    );
                    details.insert("df_id".to_string(), JsonValue::Number(df_id.0 as f64));
                    details.insert("df_label".to_string(), JsonValue::String(df_id.to_string()));
                    details.insert("col_name".to_string(), JsonValue::String(col_name.clone()));
                    details.insert("version".to_string(), JsonValue::Number(*version as f64));
                    details.insert(
                        "knowledge".to_string(),
                        JsonValue::String(knowledge.to_string()),
                    );
                    "ColumnVersion"
                }
                PdgNodeKind::RowSelection {
                    df_id,
                    predicate_node,
                    knowledge,
                } => {
                    row_cnt += 1;
                    details.insert("df_id".to_string(), JsonValue::Number(df_id.0 as f64));
                    details.insert(
                        "predicate_node".to_string(),
                        JsonValue::Number(predicate_node.raw() as f64),
                    );
                    details.insert(
                        "knowledge".to_string(),
                        JsonValue::String(knowledge.to_string()),
                    );
                    "RowSelection"
                }
                PdgNodeKind::Unknown {
                    origin_node,
                    reason,
                } => {
                    unk_node_cnt += 1;
                    details.insert(
                        "origin_node".to_string(),
                        origin_node
                            .map(|o| JsonValue::Number(o.raw() as f64))
                            .unwrap_or(JsonValue::Null),
                    );
                    details.insert("reason".to_string(), JsonValue::String(reason.clone()));
                    "Unknown"
                }
            };

            n_map.insert("kind".to_string(), JsonValue::String(kind_str.to_string()));
            n_map.insert("details".to_string(), JsonValue::Object(details));
            nodes_arr.push(JsonValue::Object(n_map));
        }
        root.insert("nodes".to_string(), JsonValue::Array(nodes_arr));

        let mut edges_arr = Vec::new();
        let mut unk_edge_cnt = 0;

        for edge in &self.edges {
            let mut e_map = BTreeMap::new();
            e_map.insert("id".to_string(), JsonValue::Number(edge.id.0 as f64));
            e_map.insert(
                "edge_id".to_string(),
                JsonValue::String(edge.id.to_string()),
            );
            e_map.insert("from".to_string(), JsonValue::Number(edge.from.0 as f64));
            e_map.insert("to".to_string(), JsonValue::Number(edge.to.0 as f64));
            e_map.insert(
                "direction".to_string(),
                JsonValue::String("source_to_dependent".to_string()),
            );
            e_map.insert("is_unknown".to_string(), JsonValue::Bool(edge.is_unknown()));

            let mut details = BTreeMap::new();
            let kind_str = match &edge.kind {
                crate::edge::PdgEdgeKind::VariableDataDependency => "VariableDataDependency",
                crate::edge::PdgEdgeKind::ColumnDerivation {
                    provenance_edge_id,
                    transform_type,
                    knowledge,
                } => {
                    details.insert(
                        "provenance_edge_id".to_string(),
                        provenance_edge_id
                            .map(|p| JsonValue::Number(p.0 as f64))
                            .unwrap_or(JsonValue::Null),
                    );
                    details.insert(
                        "transform_type".to_string(),
                        JsonValue::String(format!("{:?}", transform_type)),
                    );
                    details.insert(
                        "knowledge".to_string(),
                        JsonValue::String(knowledge.to_string()),
                    );
                    "ColumnDerivation"
                }
                crate::edge::PdgEdgeKind::ColumnStatementBinding => "ColumnStatementBinding",
                crate::edge::PdgEdgeKind::ControlDependency { edge_kind } => {
                    details.insert(
                        "edge_kind".to_string(),
                        JsonValue::String(format!("{:?}", edge_kind)),
                    );
                    "ControlDependency"
                }
                crate::edge::PdgEdgeKind::RowSelectionDependency { knowledge } => {
                    details.insert(
                        "knowledge".to_string(),
                        JsonValue::String(knowledge.to_string()),
                    );
                    "RowSelectionDependency"
                }
                crate::edge::PdgEdgeKind::UnknownDependency { reason } => {
                    unk_edge_cnt += 1;
                    details.insert("reason".to_string(), JsonValue::String(reason.clone()));
                    "UnknownDependency"
                }
            };

            e_map.insert("kind".to_string(), JsonValue::String(kind_str.to_string()));
            e_map.insert("details".to_string(), JsonValue::Object(details));
            edges_arr.push(JsonValue::Object(e_map));
        }
        root.insert("edges".to_string(), JsonValue::Array(edges_arr));

        let mut summary = BTreeMap::new();
        summary.insert(
            "node_count".to_string(),
            JsonValue::Number(self.nodes.len() as f64),
        );
        summary.insert(
            "edge_count".to_string(),
            JsonValue::Number(self.edges.len() as f64),
        );
        summary.insert(
            "statement_count".to_string(),
            JsonValue::Number(stmt_cnt as f64),
        );
        summary.insert(
            "expression_count".to_string(),
            JsonValue::Number(expr_cnt as f64),
        );
        summary.insert(
            "variable_def_count".to_string(),
            JsonValue::Number(vdef_cnt as f64),
        );
        summary.insert(
            "variable_use_count".to_string(),
            JsonValue::Number(vuse_cnt as f64),
        );
        summary.insert(
            "column_port_count".to_string(),
            JsonValue::Number(col_cnt as f64),
        );
        summary.insert(
            "row_selection_count".to_string(),
            JsonValue::Number(row_cnt as f64),
        );
        summary.insert(
            "unknown_node_count".to_string(),
            JsonValue::Number(unk_node_cnt as f64),
        );
        summary.insert(
            "unknown_edge_count".to_string(),
            JsonValue::Number(unk_edge_cnt as f64),
        );
        root.insert("summary".to_string(), JsonValue::Object(summary));

        JsonValue::Object(root)
    }

    /// Emits a deterministic compact JSON string of the PDG topology.
    pub fn to_json(&self) -> String {
        self.to_json_value().to_json_string()
    }
}

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Convenience free function validating a [`ProgramDependenceGraph`].
pub fn validate_pdg(pdg: &ProgramDependenceGraph) -> Result<(), PdgValidationError> {
    pdg.validate()
}
