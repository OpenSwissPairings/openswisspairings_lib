//! # TRF Player Data fields
//!
//! Those are all the information contained in the player line of a TRF file. They are
//! contained inside of [`crate::trf::player::Section`].
//!
//! See also: [`crate::trf::player::round`].
use crate::trf::TRFError;

use super::utils::parse_number;

/// The player's gender.
#[derive(Debug)]
pub enum Sex {
    /// Man ("m")
    Man,

    /// Woman ("w")
    Woman,
}

/// The player's title.
///
/// Corresponds to official FIDE titles.
///
/// > Note: In most TRF files a different format seems to be used ("m", "g"???).
#[derive(Debug)]
#[allow(missing_docs)] // F it
pub enum Title {
    Grandmaster,
    InternationalMaster,
    WomanGrandmaster,
    FIDEMaster,
    WomanInternationalMaster,
    CandidateMaster,
    WomanFIDEMaster,
    WomanCandidateMaster,
}

impl TryFrom<&str> for Title {
    type Error = TRFError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Parse weird format ('m', 'g')??

        Ok(match value {
            "GM" => Self::Grandmaster,
            "IM" => Self::InternationalMaster,
            "WGM" => Self::WomanGrandmaster,
            "FM" => Self::FIDEMaster,
            "WIM" => Self::WomanInternationalMaster,
            "CM" => Self::CandidateMaster,
            "WFM" => Self::WomanFIDEMaster,
            "WCM" => Self::WomanCandidateMaster,
            other => return Err(TRFError::InvalidTitleError(other.to_string())),
        })
    }
}

/// The player's name.
///
/// Is split into last name and first name according to the reference.
///
/// > Note: In some TRF files from online chess websites, an username is used instead of a
/// > full name. In that case **the player's username will not be saved**.
#[derive(Debug)]
pub struct Name {
    /// The player's last name.
    last_name: String,

    /// The player's first name.
    first_name: String,
}

impl Name {
    /// Get the player's last name.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Get the player's first name.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
}

impl TryFrom<&str> for Name {
    type Error = TRFError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.split_once(", ").map_or_else(
            || Err(TRFError::InvalidPlayerNameError(value.to_string())),
            |i| {
                Ok(Self {
                    last_name: i.0.to_string(),
                    first_name: i.1.to_string(),
                })
            },
        )
    }
}

/// A date.
///
/// This is only used in [`crate::trf::player::Section::birth_date`].
///
/// This doesn't rely on an external library because:
/// 1. It would add yet another dependency
/// 2. The documentation is unclear about this field
#[derive(Debug)]
pub struct Date {
    /// Year.
    ///
    /// Format: YYYY (max 4 digits).
    year: u16,

    /// Month.
    ///
    /// Format: MM (max 2 digits).
    month: u8,

    /// Day.
    ///
    /// Format: DD (max 2 digits).
    day: u8,
}

impl Date {
    /// Get [`Date.year`](Date#structfield.year).
    #[must_use]
    pub const fn year(&self) -> u16 {
        self.year
    }

    /// Get [`Date.month`](Date#structfield.month).
    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Get [`Date.day`](Date#structfield.day).
    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }
}

impl TryFrom<&str> for Date {
    type Error = TRFError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('/').collect();

        if parts.len() != 3 {
            return Err(TRFError::InvalidDateError(value.to_string()));
        };

        Ok(Self {
            year: parse_number(
                parts
                    .first()
                    .ok_or_else(|| TRFError::InvalidDateError(value.to_string()))?,
            )
            .and_then(|option| {
                option.ok_or_else(|| TRFError::InvalidDateError(value.to_string()))
            })?,
            month: parse_number(
                parts
                    .get(1)
                    .ok_or_else(|| TRFError::InvalidDateError(value.to_string()))?,
            )
            .and_then(|option| {
                option.ok_or_else(|| TRFError::InvalidDateError(value.to_string()))
            })?,
            day: parse_number(
                parts
                    .get(2)
                    .ok_or_else(|| TRFError::InvalidDateError(value.to_string()))?,
            )
            .and_then(|option| {
                option.ok_or_else(|| TRFError::InvalidDateError(value.to_string()))
            })?,
        })
    }
}
