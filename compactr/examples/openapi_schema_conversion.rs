//! OpenAPI Schema Conversion Example
//!
//! This example demonstrates how to convert OpenAPI 3.0.x schemas to Compactr schemas,
//! making Compactr framework-agnostic and compatible with any OpenAPI tooling.
//!
//! Run with: `cargo run --example openapi_schema_conversion`

use compactr::{Property, SchemaType};
use indexmap::IndexMap;
use openapiv3::{
    IntegerFormat as OpenAPIIntegerFormat, NumberFormat as OpenAPINumberFormat, ReferenceOr,
    Schema, SchemaKind, StringFormat as OpenAPIStringFormat, Type, VariantOrUnknownOrEmpty,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenAPI Schema Conversion Example ===\n");

    // Example 1: Convert a simple string schema
    println!("1. Simple String Schema");
    let openapi_string = create_string_schema(None);
    let compactr_string = convert_schema(&openapi_string)?;
    println!("   OpenAPI: string");
    println!("   Compactr: {:?}\n", compactr_string);

    // Example 2: Convert UUID format
    println!("2. UUID Format Schema");
    let openapi_uuid = create_string_schema(Some("uuid"));
    let compactr_uuid = convert_schema(&openapi_uuid)?;
    println!("   OpenAPI: string(format=uuid)");
    println!("   Compactr: {:?}\n", compactr_uuid);

    // Example 3: Convert datetime format
    println!("3. DateTime Format Schema");
    let openapi_datetime = create_string_schema(Some("date-time"));
    let compactr_datetime = convert_schema(&openapi_datetime)?;
    println!("   OpenAPI: string(format=date-time)");
    println!("   Compactr: {:?}\n", compactr_datetime);

    // Example 4: Convert integer types
    println!("4. Integer Schemas");
    let openapi_int32 = create_integer_schema(Some("int32"));
    let compactr_int32 = convert_schema(&openapi_int32)?;
    println!("   OpenAPI: integer(format=int32)");
    println!("   Compactr: {:?}", compactr_int32);

    let openapi_int64 = create_integer_schema(Some("int64"));
    let compactr_int64 = convert_schema(&openapi_int64)?;
    println!("   OpenAPI: integer(format=int64)");
    println!("   Compactr: {:?}\n", compactr_int64);

    // Example 5: Convert number types
    println!("5. Number Schemas");
    let openapi_float = create_number_schema(Some("float"));
    let compactr_float = convert_schema(&openapi_float)?;
    println!("   OpenAPI: number(format=float)");
    println!("   Compactr: {:?}", compactr_float);

    let openapi_double = create_number_schema(Some("double"));
    let compactr_double = convert_schema(&openapi_double)?;
    println!("   OpenAPI: number(format=double)");
    println!("   Compactr: {:?}\n", compactr_double);

    // Example 6: Convert array schema
    println!("6. Array Schema");
    let openapi_array = create_array_schema(create_string_schema(None));
    let compactr_array = convert_schema(&openapi_array)?;
    println!("   OpenAPI: array[string]");
    println!("   Compactr: {:?}\n", compactr_array);

    // Example 7: Convert object schema
    println!("7. Object Schema (User)");
    let openapi_user = create_user_schema();
    let compactr_user = convert_schema(&openapi_user)?;
    println!("   OpenAPI: object{{id: uuid, name: string, email: string?}}");
    println!("   Compactr: {:?}\n", compactr_user);

    // Example 8: All supported formats
    println!("8. All OpenAPI Formats Supported:");
    println!("   ✓ uuid      → string_uuid()");
    println!("   ✓ date-time → string_datetime()");
    println!("   ✓ date      → string_date()");
    println!("   ✓ ipv4      → string_ipv4()");
    println!("   ✓ ipv6      → string_ipv6()");
    println!("   ✓ binary    → binary()");
    println!("   ✓ int32     → int32()");
    println!("   ✓ int64     → int64()");
    println!("   ✓ float     → float()");
    println!("   ✓ double    → double()\n");

    println!("✓ OpenAPI schemas can be converted to Compactr schemas!");
    println!("  This makes Compactr framework-agnostic - works with any OpenAPI source.");

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
                    _ => Ok(SchemaType::string()), // Unknown format, treat as plain string
                },
                _ => Ok(SchemaType::string()),
            }
        }
        SchemaKind::Type(Type::Integer(int_type)) => match &int_type.format {
            VariantOrUnknownOrEmpty::Item(OpenAPIIntegerFormat::Int32) => Ok(SchemaType::int32()),
            VariantOrUnknownOrEmpty::Item(OpenAPIIntegerFormat::Int64) => Ok(SchemaType::int64()),
            _ => Ok(SchemaType::int64()), // Default to int64
        },
        SchemaKind::Type(Type::Number(num_type)) => match &num_type.format {
            VariantOrUnknownOrEmpty::Item(OpenAPINumberFormat::Float) => Ok(SchemaType::float()),
            VariantOrUnknownOrEmpty::Item(OpenAPINumberFormat::Double) => Ok(SchemaType::double()),
            _ => Ok(SchemaType::double()), // Default to double
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

// Helper functions to create OpenAPI schemas for demonstration

fn create_string_schema(format: Option<&str>) -> Schema {
    Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(Type::String(openapiv3::StringType {
            format: format.map(|s| s.to_owned()).into(),
            ..Default::default()
        })),
    }
}

fn create_integer_schema(format: Option<&str>) -> Schema {
    Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(Type::Integer(openapiv3::IntegerType {
            format: format.map(|s| s.to_owned()).into(),
            ..Default::default()
        })),
    }
}

fn create_number_schema(format: Option<&str>) -> Schema {
    Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(Type::Number(openapiv3::NumberType {
            format: format.map(|s| s.to_owned()).into(),
            ..Default::default()
        })),
    }
}

fn create_array_schema(items: Schema) -> Schema {
    Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(Type::Array(openapiv3::ArrayType {
            items: Some(ReferenceOr::Item(Box::new(items))),
            min_items: None,
            max_items: None,
            unique_items: false,
        })),
    }
}

fn create_user_schema() -> Schema {
    let mut properties = IndexMap::new();

    // id: UUID (required)
    properties.insert(
        "id".to_owned(),
        ReferenceOr::Item(Box::new(create_string_schema(Some("uuid")))),
    );

    // name: String (required)
    properties.insert(
        "name".to_owned(),
        ReferenceOr::Item(Box::new(create_string_schema(None))),
    );

    // email: String (optional)
    properties.insert(
        "email".to_owned(),
        ReferenceOr::Item(Box::new(create_string_schema(None))),
    );

    Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(Type::Object(openapiv3::ObjectType {
            properties,
            required: vec!["id".to_owned(), "name".to_owned()],
            min_properties: None,
            max_properties: None,
            additional_properties: None,
        })),
    }
}
