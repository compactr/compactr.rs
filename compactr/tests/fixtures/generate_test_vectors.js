#!/usr/bin/env node
/**
 * Generate test vectors using compactr.js
 *
 * This script generates binary test vectors that can be used to validate
 * compatibility between the JavaScript and Rust implementations.
 *
 * Usage:
 *   npm install compactr
 *   node generate_test_vectors.js
 */

const fs = require('fs');
const path = require('path');

// Note: Uncomment once compactr is installed
// const { schema } = require('compactr');

console.log('Compactr.js Test Vector Generator');
console.log('==================================\n');

// Test Vector 1: Simple Primitives
console.log('Generating test vector 1: primitives...');
const primitives = {
  boolean: {
    schema: { type: 'boolean' },
    value: true,
    expected_hex: '01'
  },
  int32: {
    schema: { type: 'integer', format: 'int32' },
    value: 42,
    expected_hex: '2a000000'
  },
  int64: {
    schema: { type: 'integer', format: 'int64' },
    value: 9007199254740991, // Max safe integer in JS
    expected_hex: 'ffffffffffff1f00'
  },
  float: {
    schema: { type: 'number', format: 'float' },
    value: 3.14,
    expected_description: 'IEEE 754 single precision'
  },
  double: {
    schema: { type: 'number', format: 'double' },
    value: 3.141592653589793,
    expected_description: 'IEEE 754 double precision'
  },
  string: {
    schema: { type: 'string' },
    value: 'Hello',
    expected_hex: '0500' + Buffer.from('Hello').toString('hex')
  }
};

// Test Vector 2: Special String Formats
console.log('Generating test vector 2: special formats...');
const formats = {
  uuid: {
    schema: { type: 'string', format: 'uuid' },
    value: '550e8400-e29b-41d4-a716-446655440000',
    expected_description: '16 bytes raw UUID'
  },
  datetime: {
    schema: { type: 'string', format: 'date-time' },
    value: '2021-01-01T00:00:00.000Z',
    expected_description: '8 bytes i64 milliseconds since epoch'
  },
  date: {
    schema: { type: 'string', format: 'date' },
    value: '2021-01-01',
    expected_description: '4 bytes i32 days since epoch'
  },
  ipv4: {
    schema: { type: 'string', format: 'ipv4' },
    value: '192.168.1.1',
    expected_hex: 'c0a80101'
  },
  ipv6: {
    schema: { type: 'string', format: 'ipv6' },
    value: '2001:db8::1',
    expected_description: '16 bytes raw IPv6'
  },
  binary: {
    schema: { type: 'string', format: 'binary' },
    value: Buffer.from([1, 2, 3]).toString('base64'),
    expected_hex: '03000000010203'
  }
};

// Test Vector 3: Arrays
console.log('Generating test vector 3: arrays...');
const arrays = {
  empty: {
    schema: { type: 'array', items: { type: 'integer', format: 'int32' } },
    value: [],
    expected_hex: '00000000'
  },
  integers: {
    schema: { type: 'array', items: { type: 'integer', format: 'int32' } },
    value: [1, 2, 3],
    expected_hex: '03000000' + '01000000' + '02000000' + '03000000'
  },
  strings: {
    schema: { type: 'array', items: { type: 'string' } },
    value: ['a', 'b'],
    expected_description: 'length u32 + each string with u16 length prefix'
  }
};

// Test Vector 4: Objects
console.log('Generating test vector 4: objects...');
const objects = {
  simple: {
    schema: {
      type: 'object',
      properties: {
        x: { type: 'integer', format: 'int32' },
        y: { type: 'integer', format: 'int32' }
      },
      required: ['x', 'y']
    },
    value: { x: 10, y: 20 },
    expected_hex: '0a000000' + '14000000', // Properties in schema order
    note: 'Properties encoded in schema definition order, not object key order'
  },
  with_optional: {
    schema: {
      type: 'object',
      properties: {
        id: { type: 'integer', format: 'int32' },
        name: { type: 'string' }
      },
      required: ['id']
    },
    value_present: { id: 1, name: 'Alice' },
    value_missing: { id: 1 },
    expected_description: 'Missing optional fields encoded as null (empty string for strings)'
  },
  nested: {
    schema: {
      type: 'object',
      properties: {
        user: {
          type: 'object',
          properties: {
            name: { type: 'string' },
            age: { type: 'integer', format: 'int32' }
          },
          required: ['name', 'age']
        }
      },
      required: ['user']
    },
    value: {
      user: {
        name: 'Bob',
        age: 25
      }
    },
    expected_description: 'Nested objects encoded depth-first'
  }
};

