//! Decoder for converting binary format to values based on schemas.

use crate::codec::buffer::{decode_binary, decode_string};
use crate::error::{DecodeError, Result, SchemaError};
use crate::formats::{datetime, ipaddr, uuid};
use crate::schema::{IntegerFormat, NumberFormat, SchemaRegistry, SchemaType, StringFormat};
use crate::value::Value;
use bytes::Buf;
use indexmap::IndexMap;

/// Decoder for deserializing values from binary format.
#[derive(Debug)]
pub struct Decoder;

impl Decoder {
    /// Creates a new decoder.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Decodes a value from a buffer according to the given schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer doesn't contain valid data for the schema.
    pub fn decode(buf: &mut impl Buf, schema: &SchemaType) -> Result<Value> {
        Self::decode_with_registry(buf, schema, &SchemaRegistry::new())
    }

    /// Decodes a value with a schema registry for resolving references.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer doesn't contain valid data for the schema.
    pub fn decode_with_registry(
        buf: &mut impl Buf,
        schema: &SchemaType,
        registry: &SchemaRegistry,
    ) -> Result<Value> {
        match schema {
            SchemaType::Boolean => Self::decode_boolean(buf),
            SchemaType::Integer(format) => Self::decode_integer(buf, *format),
            SchemaType::Number(format) => Self::decode_number(buf, *format),
            SchemaType::String(format) => Self::decode_string_format(buf, *format),
            SchemaType::Array(items) => Self::decode_array(buf, items, registry),
            SchemaType::Object(properties) => Self::decode_object(buf, properties, registry),
            SchemaType::Reference(ref_name) => {
                let resolved = registry.resolve_ref(ref_name)?;
                Self::decode_with_registry(buf, &resolved, registry)
            }
            SchemaType::Null => Self::decode_null(buf),
        }
    }

