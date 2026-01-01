//! Binary format compatibility tests.
//!
//! These tests verify that the binary format produced by the Rust implementation
//! matches the specification and can be used to validate compatibility with compactr.js.

use chrono::TimeZone;
use compactr::{Decoder, Encoder, Property, SchemaRegistry, SchemaType, Value};
use indexmap::IndexMap;

/// Test that primitive types encode to expected binary formats
#[test]
fn test_boolean_binary_format() {
    let schema = SchemaType::boolean();

    // Test true
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Boolean(true), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 1);

    // Test false
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Boolean(false), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 0);
}

#[test]
fn test_int32_binary_format() {
    let schema = SchemaType::int32();

    // Test 0
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 0]);

    // Test positive number (42 in big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(42), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 42]);

    // Test negative number (-1 in big-endian i32)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(-1), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[255, 255, 255, 255]);

    // Test max i32 (big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(i32::MAX.into()), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[127, 255, 255, 255]);
}

#[test]
fn test_int64_binary_format() {
    let schema = SchemaType::int64();

    // Test 0
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[0, 0, 0, 0, 0, 0, 0, 0]);

    // Test positive number (big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(42), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[0, 0, 0, 0, 0, 0, 0, 42]);

    // Test max i64 (big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(i64::MAX), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[127, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_float_binary_format() {
    let schema = SchemaType::float();

    // Test 0.0
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Float(0.0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 0]);

    // Test 1.0 (0x3F800000 in IEEE 754 big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Float(1.0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[63, 128, 0, 0]);
}

#[test]
fn test_double_binary_format() {
    let schema = SchemaType::double();

    // Test 0.0
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Double(0.0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[0, 0, 0, 0, 0, 0, 0, 0]);

    // Test 1.0 (0x3FF0000000000000 in IEEE 754 big-endian)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Double(1.0), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[63, 240, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_string_binary_format() {
    let schema = SchemaType::string();

    // Test empty string (2 bytes = 0x00 0x00)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::String("".to_owned()), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 2);
    assert_eq!(&bytes[..], &[0, 0]);

    // Test "A" (2 byte length + 1 byte UTF-8)
    let mut encoder = Encoder::new();
    encoder
        .encode(&Value::String("A".to_owned()), &schema)
        .unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 3);
    assert_eq!(&bytes[..], &[0, 1, 65]); // length=1 (u16 BE), UTF-8 'A'=0x41

    // Test "Hello" (2 byte length + 5 bytes UTF-8)
    let mut encoder = Encoder::new();
    encoder
        .encode(&Value::String("Hello".to_owned()), &schema)
        .unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 7);
    assert_eq!(&bytes[..], &[0, 5, 72, 101, 108, 108, 111]); // length=5 (u16 BE), UTF-8 "Hello"
}

#[test]
fn test_uuid_binary_format() {
    use uuid::Uuid;

    let schema = SchemaType::string_uuid();
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Uuid(uuid), &schema).unwrap();
    let bytes = encoder.finish();

    // UUID should be 16 bytes
    assert_eq!(bytes.len(), 16);
    // Verify it's the raw UUID bytes
    assert_eq!(&bytes[..], uuid.as_bytes());
}

#[test]
fn test_datetime_binary_format() {
    use chrono::{TimeZone, Utc};

    let schema = SchemaType::string_datetime();

    // Test Unix epoch (timestamp = 0)
    let dt = Utc.timestamp_opt(0, 0).unwrap();
    let mut encoder = Encoder::new();
    encoder.encode(&Value::DateTime(dt), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[0, 0, 0, 0, 0, 0, 0, 0]);

    // Test 1000 milliseconds (1 second) after epoch
    let dt = Utc.timestamp_millis_opt(1000).unwrap();
    let mut encoder = Encoder::new();
    encoder.encode(&Value::DateTime(dt), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[..], &[0, 0, 0, 0, 0, 0, 3, 232]); // 1000 in big-endian i64
}

#[test]
fn test_date_binary_format() {
    use chrono::NaiveDate;

    let schema = SchemaType::string_date();

    // Test Unix epoch date (1970-01-01, days = 0)
    let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Date(date), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 0]);

    // Test 1 day after epoch (1970-01-02)
    let date = NaiveDate::from_ymd_opt(1970, 1, 2).unwrap();
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Date(date), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 1]); // 1 in big-endian i32
}

