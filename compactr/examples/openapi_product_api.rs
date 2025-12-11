//! OpenAPI Product API Example
//!
//! This example demonstrates nested objects, arrays, and schema references using
//! a Product-Category relationship common in e-commerce APIs.
//!
//! Run with: `cargo run --example openapi_product_api`

use compactr::{Decoder, Encoder, Property, SchemaRegistry, SchemaType, Value};
use indexmap::IndexMap;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenAPI Product API Example ===\n");

    // Create schema registry
    let registry = SchemaRegistry::new();

    // Register Category schema
    let category_schema = create_category_schema();
    registry.register("Category", category_schema.clone())?;
    println!("Registered schemas:");
    println!("  • Category (id: i32, name: string)");

    // Create Product schema with reference to Category
    let product_schema = create_product_schema();
    println!("  • Product (id: UUID, name: string, price: double, category: Category, tags: array<string>, in_stock: boolean, discount?: double)\n");

    // Create sample categories
    let electronics = create_category(1, "Electronics");
    let books = create_category(2, "Books");

    // Create sample products
    let product1 = create_product(
        "550e8400-e29b-41d4-a716-446655440000",
        "Wireless Mouse",
        29.99,
        electronics.clone(),
        vec!["computer", "wireless", "mouse"],
        true,
        Some(5.0), // 5% discount
    )?;

    let product2 = create_product(
        "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "Programming Rust Book",
        49.99,
        books,
        vec!["programming", "rust", "book"],
        true,
        None, // No discount
    )?;

    println!("Created 2 products:");
    println!("  1. Wireless Mouse (Electronics, with discount)");
    println!("  2. Programming Rust Book (Books, no discount)\n");

    // Encode products
    let mut encoder1 = Encoder::new();
    encoder1.encode_with_registry(&product1, &product_schema, &registry)?;
    let compactr_bytes1 = encoder1.finish();

    let mut encoder2 = Encoder::new();
    encoder2.encode_with_registry(&product2, &product_schema, &registry)?;
    let compactr_bytes2 = encoder2.finish();

    println!("Compactr Binary Encoding:");
    println!("  Product 1: {} bytes", compactr_bytes1.len());
    println!("  Product 2: {} bytes", compactr_bytes2.len());

    // Compare with JSON
    let json1 = serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Wireless Mouse",
        "price": 29.99,
        "category": {
            "id": 1,
            "name": "Electronics"
        },
        "tags": ["computer", "wireless", "mouse"],
        "in_stock": true,
        "discount": 5.0
    });

    let json2 = serde_json::json!({
        "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "name": "Programming Rust Book",
        "price": 49.99,
        "category": {
            "id": 2,
            "name": "Books"
        },
        "tags": ["programming", "rust", "book"],
        "in_stock": true
    });

    let json_bytes1 = serde_json::to_vec(&json1)?;
    let json_bytes2 = serde_json::to_vec(&json2)?;

    println!("\nJSON Encoding:");
    println!("  Product 1: {} bytes", json_bytes1.len());
    println!("  Product 2: {} bytes", json_bytes2.len());

    let total_compactr = compactr_bytes1.len() + compactr_bytes2.len();
    let total_json = json_bytes1.len() + json_bytes2.len();
    let total_savings = total_json - total_compactr;
    let total_savings_pct = (total_savings as f64 / total_json as f64) * 100.0;

    println!("\nTotal Size Comparison:");
    println!("  Compactr: {} bytes", total_compactr);
    println!("  JSON:     {} bytes", total_json);
    println!("  Saved:    {} bytes ({:.1}%)\n", total_savings, total_savings_pct);

    // Decode and verify
    println!("Decoding and Verification:");
    let mut buf1 = compactr_bytes1.as_ref();
    let decoded1 = Decoder::decode_with_registry(&mut buf1, &product_schema, &registry)?;

    let mut buf2 = compactr_bytes2.as_ref();
    let decoded2 = Decoder::decode_with_registry(&mut buf2, &product_schema, &registry)?;

    // Verify Product 1
    if let Value::Object(obj) = &decoded1 {
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("category")); // Nested object
        assert!(obj.contains_key("tags")); // Array
        assert!(obj.contains_key("discount")); // Optional field present
        println!("  ✓ Product 1 decoded correctly (with nested Category and discount)");

        // Verify nested category
        if let Some(Value::Object(cat)) = obj.get("category") {
            assert!(cat.contains_key("id"));
            assert!(cat.contains_key("name"));
            println!("    ✓ Nested Category decoded correctly");
        }

        // Verify array
        if let Some(Value::Array(tags)) = obj.get("tags") {
            assert_eq!(tags.len(), 3);
            println!("    ✓ Tags array decoded correctly ({} items)", tags.len());
        }
    }

    // Verify Product 2
    if let Value::Object(obj) = &decoded2 {
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(!obj.contains_key("discount")); // Optional field omitted
        println!("  ✓ Product 2 decoded correctly (discount omitted)");
    }

    println!("\n✓ Success! This example demonstrated:");
    println!("  • Nested objects (Product contains Category)");
    println!("  • Array types (tags)");
    println!("  • Optional fields (discount)");
    println!("  • Schema registry for references");
    println!("  • {:.1}% size reduction vs JSON", total_savings_pct);

    Ok(())
}

