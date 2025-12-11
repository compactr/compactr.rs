//! Binary data format encoding and decoding.

use crate::codec::buffer::{binary_size, decode_binary as decode_bin, encode_binary as encode_bin};
use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BytesMut};

/// Encodes binary data with a 4-byte length prefix.
pub fn encode_binary(buf: &mut BytesMut, data: &[u8]) -> Result<(), EncodeError> {
    encode_bin(buf, data)
}

/// Decodes binary data from a buffer.
pub fn decode_binary(buf: &mut impl Buf) -> Result<Vec<u8>, DecodeError> {
    decode_bin(buf)
}

/// Returns the encoded size of binary data.
#[must_use]
pub fn size(data: &[u8]) -> usize {
    binary_size(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_roundtrip() {
        let mut buf = BytesMut::new();
        let data = vec![0, 1, 2, 255, 128, 64, 32];

        encode_binary(&mut buf, &data).unwrap();
        assert_eq!(buf.len(), size(&data));

        let decoded = decode_binary(&mut buf).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_empty_binary() {
        let mut buf = BytesMut::new();
        encode_binary(&mut buf, &[]).unwrap();

        let decoded = decode_binary(&mut buf).unwrap();
        assert!(decoded.is_empty());
    }
}
