//! OpenAPI User CRUD Example
//!
//! This example demonstrates a simple User CRUD schema matching common REST API patterns.
//! Shows optional fields, standard OpenAPI types, and size comparison with JSON.
//!
//! Run with: `cargo run --example openapi_user_crud`

use chrono::Utc;
use compactr::{Decoder, Encoder, Property, SchemaType, Value};
use indexmap::IndexMap;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenAPI User CRUD Example ===\n");

    // Define User schema (matches common OpenAPI User schema)
    let user_schema = create_user_schema();
    println!("User Schema (OpenAPI-compatible):");
    println!("  id:         UUID (required)");
    println!("  name:       string (required)");
    println!("  email:      string (optional)");
    println!("  created_at: date-time (required)\n");

    // Create sample users
    let user1 = create_user(
        "550e8400-e29b-41d4-a716-446655440000",
        "Alice Smith",
        Some("alice@example.com"),
    )?;

    let user2 = create_user(
        "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "Bob Jones",
        None, // No email provided (optional field)
    )?;

    println!("Created 2 users:");
    println!("  User 1: Alice Smith (with email)");
    println!("  User 2: Bob Jones (without email)\n");

    // Encode users to Compactr binary
    let mut encoder1 = Encoder::new();
    encoder1.encode(&user1, &user_schema)?;
    let compactr_bytes1 = encoder1.finish();

    let mut encoder2 = Encoder::new();
    encoder2.encode(&user2, &user_schema)?;
    let compactr_bytes2 = encoder2.finish();

    println!("Compactr Binary Encoding:");
    println!("  User 1: {} bytes", compactr_bytes1.len());
    println!("  User 2: {} bytes", compactr_bytes2.len());

    // Compare with JSON
    let json1 = serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Alice Smith",
        "email": "alice@example.com",
        "created_at": "2024-01-15T10:30:00Z"
    });

    let json2 = serde_json::json!({
        "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "name": "Bob Jones",
        "created_at": "2024-01-15T10:30:00Z"
    });

    let json_bytes1 = serde_json::to_vec(&json1)?;
    let json_bytes2 = serde_json::to_vec(&json2)?;

    println!("\nJSON Encoding:");
    println!("  User 1: {} bytes", json_bytes1.len());
    println!("  User 2: {} bytes", json_bytes2.len());

    // Calculate savings
    let savings1 = json_bytes1.len() - compactr_bytes1.len();
    let savings_pct1 = (savings1 as f64 / json_bytes1.len() as f64) * 100.0;

    let savings2 = json_bytes2.len() - compactr_bytes2.len();
    let savings_pct2 = (savings2 as f64 / json_bytes2.len() as f64) * 100.0;

    println!("\nSize Reduction:");
    println!("  User 1: {} bytes saved ({:.1}%)", savings1, savings_pct1);
    println!("  User 2: {} bytes saved ({:.1}%)", savings2, savings_pct2);

    // Decode and verify
    println!("\nDecoding and Verification:");
    let mut buf1 = compactr_bytes1.as_ref();
    let decoded1 = Decoder::decode(&mut buf1, &user_schema)?;

    let mut buf2 = compactr_bytes2.as_ref();
    let decoded2 = Decoder::decode(&mut buf2, &user_schema)?;

    // Verify User 1 (with email)
    if let Value::Object(obj) = &decoded1 {
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("email")); // Has email
        assert!(obj.contains_key("created_at"));
        println!("  ✓ User 1 decoded correctly (with email)");
    }

    // Verify User 2 (without email)
    if let Value::Object(obj) = &decoded2 {
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(!obj.contains_key("email")); // No email (optional field skipped)
        assert!(obj.contains_key("created_at"));
        println!("  ✓ User 2 decoded correctly (email omitted)");
    }

    println!("\n✓ Success! Compactr provides:");
    println!("  • Compact binary encoding ({:.0}% smaller than JSON)", savings_pct1);
    println!("  • OpenAPI-compatible schemas");
    println!("  • Proper handling of optional fields");
    println!("  • Framework-agnostic approach");

    Ok(())
}

/// Create a User schema matching common REST API patterns
fn create_user_schema() -> SchemaType {
    let mut properties = IndexMap::new();

    // Required fields
    properties.insert(
        "id".to_owned(),
        Property::required(SchemaType::string_uuid()),
    );
    properties.insert(
        "name".to_owned(),
        Property::required(SchemaType::string()),
    );

    // Optional field
    properties.insert(
        "email".to_owned(),
        Property::optional(SchemaType::string()),
    );

    // Required timestamp
    properties.insert(
        "created_at".to_owned(),
        Property::required(SchemaType::string_datetime()),
    );

    SchemaType::object(properties)
}

/// Create a User value
fn create_user(id: &str, name: &str, email: Option<&str>) -> Result<Value, Box<dyn std::error::Error>> {
    let mut user = IndexMap::new();

    user.insert("id".to_owned(), Value::Uuid(Uuid::parse_str(id)?));
    user.insert("name".to_owned(), Value::String(name.to_owned()));

    // Only include email if provided (optional field)
    if let Some(e) = email {
        user.insert("email".to_owned(), Value::String(e.to_owned()));
    }

    user.insert("created_at".to_owned(), Value::DateTime(Utc::now()));

    Ok(Value::Object(user))
}
