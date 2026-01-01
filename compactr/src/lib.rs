//! # Compactr
//!
//! A high-performance schema-based binary serialization library compatible with `OpenAPI` 3.x schemas.
//!
//! Compactr provides efficient binary encoding/decoding of data structures based on schemas,
//! with support for various formats including `UUID`, `DateTime`, IP addresses, and more.
//!
//! ## Features
//!
//! - Schema-based serialization with `OpenAPI` 3.x compatibility
//! - Compact binary format with size optimization
//! - Zero-copy deserialization where possible
//! - Support for complex types (arrays, objects, references)
//! - Built-in formats: `UUID`, `DateTime`, `Date`, `IPv4`, `IPv6`, `Binary`
//! - Thread-safe schema registry
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use compactr::{Schema, Value, SchemaType};
//!
//! // Define a schema
//! let schema = SchemaType::object([
//!     ("id", SchemaType::string_uuid()),
//!     ("name", SchemaType::string()),
//! ]);
//!
//! // Encode data
//! let data = Value::object([
//!     ("id", Value::uuid("550e8400-e29b-41d4-a716-446655440000")),
//!     ("name", Value::string("Alice")),
//! ]);
//!
//! let encoded = schema.encode(&data)?;
//!
//! // Decode data
//! let decoded = schema.decode(&encoded)?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod codec;
pub mod error;
pub mod formats;
pub mod schema;
pub mod value;

// Re-export commonly used types
pub use codec::{Decode, Decoder, Encode, Encoder};
pub use error::{DecodeError, EncodeError, Result, SchemaError};
pub use schema::{IntegerFormat, NumberFormat, Property, SchemaRegistry, SchemaType, StringFormat};
pub use value::Value;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::codec::{Decode, Decoder, Encode, Encoder};
    pub use crate::error::{DecodeError, EncodeError, Result, SchemaError};
    pub use crate::schema::{
        IntegerFormat, NumberFormat, Property, SchemaRegistry, SchemaType, StringFormat,
    };
    pub use crate::value::Value;
}
