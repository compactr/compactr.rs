//! Schema types and utilities for defining data structures.

mod definition;
mod registry;

pub use definition::{IntegerFormat, NumberFormat, Property, SchemaType, StringFormat};
pub use registry::SchemaRegistry;
