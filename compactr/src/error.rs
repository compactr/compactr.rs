//! Error types for the Compactr library.

use thiserror::Error;

/// A specialized `Result` type for Compactr operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Combined error type for all Compactr operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Schema-related error
    #[error(transparent)]
    Schema(#[from] SchemaError),

    /// Encoding error
    #[error(transparent)]
    Encode(#[from] EncodeError),

    /// Decoding error
    #[error(transparent)]
    Decode(#[from] DecodeError),
}

/// Errors that can occur during schema operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// A schema reference could not be resolved
    #[error("Unresolved reference: {0}")]
    UnresolvedReference(String),

    /// A schema reference is invalid
    #[error("Invalid reference: {0}")]
    InvalidReference(String),

    /// The schema definition is invalid
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    /// A circular reference was detected in the schema
    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    /// A required field is missing
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Errors that can occur during encoding.
#[derive(Debug, Error)]
pub enum EncodeError {
    /// Value type does not match schema type
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Actual type
        actual: String,
    },

    /// Required field is missing from object
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid format for a formatted string
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Buffer overflow during encoding
    #[error("Buffer overflow")]
    BufferOverflow,

    /// I/O error during encoding
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Schema error during encoding
    #[error(transparent)]
    Schema(#[from] SchemaError),
}

/// Errors that can occur during decoding.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// Unexpected end of input
    #[error("Unexpected end of input")]
    UnexpectedEof,

    /// Invalid data encountered
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Data does not match schema
    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),

    /// Buffer underflow during decoding
    #[error("Buffer underflow")]
    BufferUnderflow,

    /// I/O error during decoding
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Schema error during decoding
    #[error(transparent)]
    Schema(#[from] SchemaError),

    /// UTF-8 decoding error
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// String decoding error
    #[error("String error: {0}")]
    String(#[from] std::string::FromUtf8Error),
}
