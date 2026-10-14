//! Column-aware static backward program slicing over the Unified PDG for R-PARSE.
//!
//! Provides Phase 5 slicing foundations:
//! - Strongly typed [`SliceCriterion`] (scalar variable, whole dataframe, individual column port).
//! - Graph-based backward reachability over the Unified PDG.
//! - Fine-grained [`SliceElement`] model distinguishing `FullyRetained`, `PartiallyRetained`,
//!   and `NotRetained`.
//! - Mutate argument partial statement retention.
//! - Precise column port slicing excluding unrelated sibling columns.
//! - Slice confidence and static uncertainty reporting ([`SliceConfidence`]).

pub mod criterion;
pub mod element;
pub mod result;
pub mod slicer;

pub use criterion::{resolve_criterion, CriterionResolutionError, SliceCriterion};
pub use element::{RetentionState, SliceElement, SubElementSlice};
pub use result::{SliceConfidence, SliceResult, StatementSlice};
pub use slicer::{
    compute_backward_slice, compute_backward_slice_with_source, compute_column_slice,
    compute_column_slice_with_source, compute_object_slice, SlicingError,
};
