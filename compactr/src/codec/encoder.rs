//! Encoder for converting values to binary format based on schemas.

use crate::codec::buffer::{encode_binary, encode_string};
use crate::error::{EncodeError, Result, SchemaError};
use crate::formats::{datetime, ipaddr, uuid};
use crate::schema::{IntegerFormat, NumberFormat, SchemaRegistry, SchemaType, StringFormat};
use crate::value::Value;
use bytes::{BufMut, Bytes, BytesMut};

/// Encoder for serializing values to binary format.
#[derive(Debug)]
pub struct Encoder {
    buf: BytesMut,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// Creates a new encoder with an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: BytesMut::new(),
        }
    }

    /// Creates a new encoder with a pre-allocated buffer.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: BytesMut::with_capacity(capacity),
        }
    }

    /// Encodes a value according to the given schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the value doesn't match the schema or encoding fails.
    pub fn encode(&mut self, value: &Value, schema: &SchemaType) -> Result<()> {
        self.encode_with_registry(value, schema, &SchemaRegistry::new())
    }

    /// Encodes a value with a schema registry for resolving references.
    ///
    /// # Errors
    ///
    /// Returns an error if the value doesn't match the schema or encoding fails.
    pub fn encode_with_registry(
        &mut self,
        value: &Value,
        schema: &SchemaType,
        registry: &SchemaRegistry,
    ) -> Result<()> {
        match schema {
            SchemaType::Boolean => self.encode_boolean(value),
            SchemaType::Integer(format) => self.encode_integer(value, *format),
            SchemaType::Number(format) => self.encode_number(value, *format),
            SchemaType::String(format) => self.encode_string_format(value, *format),
            SchemaType::Array(items) => self.encode_array(value, items, registry),
            SchemaType::Object(properties) => self.encode_object(value, properties, registry),
            SchemaType::Reference(ref_name) => {
                let resolved = registry.resolve_ref(ref_name)?;
                self.encode_with_registry(value, &resolved, registry)
            }
            SchemaType::Null => self.encode_null(value),
        }
    }

    fn encode_boolean(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Boolean(b) => {
                self.buf.put_u8(u8::from(*b));
                Ok(())
            }
            _ => Err(EncodeError::TypeMismatch {
                expected: "boolean".to_owned(),
                actual: value_type_name(value),
            }
            .into()),
        }
    }

    fn encode_integer(&mut self, value: &Value, format: IntegerFormat) -> Result<()> {
        let int_val = match value {
            Value::Integer(i) => *i,
            _ => {
                return Err(EncodeError::TypeMismatch {
                    expected: "integer".to_owned(),
                    actual: value_type_name(value),
                }
                .into())
            }
        };

        match format {
            IntegerFormat::Int32 => {
                if int_val < i64::from(i32::MIN) || int_val > i64::from(i32::MAX) {
                    return Err(EncodeError::InvalidFormat(format!(
                        "Integer {int_val} out of range for int32"
                    ))
                    .into());
                }
                #[allow(clippy::cast_possible_truncation)]
                self.buf.put_i32(int_val as i32); // Big-endian
            }
            IntegerFormat::Int64 => {
                self.buf.put_i64(int_val); // Big-endian
            }
        }

        Ok(())
    }

    fn encode_number(&mut self, value: &Value, format: NumberFormat) -> Result<()> {
        match format {
            NumberFormat::Float => match value {
                Value::Float(f) => {
                    self.buf.put_f32(*f); // Big-endian
                    Ok(())
                }
                Value::Double(d) => {
                    #[allow(clippy::cast_possible_truncation)]
                    self.buf.put_f32(*d as f32); // Big-endian
                    Ok(())
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "float".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            NumberFormat::Double => match value {
                Value::Double(d) => {
                    self.buf.put_f64(*d); // Big-endian
                    Ok(())
                }
                Value::Float(f) => {
                    self.buf.put_f64(f64::from(*f)); // Big-endian
                    Ok(())
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "double".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
        }
    }

    fn encode_string_format(&mut self, value: &Value, format: StringFormat) -> Result<()> {
        match format {
            StringFormat::Plain => match value {
                Value::String(s) => encode_string(&mut self.buf, s).map_err(Into::into),
                _ => Err(EncodeError::TypeMismatch {
                    expected: "string".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::Uuid => match value {
                Value::Uuid(u) => uuid::encode_uuid(&mut self.buf, u).map_err(Into::into),
                Value::String(s) => {
                    let u = uuid::parse_uuid(s)?;
                    uuid::encode_uuid(&mut self.buf, &u).map_err(Into::into)
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "uuid".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::DateTime => match value {
                Value::DateTime(dt) => {
                    datetime::encode_datetime(&mut self.buf, dt).map_err(Into::into)
                }
                Value::String(s) => {
                    let dt = datetime::parse_datetime(s)?;
                    datetime::encode_datetime(&mut self.buf, &dt).map_err(Into::into)
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "datetime".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::Date => match value {
                Value::Date(d) => datetime::encode_date(&mut self.buf, d).map_err(Into::into),
                Value::String(s) => {
                    let d = datetime::parse_date(s)?;
                    datetime::encode_date(&mut self.buf, &d).map_err(Into::into)
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "date".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::Ipv4 => match value {
                Value::Ipv4(ip) => ipaddr::encode_ipv4(&mut self.buf, ip).map_err(Into::into),
                Value::String(s) => {
                    let ip = ipaddr::parse_ipv4(s)?;
                    ipaddr::encode_ipv4(&mut self.buf, &ip).map_err(Into::into)
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "ipv4".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::Ipv6 => match value {
                Value::Ipv6(ip) => ipaddr::encode_ipv6(&mut self.buf, ip).map_err(Into::into),
                Value::String(s) => {
                    let ip = ipaddr::parse_ipv6(s)?;
                    ipaddr::encode_ipv6(&mut self.buf, &ip).map_err(Into::into)
                }
                _ => Err(EncodeError::TypeMismatch {
                    expected: "ipv6".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
            StringFormat::Binary => match value {
                Value::Binary(data) => encode_binary(&mut self.buf, data).map_err(Into::into),
                _ => Err(EncodeError::TypeMismatch {
                    expected: "binary".to_owned(),
                    actual: value_type_name(value),
                }
                .into()),
            },
        }
    }

    fn encode_array(
        &mut self,
        value: &Value,
        items_schema: &SchemaType,
        registry: &SchemaRegistry,
    ) -> Result<()> {
        let Value::Array(items) = value else {
            return Err(EncodeError::TypeMismatch {
                expected: "array".to_owned(),
                actual: value_type_name(value),
            }
            .into());
        };

        // Compactr.js format: Each array element is prefixed with its size
        // No overall array length - elements are self-describing
        //
        // Format: [size1, elem1, size2, elem2, ...]
        // where size is a variable-length encoding

        for item in items {
            // Encode element to temp buffer to measure size
            let mut temp_buf = BytesMut::new();
            let mut temp_encoder = Encoder::with_buf(temp_buf);
            temp_encoder.encode_with_registry(item, items_schema, registry)?;
            temp_buf = temp_encoder.buf;

            let elem_size = temp_buf.len();

            // Encode size prefix (variable length)
            if elem_size > 255 {
                return Err(EncodeError::InvalidFormat(format!(
                    "Array element too large: {elem_size} bytes (max 255)"
                ))
                .into());
            }
            #[allow(clippy::cast_possible_truncation)]
            self.buf.put_u8(elem_size as u8);

            // Append element data
            self.buf.extend_from_slice(&temp_buf);
        }

        Ok(())
    }

    fn encode_object(
        &mut self,
        value: &Value,
        properties: &indexmap::IndexMap<String, crate::schema::Property>,
        registry: &SchemaRegistry,
    ) -> Result<()> {
        let Value::Object(obj) = value else {
            return Err(EncodeError::TypeMismatch {
                expected: "object".to_owned(),
                actual: value_type_name(value),
            }
            .into());
        };

        // Compactr.js format: Header + Content structure
        // Header: [num_props, prop0_idx, prop0_size, prop1_idx, prop1_size, ...]
        // Content: [prop0_value, prop1_value, ...]

        // Build list of present properties with their indices (alphabetical order)
        let mut present_props: Vec<(usize, &String, &crate::schema::Property, &Value)> = Vec::new();

        for (idx, (prop_name, prop_def)) in properties.iter().enumerate() {
            if let Some(prop_value) = obj.get(prop_name) {
                present_props.push((idx, prop_name, prop_def, prop_value));
            } else if prop_def.required {
                return Err(SchemaError::MissingField(prop_name.clone()).into());
            }
            // Optional properties that are missing are simply omitted
        }

        // Encode header
        // First byte: number of properties present
        if present_props.len() > 255 {
            return Err(EncodeError::InvalidFormat(format!(
                "Too many properties: {} (max 255)",
                present_props.len()
            ))
            .into());
        }
        #[allow(clippy::cast_possible_truncation)]
        self.buf.put_u8(present_props.len() as u8);

        // Encode property values to a temporary buffer to calculate sizes
        let mut content_buf = BytesMut::new();
        let mut prop_sizes: Vec<usize> = Vec::new();

        for (_idx, _prop_name, prop_def, prop_value) in &present_props {
            let before_len = content_buf.len();
            let mut temp_encoder = Encoder::with_buf(content_buf);
            temp_encoder.encode_with_registry(prop_value, &prop_def.schema_type, registry)?;
            content_buf = temp_encoder.buf;
            let after_len = content_buf.len();
            prop_sizes.push(after_len - before_len);
        }

        // Encode header: property indices and sizes
        for (i, (idx, _prop_name, _prop_def, _prop_value)) in present_props.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            self.buf.put_u8(*idx as u8); // Property index

            let size = prop_sizes[i];
            if size > 255 {
                return Err(EncodeError::InvalidFormat(format!(
                    "Property value too large: {size} bytes (max 255)"
                ))
                .into());
            }
            #[allow(clippy::cast_possible_truncation)]
            self.buf.put_u8(size as u8); // Property size
        }

        // Append content
        self.buf.extend_from_slice(&content_buf);

        Ok(())
    }

    // Helper to create encoder with existing buffer
    fn with_buf(buf: BytesMut) -> Self {
        Self { buf }
    }

    fn encode_null(&mut self, value: &Value) -> Result<()> {
        if !value.is_null() {
            return Err(EncodeError::TypeMismatch {
                expected: "null".to_owned(),
                actual: value_type_name(value),
            }
            .into());
        }
        // Null is encoded as a single 0 byte
        self.buf.put_u8(0);
        Ok(())
    }

    /// Consumes the encoder and returns the encoded bytes.
    #[must_use]
    pub fn finish(self) -> Bytes {
        self.buf.freeze()
    }

    /// Returns a reference to the current buffer.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
}

fn value_type_name(value: &Value) -> String {
    match value {
        Value::Boolean(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::Double(_) => "double",
        Value::String(_) => "string",
        Value::Uuid(_) => "uuid",
        Value::DateTime(_) => "datetime",
        Value::Date(_) => "date",
        Value::Ipv4(_) => "ipv4",
        Value::Ipv6(_) => "ipv6",
        Value::Binary(_) => "binary",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Null => "null",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_boolean() {
        let mut enc = Encoder::new();
        enc.encode(&Value::Boolean(true), &SchemaType::boolean())
            .unwrap();
        assert_eq!(enc.as_bytes(), &[1]);

        let mut enc = Encoder::new();
        enc.encode(&Value::Boolean(false), &SchemaType::boolean())
            .unwrap();
        assert_eq!(enc.as_bytes(), &[0]);
    }

    #[test]
    fn test_encode_integer() {
        let mut enc = Encoder::new();
        enc.encode(&Value::Integer(42), &SchemaType::int32())
            .unwrap();
        assert_eq!(enc.as_bytes().len(), 4);

        let mut enc = Encoder::new();
        enc.encode(&Value::Integer(42), &SchemaType::int64())
            .unwrap();
        assert_eq!(enc.as_bytes().len(), 8);
    }

    #[test]
    fn test_encode_string() {
        let mut enc = Encoder::new();
        enc.encode(&Value::String("hello".to_owned()), &SchemaType::string())
            .unwrap();
        // UTF-8: 2 byte length + 5 bytes = 7 bytes
        assert_eq!(enc.as_bytes().len(), 7);
    }

    #[test]
    fn test_encode_array() {
        let mut enc = Encoder::new();
        let arr = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        enc.encode(&arr, &SchemaType::array(SchemaType::int32()))
            .unwrap();
        // Size-prefixed format: 3 * (1 byte size + 4 bytes int32) = 15 bytes
        assert_eq!(enc.as_bytes().len(), 15);
    }
}
