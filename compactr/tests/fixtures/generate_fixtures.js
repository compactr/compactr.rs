#!/usr/bin/env node
/**
 * Generate binary test fixtures using compactr.js
 *
 * This script generates binary files that the Rust tests can validate against
 * to ensure binary format compatibility.
 */

const fs = require('fs');
const path = require('path');

let schema;
try {
  ({ schema } = require('compactr'));
  console.log('✓ compactr.js loaded successfully\n');
} catch (err) {
  console.error('✗ Failed to load compactr.js');
  console.error('  Run: npm install compactr');
  process.exit(1);
}

const fixturesDir = __dirname;

// Ensure fixtures directory exists
if (!fs.existsSync(fixturesDir)) {
  fs.mkdirSync(fixturesDir, { recursive: true });
}

console.log('Generating binary test fixtures...\n');

// Track generated fixtures
const fixtures = [];

function generateFixture(name, schemaObj, value, description) {
  try {
    const s = schema(schemaObj);
    const encoded = s.write(value);
    // Get the full buffer (header + content)
    const buffer = encoded.buffer();
    const filename = `${name}.bin`;
    const filepath = path.join(fixturesDir, filename);

    fs.writeFileSync(filepath, buffer);

    fixtures.push({
      name,
      filename,
      size: buffer.length,
      hex: buffer.toString('hex'),
      description
    });

    console.log(`✓ ${filename.padEnd(30)} ${buffer.length} bytes`);
  } catch (err) {
    console.error(`✗ Failed to generate ${name}: ${err.message}`);
  }
}

// 1. Primitives (wrapped in objects as compactr.js requires)
console.log('Primitives:');
generateFixture('boolean_true',
  { value: { type: 'boolean' } },
  { value: true },
  'Object with boolean true');

generateFixture('boolean_false',
  { value: { type: 'boolean' } },
  { value: false },
  'Object with boolean false');

generateFixture('int32_0',
  { value: { type: 'int32' } },
  { value: 0 },
  'Object with int32 = 0');

generateFixture('int32_42',
  { value: { type: 'int32' } },
  { value: 42 },
  'Object with int32 = 42');

generateFixture('int32_negative',
  { value: { type: 'int32' } },
  { value: -1 },
  'Object with int32 = -1');

generateFixture('int32_max',
  { value: { type: 'int32' } },
  { value: 2147483647 }, // i32::MAX
  'Object with int32::MAX');

generateFixture('int64_0',
  { type: 'integer', format: 'int64' },
  0,
  '8 bytes little-endian');

generateFixture('int64_42',
  { type: 'integer', format: 'int64' },
  42,
  '8 bytes little-endian');

generateFixture('int64_max_safe',
  { type: 'integer', format: 'int64' },
  9007199254740991, // Max safe integer in JS
  '8 bytes little-endian');

generateFixture('float_0',
  { type: 'number', format: 'float' },
  0.0,
  '4 bytes IEEE 754 little-endian');

generateFixture('float_1',
  { type: 'number', format: 'float' },
  1.0,
  '4 bytes IEEE 754 little-endian');

generateFixture('float_pi',
  { type: 'number', format: 'float' },
  3.14,
  '4 bytes IEEE 754 little-endian');

generateFixture('double_0',
  { type: 'number', format: 'double' },
  0.0,
  '8 bytes IEEE 754 little-endian');

generateFixture('double_1',
  { type: 'number', format: 'double' },
  1.0,
  '8 bytes IEEE 754 little-endian');

generateFixture('double_pi',
  { type: 'number', format: 'double' },
  Math.PI,
  '8 bytes IEEE 754 little-endian');

console.log('\nStrings:');
generateFixture('string_empty',
  { type: 'string' },
  '',
  '2 bytes length (0)');

generateFixture('string_a',
  { type: 'string' },
  'A',
  '2 bytes length + 1 byte');

generateFixture('string_hello',
  { type: 'string' },
  'Hello',
  '2 bytes length + 5 bytes');

generateFixture('string_unicode',
  { type: 'string' },
  'Hello 🦀',
  '2 bytes length + UTF-8 bytes');

// 2. Special Formats
console.log('\nSpecial Formats:');
generateFixture('uuid_standard',
  { type: 'string', format: 'uuid' },
  '550e8400-e29b-41d4-a716-446655440000',
  '16 bytes raw UUID');

generateFixture('uuid_zeroes',
  { type: 'string', format: 'uuid' },
  '00000000-0000-0000-0000-000000000000',
  '16 bytes raw UUID');

generateFixture('datetime_epoch',
  { type: 'string', format: 'date-time' },
  '1970-01-01T00:00:00.000Z',
  '8 bytes i64 milliseconds from epoch');

