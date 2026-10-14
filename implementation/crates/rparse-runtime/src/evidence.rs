//! Empirical runtime evidence model and orthogonal static/runtime uncertainty tracking.

use std::fmt;

use rparse_dataframe::StaticKnowledge;

/// Orthogonal runtime evidence classification representing empirical observation in GNU R.
///
/// **Boundary Rule:** Runtime evidence represents empirical observation in a specific execution context;
/// it does NOT constitute universal proof across unobserved inputs or alternate execution paths,
/// and must NEVER overwrite or fabricate static dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RuntimeEvidence {
    /// No runtime observation occurred (e.g. in STATIC mode or before execution).
    #[default]
    None,
    /// Empirically observed during a specific GNU R execution session.
    Observed(ObservationRecord),
}

impl RuntimeEvidence {
    /// Returns true if no runtime observation has occurred.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns true if this state carries empirical runtime observation.
    pub fn is_observed(&self) -> bool {
        matches!(self, Self::Observed(_))
    }

    /// Returns the observation record if observed.
    pub fn as_observed(&self) -> Option<&ObservationRecord> {
        match self {
            Self::Observed(record) => Some(record),
            Self::None => None,
        }
    }

    /// Returns the observed columns if observation succeeded.
    pub fn observed_columns(&self) -> Option<&[ObservedColumn]> {
        match self {
            Self::Observed(rec) if rec.status.is_success() => Some(&rec.observed_columns),
            _ => None,
        }
    }
}

impl fmt::Display for RuntimeEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Observed(rec) => write!(f, "OBSERVED({})", rec.target_symbol),
        }
    }
}

/// Status of an empirical observation in GNU R.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationStatus {
    /// Observation completed successfully.
    Success,
    /// Observation attempted but failed (e.g. symbol not found, expression errored).
    Error(String),
}

impl ObservationStatus {
    /// Returns true if observation succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Returns true if observation failed.
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
}

/// A single dataframe column observed empirically in GNU R.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObservedColumn {
    /// Column name.
    pub name: String,
    /// Storage/base R type (`typeof`, e.g. "numeric", "integer", "character", "logical").
    pub r_type: String,
    /// High-level S3/S4 classes (`class`, e.g. ["numeric"], ["factor"], ["POSIXct", "POSIXt"]).
    pub class_name: Vec<String>,
}

impl fmt::Display for ObservedColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.r_type)
    }
}

/// Detailed empirical record of an observation performed in GNU R.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationRecord {
    /// The target symbol or object inspected (e.g. "df", "clean").
    pub target_symbol: String,
    /// The R expression evaluated prior to inspection, if any.
    pub expression: Option<String>,
    /// Status of the observation.
    pub status: ObservationStatus,
    /// Columns observed on the dataframe object.
    pub observed_columns: Vec<ObservedColumn>,
    /// Observed row count estimate if available.
    pub row_count_estimate: Option<usize>,
    /// Execution duration in milliseconds.
    pub execution_time_ms: u64,
    /// Optional context information or diagnostic note.
    pub context_note: Option<String>,
}

impl ObservationRecord {
    /// Creates a successful observation record.
    pub fn success(
        target_symbol: impl Into<String>,
        expression: Option<String>,
        observed_columns: Vec<ObservedColumn>,
        row_count_estimate: Option<usize>,
        execution_time_ms: u64,
    ) -> Self {
        Self {
            target_symbol: target_symbol.into(),
            expression,
            status: ObservationStatus::Success,
            observed_columns,
            row_count_estimate,
            execution_time_ms,
            context_note: None,
        }
    }

    /// Creates a failed observation record.
    pub fn error(
        target_symbol: impl Into<String>,
        expression: Option<String>,
        error_message: impl Into<String>,
        execution_time_ms: u64,
    ) -> Self {
        Self {
            target_symbol: target_symbol.into(),
            expression,
            status: ObservationStatus::Error(error_message.into()),
            observed_columns: Vec::new(),
            row_count_estimate: None,
            execution_time_ms,
            context_note: None,
        }
    }
}

/// Orthogonal pairing of static knowledge and empirical runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundedEvidence {
    /// The static analysis certainty state.
    pub static_knowledge: StaticKnowledge,
    /// The empirical runtime evidence state.
    pub runtime_evidence: RuntimeEvidence,
}

impl GroundedEvidence {
    /// Creates a new grounded evidence state.
    pub fn new(static_knowledge: StaticKnowledge, runtime_evidence: RuntimeEvidence) -> Self {
        Self {
            static_knowledge,
            runtime_evidence,
        }
    }

    /// Creates an unobserved static state.
    pub fn unobserved(static_knowledge: StaticKnowledge) -> Self {
        Self {
            static_knowledge,
            runtime_evidence: RuntimeEvidence::None,
        }
    }

    /// Returns true if this fact was statically known and empirically observed.
    pub fn is_confirmed(&self) -> bool {
        self.static_knowledge.is_known() && self.runtime_evidence.is_observed()
    }

    /// Returns true if this fact was statically uncertain but empirically observed in this run.
    pub fn is_empirically_grounded(&self) -> bool {
        self.static_knowledge.is_unknown() && self.runtime_evidence.is_observed()
    }
}

impl fmt::Display for GroundedEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Static: {}, Runtime: {}]",
            self.static_knowledge, self.runtime_evidence
        )
    }
}
