//! First-class fine-grained column provenance model.

use crate::ids::{ColumnVersionId, ProvenanceEdgeId};
use crate::unknown::StaticKnowledge;
use rparse_core::{NodeId, Span};
use rparse_scope::SymbolId;
use std::collections::{BTreeMap, BTreeSet};

/// The semantic classification of a column transformation event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformationType {
    /// Derived via `mutate(...)`.
    Mutate,
    /// Derived via `transmute(...)`.
    Transmute,
    /// Derived via `rename(new = old)`.
    Rename { old_name: String },
    /// Derived via direct Base R column assignment (`df$col <- ...`).
    BaseRDirectAssignment,
    /// Derived via `transform(df, ...)`.
    Transform,
    /// Identity carried forward without modification.
    IdentityForward,
}

/// A first-class column provenance derivation event:
/// Grouped input column versions + expression -> output column version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEdge {
    /// Unique identifier for this derivation event.
    pub id: ProvenanceEdgeId,
    /// The input column versions that directly participated in this derivation.
    pub input_columns: BTreeSet<ColumnVersionId>,
    /// Lexical variables (scalar symbols) that participated in this derivation.
    pub input_variables: BTreeSet<SymbolId>,
    /// The resulting output column version.
    pub output_column: ColumnVersionId,
    /// Semantic AST node of the deriving expression, if available.
    pub expression_node: Option<NodeId>,
    /// Source coordinates of the transformation expression.
    pub span: Span,
    /// Classification of the transformation verb/form.
    pub transform_type: TransformationType,
    /// Static certainty status of the derivation.
    pub knowledge: StaticKnowledge,
}

/// Initialization parameters for constructing a [`ProvenanceEdge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEdgeInit {
    pub id: ProvenanceEdgeId,
    pub input_columns: BTreeSet<ColumnVersionId>,
    pub input_variables: BTreeSet<SymbolId>,
    pub output_column: ColumnVersionId,
    pub expression_node: Option<NodeId>,
    pub span: Span,
    pub transform_type: TransformationType,
    pub knowledge: StaticKnowledge,
}

impl From<ProvenanceEdgeInit> for ProvenanceEdge {
    fn from(init: ProvenanceEdgeInit) -> Self {
        Self {
            id: init.id,
            input_columns: init.input_columns,
            input_variables: init.input_variables,
            output_column: init.output_column,
            expression_node: init.expression_node,
            span: init.span,
            transform_type: init.transform_type,
            knowledge: init.knowledge,
        }
    }
}

impl ProvenanceEdge {
    pub fn new(init: ProvenanceEdgeInit) -> Self {
        init.into()
    }
}

/// The column provenance container supporting exact derivation query and retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProvenanceGraph {
    edges: Vec<ProvenanceEdge>,
    by_output: BTreeMap<ColumnVersionId, ProvenanceEdgeId>,
    by_input: BTreeMap<ColumnVersionId, Vec<ProvenanceEdgeId>>,
}

