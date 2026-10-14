//! Integration of runtime evidence with static analysis models.
//!
//! **Architectural Invariant:** Static models remain authoritative and deterministic.
//! Runtime evidence annotates and complements static models without mutating or overriding
//! the underlying static AST, CFG, or PDG dependency structures.

use rparse_dataframe::{DataFrameState, StaticKnowledge};

use crate::evidence::RuntimeEvidence;

/// Correlation status comparing a static column declaration with runtime observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrelationStatus {
    /// Column was statically known and confirmed by runtime observation.
    Confirmed,
    /// Column was statically unknown (e.g. dynamic/external), but observed empirically at runtime.
    EmpiricalOnly,
    /// Column was statically known, but not present in the runtime observation.
    StaticOnly,
    /// Column is statically unknown and no runtime observation exists.
    Unobserved,
}

/// Correlation record for a single column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnCorrelation {
    /// Column name.
    pub name: String,
    /// Static knowledge state.
    pub static_knowledge: StaticKnowledge,
    /// Observed runtime type if observed.
    pub observed_type: Option<String>,
    /// Correlation status.
    pub status: CorrelationStatus,
}

/// An annotated dataframe state associating static analysis schema with optional runtime evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatedDataFrameState {
    /// The original static dataframe state (from Phase 4).
    pub static_state: DataFrameState,
    /// Optional empirical runtime evidence.
    pub evidence: RuntimeEvidence,
}

impl AnnotatedDataFrameState {
    /// Creates a new annotated state.
    pub fn new(static_state: DataFrameState, evidence: RuntimeEvidence) -> Self {
        Self {
            static_state,
            evidence,
        }
    }

    /// Creates an unobserved annotated state (pure static analysis).
    pub fn unobserved(static_state: DataFrameState) -> Self {
        Self {
            static_state,
            evidence: RuntimeEvidence::None,
        }
    }

    /// Returns the symbol name of the dataframe.
    pub fn symbol_name(&self) -> &str {
        &self.static_state.symbol_name
    }

    /// Computes column-level correlation between static knowledge and empirical observation.
    pub fn correlate_columns(&self) -> Vec<ColumnCorrelation> {
        let mut correlations = Vec::new();
        let observed_cols = self.evidence.observed_columns();

        for col_name in self.static_state.schema.keys() {
            let static_know = self.static_state.knowledge;
            let observed_match =
                observed_cols.and_then(|cols| cols.iter().find(|c| &c.name == col_name));

            let (status, obs_type) = match (static_know, observed_match) {
                (StaticKnowledge::KnownStatic, Some(c)) => {
                    (CorrelationStatus::Confirmed, Some(c.r_type.clone()))
                }
                (StaticKnowledge::KnownStatic, None) => (CorrelationStatus::StaticOnly, None),
                (StaticKnowledge::UnknownStatic, Some(c)) => {
                    (CorrelationStatus::EmpiricalOnly, Some(c.r_type.clone()))
                }
                (StaticKnowledge::UnknownStatic, None) => (CorrelationStatus::Unobserved, None),
            };

            correlations.push(ColumnCorrelation {
                name: col_name.clone(),
                static_knowledge: static_know,
                observed_type: obs_type,
                status,
            });
        }

        // Also include columns that were observed at runtime but were not present in static columns
        if let Some(cols) = observed_cols {
            for c in cols {
                if !self.static_state.schema.contains_key(&c.name) {
                    correlations.push(ColumnCorrelation {
                        name: c.name.clone(),
                        static_knowledge: StaticKnowledge::UnknownStatic,
                        observed_type: Some(c.r_type.clone()),
                        status: CorrelationStatus::EmpiricalOnly,
                    });
                }
            }
        }

        correlations
    }
}