#[test]
fn test_ipv4_binary_format() {
    use std::net::Ipv4Addr;

    let schema = SchemaType::string_ipv4();
    let ip = Ipv4Addr::new(192, 168, 1, 1);

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Ipv4(ip), &schema).unwrap();
    let bytes = encoder.finish();

    // IPv4 should be 4 bytes
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[192, 168, 1, 1]);
}

#[test]
fn test_ipv6_binary_format() {
    use std::net::Ipv6Addr;

    let schema = SchemaType::string_ipv6();
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Ipv6(ip), &schema).unwrap();
    let bytes = encoder.finish();

    // IPv6 should be 16 bytes
    assert_eq!(bytes.len(), 16);
    assert_eq!(
        &bytes[..],
        &[32, 1, 13, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
}

#[test]
fn test_binary_binary_format() {
    let schema = SchemaType::binary();

    // Test empty binary (4 bytes length = 0)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Binary(vec![]), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 4);
    assert_eq!(&bytes[..], &[0, 0, 0, 0]);

    // Test small binary data (4 bytes length + 3 bytes content)
    let data = vec![1, 2, 3];
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Binary(data.clone()), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 7);
    assert_eq!(&bytes[..], &[0, 0, 0, 3, 1, 2, 3]); // length=3 (big-endian u32), data
}

#[test]
fn test_array_binary_format() {
    let schema = SchemaType::array(SchemaType::int32());

    // Test empty array (no bytes - new size-prefixed format)
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Array(vec![]), &schema).unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 0);
    assert_eq!(&bytes[..], &[]);

    // Test array with 2 elements: [1, 2]
    // New format: [size1, elem1, size2, elem2]
    let mut encoder = Encoder::new();
    encoder
        .encode(
            &Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            &schema,
        )
        .unwrap();
    let bytes = encoder.finish();
    assert_eq!(bytes.len(), 10); // 1 + 4 + 1 + 4
    assert_eq!(
        &bytes[..],
        &[
            4, 0, 0, 0, 1, // size=4, value=1 (big-endian)
            4, 0, 0, 0, 2  // size=4, value=2 (big-endian)
        ]
    );
}

#[test]
fn test_object_binary_format() {
    let mut properties = IndexMap::new();
    properties.insert("x".to_owned(), Property::required(SchemaType::int32()));
    properties.insert("y".to_owned(), Property::required(SchemaType::int32()));
    let schema = SchemaType::object(properties);

    let mut obj = IndexMap::new();
    obj.insert("x".to_owned(), Value::Integer(10));
    obj.insert("y".to_owned(), Value::Integer(20));

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Object(obj), &schema).unwrap();
    let bytes = encoder.finish();

    // New header/content format:
    // Header: [num_props, prop0_idx, prop0_size, prop1_idx, prop1_size]
    // Content: [prop0_value, prop1_value]
    assert_eq!(bytes.len(), 13); // 5 bytes header + 8 bytes content
    assert_eq!(
        &bytes[..],
        &[
            2,          // 2 properties
            0, 4,       // property 0 (x), size 4
            1, 4,       // property 1 (y), size 4
            0, 0, 0, 10, // x = 10 (big-endian)
            0, 0, 0, 20  // y = 20 (big-endian)
        ]
    );
}

