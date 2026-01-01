//! Buffer utilities for reading and writing encoded data.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};

/// Encodes a string into the buffer with UTF-8 encoding.
///
/// Format: 2-byte length (u16 BE) + UTF-8 encoded bytes
/// - Empty string: `0x00 0x00`
/// - Non-empty: 2-byte length + UTF-8 bytes (variable length per character)
///
/// # Errors
///
/// Returns an error if the string is too long to encode.
pub fn encode_string(buf: &mut BytesMut, s: &str) -> Result<(), EncodeError> {
    // UTF-8 encoding: Rust strings are already UTF-8
    let utf8_bytes = s.as_bytes();
    let byte_len = utf8_bytes.len();

    if byte_len > u16::MAX as usize {
        return Err(EncodeError::InvalidFormat(format!(
            "String too long: {} bytes (max {})",
            byte_len,
            u16::MAX
        )));
    }

    #[allow(clippy::cast_possible_truncation)]
    buf.put_u16(byte_len as u16);  // Big-endian length prefix

    buf.put_slice(utf8_bytes);  // Raw UTF-8 bytes

    Ok(())
}

/// Decodes a string from the buffer.
///
/// Expects: 2-byte length (u16 BE) + UTF-8 encoded bytes
/// - `0x00 0x00` → empty string
/// - Otherwise: 2-byte length + UTF-8 bytes
///
/// # Errors
///
/// Returns an error if:
/// - The buffer has insufficient data
/// - The data is not valid UTF-8
pub fn decode_string(buf: &mut impl Buf) -> Result<String, DecodeError> {
    if buf.remaining() < 2 {
        return Err(DecodeError::UnexpectedEof);
    }

    let len = buf.get_u16() as usize;  // Big-endian length prefix

    if len == 0 {
        return Ok(String::new());
    }

    if buf.remaining() < len {
        return Err(DecodeError::UnexpectedEof);
    }

    // Read raw UTF-8 bytes
    let mut bytes = vec![0u8; len];
    buf.copy_to_slice(&mut bytes);

    // Validate and convert UTF-8 to String
    String::from_utf8(bytes).map_err(|e|
        DecodeError::InvalidData(format!("Invalid UTF-8: {}", e))
    )
}

/// Encodes binary data into the buffer with a 4-byte length prefix.
///
/// Format: 4 bytes (u32 big-endian) length + raw bytes
///
/// # Errors
///
/// Returns an error if the binary data length exceeds `u32::MAX` bytes.
pub fn encode_binary(buf: &mut BytesMut, data: &[u8]) -> Result<(), EncodeError> {
    let len = data.len();

    if len > u32::MAX as usize {
        return Err(EncodeError::InvalidFormat(format!(
            "Binary data too long: {} bytes (max {})",
            len,
            u32::MAX
        )));
    }

    #[allow(clippy::cast_possible_truncation)]
    buf.put_u32(len as u32); // Big-endian
    buf.put_slice(data);
    Ok(())
}

/// Decodes binary data from the buffer.
///
/// Expects: 4 bytes (u32 big-endian) length + raw bytes
///
/// # Errors
///
/// Returns an error if the buffer has insufficient data.
pub fn decode_binary(buf: &mut impl Buf) -> Result<Vec<u8>, DecodeError> {
    if buf.remaining() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }

    let len = buf.get_u32() as usize; // Big-endian

    if buf.remaining() < len {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut bytes = vec![0u8; len];
    buf.copy_to_slice(&mut bytes);

    Ok(bytes)
}

/// Returns the encoded size of a string (2 byte length + UTF-8 bytes).
///
/// - Empty string: 2 bytes (0x00 0x00)
/// - Non-empty: 2 byte length + UTF-8 byte count
#[must_use]
pub fn string_size(s: &str) -> usize {
    2 + s.len()  // 2-byte prefix + UTF-8 bytes (s.len() returns byte count)
}

/// Returns the encoded size of binary data (4 bytes length + raw bytes).
#[must_use]
pub fn binary_size(data: &[u8]) -> usize {
    4 + data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_roundtrip() {
        let mut buf = BytesMut::new();
        let original = "Hello, World!";

        encode_string(&mut buf, original).unwrap();
        assert_eq!(buf.len(), string_size(original));

        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_empty_string() {
        let mut buf = BytesMut::new();
        encode_string(&mut buf, "").unwrap();

        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, "");
    }

    #[test]
    fn test_unicode_string() {
        let mut buf = BytesMut::new();
        let original = "こんにちは世界 🌍";

        encode_string(&mut buf, original).unwrap();
        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_binary_roundtrip() {
        let mut buf = BytesMut::new();
        let original = vec![1, 2, 3, 4, 5, 255, 128, 0];

        encode_binary(&mut buf, &original).unwrap();
        assert_eq!(buf.len(), binary_size(&original));

        let decoded = decode_binary(&mut buf).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_empty_binary() {
        let mut buf = BytesMut::new();
        encode_binary(&mut buf, &[]).unwrap();

        let decoded = decode_binary(&mut buf).unwrap();
        assert_eq!(decoded, Vec::<u8>::new());
    }
}
