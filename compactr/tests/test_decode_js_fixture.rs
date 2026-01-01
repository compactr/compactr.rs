//! Test decoding JS-generated fixtures

use compactr::{Decoder, Property, SchemaType};
use indexmap::IndexMap;
use std::path::Path;

#[test]
fn test_decode_int32_42() {
    let fixture_path = Path::new("compactr/tests/fixtures/int32_42.bin");

    if !fixture_path.exists() {
        println!("⚠️  Fixture not found, skipping test");
        println!("   Run: node compactr/tests/fixtures/generate_fixtures.js");
        return;
    }

    let bytes = std::fs::read(fixture_path).expect("Failed to read fixture");

    println!("Fixture bytes: {:?}", bytes);
    println!(
        "Fixture hex:   {}",
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );

    // Create schema: {value: int32}
    let mut properties = IndexMap::new();
    properties.insert("value".to_owned(), Property::required(SchemaType::int32()));
    let schema = SchemaType::object(properties);

    // Decode
    let mut buf = bytes.as_slice();
    let value = Decoder::decode(&mut buf, &schema).expect("Failed to decode");

    println!("Decoded value: {:?}", value);

    // Verify it's an object with value=42
    if let compactr::Value::Object(obj) = value {
        let val = obj.get("value").expect("Missing 'value' property");
        if let compactr::Value::Integer(n) = val {
            assert_eq!(*n, 42, "Decoded value should be 42");
            println!("✓ Successfully decoded JS fixture: {{value: 42}}");
        } else {
            panic!("Expected Integer value, got {:?}", val);
        }
    } else {
        panic!("Expected Object, got {:?}", value);
    }
}