/// Test deterministic encoding - same input should produce same output
#[test]
fn test_deterministic_encoding() {
    use chrono::Utc;
    use uuid::Uuid;

    let mut properties = IndexMap::new();
    properties.insert(
        "id".to_owned(),
        Property::required(SchemaType::string_uuid()),
    );
    properties.insert("name".to_owned(), Property::required(SchemaType::string()));
    properties.insert(
        "count".to_owned(),
        Property::required(SchemaType::int32()),
    );
    properties.insert(
        "created".to_owned(),
        Property::required(SchemaType::string_datetime()),
    );
    let schema = SchemaType::object(properties);

    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let dt = Utc.timestamp_millis_opt(1_609_459_200_000).unwrap(); // 2021-01-01 00:00:00 UTC

    let mut obj = IndexMap::new();
    obj.insert("id".to_owned(), Value::Uuid(uuid));
    obj.insert("name".to_owned(), Value::String("Test".to_owned()));
    obj.insert("count".to_owned(), Value::Integer(42));
    obj.insert("created".to_owned(), Value::DateTime(dt));

    // Encode multiple times
    let mut encoder1 = Encoder::new();
    encoder1.encode(&Value::Object(obj.clone()), &schema).unwrap();
    let bytes1 = encoder1.finish();

    let mut encoder2 = Encoder::new();
    encoder2.encode(&Value::Object(obj.clone()), &schema).unwrap();
    let bytes2 = encoder2.finish();

    let mut encoder3 = Encoder::new();
    encoder3.encode(&Value::Object(obj), &schema).unwrap();
    let bytes3 = encoder3.finish();

    // All encodings should be identical
    assert_eq!(bytes1, bytes2);
    assert_eq!(bytes2, bytes3);
}

/// Test that property order in schema matters (not in value)
#[test]
fn test_property_order_matters() {
    // Schema 1: x first, then y
    let mut properties1 = IndexMap::new();
    properties1.insert("x".to_owned(), Property::required(SchemaType::int32()));
    properties1.insert("y".to_owned(), Property::required(SchemaType::int32()));
    let schema1 = SchemaType::object(properties1);

    // Schema 2: y first, then x
    let mut properties2 = IndexMap::new();
    properties2.insert("y".to_owned(), Property::required(SchemaType::int32()));
    properties2.insert("x".to_owned(), Property::required(SchemaType::int32()));
    let schema2 = SchemaType::object(properties2);

    // Same value for both
    let mut obj = IndexMap::new();
    obj.insert("x".to_owned(), Value::Integer(10));
    obj.insert("y".to_owned(), Value::Integer(20));

    // Encode with schema1 (x, y order)
    let mut encoder1 = Encoder::new();
    encoder1
        .encode(&Value::Object(obj.clone()), &schema1)
        .unwrap();
    let bytes1 = encoder1.finish();

    // Encode with schema2 (y, x order)
    let mut encoder2 = Encoder::new();
    encoder2.encode(&Value::Object(obj), &schema2).unwrap();
    let bytes2 = encoder2.finish();

    // Binary output should be different (different property indices)
    assert_ne!(bytes1, bytes2);

    // bytes1 schema order: x (idx 0), y (idx 1)
    // Header: [2, 0, 4, 1, 4], Content: [x=10, y=20]
    assert_eq!(&bytes1[..], &[2, 0, 4, 1, 4, 0, 0, 0, 10, 0, 0, 0, 20]);

    // bytes2 schema order: y (idx 0), x (idx 1)
    // Header: [2, 0, 4, 1, 4], Content: [y=20, x=10]
    assert_eq!(&bytes2[..], &[2, 0, 4, 1, 4, 0, 0, 0, 20, 0, 0, 0, 10]);
}

/// Test optional fields with null encoding
#[test]
fn test_optional_fields_binary_format() {
    let mut properties = IndexMap::new();
    properties.insert("id".to_owned(), Property::required(SchemaType::int32()));
    properties.insert("name".to_owned(), Property::optional(SchemaType::string()));
    let schema = SchemaType::object(properties);

    // Test with optional field present
    let mut obj1 = IndexMap::new();
    obj1.insert("id".to_owned(), Value::Integer(1));
    obj1.insert("name".to_owned(), Value::String("Alice".to_owned()));

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Object(obj1), &schema).unwrap();
    let bytes_with_name = encoder.finish();

    // Test with optional field missing (should encode as null)
    let mut obj2 = IndexMap::new();
    obj2.insert("id".to_owned(), Value::Integer(1));
    // name is missing

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Object(obj2), &schema).unwrap();
    let bytes_without_name = encoder.finish();

    // With name should be longer
    assert!(bytes_with_name.len() > bytes_without_name.len());

    // Without name: new format omits missing optional fields entirely
    // Header: [1, 0, 4] (1 property, idx 0, size 4)
    // Content: [id=1]
    assert_eq!(bytes_without_name.len(), 7);
    assert_eq!(&bytes_without_name[..], &[1, 0, 4, 0, 0, 0, 1]); // header + id=1
}

