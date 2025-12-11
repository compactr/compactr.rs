//! UUID format encoding and decoding (16 bytes compact).

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};
use uuid::Uuid;

/// Encodes a UUID in compact 16-byte format.
pub fn encode_uuid(buf: &mut BytesMut, uuid: &Uuid) -> Result<(), EncodeError> {
    buf.put_slice(uuid.as_bytes());
    Ok(())
}

/// Decodes a UUID from 16 bytes.
pub fn decode_uuid(buf: &mut impl Buf) -> Result<Uuid, DecodeError> {
    if buf.remaining() < 16 {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut bytes = [0u8; 16];
    buf.copy_to_slice(&mut bytes);

    Ok(Uuid::from_bytes(bytes))
}

/// Parses a UUID from a string and returns the UUID.
pub fn parse_uuid(s: &str) -> Result<Uuid, EncodeError> {
    Uuid::parse_str(s).map_err(|e| EncodeError::InvalidFormat(format!("Invalid UUID: {e}")))
}

/// Returns the encoded size of a UUID (always 16 bytes).
#[must_use]
pub const fn uuid_size() -> usize {
    16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_roundtrip() {
        let mut buf = BytesMut::new();
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        encode_uuid(&mut buf, &uuid).unwrap();
        assert_eq!(buf.len(), uuid_size());

        let decoded = decode_uuid(&mut buf).unwrap();
        assert_eq!(decoded, uuid);
    }

    #[test]
    fn test_parse_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let uuid = parse_uuid(uuid_str).unwrap();
        assert_eq!(uuid.to_string(), uuid_str);
    }

    #[test]
    fn test_parse_invalid_uuid() {
        let result = parse_uuid("not-a-uuid");
        assert!(result.is_err());
    }
}
