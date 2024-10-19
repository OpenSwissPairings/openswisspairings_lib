//! # TRF Player Data fields
//!
//! Those are all the information contained in the player line of a TRF file. They are
//! contained inside of [`crate::trf::player::Section`].
//!
//! See also: [`crate::trf::player::round`].
use crate::trf::TRFError;

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
#[allow(clippy::upper_case_acronyms, missing_docs)] // F it
pub enum Title {
    GM,
    IM,
    WGM,
    FM,
    WIM,
    CM,
    WFM,
    WCM,
}

impl TryFrom<&str> for Title {
    type Error = TRFError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Parse weird format ('m', 'g')??

        Ok(match value {
            "GM" => Self::GM,
            "IM" => Self::IM,
            "WGM" => Self::WGM,
            "FM" => Self::FM,
            "WIM" => Self::WIM,
            "CM" => Self::CM,
            "WFM" => Self::WFM,
            "WCM" => Self::WCM,
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