impl ProvenanceGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a provenance derivation edge to the graph.
    pub fn add_edge(&mut self, edge: ProvenanceEdge) {
        let edge_id = edge.id;
        let cout = edge.output_column;

        self.by_output.insert(cout, edge_id);
        for &cin in &edge.input_columns {
            self.by_input.entry(cin).or_default().push(edge_id);
        }
        self.edges.push(edge);
    }

    /// Looks up the derivation edge that produced a given output column version.
    pub fn lookup_by_output(&self, cout: ColumnVersionId) -> Option<&ProvenanceEdge> {
        self.by_output.get(&cout).and_then(|&id| self.edge(id))
    }

    /// Looks up derivation edge IDs where `cin` is an immediate input column.
    pub fn lookup_by_input(&self, cin: ColumnVersionId) -> &[ProvenanceEdgeId] {
        self.by_input.get(&cin).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Returns the immediate input column versions for an output column version.
    pub fn incoming_columns(&self, cout: ColumnVersionId) -> BTreeSet<ColumnVersionId> {
        self.lookup_by_output(cout)
            .map(|e| e.input_columns.clone())
            .unwrap_or_default()
    }

    /// Access an edge by its [`ProvenanceEdgeId`].
    pub fn edge(&self, id: ProvenanceEdgeId) -> Option<&ProvenanceEdge> {
        self.edges.get(id.0 as usize)
    }

    /// Alias for [`Self::edge`].
    pub fn get_edge(&self, id: ProvenanceEdgeId) -> Option<&ProvenanceEdge> {
        self.edge(id)
    }

    /// Looks up output column versions derived immediately from a given input column version.
    pub fn derived_from_input(&self, cin: ColumnVersionId) -> BTreeSet<ColumnVersionId> {
        let mut outputs = BTreeSet::new();
        for &edge_id in self.lookup_by_input(cin) {
            if let Some(edge) = self.edge(edge_id) {
                outputs.insert(edge.output_column);
            }
        }
        outputs
    }

    /// Computes all transitive ancestor column versions for a given column version.
    pub fn ancestor_columns(&self, cout: ColumnVersionId) -> BTreeSet<ColumnVersionId> {
        let mut ancestors = BTreeSet::new();
        let mut queue = vec![cout];
        while let Some(curr) = queue.pop() {
            if let Some(edge) = self.lookup_by_output(curr) {
                for &cin in &edge.input_columns {
                    if ancestors.insert(cin) {
                        queue.push(cin);
                    }
                }
            }
        }
        ancestors
    }

    /// Slice of all provenance derivation edges in deterministic generation order.
    pub fn edges(&self) -> &[ProvenanceEdge] {
        &self.edges
    }

    /// Emits a deterministic Graphviz DOT string of the provenance graph using default ID labels.
    pub fn to_dot(&self) -> String {
        self.to_dot_with_labels(&|_| None)
    }

    /// Emits a deterministic Graphviz DOT string of the provenance graph with resolved column labels.
    pub fn to_dot_with_labels<L: ColumnLabelProvider>(&self, labels: &L) -> String {
        let mut dot = String::new();
        dot.push_str("digraph ColumnProvenance {\n");
        dot.push_str("  node [fontname=\"Courier\"];\n");
        dot.push_str("  edge [fontname=\"Courier\", fontsize=10];\n");

        // Collect all column version IDs deterministically
        let mut all_cols = BTreeSet::new();
        for cid in labels.all_columns() {
            all_cols.insert(cid);
        }
        for edge in &self.edges {
            all_cols.insert(edge.output_column);
            for &cin in &edge.input_columns {
                all_cols.insert(cin);
            }
        }

        // Render column nodes
        for &col_id in &all_cols {
            let label = if let Some(lbl) = labels.column_label(col_id) {
                format!("{}\\n{}", escape_dot(&lbl), col_id)
            } else {
                format!("{}", col_id)
            };
            dot.push_str(&format!(
                "  col_{} [label=\"{}\", shape=ellipse, style=filled, fillcolor=\"#e1f5fe\", color=\"#0288d1\"];\n",
                col_id.0, label
            ));
        }

        // Track scalar symbols declared
        let mut declared_symbols = BTreeSet::new();

        // Render derivation event nodes and edges
        for edge in &self.edges {
            let (shape_attrs, edge_color, edge_style) = match edge.knowledge {
                StaticKnowledge::KnownStatic => (
                    "shape=box, style=\"filled,rounded\", fillcolor=\"#e8f5e9\", color=\"#388e3c\"",
                    "#388e3c",
                    "",
                ),
                StaticKnowledge::UnknownStatic => (
                    "shape=box, style=\"filled,rounded,bold\", fillcolor=\"#ffebee\", color=\"#d32f2f\", fontcolor=\"#b71c1c\"",
                    "#d32f2f",
                    ", style=\"dashed,bold\"",
                ),
            };

            let deriv_label = format!(
                "{}\\n{:?}\\n{}\\n[{}]",
                edge.id, edge.transform_type, edge.span, edge.knowledge
            );
            dot.push_str(&format!(
                "  pedge_{} [label=\"{}\", {}];\n",
                edge.id.0,
                escape_dot(&deriv_label),
                shape_attrs
            ));

            // Input columns -> Derivation node
            for &cin in &edge.input_columns {
                dot.push_str(&format!(
                    "  col_{} -> pedge_{} [color=\"{}\"{}];\n",
                    cin.0, edge.id.0, edge_color, edge_style
                ));
            }

            // Input scalar variables -> Derivation node
            for &sym in &edge.input_variables {
                if declared_symbols.insert(sym) {
                    dot.push_str(&format!(
                        "  sym_{} [label=\"sym#{}\", shape=note, style=filled, fillcolor=\"#fff9c4\", color=\"#fbc02d\"];\n",
                        sym.0, sym.0
                    ));
                }
                dot.push_str(&format!(
                    "  sym_{} -> pedge_{} [color=\"#fbc02d\", style=dashed, label=\"scalar\"];\n",
                    sym.0, edge.id.0
                ));
            }

            // Derivation node -> Output column
            let out_edge_style = match edge.knowledge {
                StaticKnowledge::KnownStatic => "color=\"#1b5e20\", style=bold",
                StaticKnowledge::UnknownStatic => "color=\"#d32f2f\", style=\"dashed,bold\"",
            };
            dot.push_str(&format!(
                "  pedge_{} -> col_{} [{}];\n",
                edge.id.0, edge.output_column.0, out_edge_style
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Emits a structured JSON value of the column provenance topology.
    pub fn to_json_value_with_labels<L: ColumnLabelProvider>(
        &self,
        labels: &L,
    ) -> rparse_core::JsonValue {
        use rparse_core::JsonValue;

        let mut root = BTreeMap::new();
        root.insert(
            "graph_type".to_string(),
            JsonValue::String("provenance".to_string()),
        );

        // Collect all column version IDs deterministically
        let mut all_cols = BTreeSet::new();
        for cid in labels.all_columns() {
            all_cols.insert(cid);
        }
        for edge in &self.edges {
            all_cols.insert(edge.output_column);
            for &cin in &edge.input_columns {
                all_cols.insert(cin);
            }
        }

        let mut columns_arr = Vec::new();
        let mut generic_nodes = Vec::new();
        for &col_id in &all_cols {
            let label = labels
                .column_label(col_id)
                .unwrap_or_else(|| col_id.to_string());

            let mut c_map = BTreeMap::new();
            c_map.insert("id".to_string(), JsonValue::Number(col_id.0 as f64));
            c_map.insert(
                "col_ver_id".to_string(),
                JsonValue::String(col_id.to_string()),
            );
            c_map.insert("label".to_string(), JsonValue::String(label.clone()));
            columns_arr.push(JsonValue::Object(c_map));

            let mut gn_map = BTreeMap::new();
            gn_map.insert(
                "id".to_string(),
                JsonValue::String(format!("col_{}", col_id.0)),
            );
            gn_map.insert(
                "type".to_string(),
                JsonValue::String("column_version".to_string()),
            );
            gn_map.insert("col_id".to_string(), JsonValue::Number(col_id.0 as f64));
            gn_map.insert("label".to_string(), JsonValue::String(label));
            generic_nodes.push(JsonValue::Object(gn_map));
        }
        root.insert("columns".to_string(), JsonValue::Array(columns_arr));

        let mut derivations_arr = Vec::new();
        let mut generic_edges = Vec::new();
        let mut unk_deriv_cnt = 0;

        for edge in &self.edges {
            let is_unk = edge.knowledge == StaticKnowledge::UnknownStatic;
            if is_unk {
                unk_deriv_cnt += 1;
            }

            let mut d_map = BTreeMap::new();
            d_map.insert("id".to_string(), JsonValue::Number(edge.id.0 as f64));
            d_map.insert(
                "pedge_id".to_string(),
                JsonValue::String(edge.id.to_string()),
            );
            d_map.insert(
                "transform_type".to_string(),
                JsonValue::String(format!("{:?}", edge.transform_type)),
            );
            d_map.insert(
                "knowledge".to_string(),
                JsonValue::String(edge.knowledge.to_string()),
            );
            d_map.insert("is_unknown".to_string(), JsonValue::Bool(is_unk));
            d_map.insert("span".to_string(), edge.span.to_json());
            d_map.insert(
                "expression_node".to_string(),
                edge.expression_node
                    .map(|n| JsonValue::Number(n.raw() as f64))
                    .unwrap_or(JsonValue::Null),
            );

            let in_cols: Vec<JsonValue> = edge
                .input_columns
                .iter()
                .map(|c| JsonValue::Number(c.0 as f64))
                .collect();
            d_map.insert("input_columns".to_string(), JsonValue::Array(in_cols));

            let in_vars: Vec<JsonValue> = edge
                .input_variables
                .iter()
                .map(|v| JsonValue::Number(v.0 as f64))
                .collect();
            d_map.insert("input_variables".to_string(), JsonValue::Array(in_vars));

            d_map.insert(
                "output_column".to_string(),
                JsonValue::Number(edge.output_column.0 as f64),
            );
            derivations_arr.push(JsonValue::Object(d_map));

            // Derivation node in generic bipartite graph
            let mut deriv_node = BTreeMap::new();
            deriv_node.insert(
                "id".to_string(),
                JsonValue::String(format!("pedge_{}", edge.id.0)),
            );
            deriv_node.insert(
                "type".to_string(),
                JsonValue::String("derivation".to_string()),
            );
            deriv_node.insert(
                "transform_type".to_string(),
                JsonValue::String(format!("{:?}", edge.transform_type)),
            );
            deriv_node.insert(
                "knowledge".to_string(),
                JsonValue::String(edge.knowledge.to_string()),
            );
            deriv_node.insert("span".to_string(), edge.span.to_json());
            generic_nodes.push(JsonValue::Object(deriv_node));

            // Input column -> derivation edges
            for &cin in &edge.input_columns {
                let mut ge = BTreeMap::new();
                ge.insert(
                    "from".to_string(),
                    JsonValue::String(format!("col_{}", cin.0)),
                );
                ge.insert(
                    "to".to_string(),
                    JsonValue::String(format!("pedge_{}", edge.id.0)),
                );
                ge.insert(
                    "type".to_string(),
                    JsonValue::String("input_column".to_string()),
                );
                generic_edges.push(JsonValue::Object(ge));
            }

            // Input variable -> derivation edges
            for &sym in &edge.input_variables {
                let mut ge = BTreeMap::new();
                ge.insert(
                    "from".to_string(),
                    JsonValue::String(format!("sym_{}", sym.0)),
                );
                ge.insert(
                    "to".to_string(),
                    JsonValue::String(format!("pedge_{}", edge.id.0)),
                );
                ge.insert(
                    "type".to_string(),
                    JsonValue::String("input_variable".to_string()),
                );
                generic_edges.push(JsonValue::Object(ge));
            }

            // Derivation -> output column edge
            let mut ge = BTreeMap::new();
            ge.insert(
                "from".to_string(),
                JsonValue::String(format!("pedge_{}", edge.id.0)),
            );
            ge.insert(
                "to".to_string(),
                JsonValue::String(format!("col_{}", edge.output_column.0)),
            );
            ge.insert(
                "type".to_string(),
                JsonValue::String("output_column".to_string()),
            );
            generic_edges.push(JsonValue::Object(ge));
        }
        root.insert("derivations".to_string(), JsonValue::Array(derivations_arr));
        root.insert("nodes".to_string(), JsonValue::Array(generic_nodes));
        root.insert("edges".to_string(), JsonValue::Array(generic_edges));

        let mut summary = BTreeMap::new();
        summary.insert(
            "column_count".to_string(),
            JsonValue::Number(all_cols.len() as f64),
        );
        summary.insert(
            "derivation_count".to_string(),
            JsonValue::Number(self.edges.len() as f64),
        );
        summary.insert(
            "uncertain_derivation_count".to_string(),
            JsonValue::Number(unk_deriv_cnt as f64),
        );
        root.insert("summary".to_string(), JsonValue::Object(summary));

        JsonValue::Object(root)
    }

    /// Emits a deterministic compact JSON string of the provenance topology with default ID labels.
    pub fn to_json(&self) -> String {
        self.to_json_with_labels(&|_| None)
    }

    /// Emits a deterministic compact JSON string of the provenance topology with resolved column labels.
    pub fn to_json_with_labels<L: ColumnLabelProvider>(&self, labels: &L) -> String {
        self.to_json_value_with_labels(labels).to_json_string()
    }
}

/// Provider of human-readable column label annotations.
pub trait ColumnLabelProvider {
    fn column_label(&self, col_id: ColumnVersionId) -> Option<String>;
    fn all_columns(&self) -> Vec<ColumnVersionId> {
        Vec::new()
    }
}

impl<F> ColumnLabelProvider for F
where
    F: Fn(ColumnVersionId) -> Option<String>,
{
    fn column_label(&self, col_id: ColumnVersionId) -> Option<String> {
        self(col_id)
    }
}

impl ColumnLabelProvider for BTreeMap<ColumnVersionId, String> {
    fn column_label(&self, col_id: ColumnVersionId) -> Option<String> {
        self.get(&col_id).cloned()
    }

    fn all_columns(&self) -> Vec<ColumnVersionId> {
        self.keys().copied().collect()
    }
}

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
