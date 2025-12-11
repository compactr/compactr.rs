//! Schema type definitions.

use indexmap::IndexMap;
use std::fmt;

/// Represents the type of a schema field.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    /// Boolean type
    Boolean,

    /// Integer type with format specification
    Integer(IntegerFormat),

    /// Floating-point number with format specification
    Number(NumberFormat),

    /// String with optional format
    String(StringFormat),

    /// Array of items with a specific schema
    Array(Box<SchemaType>),

    /// Object with named properties
    Object(IndexMap<String, Property>),

    /// Reference to another schema (e.g., "#/ComponentName")
    Reference(String),

    /// Null type
    Null,
}

/// Integer format specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerFormat {
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
}

/// Number (floating-point) format specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberFormat {
    /// 32-bit IEEE 754 floating point
    Float,
    /// 64-bit IEEE 754 floating point
    Double,
}

/// String format specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringFormat {
    /// Plain UTF-8 string
    Plain,
    /// UUID in standard format (stored as 16 bytes)
    Uuid,
    /// ISO 8601 datetime (stored as Unix timestamp in ms)
    DateTime,
    /// ISO 8601 date (stored as days since Unix epoch)
    Date,
    /// IPv4 address (stored as 4 bytes)
    Ipv4,
    /// IPv6 address (stored as 16 bytes)
    Ipv6,
    /// Binary data (Base64 encoded in JSON, raw bytes in binary)
    Binary,
}

/// Represents a property in an object schema.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    /// The schema type of this property
    pub schema_type: SchemaType,
    /// Whether this property is required
    pub required: bool,
}

impl Property {
    /// Creates a new required property.
    #[must_use]
    pub const fn required(schema_type: SchemaType) -> Self {
        Self {
            schema_type,
            required: true,
        }
    }

    /// Creates a new optional property.
    #[must_use]
    pub const fn optional(schema_type: SchemaType) -> Self {
        Self {
            schema_type,
            required: false,
        }
    }
}

impl SchemaType {
    /// Creates a boolean schema.
    #[must_use]
    pub const fn boolean() -> Self {
        Self::Boolean
    }

    /// Creates an int32 schema.
    #[must_use]
    pub const fn int32() -> Self {
        Self::Integer(IntegerFormat::Int32)
    }

    /// Creates an int64 schema.
    #[must_use]
    pub const fn int64() -> Self {
        Self::Integer(IntegerFormat::Int64)
    }

    /// Creates a float schema.
    #[must_use]
    pub const fn float() -> Self {
        Self::Number(NumberFormat::Float)
    }

    /// Creates a double schema.
    #[must_use]
    pub const fn double() -> Self {
        Self::Number(NumberFormat::Double)
    }

    /// Creates a plain string schema.
    #[must_use]
    pub const fn string() -> Self {
        Self::String(StringFormat::Plain)
    }

    /// Creates a UUID string schema.
    #[must_use]
    pub const fn string_uuid() -> Self {
        Self::String(StringFormat::Uuid)
    }

    /// Creates a datetime string schema.
    #[must_use]
    pub const fn string_datetime() -> Self {
        Self::String(StringFormat::DateTime)
    }

    /// Creates a date string schema.
    #[must_use]
    pub const fn string_date() -> Self {
        Self::String(StringFormat::Date)
    }

    /// Creates an IPv4 string schema.
    #[must_use]
    pub const fn string_ipv4() -> Self {
        Self::String(StringFormat::Ipv4)
    }

    /// Creates an IPv6 string schema.
    #[must_use]
    pub const fn string_ipv6() -> Self {
        Self::String(StringFormat::Ipv6)
    }

    /// Creates a binary string schema.
    #[must_use]
    pub const fn binary() -> Self {
        Self::String(StringFormat::Binary)
    }

    /// Creates an array schema with the given item type.
    #[must_use]
    pub fn array(items: SchemaType) -> Self {
        Self::Array(Box::new(items))
    }

    /// Creates an object schema with the given properties.
    #[must_use]
    pub fn object(properties: IndexMap<String, Property>) -> Self {
        Self::Object(properties)
    }

    /// Creates a reference to another schema.
    #[must_use]
    pub fn reference(name: impl Into<String>) -> Self {
        Self::Reference(name.into())
    }

    /// Creates a null schema.
    #[must_use]
    pub const fn null() -> Self {
        Self::Null
    }
}

impl fmt::Display for SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Integer(format) => write!(f, "integer({format:?})"),
            Self::Number(format) => write!(f, "number({format:?})"),
            Self::String(format) => write!(f, "string({format:?})"),
            Self::Array(items) => write!(f, "array[{items}]"),
            Self::Object(_) => write!(f, "object"),
            Self::Reference(r) => write!(f, "ref({r})"),
            Self::Null => write!(f, "null"),
        }
    }
}
