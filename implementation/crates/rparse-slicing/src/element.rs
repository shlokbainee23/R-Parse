//! Fine-grained semantic slice element model and retention status.

use rparse_core::Span;
use rparse_pdg::{PdgNodeId, PdgNodeKind};
use std::fmt;

/// Retention status of a program construct in the computed slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RetentionState {
    /// Fully included in the slice dependency closure.
    FullyRetained,
    /// Partially included (e.g. mutate statement where some named arguments are retained and others excluded).
    PartiallyRetained,
    /// Not included in the slice; eliminated as irrelevant to the criterion.
    NotRetained,
}

impl fmt::Display for RetentionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FullyRetained => write!(f, "FULLY_RETAINED"),
            Self::PartiallyRetained => write!(f, "PARTIALLY_RETAINED"),
            Self::NotRetained => write!(f, "NOT_RETAINED"),
        }
    }
}

/// A fine-grained semantic element participating in the slice analysis result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceElement {
    /// Corresponding PDG node identifier.
    pub node_id: PdgNodeId,
    /// Semantic classification of the element.
    pub kind: PdgNodeKind,
    /// Source coordinates for this semantic unit.
    pub span: Span,
    /// Retention state.
    pub retention: RetentionState,
}

impl SliceElement {
    pub fn new(
        node_id: PdgNodeId,
        kind: PdgNodeKind,
        span: Span,
        retention: RetentionState,
    ) -> Self {
        Self {
            node_id,
            kind,
            span,
            retention,
        }
    }

    pub fn is_retained(&self) -> bool {
        self.retention != RetentionState::NotRetained
    }
}

impl fmt::Display for SliceElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} ({})", self.retention, self.kind, self.span)
    }
}

/// A sub-statement semantic element (such as an individual named argument in `mutate`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubElementSlice {
    /// Source label or description of the sub-element.
    pub label: String,
    /// Retention status of this sub-element.
    pub retention: RetentionState,
    /// Source coordinates of the sub-element, if known.
    pub span: Option<Span>,
}

impl SubElementSlice {
    pub fn new(label: impl Into<String>, retention: RetentionState, span: Option<Span>) -> Self {
        Self {
            label: label.into(),
            retention,
            span,
        }
    }

    pub fn is_retained(&self) -> bool {
        self.retention != RetentionState::NotRetained
    }
}

impl fmt::Display for SubElementSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(sp) = self.span {
            write!(f, "[{}] {} ({})", self.retention, self.label, sp)
        } else {
            write!(f, "[{}] {}", self.retention, self.label)
        }
    }
}