    fn decode_boolean(buf: &mut impl Buf) -> Result<Value> {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof.into());
        }

        let byte = buf.get_u8();
        match byte {
            0 => Ok(Value::Boolean(false)),
            1 => Ok(Value::Boolean(true)),
            _ => Err(DecodeError::InvalidData(format!("Invalid boolean value: {byte}")).into()),
        }
    }

    fn decode_integer(buf: &mut impl Buf, format: IntegerFormat) -> Result<Value> {
        let value = match format {
            IntegerFormat::Int32 => {
                if buf.remaining() < 4 {
                    return Err(DecodeError::UnexpectedEof.into());
                }
                i64::from(buf.get_i32()) // Big-endian
            }
            IntegerFormat::Int64 => {
                if buf.remaining() < 8 {
                    return Err(DecodeError::UnexpectedEof.into());
                }
                buf.get_i64() // Big-endian
            }
        };

        Ok(Value::Integer(value))
    }

    fn decode_number(buf: &mut impl Buf, format: NumberFormat) -> Result<Value> {
        match format {
            NumberFormat::Float => {
                if buf.remaining() < 4 {
                    return Err(DecodeError::UnexpectedEof.into());
                }
                Ok(Value::Float(buf.get_f32())) // Big-endian
            }
            NumberFormat::Double => {
                if buf.remaining() < 8 {
                    return Err(DecodeError::UnexpectedEof.into());
                }
                Ok(Value::Double(buf.get_f64())) // Big-endian
            }
        }
    }

    fn decode_string_format(buf: &mut impl Buf, format: StringFormat) -> Result<Value> {
        match format {
            StringFormat::Plain => {
                let s = decode_string(buf)?;
                Ok(Value::String(s))
            }
            StringFormat::Uuid => {
                let u = uuid::decode_uuid(buf)?;
                Ok(Value::Uuid(u))
            }
            StringFormat::DateTime => {
                let dt = datetime::decode_datetime(buf)?;
                Ok(Value::DateTime(dt))
            }
            StringFormat::Date => {
                let d = datetime::decode_date(buf)?;
                Ok(Value::Date(d))
            }
            StringFormat::Ipv4 => {
                let ip = ipaddr::decode_ipv4(buf)?;
                Ok(Value::Ipv4(ip))
            }
            StringFormat::Ipv6 => {
                let ip = ipaddr::decode_ipv6(buf)?;
                Ok(Value::Ipv6(ip))
            }
            StringFormat::Binary => {
                let data = decode_binary(buf)?;
                Ok(Value::Binary(data))
            }
        }
    }

    fn decode_array(
        buf: &mut impl Buf,
        items_schema: &SchemaType,
        registry: &SchemaRegistry,
    ) -> Result<Value> {
        // Compactr.js format: Each array element is prefixed with its size
        // No overall array length - read elements until buffer is exhausted
        //
        // Format: [size1, elem1, size2, elem2, ...]
        // where size is a 1-byte value

        let mut items = Vec::new();

        while buf.has_remaining() {
            // Read element size
            let elem_size = buf.get_u8() as usize;

            // Read element data
            if buf.remaining() < elem_size {
                return Err(DecodeError::UnexpectedEof.into());
            }

            let mut elem_bytes = vec![0u8; elem_size];
            buf.copy_to_slice(&mut elem_bytes);
            let mut elem_buf = &elem_bytes[..];

            let item = Self::decode_with_registry(&mut elem_buf, items_schema, registry)?;
            items.push(item);
        }

        Ok(Value::Array(items))
    }

    fn decode_object(
        buf: &mut impl Buf,
        properties: &IndexMap<String, crate::schema::Property>,
        registry: &SchemaRegistry,
    ) -> Result<Value> {
        // Compactr.js format: Header + Content structure
        // Header: [num_props, prop0_idx, prop0_size, prop1_idx, prop1_size, ...]
        // Content: [prop0_value, prop1_value, ...]

        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof.into());
        }

        // Read number of properties present
        let num_props = buf.get_u8() as usize;

        // Read header: property indices and sizes
        let mut prop_info: Vec<(usize, usize)> = Vec::with_capacity(num_props);
        for _ in 0..num_props {
            if buf.remaining() < 2 {
                return Err(DecodeError::UnexpectedEof.into());
            }
            let prop_idx = buf.get_u8() as usize;
            let prop_size = buf.get_u8() as usize;
            prop_info.push((prop_idx, prop_size));
        }

        // Convert properties IndexMap to Vec for index-based access
        let props_vec: Vec<(String, crate::schema::Property)> = properties
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Decode content: property values
        let mut obj = IndexMap::new();
        for (prop_idx, prop_size) in prop_info {
            if prop_idx >= props_vec.len() {
                return Err(DecodeError::InvalidData(format!(
                    "Property index {} out of range (max {})",
                    prop_idx,
                    props_vec.len() - 1
                ))
                .into());
            }

            let (prop_name, prop_def) = &props_vec[prop_idx];

            // Read exactly prop_size bytes for this property
            if buf.remaining() < prop_size {
                return Err(DecodeError::UnexpectedEof.into());
            }

            let mut prop_bytes = vec![0u8; prop_size];
            buf.copy_to_slice(&mut prop_bytes);
            let mut prop_buf = &prop_bytes[..];

            let prop_value =
                Self::decode_with_registry(&mut prop_buf, &prop_def.schema_type, registry)?;

            obj.insert(prop_name.clone(), prop_value);
        }

        // Check for missing required fields
        for (prop_name, prop_def) in properties {
            if prop_def.required && !obj.contains_key(prop_name) {
                return Err(SchemaError::MissingField(prop_name.clone()).into());
            }
        }

        Ok(Value::Object(obj))
    }

    fn decode_null(buf: &mut impl Buf) -> Result<Value> {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof.into());
        }

        let byte = buf.get_u8();
        if byte != 0 {
            return Err(DecodeError::InvalidData(format!(
                "Invalid null value: {byte}, expected 0"
            ))
            .into());
        }

        Ok(Value::Null)
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::Encoder;

    #[test]
    fn test_decode_boolean() {
        let mut enc = Encoder::new();
        enc.encode(&Value::Boolean(true), &SchemaType::boolean())
            .unwrap();
        let bytes = enc.finish();

        let mut buf = bytes.as_ref();
        let decoded = Decoder::decode(&mut buf, &SchemaType::boolean()).unwrap();
        assert_eq!(decoded, Value::Boolean(true));
    }

    #[test]
    fn test_decode_integer() {
        let mut enc = Encoder::new();
        enc.encode(&Value::Integer(42), &SchemaType::int32())
            .unwrap();
        let bytes = enc.finish();

        let mut buf = bytes.as_ref();
        let decoded = Decoder::decode(&mut buf, &SchemaType::int32()).unwrap();
        assert_eq!(decoded, Value::Integer(42));
    }

    #[test]
    fn test_decode_string() {
        let mut enc = Encoder::new();
        enc.encode(&Value::String("hello".to_owned()), &SchemaType::string())
            .unwrap();
        let bytes = enc.finish();

        let mut buf = bytes.as_ref();
        let decoded = Decoder::decode(&mut buf, &SchemaType::string()).unwrap();
        assert_eq!(decoded, Value::String("hello".to_owned()));
    }

    #[test]
    fn test_roundtrip_array() {
        let mut enc = Encoder::new();
        let arr = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        enc.encode(&arr, &SchemaType::array(SchemaType::int32()))
            .unwrap();
        let bytes = enc.finish();

        let mut buf = bytes.as_ref();
        let decoded = Decoder::decode(&mut buf, &SchemaType::array(SchemaType::int32())).unwrap();
        assert_eq!(decoded, arr);
    }

    #[test]
    fn test_roundtrip_object() {
        use crate::schema::Property;

        let mut properties = IndexMap::new();
        properties.insert("name".to_owned(), Property::required(SchemaType::string()));
        properties.insert("age".to_owned(), Property::required(SchemaType::int32()));

        let schema = SchemaType::object(properties);

        let mut obj = IndexMap::new();
        obj.insert("name".to_owned(), Value::String("Alice".to_owned()));
        obj.insert("age".to_owned(), Value::Integer(30));
        let value = Value::Object(obj);

        let mut enc = Encoder::new();
        enc.encode(&value, &schema).unwrap();
        let bytes = enc.finish();

        let mut buf = bytes.as_ref();
        let decoded = Decoder::decode(&mut buf, &schema).unwrap();
        assert_eq!(decoded, value);
    }
}
