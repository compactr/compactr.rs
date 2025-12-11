//! Integration tests for the Compactr library.

use compactr::{Decoder, Encoder, Property, SchemaType, Value};
use indexmap::IndexMap;

#[test]
fn test_simple_boolean() {
    let schema = SchemaType::boolean();
    let value = Value::Boolean(true);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn test_integers() {
    // Int32
    let schema = SchemaType::int32();
    let value = Value::Integer(42);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);

    // Int64
    let schema = SchemaType::int64();
    let value = Value::Integer(i64::MAX);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_floats() {
    // Float
    let schema = SchemaType::float();
    let value = Value::Float(3.14);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);

    // Double
    let schema = SchemaType::double();
    let value = Value::Double(3.141_592_653_589_793);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_string() {
    let schema = SchemaType::string();
    let value = Value::String("Hello, Compactr!".to_owned());

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_uuid() {
    use uuid::Uuid;

    let schema = SchemaType::string_uuid();
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let value = Value::Uuid(uuid);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_datetime() {
    use chrono::Utc;

    let schema = SchemaType::string_datetime();
    let dt = Utc::now();
    let value = Value::DateTime(dt);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();

    if let Value::DateTime(decoded_dt) = decoded {
        assert_eq!(decoded_dt.timestamp_millis(), dt.timestamp_millis());
    } else {
        panic!("Expected DateTime value");
    }
}

#[test]
fn test_date() {
    use chrono::NaiveDate;

    let schema = SchemaType::string_date();
    let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    let value = Value::Date(date);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_ipv4() {
    use std::net::Ipv4Addr;

    let schema = SchemaType::string_ipv4();
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let value = Value::Ipv4(ip);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_ipv6() {
    use std::net::Ipv6Addr;

    let schema = SchemaType::string_ipv6();
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
    let value = Value::Ipv6(ip);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_binary() {
    let schema = SchemaType::binary();
    let data = vec![0, 1, 2, 3, 255, 128, 64];
    let value = Value::Binary(data);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_array() {
    let schema = SchemaType::array(SchemaType::int32());
    let value = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_object() {
    let mut properties = IndexMap::new();
    properties.insert("name".to_owned(), Property::required(SchemaType::string()));
    properties.insert("age".to_owned(), Property::required(SchemaType::int32()));
    properties.insert(
        "active".to_owned(),
        Property::required(SchemaType::boolean()),
    );

    let schema = SchemaType::object(properties);

    let mut obj = IndexMap::new();
    obj.insert("name".to_owned(), Value::String("Alice".to_owned()));
    obj.insert("age".to_owned(), Value::Integer(30));
    obj.insert("active".to_owned(), Value::Boolean(true));

    let value = Value::Object(obj);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_nested_object() {
    let mut address_props = IndexMap::new();
    address_props.insert(
        "street".to_owned(),
        Property::required(SchemaType::string()),
    );
    address_props.insert("city".to_owned(), Property::required(SchemaType::string()));

    let address_schema = SchemaType::object(address_props);

    let mut user_props = IndexMap::new();
    user_props.insert("name".to_owned(), Property::required(SchemaType::string()));
    user_props.insert("address".to_owned(), Property::required(address_schema));

    let schema = SchemaType::object(user_props);

    let mut address = IndexMap::new();
    address.insert(
        "street".to_owned(),
        Value::String("123 Main St".to_owned()),
    );
    address.insert("city".to_owned(), Value::String("Springfield".to_owned()));

    let mut user = IndexMap::new();
    user.insert("name".to_owned(), Value::String("Bob".to_owned()));
    user.insert("address".to_owned(), Value::Object(address));

    let value = Value::Object(user);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn test_complex_structure() {
    use chrono::Utc;
    use uuid::Uuid;

    // Create a complex schema
    let mut properties = IndexMap::new();
    properties.insert(
        "id".to_owned(),
        Property::required(SchemaType::string_uuid()),
    );
    properties.insert("name".to_owned(), Property::required(SchemaType::string()));
    properties.insert(
        "tags".to_owned(),
        Property::required(SchemaType::array(SchemaType::string())),
    );
    properties.insert(
        "created_at".to_owned(),
        Property::required(SchemaType::string_datetime()),
    );

    let schema = SchemaType::object(properties);

    // Create a complex value
    let mut obj = IndexMap::new();
    obj.insert(
        "id".to_owned(),
        Value::Uuid(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()),
    );
    obj.insert("name".to_owned(), Value::String("Test Item".to_owned()));
    obj.insert(
        "tags".to_owned(),
        Value::Array(vec![
            Value::String("rust".to_owned()),
            Value::String("serialization".to_owned()),
        ]),
    );
    obj.insert("created_at".to_owned(), Value::DateTime(Utc::now()));

    let value = Value::Object(obj);

    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    // Verify the data is compact (smaller than JSON)
    let json_size = serde_json::to_string(&serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Test Item",
        "tags": ["rust", "serialization"],
        "created_at": "2024-01-15T10:30:00Z"
    }))
    .unwrap()
    .len();

    println!("Binary size: {} bytes", bytes.len());
    println!("JSON size: {} bytes", json_size);
    assert!(bytes.len() < json_size);

    let mut buf = bytes.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema).unwrap();

    // Compare the key fields
    if let (Value::Object(orig), Value::Object(dec)) = (&value, &decoded) {
        assert_eq!(orig.get("id"), dec.get("id"));
        assert_eq!(orig.get("name"), dec.get("name"));
        assert_eq!(orig.get("tags"), dec.get("tags"));
    } else {
        panic!("Expected object values");
    }
}
