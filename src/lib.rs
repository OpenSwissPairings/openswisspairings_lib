//! # `OpenSwissPairings` Lib
//!
//! A set of utilities for chess pairing programs and tournament managers.
//!
//! ## Usage
//!
//! 1. Read a TRF file.
//! 2. TODO
//!
//! ### Reading TRF files
//!
//! Use [`Situation::try_from`] to read TRF files:
//!
//! ```
//! use std::fs;
//! # use openswisspairings_lib::Situation;
//!
//! # const FILE_PATH: &str = "test_file.trf";
//! // Read a TRF file:
//! let file_contents = fs::read_to_string(FILE_PATH).unwrap();
//!
//! // Load the Situation:
//! let situation = Situation::try_from(file_contents).unwrap();
//!
//! // Display a debug view of the Situation:
//! println!("{:#?}", situation);
//! ```
#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::indexing_slicing
)]
#![feature(iter_next_chunk)]

use std::{
    error::Error,
    str::{Chars, Split},
};

use trf::{player::Section, TRFError};

pub mod trf;

/// A situation represents a single tournament. It is the root object.
///
/// You can load a situation from a TRF file with [`Situation::try_from`]
#[derive(Debug)]
pub struct Situation {
    /// A vector containing all tournament players.
    players: Vec<Section>,
    // tournament: TournamentData,
    // teams: Vec<TeamData>,
}

impl Situation {
    /// Get a list of all tournament players.
    ///
    /// See [`field@Situation::players`].
    #[must_use]
    pub fn players(&self) -> &[Section] {
        &self.players
    }
}

impl TryFrom<String> for Situation {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split_char: char = if value.contains('\r') {
            '\r'
        } else {
            eprintln!("Using non-FIDE \\n line separator");
            '\n'
        };
        let lines: Split<'_, char> = value.split(split_char);

        let mut players: Vec<Section> = vec![];

        for line in lines {
            if line.len() > 4 {
                let mut chars: Chars<'_> = line.chars();
                let din = String::from_iter(
                    chars
                        .next_chunk::<3>()
                        .map_err(|_| TRFError::UnexpectedEndOfString())?,
                );
                chars.next(); // Drop space between DIN and rest of data
                let argument: String = chars.collect::<String>();

                println!(": {din} {argument}");

                match din.as_str() {
                    "001" => {
                        if let Ok(i) = Section::try_from(argument.clone()) {
                            players.push(i);
                        } else {
                            eprintln!("### Failed to read player data: {argument}");
                        }
                    } // (player data)
                    "012" => println!("Tournament Name"),
                    "022" => println!("City"),
                    "032" => println!("Federation"),
                    "042" => println!("Date of start"),
                    "052" => println!("Date of end"),
                    "062" => println!("Number of players"),
                    "072" => println!("Number of rated players"),
                    "082" => println!("Number of teams"),
                    "092" => println!("Type of tournament"),
                    "102" => println!("Chief Arbiter"),
                    "112" => println!("Deputy Chief Arbiter"),
                    "122" => println!("Alloted time per moves/game"),
                    "132" => println!("Dates of the round"),
                    "013" => println!("(team data)"),
                    "XXR" => println!("JaVaFo TRF(x)"),
                    "XRS" => println!(
                        "Unknown extension - Found in Tornelo TRF(x) files (supposed: Source)"
                    ),
                    "DAT" => println!("Tornelo TRF(x) Extension"),
                    _ => println!("{}", &line[0..3]),
                }
            }
        }

        Ok(Self { players })
    }
}
