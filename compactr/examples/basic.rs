//! Basic usage example for Compactr.

use compactr::{Decoder, Encoder, Property, SchemaType, Value};
use indexmap::IndexMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Compactr Basic Example ===\n");

    // Define a simple user schema
    let mut properties = IndexMap::new();
    properties.insert("name".to_owned(), Property::required(SchemaType::string()));
    properties.insert("age".to_owned(), Property::required(SchemaType::int32()));
    properties.insert(
        "active".to_owned(),
        Property::required(SchemaType::boolean()),
    );

    let schema = SchemaType::object(properties);

    // Create a user value
    let mut user = IndexMap::new();
    user.insert("name".to_owned(), Value::String("Alice".to_owned()));
    user.insert("age".to_owned(), Value::Integer(30));
    user.insert("active".to_owned(), Value::Boolean(true));

    let value = Value::Object(user);

    println!("Original value:");
    println!("{:#?}\n", value);

    // Encode the value
    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema)?;
    let encoded = encoder.finish();

    println!("Encoded size: {} bytes", encoded.len());
    println!("Encoded bytes: {:?}\n", encoded.as_ref());

    // Decode the value
    let mut buf = encoded.as_ref();
    let decoded = Decoder::decode(&mut buf, &schema)?;

    println!("Decoded value:");
    println!("{:#?}\n", decoded);

    // Verify they match
    assert_eq!(value, decoded);
    println!("✓ Roundtrip successful!");

    Ok(())
}
