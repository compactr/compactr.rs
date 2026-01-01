//! OpenAPI Spec File Loading Example
//!
//! This example demonstrates the complete workflow:
//! 1. Load an OpenAPI 3.0.x specification from a JSON file
//! 2. Extract schemas for specific components
//! 3. Convert to Compactr schemas
//! 4. Encode data to binary format
//! 5. Decode and verify
//!
//! Run with: `cargo run --example openapi_from_spec_file`

use chrono::Utc;
use compactr::{Decoder, Encoder, Property, SchemaRegistry, SchemaType, Value};
use indexmap::IndexMap;
use openapiv3::{
    IntegerFormat as OpenAPIIntegerFormat, NumberFormat as OpenAPINumberFormat, OpenAPI,
    ReferenceOr, Schema, SchemaKind, StringFormat as OpenAPIStringFormat, Type,
    VariantOrUnknownOrEmpty,
};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenAPI Spec File Loading Example ===\n");

    // Step 1: Load OpenAPI spec from file
    let spec_path = "examples/fixtures/api_spec.json";
    println!("1. Loading OpenAPI spec from: {}", spec_path);

    let spec_json = std::fs::read_to_string(spec_path)?;
    let spec: OpenAPI = serde_json::from_str(&spec_json)?;

    println!("   ✓ Loaded OpenAPI {}", spec.openapi);
    println!("   ✓ API: {} v{}", spec.info.title, spec.info.version);
    println!();

    // Step 2: Extract and convert User schema
    println!("2. Extracting schemas from components:");

    let components = spec
        .components
        .as_ref()
        .ok_or("No components section in spec")?;

    let user_schema_ref = components
        .schemas
        .get("User")
        .ok_or("User schema not found")?;

    let user_schema = match user_schema_ref {
        ReferenceOr::Item(schema) => convert_schema(schema)?,
        ReferenceOr::Reference { .. } => return Err("Unexpected reference".into()),
    };

    println!("   ✓ User schema converted");

    // Also extract Article schema (which references User)
    let article_schema_ref = components
        .schemas
        .get("Article")
        .ok_or("Article schema not found")?;

    // Create registry for handling references
    let registry = SchemaRegistry::new();
    registry.register("User", user_schema.clone())?;

    let article_schema = match article_schema_ref {
        ReferenceOr::Item(schema) => convert_schema(schema)?,
        ReferenceOr::Reference { .. } => return Err("Unexpected reference".into()),
    };

    registry.register("Article", article_schema.clone())?;
    println!("   ✓ Article schema converted (with User reference)");
    println!();

    // Step 3: Create data matching the User schema
    println!("3. Creating data matching User schema:");

    let user = create_user(
        "550e8400-e29b-41d4-a716-446655440000",
        "Alice Johnson",
        Some("alice@example.com"),
        Some(28),
    )?;

    println!("   ✓ Created User: Alice Johnson");
    println!();

    // Step 4: Encode with Compactr
    println!("4. Encoding with Compactr:");

    let mut encoder = Encoder::new();
    encoder.encode_with_registry(&user, &user_schema, &registry)?;
    let compactr_bytes = encoder.finish();

    println!("   ✓ Encoded to {} bytes", compactr_bytes.len());

    // Compare with JSON
    let json_value = serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Alice Johnson",
        "email": "alice@example.com",
        "age": 28,
        "created_at": "2024-01-15T10:30:00Z"
    });

    let json_bytes = serde_json::to_vec(&json_value)?;
    println!("   • JSON:     {} bytes", json_bytes.len());
    println!("   • Compactr: {} bytes", compactr_bytes.len());

    let savings = json_bytes.len() - compactr_bytes.len();
    let savings_pct = (savings as f64 / json_bytes.len() as f64) * 100.0;
    println!("   • Saved:    {} bytes ({:.1}%)", savings, savings_pct);
    println!();

    // Step 5: Decode and validate
    println!("5. Decoding and validation:");

    let mut buf = compactr_bytes.as_ref();
    let decoded = Decoder::decode_with_registry(&mut buf, &user_schema, &registry)?;

    if let Value::Object(obj) = &decoded {
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("email"));
        assert!(obj.contains_key("age"));
        assert!(obj.contains_key("created_at"));

        println!("   ✓ All required fields present");
        println!("   ✓ Data matches schema specification");
        println!("   ✓ Roundtrip successful");
    } else {
        return Err("Expected object value".into());
    }

    println!("\n✓ Complete workflow demonstrated:");
    println!("  1. Loaded OpenAPI 3.0.x spec from JSON file");
    println!("  2. Extracted and converted component schemas");
    println!("  3. Created data matching schema");
    println!("  4. Encoded to compact binary format");
    println!("  5. Decoded and validated");
    println!("\n  This is the real-world integration pattern!");

    Ok(())
}

