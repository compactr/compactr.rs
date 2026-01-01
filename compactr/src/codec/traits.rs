//! Traits for encoding and decoding values.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};

/// Trait for types that can be encoded to binary format.
pub trait Encode {
    /// Encodes this value into the provided buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails.
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError>;

    /// Returns the size in bytes that this value will occupy when encoded.
    fn encoded_size(&self) -> usize;
}

/// Trait for types that can be decoded from binary format.
pub trait Decode: Sized {
    /// Decodes a value from the provided buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if decoding fails or the buffer doesn't contain valid data.
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError>;
}

// Implement for primitive types
impl Encode for bool {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(*self));
        Ok(())
    }

    fn encoded_size(&self) -> usize {
        1
    }
}

impl Decode for bool {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof);
        }
        let byte = buf.get_u8();
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidData(format!(
                "Invalid boolean value: {byte}"
            ))),
        }
    }
}

impl Encode for i32 {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_i32(*self); // Big-endian
        Ok(())
    }

    fn encoded_size(&self) -> usize {
        4
    }
}

impl Decode for i32 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(buf.get_i32()) // Big-endian
    }
}

impl Encode for i64 {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_i64(*self); // Big-endian
        Ok(())
    }

    fn encoded_size(&self) -> usize {
        8
    }
}

impl Decode for i64 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(buf.get_i64()) // Big-endian
    }
}

impl Encode for f32 {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_f32(*self); // Big-endian
        Ok(())
    }

    fn encoded_size(&self) -> usize {
        4
    }
}

impl Decode for f32 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(buf.get_f32()) // Big-endian
    }
}

impl Encode for f64 {
    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_f64(*self); // Big-endian
        Ok(())
    }

    fn encoded_size(&self) -> usize {
        8
    }
}

impl Decode for f64 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(buf.get_f64()) // Big-endian
    }
}
