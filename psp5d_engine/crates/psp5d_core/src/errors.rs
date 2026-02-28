use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("json number is not finite")]
    NonFiniteNumber,
    #[error("failed to canonicalize number during float policy application")]
    NumberCanonicalizationFailed,
    #[error("schema '{schema_name}' validation failed: {message}")]
    SchemaValidation {
        schema_name: String,
        message: String,
    },
    #[error("unsupported schema '{0}'")]
    UnsupportedSchema(String),
    #[error("invariant violation: {0}")]
    InvariantViolation(String),
    #[error("operator not registered for role: {0}")]
    OperatorNotRegistered(String),
    #[error("determinism violation: {0}")]
    DeterminismViolation(String),
    #[error("replay divergence: {0}")]
    ReplayDivergence(String),
    #[error("audit failure: {0}")]
    AuditFailure(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