/// Test complex nested structure
#[test]
fn test_nested_structure_format() {
    // Create Address schema
    let mut address_props = IndexMap::new();
    address_props.insert(
        "street".to_owned(),
        Property::required(SchemaType::string()),
    );
    address_props.insert("city".to_owned(), Property::required(SchemaType::string()));
    let address_schema = SchemaType::object(address_props);

    // Create User schema with nested Address
    let mut user_props = IndexMap::new();
    user_props.insert("name".to_owned(), Property::required(SchemaType::string()));
    user_props.insert("age".to_owned(), Property::required(SchemaType::int32()));
    user_props.insert("address".to_owned(), Property::required(address_schema));
    let schema = SchemaType::object(user_props);

    // Create nested value
    let mut address = IndexMap::new();
    address.insert("street".to_owned(), Value::String("123 Main".to_owned()));
    address.insert("city".to_owned(), Value::String("NYC".to_owned()));

    let mut user = IndexMap::new();
    user.insert("name".to_owned(), Value::String("Bob".to_owned()));
    user.insert("age".to_owned(), Value::Integer(25));
    user.insert("address".to_owned(), Value::Object(address));

    let mut encoder = Encoder::new();
    encoder.encode(&Value::Object(user), &schema).unwrap();
    let bytes = encoder.finish();

    // Verify roundtrip
    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();

    // Decode and verify structure
    if let Value::Object(obj) = decoded {
        assert_eq!(obj.get("name"), Some(&Value::String("Bob".to_owned())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(25)));

        if let Some(Value::Object(addr)) = obj.get("address") {
            assert_eq!(
                addr.get("street"),
                Some(&Value::String("123 Main".to_owned()))
            );
            assert_eq!(addr.get("city"), Some(&Value::String("NYC".to_owned())));
        } else {
            panic!("Expected nested address object");
        }
    } else {
        panic!("Expected user object");
    }
}

/// Test reference resolution with registry
#[test]
fn test_reference_binary_format() {
    let registry = SchemaRegistry::new();

    // Register a simple schema
    let user_schema = SchemaType::int32();
    registry.register("User", user_schema.clone()).unwrap();

    // Create a schema that references User
    let ref_schema = SchemaType::reference("#/User");

    // Encode using reference
    let mut encoder = Encoder::new();
    encoder
        .encode_with_registry(&Value::Integer(42), &ref_schema, &registry)
        .unwrap();
    let bytes_ref = encoder.finish();

    // Encode directly with User schema
    let mut encoder = Encoder::new();
    encoder.encode(&Value::Integer(42), &user_schema).unwrap();
    let bytes_direct = encoder.finish();

    // Reference and direct encoding should produce identical binary
    assert_eq!(bytes_ref, bytes_direct);
}

/// Test that encoding validates against schema
#[test]
fn test_schema_validation() {
    let schema = SchemaType::int32();

    // Wrong type should fail
    let result = Encoder::new().encode(&Value::String("not an int".to_owned()), &schema);
    assert!(result.is_err());

    // Out of range should fail
    let result = Encoder::new().encode(&Value::Integer(i64::from(i32::MAX) + 1), &schema);
    assert!(result.is_err());
}

/// Test size limits
#[test]
fn test_size_limits() {
    // String length limit (u16::MAX bytes for UTF-8 = 65535 bytes)
    let schema = SchemaType::string();
    let long_string = "x".repeat(65535); // Max valid length (65535 bytes UTF-8 for ASCII)
    let result = Encoder::new().encode(&Value::String(long_string), &schema);
    assert!(result.is_ok());

    let too_long_string = "x".repeat(65536); // Too long (65536 bytes UTF-8)
    let result = Encoder::new().encode(&Value::String(too_long_string), &schema);
    assert!(result.is_err());

    // Array element size limit (u8::MAX = 255 bytes per element)
    let array_schema = SchemaType::array(SchemaType::int32());
    let small_array = Value::Array(vec![Value::Integer(1); 100]);
    let result = Encoder::new().encode(&small_array, &array_schema);
    assert!(result.is_ok());
}
