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
    // In compactr 3.x, write() returns a Buffer directly
    const buffer = s.write(value);
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
  { value: { type: 'int64' } },
  { value: 0 },
  'Object with int64 = 0');

generateFixture('int64_42',
  { value: { type: 'int64' } },
  { value: 42 },
  'Object with int64 = 42');

generateFixture('int64_max_safe',
  { value: { type: 'int64' } },
  { value: 9007199254740991 }, // Max safe integer in JS
  'Object with int64::MAX_SAFE');

generateFixture('float_0',
  { value: { type: 'float' } },
  { value: 0.0 },
  'Object with float = 0.0');

generateFixture('float_1',
  { value: { type: 'float' } },
  { value: 1.0 },
  'Object with float = 1.0');

generateFixture('float_pi',
  { value: { type: 'float' } },
  { value: 3.14 },
  'Object with float = 3.14');

generateFixture('double_0',
  { value: { type: 'double' } },
  { value: 0.0 },
  'Object with double = 0.0');

generateFixture('double_1',
  { value: { type: 'double' } },
  { value: 1.0 },
  'Object with double = 1.0');

generateFixture('double_pi',
  { value: { type: 'double' } },
  { value: Math.PI },
  'Object with double = Math.PI');

console.log('\nStrings:');
generateFixture('string_empty',
  { value: { type: 'string' } },
  { value: '' },
  'Object with empty string');

generateFixture('string_a',
  { value: { type: 'string' } },
  { value: 'A' },
  'Object with string "A"');

generateFixture('string_hello',
  { value: { type: 'string' } },
  { value: 'Hello' },
  'Object with string "Hello"');

generateFixture('string_unicode',
  { value: { type: 'string' } },
  { value: 'Hello 🦀' },
  'Object with Unicode string');

// 2. Special Formats
console.log('\nSpecial Formats:');
generateFixture('uuid_standard',
  { value: { type: 'uuid' } },
  { value: '550e8400-e29b-41d4-a716-446655440000' },
  'Object with standard UUID');

generateFixture('uuid_zeroes',
  { value: { type: 'uuid' } },
  { value: '00000000-0000-0000-0000-000000000000' },
  'Object with zero UUID');

generateFixture('datetime_epoch',
  { value: { type: 'date-time' } },
  { value: '1970-01-01T00:00:00.000Z' },
  'Object with epoch datetime');

generateFixture('datetime_2021',
  { value: { type: 'date-time' } },
  { value: '2021-01-01T00:00:00.000Z' },
  'Object with datetime 2021');

generateFixture('datetime_2024',
  { value: { type: 'date-time' } },
  { value: '2024-01-15T10:30:00.000Z' },
  'Object with datetime 2024');

generateFixture('date_epoch',
  { value: { type: 'date' } },
  { value: '1970-01-01' },
  'Object with epoch date');

generateFixture('date_2021',
  { value: { type: 'date' } },
  { value: '2021-01-01' },
  'Object with date 2021');

generateFixture('ipv4_localhost',
  { value: { type: 'ipv4' } },
  { value: '127.0.0.1' },
  'Object with IPv4 localhost');

generateFixture('ipv4_192_168_1_1',
  { value: { type: 'ipv4' } },
  { value: '192.168.1.1' },
  'Object with IPv4 192.168.1.1');

generateFixture('ipv6_localhost',
  { value: { type: 'ipv6' } },
  { value: '::1' },
  'Object with IPv6 localhost');

generateFixture('ipv6_standard',
  { value: { type: 'ipv6' } },
  { value: '2001:db8::1' },
  'Object with IPv6 standard');

generateFixture('binary_empty',
  { value: { type: 'binary' } },
  { value: Buffer.from([]).toString('base64') },
  'Object with empty binary');

generateFixture('binary_small',
  { value: { type: 'binary' } },
  { value: Buffer.from([1, 2, 3]).toString('base64') },
  'Object with small binary');

// 3. Arrays
console.log('\nArrays:');
generateFixture('array_empty',
  { value: { type: 'array', items: { type: 'int32' } } },
  { value: [] },
  'Object with empty array');

generateFixture('array_int32_1_2_3',
  { value: { type: 'array', items: { type: 'int32' } } },
  { value: [1, 2, 3] },
  'Object with int32 array [1,2,3]');

generateFixture('array_strings',
  { value: { type: 'array', items: { type: 'string' } } },
  { value: ['a', 'b', 'c'] },
  'Object with string array');

// 4. Objects
console.log('\nObjects:');
generateFixture('object_simple',
  {
    value: { type: 'int32' }
  },
  { value: 42 },
  'Single int32 property');

generateFixture('object_x_y',
  {
    x: { type: 'int32' },
    y: { type: 'int32' }
  },
  { x: 10, y: 20 },
  'Two int32 properties in schema order');

// Test that property order in VALUE doesn't matter
generateFixture('object_y_x_value_order',
  {
    x: { type: 'int32' },
    y: { type: 'int32' }
  },
  { y: 20, x: 10 }, // Reversed order in value
  'Same as object_x_y (schema order wins)');

generateFixture('object_with_optional',
  {
    id: { type: 'int32' },
    name: { type: 'string', optional: true }
  },
  { id: 1, name: 'Alice' },
  'Required + optional field present');

generateFixture('object_optional_missing',
  {
    id: { type: 'int32' },
    name: { type: 'string', optional: true }
  },
  { id: 1 },
  'Required + optional field missing');

// 5. Nested Objects
console.log('\nNested Structures:');
generateFixture('nested_simple',
  {
    user: {
      type: 'object',
      schema: {
        name: { type: 'string' },
        age: { type: 'int32' }
      }
    }
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
    id: { type: 'uuid' },
    name: { type: 'string' },
    email: { type: 'string' },
    age: { type: 'int32' },
    created_at: { type: 'date-time' }
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
