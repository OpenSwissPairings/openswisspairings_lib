//! Utility functions for reading and parsing TRF data types.
//!
//! All functions in this module return a [`core::result::Result<core::option::Option>`]
//! for use in structs.
//!
//! Those functions aren't meant to be used directly and are only used by the TRF file
//! parser.
//!
//! See [`crate::trf::player#Usage`].
use std::str::FromStr;

/// Parse an integer from a TRF file.
pub fn parse_int<T: FromStr, E: From<<T as FromStr>::Err>>(
    value: &str,
) -> Result<Option<T>, E> {
    match value.trim() {
        "" => Ok(None),
        other => other.parse::<T>().map(Some).map_err(From::from),
    }
}

/// Parse a value from a TRF file using [`core::convert::TryInto`].
pub fn parse_into<T, E>(value: &str) -> Result<Option<T>, E>
where
    T: for<'a> TryFrom<&'a str, Error = E>,
{
    match value.trim() {
        "" => Ok(None),
        other => other.try_into().map(Some).map_err(From::from),
    }
}

/// Parse a date from a TRF file using the [`icu_calendar`] library.
///
/// This might change in the future since `icu` is a big dependency. [`regex`] is also
/// used for parsing the string.
pub fn parse_date(value: &str) -> Option<icu_calendar::Date<icu_calendar::Iso>> {
    let re: regex::Regex =
        regex::Regex::new("(?<year>\\d{4})/(?<month>\\d{2})/(?<day>\\d{2})").ok()?;
    let caps: regex::Captures<'_> = re.captures(value)?;
    let year: i32 = if let Ok(i) = caps["year"].parse::<i32>() {
        i
    } else {
        return None;
    };
    let month: u8 = if let Ok(i) = caps["month"].parse::<u8>() {
        i
    } else {
        return None;
    };
    let day: u8 = if let Ok(i) = caps["day"].parse::<u8>() {
        i
    } else {
        return None;
    };
    icu_calendar::Date::try_new_iso_date(year, month, day).ok()
}