/// Convert an OpenAPI schema to a Compactr SchemaType
fn convert_schema(schema: &Schema) -> Result<SchemaType, String> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(string_type)) => {
            match &string_type.format {
                VariantOrUnknownOrEmpty::Item(OpenAPIStringFormat::Date) => {
                    Ok(SchemaType::string_date())
                }
                VariantOrUnknownOrEmpty::Item(OpenAPIStringFormat::DateTime) => {
                    Ok(SchemaType::string_datetime())
                }
                VariantOrUnknownOrEmpty::Item(
                    OpenAPIStringFormat::Binary | OpenAPIStringFormat::Byte,
                ) => Ok(SchemaType::binary()),
                VariantOrUnknownOrEmpty::Unknown(s) => match s.as_str() {
                    "uuid" => Ok(SchemaType::string_uuid()),
                    "ipv4" => Ok(SchemaType::string_ipv4()),
                    "ipv6" => Ok(SchemaType::string_ipv6()),
                    _ => Ok(SchemaType::string()), // email, etc. treated as plain string
                },
                _ => Ok(SchemaType::string()),
            }
        }
        SchemaKind::Type(Type::Integer(int_type)) => match &int_type.format {
            VariantOrUnknownOrEmpty::Item(OpenAPIIntegerFormat::Int32) => Ok(SchemaType::int32()),
            VariantOrUnknownOrEmpty::Item(OpenAPIIntegerFormat::Int64) => Ok(SchemaType::int64()),
            _ => Ok(SchemaType::int64()),
        },
        SchemaKind::Type(Type::Number(num_type)) => match &num_type.format {
            VariantOrUnknownOrEmpty::Item(OpenAPINumberFormat::Float) => Ok(SchemaType::float()),
            VariantOrUnknownOrEmpty::Item(OpenAPINumberFormat::Double) => Ok(SchemaType::double()),
            _ => Ok(SchemaType::double()),
        },
        SchemaKind::Type(Type::Boolean(_)) => Ok(SchemaType::boolean()),
        SchemaKind::Type(Type::Array(array_type)) => {
            if let Some(ref items) = array_type.items {
                match items {
                    ReferenceOr::Item(schema) => {
                        let item_schema = convert_schema(schema)?;
                        Ok(SchemaType::array(item_schema))
                    }
                    ReferenceOr::Reference { reference } => {
                        Ok(SchemaType::array(SchemaType::reference(reference.clone())))
                    }
                }
            } else {
                Err("Array schema missing items".to_owned())
            }
        }
        SchemaKind::Type(Type::Object(object_type)) => {
            let mut properties = IndexMap::new();

            for (name, prop_ref) in &object_type.properties {
                let is_required = object_type.required.contains(name);

                match prop_ref {
                    ReferenceOr::Item(prop_schema) => {
                        let schema_type = convert_schema(prop_schema)?;
                        let property = if is_required {
                            Property::required(schema_type)
                        } else {
                            Property::optional(schema_type)
                        };
                        properties.insert(name.clone(), property);
                    }
                    ReferenceOr::Reference { reference } => {
                        let schema_type = SchemaType::reference(reference.clone());
                        let property = if is_required {
                            Property::required(schema_type)
                        } else {
                            Property::optional(schema_type)
                        };
                        properties.insert(name.clone(), property);
                    }
                }
            }

            Ok(SchemaType::object(properties))
        }
        _ => Err(format!("Unsupported schema kind: {:?}", schema.schema_kind)),
    }
}

/// Create a User value
fn create_user(
    id: &str,
    name: &str,
    email: Option<&str>,
    age: Option<i32>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut user = IndexMap::new();

    user.insert("id".to_owned(), Value::Uuid(Uuid::parse_str(id)?));
    user.insert("name".to_owned(), Value::String(name.to_owned()));

    if let Some(e) = email {
        user.insert("email".to_owned(), Value::String(e.to_owned()));
    }

    if let Some(a) = age {
        user.insert("age".to_owned(), Value::Integer(i64::from(a)));
    }

    user.insert("created_at".to_owned(), Value::DateTime(Utc::now()));

    Ok(Value::Object(user))
}
