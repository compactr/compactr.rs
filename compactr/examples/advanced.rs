//! Advanced example showcasing various data types and formats.

use chrono::{NaiveDate, Utc};
use compactr::{Decoder, Encoder, Property, SchemaType, Value};
use indexmap::IndexMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Compactr Advanced Example ===\n");

    // Define a complex schema with various formats
    let mut properties = IndexMap::new();
    properties.insert(
        "id".to_owned(),
        Property::required(SchemaType::string_uuid()),
    );
    properties.insert("name".to_owned(), Property::required(SchemaType::string()));
    properties.insert("age".to_owned(), Property::required(SchemaType::int32()));
    properties.insert("score".to_owned(), Property::required(SchemaType::double()));
    properties.insert(
        "created_at".to_owned(),
        Property::required(SchemaType::string_datetime()),
    );
    properties.insert(
        "birth_date".to_owned(),
        Property::required(SchemaType::string_date()),
    );
    properties.insert(
        "ip_address".to_owned(),
        Property::required(SchemaType::string_ipv4()),
    );
    properties.insert(
        "ipv6_address".to_owned(),
        Property::required(SchemaType::string_ipv6()),
    );
    properties.insert(
        "tags".to_owned(),
        Property::required(SchemaType::array(SchemaType::string())),
    );
    properties.insert(
        "metadata".to_owned(),
        Property::required(SchemaType::binary()),
    );

    let schema = SchemaType::object(properties);

    // Create a complex value
    let mut obj = IndexMap::new();
    obj.insert(
        "id".to_owned(),
        Value::Uuid(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?),
    );
    obj.insert("name".to_owned(), Value::String("Bob Smith".to_owned()));
    obj.insert("age".to_owned(), Value::Integer(42));
    obj.insert("score".to_owned(), Value::Double(98.7));
    obj.insert("created_at".to_owned(), Value::DateTime(Utc::now()));
    obj.insert(
        "birth_date".to_owned(),
        Value::Date(NaiveDate::from_ymd_opt(1982, 5, 15).unwrap()),
    );
    obj.insert(
        "ip_address".to_owned(),
        Value::Ipv4(Ipv4Addr::new(192, 168, 1, 100)),
    );
    obj.insert(
        "ipv6_address".to_owned(),
        Value::Ipv6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)),
    );
    obj.insert(
        "tags".to_owned(),
        Value::Array(vec![
            Value::String("rust".to_owned()),
            Value::String("serialization".to_owned()),
            Value::String("compactr".to_owned()),
        ]),
    );
    obj.insert(
        "metadata".to_owned(),
        Value::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    );

    let value = Value::Object(obj);

    println!("Original value:");
    println!("{:#?}\n", value);

    // Encode the value
    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema)?;
    let encoded = encoder.finish();

    println!("Encoded size: {} bytes", encoded.len());

    // Compare with JSON
    let json_value = serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Bob Smith",
        "age": 42,
        "score": 98.7,
        "created_at": "2024-01-15T10:30:00Z",
        "birth_date": "1982-05-15",
        "ip_address": "192.168.1.100",
        "ipv6_address": "2001:db8::1",
        "tags": ["rust", "serialization", "compactr"],
        "metadata": [0xDE, 0xAD, 0xBE, 0xEF]
    });

    let json_size = serde_json::to_vec(&json_value)?.len();
    println!("JSON size: {} bytes", json_size);

    let reduction = ((json_size - encoded.len()) as f64 / json_size as f64) * 100.0;
    println!(
        "Size reduction: {:.1}% ({} bytes saved)\n",
        reduction,
        json_size - encoded.len()
    );

    // Decode the value
    let mut buf = encoded.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema)?;

    println!("Decoded value:");
    println!("{:#?}\n", decoded);

    // Verify they match (comparing UUID and other fields separately due to DateTime precision)
    if let (Value::Object(orig), Value::Object(dec)) = (&value, &decoded) {
        assert_eq!(orig.get("id"), dec.get("id"));
        assert_eq!(orig.get("name"), dec.get("name"));
        assert_eq!(orig.get("age"), dec.get("age"));
        assert_eq!(orig.get("tags"), dec.get("tags"));
        println!("✓ Roundtrip successful!");
    }

    Ok(())
}