/// Create a Category schema
fn create_category_schema() -> SchemaType {
    let mut properties = IndexMap::new();

    properties.insert("id".to_owned(), Property::required(SchemaType::int32()));
    properties.insert(
        "name".to_owned(),
        Property::required(SchemaType::string()),
    );

    SchemaType::object(properties)
}

/// Create a Product schema
fn create_product_schema() -> SchemaType {
    let mut properties = IndexMap::new();

    properties.insert(
        "id".to_owned(),
        Property::required(SchemaType::string_uuid()),
    );
    properties.insert(
        "name".to_owned(),
        Property::required(SchemaType::string()),
    );
    properties.insert(
        "price".to_owned(),
        Property::required(SchemaType::double()),
    );

    // Nested object (Category)
    properties.insert(
        "category".to_owned(),
        Property::required(create_category_schema()),
    );

    // Array of strings
    properties.insert(
        "tags".to_owned(),
        Property::required(SchemaType::array(SchemaType::string())),
    );

    properties.insert(
        "in_stock".to_owned(),
        Property::required(SchemaType::boolean()),
    );

    // Optional field
    properties.insert(
        "discount".to_owned(),
        Property::optional(SchemaType::double()),
    );

    SchemaType::object(properties)
}

/// Create a Category value
fn create_category(id: i32, name: &str) -> Value {
    let mut category = IndexMap::new();
    category.insert("id".to_owned(), Value::Integer(i64::from(id)));
    category.insert("name".to_owned(), Value::String(name.to_owned()));
    Value::Object(category)
}

/// Create a Product value
fn create_product(
    id: &str,
    name: &str,
    price: f64,
    category: Value,
    tags: Vec<&str>,
    in_stock: bool,
    discount: Option<f64>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut product = IndexMap::new();

    product.insert("id".to_owned(), Value::Uuid(Uuid::parse_str(id)?));
    product.insert("name".to_owned(), Value::String(name.to_owned()));
    product.insert("price".to_owned(), Value::Double(price));
    product.insert("category".to_owned(), category);
    product.insert(
        "tags".to_owned(),
        Value::Array(tags.into_iter().map(|t| Value::String(t.to_owned())).collect()),
    );
    product.insert("in_stock".to_owned(), Value::Boolean(in_stock));

    // Only include discount if provided
    if let Some(d) = discount {
        product.insert("discount".to_owned(), Value::Double(d));
    }

    Ok(Value::Object(product))
}
