//! IP address format encoding and decoding.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Encodes an IPv4 address (4 bytes).
pub fn encode_ipv4(buf: &mut BytesMut, addr: &Ipv4Addr) -> Result<(), EncodeError> {
    buf.put_slice(&addr.octets());
    Ok(())
}

/// Decodes an IPv4 address from 4 bytes.
pub fn decode_ipv4(buf: &mut impl Buf) -> Result<Ipv4Addr, DecodeError> {
    if buf.remaining() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut octets = [0u8; 4];
    buf.copy_to_slice(&mut octets);

    Ok(Ipv4Addr::from(octets))
}

/// Encodes an IPv6 address (16 bytes).
pub fn encode_ipv6(buf: &mut BytesMut, addr: &Ipv6Addr) -> Result<(), EncodeError> {
    buf.put_slice(&addr.octets());
    Ok(())
}

/// Decodes an IPv6 address from 16 bytes.
pub fn decode_ipv6(buf: &mut impl Buf) -> Result<Ipv6Addr, DecodeError> {
    if buf.remaining() < 16 {
        return Err(DecodeError::UnexpectedEof);
    }

    let mut octets = [0u8; 16];
    buf.copy_to_slice(&mut octets);

    Ok(Ipv6Addr::from(octets))
}

/// Parses an IPv4 address from a string.
pub fn parse_ipv4(s: &str) -> Result<Ipv4Addr, EncodeError> {
    s.parse::<Ipv4Addr>()
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid IPv4 address: {e}")))
}

/// Parses an IPv6 address from a string.
pub fn parse_ipv6(s: &str) -> Result<Ipv6Addr, EncodeError> {
    s.parse::<Ipv6Addr>()
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid IPv6 address: {e}")))
}

/// Returns the encoded size of an IPv4 address (always 4 bytes).
#[must_use]
pub const fn ipv4_size() -> usize {
    4
}

/// Returns the encoded size of an IPv6 address (always 16 bytes).
#[must_use]
pub const fn ipv6_size() -> usize {
    16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_roundtrip() {
        let mut buf = BytesMut::new();
        let addr = Ipv4Addr::new(192, 168, 1, 1);

        encode_ipv4(&mut buf, &addr).unwrap();
        assert_eq!(buf.len(), ipv4_size());

        let decoded = decode_ipv4(&mut buf).unwrap();
        assert_eq!(decoded, addr);
    }

    #[test]
    fn test_parse_ipv4() {
        let addr_str = "192.168.1.1";
        let addr = parse_ipv4(addr_str).unwrap();
        assert_eq!(addr.to_string(), addr_str);
    }

    #[test]
    fn test_ipv6_roundtrip() {
        let mut buf = BytesMut::new();
        let addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);

        encode_ipv6(&mut buf, &addr).unwrap();
        assert_eq!(buf.len(), ipv6_size());

        let decoded = decode_ipv6(&mut buf).unwrap();
        assert_eq!(decoded, addr);
    }

    #[test]
    fn test_parse_ipv6() {
        let addr_str = "2001:db8::1";
        let addr = parse_ipv6(addr_str).unwrap();
        assert_eq!(addr, Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1));
    }

    #[test]
    fn test_ipv6_localhost() {
        let mut buf = BytesMut::new();
        let addr = Ipv6Addr::LOCALHOST;

        encode_ipv6(&mut buf, &addr).unwrap();
        let decoded = decode_ipv6(&mut buf).unwrap();
        assert_eq!(decoded, addr);
    }
}
