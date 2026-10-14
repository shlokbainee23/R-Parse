//! Diagnostics and severity levels for R-PARSE analysis and parsing.

use crate::span::Span;
use std::fmt;

/// Severity level of an R-PARSE diagnostic message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// Informational notice that does not impact semantics or execution.
    Info,
    /// Potential issue, ambiguous construct, or deprecated pattern.
    Warning,
    /// Fatal or non-recoverable syntax error or semantic violation.
    Error,
}

impl DiagnosticSeverity {
    /// Returns the lowercase string tag for the severity level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A structured diagnostic message emitted by the parser or analysis passes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity level of the diagnostic.
    pub severity: DiagnosticSeverity,
    /// Descriptive human-readable diagnostic message.
    pub message: String,
    /// Physical source location associated with the diagnostic, if available.
    pub span: Option<Span>,
    /// Optional machine-readable diagnostic code (e.g. "R001", "PARSE_ERROR").
    pub code: Option<String>,
}

impl Diagnostic {
    /// Constructs a new [`Diagnostic`] with the given severity, message, and optional span.
    pub fn new(
        severity: DiagnosticSeverity,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            severity,
            message: message.into(),
            span,
            code: None,
        }
    }

    /// Constructs an error diagnostic.
    pub fn error(message: impl Into<String>, span: Option<Span>) -> Self {
        Self::new(DiagnosticSeverity::Error, message, span)
    }

    /// Constructs a warning diagnostic.
    pub fn warning(message: impl Into<String>, span: Option<Span>) -> Self {
        Self::new(DiagnosticSeverity::Warning, message, span)
    }

    /// Constructs an info diagnostic.
    pub fn info(message: impl Into<String>, span: Option<Span>) -> Self {
        Self::new(DiagnosticSeverity::Info, message, span)
    }

    /// Attaches an error code to the diagnostic.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "[{} - {}] {}", self.severity, code, self.message)?;
        } else {
            write!(f, "[{}] {}", self.severity, self.message)?;
        }
        if let Some(span) = &self.span {
            write!(f, " at {span}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_display_and_codes() {
        let span = Span::new(0, 5, 1, 0, 1, 5);
        let diag = Diagnostic::error("Unexpected token", Some(span)).with_code("SYNTAX001");

        assert_eq!(diag.severity, DiagnosticSeverity::Error);
        assert_eq!(diag.code.as_deref(), Some("SYNTAX001"));
        assert_eq!(
            format!("{diag}"),
            "[error - SYNTAX001] Unexpected token at 1:0-1:5"
        );
    }
}
