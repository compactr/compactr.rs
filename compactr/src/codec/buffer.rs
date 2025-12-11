//! Buffer utilities for reading and writing encoded data.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};

/// Encodes a string into the buffer with a 2-byte length prefix.
///
/// Format: 2 bytes (u16 little-endian) length + UTF-8 bytes
pub fn encode_string(buf: &mut BytesMut, s: &str) -> Result<(), EncodeError> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len > u16::MAX as usize {
        return Err(EncodeError::InvalidFormat(format!(
            "String too long: {} bytes (max {})",
            len,
            u16::MAX
        )));
    }

    buf.put_u16_le(len as u16);
    buf.put_slice(bytes);
    Ok(())
}

/// Decodes a string from the buffer.
///
/// Expects: 2 bytes (u16 little-endian) length + UTF-8 bytes
pub fn decode_string(buf: &mut impl Buf) -> Result<String, DecodeError> {
    if buf.remaining() < 2 {
        return Err(DecodeError::UnexpectedEof);
    }

    let len = buf.get_u16_le() as usize;

    if buf.remaining() < len {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut bytes = vec![0u8; len];
    buf.copy_to_slice(&mut bytes);

    String::from_utf8(bytes).map_err(DecodeError::String)
}

/// Encodes binary data into the buffer with a 4-byte length prefix.
///
/// Format: 4 bytes (u32 little-endian) length + raw bytes
pub fn encode_binary(buf: &mut BytesMut, data: &[u8]) -> Result<(), EncodeError> {
    let len = data.len();

    if len > u32::MAX as usize {
        return Err(EncodeError::InvalidFormat(format!(
            "Binary data too long: {} bytes (max {})",
            len,
            u32::MAX
        )));
    }

    buf.put_u32_le(len as u32);
    buf.put_slice(data);
    Ok(())
}

/// Decodes binary data from the buffer.
///
/// Expects: 4 bytes (u32 little-endian) length + raw bytes
pub fn decode_binary(buf: &mut impl Buf) -> Result<Vec<u8>, DecodeError> {
    if buf.remaining() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }

    let len = buf.get_u32_le() as usize;

    if buf.remaining() < len {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut bytes = vec![0u8; len];
    buf.copy_to_slice(&mut bytes);

    Ok(bytes)
}

/// Returns the encoded size of a string (2 bytes length + UTF-8 bytes).
#[must_use]
pub fn string_size(s: &str) -> usize {
    2 + s.len()
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
