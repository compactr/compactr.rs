//! `DateTime` and `Date` format encoding and decoding.

use crate::error::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, BytesMut};
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc};

/// Encodes a `DateTime` in compactr.js format: 9 bytes (year, month, day, hour, minute, second, milliseconds).
///
/// Format:
/// - 2 bytes: year (u16 big-endian)
/// - 1 byte: month (1-12)
/// - 1 byte: day (1-31)
/// - 1 byte: hour (0-23)
/// - 1 byte: minute (0-59)
/// - 1 byte: second (0-59)
/// - 2 bytes: milliseconds (0-999, u16 big-endian)
///
/// # Errors
///
/// Returns an error if the datetime components are out of valid ranges.
pub fn encode_datetime(buf: &mut BytesMut, dt: &DateTime<Utc>) -> Result<(), EncodeError> {
    let year = dt.year();
    if !(0..=65535).contains(&year) {
        return Err(EncodeError::InvalidFormat(format!(
            "Year out of range: {year}"
        )));
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    buf.put_u16(year as u16); // Big-endian

    #[allow(clippy::cast_possible_truncation)]
    {
        buf.put_u8(dt.month() as u8);
        buf.put_u8(dt.day() as u8);
        buf.put_u8(dt.hour() as u8);
        buf.put_u8(dt.minute() as u8);
        buf.put_u8(dt.second() as u8);

        // Milliseconds from nanoseconds
        let millis = dt.timestamp_subsec_millis();
        buf.put_u16(millis as u16); // Big-endian
    }

    Ok(())
}

/// Decodes a `DateTime` from compactr.js format (9 bytes).
///
/// # Errors
///
/// Returns an error if:
/// - The buffer has insufficient data
/// - The datetime components are invalid
pub fn decode_datetime(buf: &mut impl Buf) -> Result<DateTime<Utc>, DecodeError> {
    if buf.remaining() < 9 {
        return Err(DecodeError::UnexpectedEof);
    }

    let year = i32::from(buf.get_u16()); // Big-endian
    let month = u32::from(buf.get_u8());
    let day = u32::from(buf.get_u8());
    let hour = u32::from(buf.get_u8());
    let minute = u32::from(buf.get_u8());
    let second = u32::from(buf.get_u8());
    let millis = u32::from(buf.get_u16()); // Big-endian

    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .and_then(|dt| dt.checked_add_signed(chrono::Duration::milliseconds(i64::from(millis))))
        .ok_or_else(|| {
            DecodeError::InvalidData(format!(
                "Invalid datetime: {year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}.{millis:03}"
            ))
        })
}

/// Encodes a `Date` as days since Unix epoch (4 bytes, i32 big-endian).
///
/// # Errors
///
/// Returns an error if:
/// - The epoch date cannot be created
/// - The date is out of the representable range (beyond ±`i32::MAX` days from epoch)
pub fn encode_date(buf: &mut BytesMut, date: &NaiveDate) -> Result<(), EncodeError> {
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
        .ok_or_else(|| EncodeError::InvalidFormat("Failed to create epoch date".to_owned()))?;

    let days = date.signed_duration_since(epoch).num_days();

    if days < i64::from(i32::MIN) || days > i64::from(i32::MAX) {
        return Err(EncodeError::InvalidFormat(format!(
            "Date out of range: {days} days from epoch"
        )));
    }

    #[allow(clippy::cast_possible_truncation)]
    buf.put_i32(days as i32); // Big-endian
    Ok(())
}

/// Decodes a `Date` from days since Unix epoch.
///
/// # Errors
///
/// Returns an error if:
/// - The buffer has insufficient data
/// - The epoch date cannot be created
/// - The date offset is invalid or out of range
pub fn decode_date(buf: &mut impl Buf) -> Result<NaiveDate, DecodeError> {
    if buf.remaining() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }

    let days = buf.get_i32(); // Big-endian
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
        .ok_or_else(|| DecodeError::InvalidData("Failed to create epoch date".to_owned()))?;

    epoch
        .checked_add_days(chrono::Days::new(u64::from(days.unsigned_abs())))
        .or_else(|| epoch.checked_sub_days(chrono::Days::new(u64::from((-days).unsigned_abs()))))
        .ok_or_else(|| DecodeError::InvalidData(format!("Invalid date offset: {days} days")))
}

/// Parses a `DateTime` from an ISO 8601 string.
///
/// # Errors
///
/// Returns an error if the string is not a valid ISO 8601 datetime.
pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>, EncodeError> {
    s.parse::<DateTime<Utc>>()
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid datetime: {e}")))
}

/// Parses a `Date` from an ISO 8601 date string (YYYY-MM-DD).
///
/// # Errors
///
/// Returns an error if the string is not a valid date in YYYY-MM-DD format.
pub fn parse_date(s: &str) -> Result<NaiveDate, EncodeError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| EncodeError::InvalidFormat(format!("Invalid date: {e}")))
}

/// Returns the encoded size of a `DateTime` (always 9 bytes).
#[must_use]
pub const fn datetime_size() -> usize {
    9
}

/// Returns the encoded size of a `Date` (always 4 bytes).
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