generateFixture('datetime_2021',
  { type: 'string', format: 'date-time' },
  '2021-01-01T00:00:00.000Z',
  '8 bytes i64 milliseconds from epoch');

generateFixture('datetime_2024',
  { type: 'string', format: 'date-time' },
  '2024-01-15T10:30:00.000Z',
  '8 bytes i64 milliseconds from epoch');

generateFixture('date_epoch',
  { type: 'string', format: 'date' },
  '1970-01-01',
  '4 bytes i32 days from epoch');

generateFixture('date_2021',
  { type: 'string', format: 'date' },
  '2021-01-01',
  '4 bytes i32 days from epoch');

generateFixture('ipv4_localhost',
  { type: 'string', format: 'ipv4' },
  '127.0.0.1',
  '4 bytes');

generateFixture('ipv4_192_168_1_1',
  { type: 'string', format: 'ipv4' },
  '192.168.1.1',
  '4 bytes');

generateFixture('ipv6_localhost',
  { type: 'string', format: 'ipv6' },
  '::1',
  '16 bytes');

generateFixture('ipv6_standard',
  { type: 'string', format: 'ipv6' },
  '2001:db8::1',
  '16 bytes');

generateFixture('binary_empty',
  { type: 'string', format: 'binary' },
  Buffer.from([]).toString('base64'),
  '4 bytes length (0)');

generateFixture('binary_small',
  { type: 'string', format: 'binary' },
  Buffer.from([1, 2, 3]).toString('base64'),
  '4 bytes length + 3 bytes data');

// 3. Arrays
console.log('\nArrays:');
generateFixture('array_empty',
  { type: 'array', items: { type: 'integer', format: 'int32' } },
  [],
  '4 bytes length (0)');

generateFixture('array_int32_1_2_3',
  { type: 'array', items: { type: 'integer', format: 'int32' } },
  [1, 2, 3],
  '4 bytes length + (3 × 4 bytes)');

generateFixture('array_strings',
  { type: 'array', items: { type: 'string' } },
  ['a', 'b', 'c'],
  '4 bytes length + strings');

// 4. Objects
console.log('\nObjects:');
generateFixture('object_simple',
  {
    type: 'object',
    properties: {
      value: { type: 'integer', format: 'int32' }
    },
    required: ['value']
  },
  { value: 42 },
  'Single int32 property');

generateFixture('object_x_y',
  {
    type: 'object',
    properties: {
      x: { type: 'integer', format: 'int32' },
      y: { type: 'integer', format: 'int32' }
    },
    required: ['x', 'y']
  },
  { x: 10, y: 20 },
  'Two int32 properties in schema order');

// Test that property order in VALUE doesn't matter
generateFixture('object_y_x_value_order',
  {
    type: 'object',
    properties: {
      x: { type: 'integer', format: 'int32' },
      y: { type: 'integer', format: 'int32' }
    },
    required: ['x', 'y']
  },
  { y: 20, x: 10 }, // Reversed order in value
  'Same as object_x_y (schema order wins)');

generateFixture('object_with_optional',
  {
    type: 'object',
    properties: {
      id: { type: 'integer', format: 'int32' },
      name: { type: 'string' }
    },
    required: ['id']
  },
  { id: 1, name: 'Alice' },
  'Required + optional field present');

generateFixture('object_optional_missing',
  {
    type: 'object',
    properties: {
      id: { type: 'integer', format: 'int32' },
      name: { type: 'string' }
    },
    required: ['id']
  },
  { id: 1 },
  'Required + optional field missing (null)');

// 5. Nested Objects
console.log('\nNested Structures:');
generateFixture('nested_simple',
  {
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
  {
    user: {
      name: 'Bob',
      age: 25
    }
  },
  'Nested object');

// 6. Complete User Object (complex example)
console.log('\nComplex Examples:');
generateFixture('user_complete',
  {
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
  {
    id: '550e8400-e29b-41d4-a716-446655440000',
    name: 'Alice Johnson',
    email: 'alice@example.com',
    age: 28,
    created_at: '2024-01-15T10:30:00.000Z'
  },
  'Complete user with all field types');

// Generate manifest
console.log('\nGenerating manifest...');
const manifest = {
  generated_at: new Date().toISOString(),
  compactr_version: require('compactr/package.json').version,
  node_version: process.version,
  fixture_count: fixtures.length,
  fixtures: fixtures
};

fs.writeFileSync(
  path.join(fixturesDir, 'manifest.json'),
  JSON.stringify(manifest, null, 2)
);

console.log(`✓ manifest.json\n`);
console.log(`Generated ${fixtures.length} fixtures successfully!`);
console.log(`\nRun Rust tests with: cargo test --test cross_compatibility`);