// Test Vector 5: Complete User Object (like in spec)
console.log('Generating test vector 5: complete user object...');
const completeUser = {
  schema: {
    type: 'object',
    properties: {
      id: { type: 'string', format: 'uuid' },
      name: { type: 'string' },
      email: { type: 'string' },
      age: { type: 'integer', format: 'int32' },
      created_at: { type: 'string', format: 'date-time' }
    },
    required: ['id', 'name', 'email', 'age', 'created_at']
  },
  value: {
    id: '550e8400-e29b-41d4-a716-446655440000',
    name: 'Alice Johnson',
    email: 'alice@example.com',
    age: 28,
    created_at: '2024-01-15T10:30:00.000Z'
  }
};

// Write documentation
const documentation = `# Compactr Binary Format Test Vectors

These test vectors define the expected binary format for cross-validation
between compactr.js and compactr.rs implementations.

## Format Specification

### Primitive Types

- **Boolean**: 1 byte (0x00 = false, 0x01 = true)
- **Int32**: 4 bytes, little-endian signed integer
- **Int64**: 8 bytes, little-endian signed integer
- **Float**: 4 bytes, IEEE 754 single precision, little-endian
- **Double**: 8 bytes, IEEE 754 double precision, little-endian
- **String**: 2 bytes length (u16 LE) + UTF-8 bytes
- **Binary**: 4 bytes length (u32 LE) + raw bytes

### Special Formats

- **UUID**: 16 bytes raw (not string representation)
- **DateTime**: 8 bytes i64 LE (milliseconds since Unix epoch)
- **Date**: 4 bytes i32 LE (days since Unix epoch)
- **IPv4**: 4 bytes (octets in order)
- **IPv6**: 16 bytes (raw address bytes)

### Complex Types

- **Array**: 4 bytes length (u32 LE) + elements
- **Object**: Properties in schema definition order (not key order!)
  - Required properties: encoded directly
  - Missing optional properties: encoded as null (type-specific)

## Property Order

**CRITICAL**: Objects encode properties in **schema definition order**,
NOT in the order of object keys. This ensures deterministic encoding.

Example:
\`\`\`javascript
// Schema defines: x, then y
{ type: 'object', properties: { x: {}, y: {} } }

// Value can have any key order
{ y: 20, x: 10 }

// But binary will always be: x value, then y value
\`\`\`

## Test Vectors

${JSON.stringify({ primitives, formats, arrays, objects, completeUser }, null, 2)}

## Generating Actual Binary

To generate binary files:

\`\`\`javascript
const { schema } = require('compactr');

// Create schema
const userSchema = schema({
  type: 'object',
  properties: {
    id: { type: 'string', format: 'uuid' },
    name: { type: 'string' }
  }
});

// Encode
const buffer = userSchema.write({
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test'
});

// Save to file
fs.writeFileSync('user.bin', buffer);
\`\`\`

## Validation in Rust

\`\`\`rust
use compactr::{Encoder, SchemaType, Value};

let schema = SchemaType::object(properties);
let value = Value::Object(obj);

let mut encoder = Encoder::new();
encoder.encode(&value, &schema)?;
let rust_bytes = encoder.finish();

// Compare with JS output
let js_bytes = std::fs::read("user.bin")?;
assert_eq!(rust_bytes, js_bytes);
\`\`\`
`;

console.log('\nWriting documentation...');
fs.writeFileSync(
  path.join(__dirname, 'TEST_VECTORS.md'),
  documentation
);

console.log('\n✓ Test vectors documentation generated!');
console.log('\nTo generate actual binary files:');
console.log('1. Install compactr: npm install compactr');
console.log('2. Uncomment the require() line in this script');
console.log('3. Add code to actually encode and save binary files');
console.log('4. Run: node generate_test_vectors.js');
