//! DateTime and Date format encoding and decoding.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

/// Encodes a DateTime as Unix timestamp in milliseconds (8 bytes, i64 little-endian).
pub fn encode_datetime(buf: &mut BytesMut, dt: &DateTime<Utc>) -> Result<(), EncodeError> {
    let timestamp_ms = dt.timestamp_millis();
    buf.put_i64_le(timestamp_ms);
    Ok(())
}

/// Decodes a DateTime from Unix timestamp in milliseconds.
pub fn decode_datetime(buf: &mut impl Buf) -> Result<DateTime<Utc>, DecodeError> {
    if buf.remaining() < 8 {
        return Err(DecodeError::UnexpectedEof);
    }

    let timestamp_ms = buf.get_i64_le();
    Utc.timestamp_millis_opt(timestamp_ms)
        .single()
        .ok_or_else(|| DecodeError::InvalidData(format!("Invalid timestamp: {timestamp_ms}")))
}

/// Encodes a Date as days since Unix epoch (4 bytes, i32 little-endian).
pub fn encode_date(buf: &mut BytesMut, date: &NaiveDate) -> Result<(), EncodeError> {
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
        .ok_or_else(|| EncodeError::InvalidFormat("Failed to create epoch date".to_owned()))?;

    let days = date.signed_duration_since(epoch).num_days();

    if days < i32::MIN as i64 || days > i32::MAX as i64 {
        return Err(EncodeError::InvalidFormat(format!(
            "Date out of range: {days} days from epoch"
        )));
    }

    buf.put_i32_le(days as i32);
    Ok(())
}

/// Decodes a Date from days since Unix epoch.
pub fn decode_date(buf: &mut impl Buf) -> Result<NaiveDate, DecodeError> {
    if buf.remaining() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }

    let days = buf.get_i32_le();
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
        .ok_or_else(|| DecodeError::InvalidData("Failed to create epoch date".to_owned()))?;

    epoch
        .checked_add_days(chrono::Days::new(days.unsigned_abs() as u64))
        .or_else(|| epoch.checked_sub_days(chrono::Days::new((-days).unsigned_abs() as u64)))
        .ok_or_else(|| DecodeError::InvalidData(format!("Invalid date offset: {days} days")))
}

/// Parses a DateTime from an ISO 8601 string.
pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>, EncodeError> {
    s.parse::<DateTime<Utc>>()
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid datetime: {e}")))
}

/// Parses a Date from an ISO 8601 date string (YYYY-MM-DD).
pub fn parse_date(s: &str) -> Result<NaiveDate, EncodeError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid date: {e}")))
}

/// Returns the encoded size of a DateTime (always 8 bytes).
#[must_use]
pub const fn datetime_size() -> usize {
    8
}

/// Returns the encoded size of a Date (always 4 bytes).
#[must_use]
pub const fn date_size() -> usize {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_roundtrip() {
        let mut buf = BytesMut::new();
        let dt = Utc::now();

        encode_datetime(&mut buf, &dt).unwrap();
        assert_eq!(buf.len(), datetime_size());

        let decoded = decode_datetime(&mut buf).unwrap();
        // Compare timestamps to avoid subsecond precision issues
        assert_eq!(decoded.timestamp_millis(), dt.timestamp_millis());
    }

    #[test]
    fn test_parse_datetime() {
        let dt_str = "2024-01-15T10:30:00Z";
        let dt = parse_datetime(dt_str).unwrap();
        assert_eq!(dt.to_rfc3339(), "2024-01-15T10:30:00+00:00");
    }

    #[test]
    fn test_date_roundtrip() {
        let mut buf = BytesMut::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        encode_date(&mut buf, &date).unwrap();
        assert_eq!(buf.len(), date_size());

        let decoded = decode_date(&mut buf).unwrap();
        assert_eq!(decoded, date);
    }

    #[test]
    fn test_epoch_date() {
        let mut buf = BytesMut::new();
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        encode_date(&mut buf, &epoch).unwrap();
        let decoded = decode_date(&mut buf).unwrap();
        assert_eq!(decoded, epoch);
    }

    #[test]
    fn test_parse_date() {
        let date_str = "2024-01-15";
        let date = parse_date(date_str).unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    }
}
