//! `rparse-runtime`: GNU R runtime bridge, process management, and empirical evidence observation.
//!
//! Provides optional, bounded execution of R expressions in a controlled child process environment.
//!
//! # Core Architectural Invariants
//! 1. **Static Analysis is Authoritative:** Static models and program dependence graphs remain 100%
//!    self-contained and offline.
//! 2. **Never Execute in Static Mode:** In [`AnalysisMode::Static`], no GNU R child process is ever spawned.
//! 3. **Explicit Uncertainty Distinction:**
//!    - Static knowledge is [`rparse_dataframe::StaticKnowledge::KnownStatic`] or [`rparse_dataframe::StaticKnowledge::UnknownStatic`].
//!    - Empirical runtime evidence is [`RuntimeEvidence::None`] or [`RuntimeEvidence::Observed`].
//!    - Runtime observation annotates states with empirical facts without claiming universal whole-program proof.
//! 4. **No Security Sandbox Claim:** GNU R execution executes real code; `new.env()` provides a controlled
//!    evaluation environment and isolated child process, not an OS-level container or security sandbox.

pub mod bridge;
pub mod config;
pub mod discovery;
pub mod error;
pub mod evidence;
pub mod integration;
pub mod json;
pub mod mock;
pub mod process;
pub mod protocol;

pub use bridge::RuntimeBridge;
pub use config::{AnalysisMode, RuntimeConfig, DEFAULT_TIMEOUT_MS};
pub use discovery::{discover_r_executable, is_r_available};
pub use error::RuntimeError;
pub use evidence::{
    GroundedEvidence, ObservationRecord, ObservationStatus, ObservedColumn, RuntimeEvidence,
};
pub use integration::{AnnotatedDataFrameState, ColumnCorrelation, CorrelationStatus};
pub use json::JsonValue;
pub use mock::MockBackend;
pub use process::{RProcess, RuntimeBackend, R_WORKER_SCRIPT};
pub use protocol::{
    read_message, write_message, DataFrameSchemaObservation, EvalParams, EvalResult,
    InspectDataFrameParams, InspectObjectParams, ObjectMetadata, RpcError, RpcRequest, RpcResponse,
};
