//! # TRF Player
//!
//! This module contains the player model for TRF files (DIN 001). It is used not only to
//! retrieve information about players but also for round results.
//!
//! ## Usage
//!
//! Most fields are [`core::result::Result<core::option::Option>`], where:
//! - [`Err`] means the field was incorrectly formatted in the file
//! - [`Ok`]\([`None`]) means the field was empty
//! - [`Ok`]\([`Some`]) contains data
//!
//! Currently no getters are available, however we do plan to add them.
use std::error::Error;

use fields::{Name, Sex, Title};
use round::PlayerRoundSection;

use super::TRFError;

pub mod fields;
pub mod round;
pub(crate) mod utils;

use utils::{parse_date, parse_int, parse_into};

/// Player section, stores all information about a player.
///
/// You can load this struct from a TRF file using [`crate::Situation::try_from`].
///
/// > You might think this is ugly
/// >
/// > Well it is
/// >
/// > Deal with it
///
/// ###### TODO
///
/// - [ ] Getters
/// - [ ] Setters
/// - [ ] Export to TRF
#[derive(Debug)]
#[allow(dead_code)]
pub struct Section {
    /// Player starting rank number.
    ///
    /// Serves as a unique ID assigned to each player for each [`crate::Situation`].
    starting_rank_number: Result<u16, TRFError>,

    /// Player gender.
    ///
    /// See [`Sex`].
    sex: Result<Option<Sex>, TRFError>,

    /// Player title.
    ///
    /// See [`Title`].
    title: Result<Option<Title>, TRFError>,

    /// Player name.
    ///
    /// Split into last name and first name according to the TRF file reference, see
    /// [`Name`].
    name: Result<Option<Name>, TRFError>,

    /// Player FIDE rating.
    fide_rating: Result<Option<u16>, TRFError>,

    /// FIDE Federation of player
    ///
    /// This isn't a [`Result`] because we just take the raw string from the
    /// file and don't parse it. This value is [`None`] if the string is empty.
    fide_federation: Option<String>,

    /// Player FIDE number.
    ///
    /// This is a unique ID identifiying the player in FIDE's database, however it isn't
    /// always present, rely on [`Section::starting_rank_number`] instead. You should only
    /// use this with FIDE services.
    fide_number: Result<Option<u64>, TRFError>,

    /// Player birth date.
    ///
    /// TODO: Currently we rely on the [`icu_calendar`] library, however it is a pretty
    /// big dependency, so we might change later. We also assume the date is using
    /// [`icu_calendar::Iso`], but this isn't specified in the reference.
    ///
    /// TODO: We use [`regex`] to parse the date, however this is also a big dependency
    /// and we might remove it later since it's slightly overkill. However since it is an
    /// official Rust library we might keep it.
    ///
    /// TODO: Use [`Result<Option>`]
    birth_date: Option<icu_calendar::Date<icu_calendar::Iso>>,

    /// Player total points.
    ///
    /// This is the number of points in the tournament standings, which depends on the
    /// scoring points system used and on the value of the pairing-allocated bye (usually
    /// the same as a win). If, for instance, the 3/1/0 scoring point system is applied in
    /// a tournament and a player scored 5 wins, 2 draws and 2 losses, this field should
    /// contain "17.0".
    ///
    /// TODO: Use [`Result<Option>`]
    points: Option<f32>,

    /// Player final ranking.
    ///
    /// Exact definition, especially for Team.
    ///
    /// > I don't know what any of this means...
    ///
    /// TODO: Use [`Result<Option>`]
    rank: Option<u16>,

    /// Information about each round played by the player in the tournament.
    ///
    /// TODO: Use [`Result<Option>`]
    rounds: Vec<Option<PlayerRoundSection>>,
}

impl Section {
    /// The data identification number of a player section.
    pub const DATA_IDENTIFICATION_NUMBER: &str = "001";
}

impl TryFrom<String> for Section {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // The first four characters (Data Identification Number and space)
        // shouldn't be included in 'value'
        //
        // Range: [a-5, b-4]
        println!("> {}", &value[0..4]);

        let mut rounds: Vec<Option<PlayerRoundSection>> = vec![];
        let mut n: usize = 87;
        while value.len() >= n + 7 {
            rounds.push(PlayerRoundSection::try_from(&value[n..n + 8]).ok());
            n += 10;
        }

        Ok(Self {
            starting_rank_number: value[0..4].trim().parse::<u16>().map_err(Into::into),
            sex: match &value[5..6] {
                "m" => Ok(Some(Sex::Man)),
                "w" => Ok(Some(Sex::Woman)),
                " " => Ok(None),
                other => Err(TRFError::InvalidGenderError(other.to_string())),
            },
            title: parse_into(&value[6..9]),
            name: parse_into(&value[10..43]),
            fide_rating: parse_int(&value[44..48]),
            fide_federation: Some(value[49..52].trim().to_string())
                .filter(|s| !s.is_empty()),
            fide_number: parse_int(&value[53..64]),
            birth_date: parse_date(&value[65..75]), // [65..75]
            points: value[76..80].trim().parse::<f32>().ok(),
            rank: value[81..85].trim().parse::<u16>().ok(),
            rounds,
        })
    }
}