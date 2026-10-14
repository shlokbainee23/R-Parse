//! High-level GNU R Runtime Bridge orchestrator and lifecycle manager.

use crate::config::{AnalysisMode, RuntimeConfig};
use crate::discovery::discover_r_executable;
use crate::error::RuntimeError;
use crate::evidence::{ObservationRecord, RuntimeEvidence};
use crate::process::{RProcess, RuntimeBackend};
use crate::protocol::{
    DataFrameSchemaObservation, EvalParams, EvalResult, InspectDataFrameParams,
    InspectObjectParams, ObjectMetadata,
};

/// The GNU R runtime bridge managing process lifecycle and empirical observation queries.
///
/// **Architectural Invariant:** Static analysis remains authoritative. In `AnalysisMode::Static`,
/// no GNU R subprocess is ever launched.
pub struct RuntimeBridge {
    config: RuntimeConfig,
    backend: Option<Box<dyn RuntimeBackend>>,
}

impl RuntimeBridge {
    /// Creates a new runtime bridge with the given configuration.
    ///
    /// In accordance with decoupled lifecycle rules, no child process is launched at construction.
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            backend: None,
        }
    }

    /// Creates a runtime bridge configured with an explicit runtime backend (e.g. for testing).
    pub fn with_backend(config: RuntimeConfig, backend: Box<dyn RuntimeBackend>) -> Self {
        Self {
            config,
            backend: Some(backend),
        }
    }

    /// Returns the active analysis mode.
    pub fn mode(&self) -> AnalysisMode {
        self.config.mode
    }

    /// Returns a reference to the active configuration.
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Returns true if an active backend process is currently connected.
    pub fn is_connected(&self) -> bool {
        self.backend.as_ref().is_some_and(|b| b.is_alive())
    }

    /// Ensures the backend is active, lazily spawning the GNU R worker if needed.
    ///
    /// # Errors
    /// Returns `RuntimeError::ExecutionForbiddenInStaticMode` if invoked in `AnalysisMode::Static`.
    fn ensure_backend(&mut self) -> Result<&mut dyn RuntimeBackend, RuntimeError> {
        if self.config.mode.is_static() {
            return Err(RuntimeError::ExecutionForbiddenInStaticMode);
        }

        if self.backend.is_none() {
            let r_path = discover_r_executable(self.config.r_executable.as_deref())?;
            let process = RProcess::spawn(&r_path, &self.config)?;
            self.backend = Some(Box::new(process));
        }

        let backend = self.backend.as_mut().unwrap().as_mut();
        if !backend.is_alive() {
            return Err(RuntimeError::ProcessTerminated {
                code: None,
                stderr: "Runtime backend process has terminated".to_string(),
            });
        }

        Ok(backend)
    }

    /// Evaluates arbitrary R code in the controlled evaluation environment.
    ///
    /// # Errors
    /// Fails immediately if in `AnalysisMode::Static`, or if execution fails/times out.
    pub fn evaluate(&mut self, expression: &str) -> Result<EvalResult, RuntimeError> {
        let timeout_ms = self.config.timeout_ms;
        let backend = self.ensure_backend()?;
        let params = EvalParams {
            expression: expression.to_string(),
            timeout_ms: Some(timeout_ms),
        }
        .to_json();

        let response = backend.send_request("evaluate", params, timeout_ms)?;

        if let Some(err) = response.error {
            return Err(RuntimeError::RRuntimeError {
                message: err.message,
                call: None,
            });
        }

        let result_val = response
            .result
            .ok_or_else(|| RuntimeError::ProtocolError("Missing result payload".to_string()))?;

        let eval_res = EvalResult::from_json(&result_val)?;
        Ok(eval_res)
    }

    /// Inspects structural metadata for an R symbol in the evaluation environment.
    pub fn inspect_object(
        &mut self,
        target_symbol: &str,
        expression: Option<&str>,
    ) -> Result<ObjectMetadata, RuntimeError> {
        let timeout_ms = self.config.timeout_ms;
        let backend = self.ensure_backend()?;
        let params = InspectObjectParams {
            target_symbol: target_symbol.to_string(),
            expression: expression.map(|s| s.to_string()),
            timeout_ms: Some(timeout_ms),
        }
        .to_json();

        let response = backend.send_request("inspect_object", params, timeout_ms)?;

        if let Some(err) = response.error {
            return Err(RuntimeError::RRuntimeError {
                message: err.message,
                call: None,
            });
        }

        let result_val = response
            .result
            .ok_or_else(|| RuntimeError::ProtocolError("Missing result payload".to_string()))?;

        let metadata = ObjectMetadata::from_json(&result_val)?;
        Ok(metadata)
    }

    /// Inspects and discovers dataframe schema details for a target symbol.
    pub fn inspect_dataframe(
        &mut self,
        target_symbol: &str,
        expression: Option<&str>,
    ) -> Result<DataFrameSchemaObservation, RuntimeError> {
        let timeout_ms = self.config.timeout_ms;
        let backend = self.ensure_backend()?;
        let params = InspectDataFrameParams {
            target_symbol: target_symbol.to_string(),
            expression: expression.map(|s| s.to_string()),
            timeout_ms: Some(timeout_ms),
        }
        .to_json();

        let response = backend.send_request("inspect_dataframe", params, timeout_ms)?;

        if let Some(err) = response.error {
            return Err(RuntimeError::RRuntimeError {
                message: err.message,
                call: None,
            });
        }

        let result_val = response
            .result
            .ok_or_else(|| RuntimeError::ProtocolError("Missing result payload".to_string()))?;

        let schema = DataFrameSchemaObservation::from_json(&result_val)?;
        Ok(schema)
    }

    /// Performs dataframe schema observation, returning structured [`RuntimeEvidence`].
    ///
    /// - In `AnalysisMode::Static`: Strictly returns [`RuntimeEvidence::None`] without launching GNU R.
    /// - In `AnalysisMode::Assisted` or `AnalysisMode::Execute`: Performs inspection and returns [`RuntimeEvidence::Observed`].
    pub fn observe_dataframe(
        &mut self,
        target_symbol: &str,
        expression: Option<&str>,
    ) -> RuntimeEvidence {
        if self.config.mode.is_static() {
            return RuntimeEvidence::None;
        }

        match self.inspect_dataframe(target_symbol, expression) {
            Ok(schema) => RuntimeEvidence::Observed(schema.into_record(expression.map(Into::into))),
            Err(err) => RuntimeEvidence::Observed(ObservationRecord::error(
                target_symbol,
                expression.map(Into::into),
                err.to_string(),
                0,
            )),
        }
    }

    /// Explicitly shuts down the backend process and cleans up resources.
    pub fn shutdown(&mut self) {
        if let Some(mut backend) = self.backend.take() {
            backend.shutdown();
        }
    }
}

impl Drop for RuntimeBridge {
    fn drop(&mut self) {
        self.shutdown();
    }
}
