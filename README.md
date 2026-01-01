<h1 align="center">
  <a title="OpenAPI serialization" href="http://compactr.js.org">
    <img alt="Compactr" width="320px" src="http://res.cloudinary.com/kalm/image/upload/v1494589244/compactr_header_rev1.png" />
    <br/><br/>
  </a>
  Compactr
</h1>
<h3 align="center">
  OpenAPI serialization
  <br/><br/><br/>
</h3>
<br/>

- **Schema-based serialization**: Define data structures using OpenAPI 3.x compatible schemas
- **Compact binary format**: 3-5x size reduction compared to JSON
- **Type-safe**: Full Rust type safety with optional derive macros
- **Cross-language**: Binary output compatible with all other Compactr clients (.js, .cs)
- **Thread-safe**: Schema registry with concurrent access support

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
compactr = "0.1"

# For derive macro support
compactr = { version = "0.1", features = ["derive"] }

# For serde integration
compactr = { version = "0.1", features = ["serde"] }

# For all features
compactr = { version = "0.1", features = ["full"] }
```

## Usage

### Using the Value API

```rust
use compactr::{SchemaType, Value, Property};
use indexmap::IndexMap;

// Define a schema
let mut properties = IndexMap::new();
properties.insert("id".to_string(), Property::required(SchemaType::string_uuid()));
properties.insert("name".to_string(), Property::required(SchemaType::string()));
properties.insert("age".to_string(), Property::required(SchemaType::int32()));

let schema = SchemaType::object(properties);

// Create a value
let mut obj = IndexMap::new();
obj.insert("id".to_string(), Value::from("550e8400-e29b-41d4-a716-446655440000"));
obj.insert("name".to_string(), Value::from("Alice"));
obj.insert("age".to_string(), Value::from(30_i32));

let value = Value::Object(obj);

// Encode and decode (implementation in progress)
// let encoded = encode(&value, &schema)?;
// let decoded = decode(&encoded, &schema)?;
```

### Using Derive Macros (WIP)

```rust
use compactr_derive::Compactr;
use uuid::Uuid;

#[derive(Compactr)]
struct User {
    #[compactr(format = "uuid")]
    id: Uuid,
    name: String,
    age: i32,
}
```


### Load Existing OpenAPI Specs

```rust
use openapiv3::OpenAPI;
use compactr::{SchemaType, Encoder, Decoder};

// Load from JSON/YAML
let spec: OpenAPI = serde_json::from_str(&spec_json)?;
let user_schema = spec.components.schemas.get("User")?;

// Convert to Compactr schema
let compactr_schema = convert_schema(user_schema)?;

// Use for encoding/decoding
let mut encoder = Encoder::new();
encoder.encode(&user_data, &compactr_schema)?;
```

### Manual Schema Construction

```rust
// Matches this OpenAPI schema:
// components:
//   schemas:
//     User:
//       type: object
//       required: [id, name]
//       properties:
//         id: { type: string, format: uuid }
//         name: { type: string }
//         email: { type: string }

let mut props = IndexMap::new();
props.insert("id", Property::required(SchemaType::string_uuid()));
props.insert("name", Property::required(SchemaType::string()));
props.insert("email", Property::optional(SchemaType::string()));
let schema = SchemaType::object(props);
```

### Integration with OpenAPI Tools

Compactr works with popular Rust OpenAPI libraries:
- **[openapiv3](https://crates.io/crates/openapiv3)** - Parse OpenAPI 3.0.x specs
- **[oas3](https://crates.io/crates/oas3)** - Parse OpenAPI 3.1.x specs
- **[utoipa](https://crates.io/crates/utoipa)** - Code-first OpenAPI generation
- **[progenitor](https://crates.io/crates/progenitor)** - Client generation

See `examples/openapi_*.rs` for complete integration examples.

## Supported Types

| Schema Type | Rust Type | Binary Size |
|------------|-----------|-------------|
| `boolean` | `bool` | 1 byte |
| `integer(int32)` | `i32` | 4 bytes |
| `integer(int64)` | `i64` | 8 bytes |
| `number(float)` | `f32` | 4 bytes |
| `number(double)` | `f64` | 8 bytes |
| `string` | `String` | 2 + N bytes |
| `string(uuid)` | `Uuid` | 16 bytes |
| `string(datetime)` | `DateTime<Utc>` | 8 bytes |
| `string(date)` | `NaiveDate` | 4 bytes |
| `string(ipv4)` | `Ipv4Addr` | 4 bytes |
| `string(ipv6)` | `Ipv6Addr` | 16 bytes |
| `binary` | `Vec<u8>` | 4 + N bytes |
| `array` | `Vec<T>` | 4 + items |
| `object` | `IndexMap<String, T>` | sum of fields |

## Development Status

- [x] Project structure and dependencies
- [x] Error types
- [x] Value types
- [x] Schema definitions
- [x] Schema registry
- [x] Encoder/Decoder implementation
- [x] Format implementations (UUID, DateTime, Date, IPv4, IPv6, Binary)
- [x] OpenAPI integration examples
- [ ] Derive macros
- [ ] Cross-language compatibility tests

## Testing

### Building from Source

```bash
git clone https://github.com/yourusername/compactr.rs
cd compactr.rs
cargo build
cargo test
```

### Running Tests

```bash
cargo test --all-features
```

### Running Benchmarks

```bash
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request


## License

[Apache 2.0](LICENSE) (c) 2025 Frederic Charette