//! Dynamic value type for runtime representation of data.

use chrono::{DateTime, NaiveDate, Utc};
use indexmap::IndexMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use uuid::Uuid;

/// A dynamic value that can represent any type supported by Compactr.
///
/// This enum provides a way to work with values at runtime without
/// compile-time type information.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Boolean value
    Boolean(bool),

    /// 32-bit signed integer
    Integer(i64),

    /// 32-bit floating point number
    Float(f32),

    /// 64-bit floating point number
    Double(f64),

    /// UTF-8 string
    String(String),

    /// UUID (stored in compact binary form)
    Uuid(Uuid),

    /// Date and time with timezone
    DateTime(DateTime<Utc>),

    /// Date without time
    Date(NaiveDate),

    /// IPv4 address
    Ipv4(Ipv4Addr),

    /// IPv6 address
    Ipv6(Ipv6Addr),

    /// Binary data
    Binary(Vec<u8>),

    /// Array of values
    Array(Vec<Value>),

    /// Object with string keys and value values
    /// Uses IndexMap to preserve insertion order
    Object(IndexMap<String, Value>),

    /// Null value
    Null,
}

impl Value {
    /// Returns `true` if the value is `Null`.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the value is a `Boolean`.
    #[must_use]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns `true` if the value is an `Integer`.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns `true` if the value is a `Float`.
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns `true` if the value is a `Double`.
    #[must_use]
    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    /// Returns `true` if the value is a `String`.
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if the value is an `Array`.
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if the value is an `Object`.
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Attempts to get the value as a `bool`.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Attempts to get the value as an `i64`.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        if let Self::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    /// Attempts to get the value as an `f32`.
    #[must_use]
    pub const fn as_f32(&self) -> Option<f32> {
        if let Self::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    /// Attempts to get the value as an `f64`.
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        if let Self::Double(d) = self {
            Some(*d)
        } else {
            None
        }
    }

    /// Attempts to get the value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Attempts to get the value as an array slice.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Value]> {
        if let Self::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    /// Attempts to get the value as an object reference.
    #[must_use]
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        if let Self::Object(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    /// Attempts to get a field from an object by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_object()?.get(key)
    }
}

// Convenient From implementations
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Self::Integer(i64::from(i))
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Self::Float(f)
    }
}

impl From<f64> for Value {
    fn from(d: f64) -> Self {
        Self::Double(d)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<Uuid> for Value {
    fn from(uuid: Uuid) -> Self {
        Self::Uuid(uuid)
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::DateTime(dt)
    }
}

impl From<NaiveDate> for Value {
    fn from(date: NaiveDate) -> Self {
        Self::Date(date)
    }
}

impl From<Ipv4Addr> for Value {
    fn from(ip: Ipv4Addr) -> Self {
        Self::Ipv4(ip)
    }
}

impl From<Ipv6Addr> for Value {
    fn from(ip: Ipv6Addr) -> Self {
        Self::Ipv6(ip)
    }
}

impl From<Vec<u8>> for Value {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Binary(bytes)
    }
}

impl From<Vec<Value>> for Value {
    fn from(arr: Vec<Value>) -> Self {
        Self::Array(arr)
    }
}

impl From<IndexMap<String, Value>> for Value {
    fn from(obj: IndexMap<String, Value>) -> Self {
        Self::Object(obj)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => val.into(),
            None => Self::Null,
        }
    }
}
