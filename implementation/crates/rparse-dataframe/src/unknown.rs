//! Explicit static knowledge model preserving analytical certainty versus uncertainty.

use std::fmt;

/// The static certainty state of an analysis fact, column derivation, or dataframe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StaticKnowledge {
    /// Fully established statically from known source code and schema semantics.
    KnownStatic,
    /// Statically uncertain, dynamic, or unmodeled (must never silently disappear).
    UnknownStatic,
}

impl StaticKnowledge {
    /// Returns true if this fact is statically known with certainty.
    pub fn is_known(&self) -> bool {
        matches!(self, Self::KnownStatic)
    }

    /// Returns true if this fact involves static uncertainty.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::UnknownStatic)
    }

    /// Combines two static knowledge values: if either is unknown, the result is unknown.
    pub fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::KnownStatic, Self::KnownStatic) => Self::KnownStatic,
            _ => Self::UnknownStatic,
        }
    }
}

impl fmt::Display for StaticKnowledge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KnownStatic => write!(f, "KNOWN_STATIC"),
            Self::UnknownStatic => write!(f, "UNKNOWN_STATIC"),
        }
    }
}
