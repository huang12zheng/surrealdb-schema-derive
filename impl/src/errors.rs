use thiserror::Error;

#[derive(Debug, Error)]
pub enum SurrealDbSchemaDeriveQueryError {
    #[error("Value conversion failed")]
    InvalidValueTypeError(InvalidValueTypeError),
}

#[derive(Debug, Error)]
#[error("Expected {} but received {}", expected_type, received_type)]
pub struct InvalidValueTypeError {
    pub expected_type: String,
    pub received_type: String,
}

impl From<InvalidValueTypeError> for SurrealDbSchemaDeriveQueryError {
    fn from(error: InvalidValueTypeError) -> Self {
        SurrealDbSchemaDeriveQueryError::InvalidValueTypeError(error)
    }
}
