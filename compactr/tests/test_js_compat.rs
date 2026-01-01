//! Test compatibility with compactr.js binary format

use compactr::{Encoder, Property, SchemaType, Value};
use indexmap::IndexMap;

#[test]
fn test_simple_int32_object() {
    // Create schema: {value: int32}
    let mut properties = IndexMap::new();
    properties.insert("value".to_owned(), Property::required(SchemaType::int32()));
    let schema = SchemaType::object(properties);

    // Create value: {value: 42}
    let mut obj = IndexMap::new();
    obj.insert("value".to_owned(), Value::Integer(42));
    let value = Value::Object(obj);

    // Encode
    let mut encoder = Encoder::new();
    encoder.encode(&value, &schema).unwrap();
    let bytes = encoder.finish();

    // Expected from compactr.js:
    // node -e "const {schema} = require('compactr'); const s = schema({value: {type: 'int32'}}); const buf = s.write({value: 42}).buffer(); console.log(Array.from(buf));"
    // Output: [ 1, 0, 4, 0, 0, 0, 42 ]

    let expected = vec![1, 0, 4, 0, 0, 0, 42];

    println!("Rust bytes: {:?}", bytes.as_ref());
    println!("Expected:   {:?}", expected);

    assert_eq!(bytes.as_ref(), expected.as_slice(),
        "Rust encoding should match compactr.js format");
}
