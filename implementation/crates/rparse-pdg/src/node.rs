//! Node representations in the Unified Program Dependence Graph.

use crate::ids::PdgNodeId;
use rparse_core::{NodeId, Span};
use rparse_dataflow::{VarDefId, VarUseId};
use rparse_dataframe::{ColumnVersionId, DataFrameId, StaticKnowledge};
use rparse_scope::SymbolId;
use std::fmt;

/// Semantic classification and payload of a node in the PDG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdgNodeKind {
    /// A normalized statement in the program AST/IR.
    Statement {
        stmt_id: NodeId,
        description: String,
    },
    /// A sub-statement expression or argument (e.g. mutate named argument expression).
    Expression {
        expr_id: NodeId,
        description: String,
    },
    /// A variable definition point where a symbol is assigned or bound.
    VariableDef {
        def_id: VarDefId,
        symbol_id: SymbolId,
        name: String,
    },
    /// A variable use point where a symbol is read.
    VariableUse {
        use_id: VarUseId,
        symbol_id: Option<SymbolId>,
        name: String,
    },
    /// A specific immutable column version port (`df@v#n.col`).
    ColumnVersion {
        col_id: ColumnVersionId,
        df_id: DataFrameId,
        col_name: String,
        version: u32,
        knowledge: StaticKnowledge,
    },
    /// A dataframe row-selection filtering predicate node (`filter(...)`).
    RowSelection {
        df_id: DataFrameId,
        predicate_node: NodeId,
        knowledge: StaticKnowledge,
    },
    /// An explicit unknown dependency origin (`UNKNOWN_STATIC`).
    Unknown {
        origin_node: Option<NodeId>,
        reason: String,
    },
}

impl PdgNodeKind {
    pub fn is_statement(&self) -> bool {
        matches!(self, Self::Statement { .. })
    }

    pub fn is_expression(&self) -> bool {
        matches!(self, Self::Expression { .. })
    }

    pub fn is_variable_def(&self) -> bool {
        matches!(self, Self::VariableDef { .. })
    }

    pub fn is_variable_use(&self) -> bool {
        matches!(self, Self::VariableUse { .. })
    }

    pub fn is_column_version(&self) -> bool {
        matches!(self, Self::ColumnVersion { .. })
    }

    pub fn is_row_selection(&self) -> bool {
        matches!(self, Self::RowSelection { .. })
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }

    pub fn col_id(&self) -> Option<ColumnVersionId> {
        match self {
            Self::ColumnVersion { col_id, .. } => Some(*col_id),
            _ => None,
        }
    }

    pub fn stmt_id(&self) -> Option<NodeId> {
        match self {
            Self::Statement { stmt_id, .. } => Some(*stmt_id),
            _ => None,
        }
    }

    pub fn expr_id(&self) -> Option<NodeId> {
        match self {
            Self::Expression { expr_id, .. } => Some(*expr_id),
            _ => None,
        }
    }

    pub fn symbol_id(&self) -> Option<SymbolId> {
        match self {
            Self::VariableDef { symbol_id, .. } => Some(*symbol_id),
            Self::VariableUse { symbol_id, .. } => *symbol_id,
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::VariableDef { name, .. } | Self::VariableUse { name, .. } => Some(name.as_str()),
            Self::ColumnVersion { col_name, .. } => Some(col_name.as_str()),
            _ => None,
        }
    }
}

/// A node in the Unified Program Dependence Graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdgNode {
    /// Deterministic identifier for this graph node.
    pub id: PdgNodeId,
    /// Detailed classification and semantic metadata.
    pub kind: PdgNodeKind,
    /// Source coordinates for this semantic unit.
    pub span: Span,
}

impl PdgNode {
    pub fn new(id: PdgNodeId, kind: PdgNodeKind, span: Span) -> Self {
        Self { id, kind, span }
    }

    pub fn is_statement(&self) -> bool {
        self.kind.is_statement()
    }

    pub fn is_expression(&self) -> bool {
        self.kind.is_expression()
    }

    pub fn is_variable_def(&self) -> bool {
        self.kind.is_variable_def()
    }

    pub fn is_variable_use(&self) -> bool {
        self.kind.is_variable_use()
    }

    pub fn is_column_version(&self) -> bool {
        self.kind.is_column_version()
    }

    pub fn is_row_selection(&self) -> bool {
        self.kind.is_row_selection()
    }

    pub fn is_unknown(&self) -> bool {
        self.kind.is_unknown()
    }

    pub fn col_id(&self) -> Option<ColumnVersionId> {
        self.kind.col_id()
    }

    pub fn stmt_id(&self) -> Option<NodeId> {
        self.kind.stmt_id()
    }

    pub fn expr_id(&self) -> Option<NodeId> {
        self.kind.expr_id()
    }

    pub fn symbol_id(&self) -> Option<SymbolId> {
        self.kind.symbol_id()
    }

    pub fn name(&self) -> Option<&str> {
        self.kind.name()
    }
}

impl fmt::Display for PdgNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Statement {
                stmt_id,
                description,
            } => {
                write!(f, "Statement({} - {})", stmt_id, description)
            }
            Self::Expression {
                expr_id,
                description,
            } => {
                write!(f, "Expression({} - {})", expr_id, description)
            }
            Self::VariableDef {
                def_id,
                name,
                symbol_id,
            } => {
                write!(f, "VarDef({} '{}' sym#{})", def_id, name, symbol_id)
            }
            Self::VariableUse {
                use_id,
                name,
                symbol_id,
            } => {
                write!(f, "VarUse({} '{}' sym={:?})", use_id, name, symbol_id)
            }
            Self::ColumnVersion {
                col_id,
                df_id,
                col_name,
                version,
                knowledge,
            } => {
                write!(
                    f,
                    "ColumnPort({} {}@v{} col_id={} [{}])",
                    df_id, col_name, version, col_id, knowledge
                )
            }
            Self::RowSelection {
                df_id,
                predicate_node,
                knowledge,
            } => {
                write!(
                    f,
                    "RowSelection({} pred_node={} [{}])",
                    df_id, predicate_node, knowledge
                )
            }
            Self::Unknown {
                origin_node,
                reason,
            } => {
                write!(f, "Unknown(origin={:?} reason='{}')", origin_node, reason)
            }
        }
    }
}

impl fmt::Display for PdgNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.kind)
    }
}
