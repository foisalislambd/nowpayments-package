//! Error types for NOWPayments API

use std::fmt;

/// Error returned by NOWPayments API or client
#[derive(Debug, Clone)]
pub struct NowPaymentsError {
    /// Human-readable error message
    pub message: String,
    /// HTTP status code if from API response
    pub status_code: Option<u16>,
    /// Error code from API
    pub code: Option<String>,
    /// Raw API error body when available
    pub response: Option<serde_json::Value>,
}

impl fmt::Display for NowPaymentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(sc) = self.status_code {
            write!(f, " (status: {})", sc)?;
        }
        if let Some(ref c) = self.code {
            write!(f, " [{}]", c)?;
        }
        Ok(())
    }
}

impl std::error::Error for NowPaymentsError {}

impl NowPaymentsError {
    /// Create error with optional status code, API code, and raw response
    pub fn new(
        message: impl Into<String>,
        status_code: Option<u16>,
        code: Option<String>,
        response: Option<serde_json::Value>,
    ) -> Self {
        Self {
            message: message.into(),
            status_code,
            code,
            response,
        }
    }

    pub fn with_status(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}
