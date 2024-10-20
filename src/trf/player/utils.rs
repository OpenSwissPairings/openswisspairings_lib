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

/// Parse a number from a TRF file.
pub fn parse_number<T: FromStr, E: From<<T as FromStr>::Err>>(
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
