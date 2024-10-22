//! # TRF Player Round Result
//!
//! This module contains the [`PlayerRoundSection`] struct representing a single round's
//! result for a player. See [`crate::trf::player::Section`].
//!
//! ## Usage
//!
//! This struct isn't meant to be instanced directly. See [`crate::Situation::try_from`]
//! instead.
//!
//! Getter methods are available to retrieve information. Currently no setter methods are
//! available.
use crate::trf::TRFError;

use super::utils::{parse_into, parse_number};

/// Scheduled color or forfeit in round.
#[derive(Debug, Copy, Clone)]
pub enum Color {
    /// White side.
    White,

    /// Black side.
    Black,

    /// If the player had a bye or was not paired.
    None,
}

impl TryFrom<&str> for Color {
    type Error = TRFError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            "w" => Self::White,
            "b" => Self::Black,
            "-" | " " => Self::None,

            other => return Err(TRFError::InvalidColorError(other.to_string())),
        })
    }
}

/// Result of round
#[derive(Debug, Copy, Clone)]
pub enum Result {
    /// The scheduled game was not played
    NotPlayed(NotPlayedRoundResult),

    /// The scheduled game lasted less than one move
    LessThanOneMove(LessThanOneMoveRoundResult),

    /// Regular game
    Regular(RegularRoundResult),

    /// Bye
    Bye(ByeRoundResult),
}

impl TryFrom<&str> for Result {
    type Error = TRFError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value.to_uppercase().as_str() {
            "-" => Self::NotPlayed(NotPlayedRoundResult::ForfeitLoss),
            "+" => Self::NotPlayed(NotPlayedRoundResult::ForfeitWin),

            "W" => Self::LessThanOneMove(LessThanOneMoveRoundResult::Win),
            "D" => Self::LessThanOneMove(LessThanOneMoveRoundResult::Draw),
            "L" => Self::LessThanOneMove(LessThanOneMoveRoundResult::Loss),

            "1" => Self::Regular(RegularRoundResult::Win),
            "=" => Self::Regular(RegularRoundResult::Draw),
            "0" => Self::Regular(RegularRoundResult::Loss),

            "H" => Self::Bye(ByeRoundResult::HalfPointBye),
            "F" => Self::Bye(ByeRoundResult::FullPointBye),
            "U" => Self::Bye(ByeRoundResult::PairingAllocatedBye),
            "Z" | " " => Self::Bye(ByeRoundResult::ZeroPointBye),

            other => return Err(TRFError::InvalidRoundResultError(other.to_string())),
        })
    }
}

impl From<Result> for String {
    fn from(val: Result) -> Self {
        match val {
            Result::NotPlayed(np) => np.into(),
            Result::LessThanOneMove(ltom) => ltom.into(),
            Result::Regular(r) => r.into(),
            Result::Bye(b) => b.into(),
        }
    }
}

/// The scheduled game was not played
#[derive(Debug, Copy, Clone)]
pub enum NotPlayedRoundResult {
    /// Forfeit loss
    ForfeitLoss,

    /// Forfeit win
    ForfeitWin,
}

impl From<NotPlayedRoundResult> for String {
    fn from(val: NotPlayedRoundResult) -> Self {
        match val {
            NotPlayedRoundResult::ForfeitLoss => "-",
            NotPlayedRoundResult::ForfeitWin => "+",
        }
        .to_string()
    }
}

/// The scheduled game lasted less than one move
///
/// Not rated
#[derive(Debug, Copy, Clone)]
pub enum LessThanOneMoveRoundResult {
    /// Win
    ///
    /// Not rated
    Win,

    /// Draw
    ///
    /// Not rated
    Draw,

    /// Loss
    ///
    /// Not rated
    Loss,
}

impl From<LessThanOneMoveRoundResult> for String {
    fn from(val: LessThanOneMoveRoundResult) -> Self {
        match val {
            LessThanOneMoveRoundResult::Win => "W",
            LessThanOneMoveRoundResult::Draw => "D",
            LessThanOneMoveRoundResult::Loss => "L",
        }
        .to_string()
    }
}

/// Regular game
#[derive(Debug, Copy, Clone)]
pub enum RegularRoundResult {
    /// Win
    Win,

    /// Draw
    Draw,

    /// Loss
    Loss,
}

impl From<RegularRoundResult> for String {
    fn from(val: RegularRoundResult) -> Self {
        match val {
            RegularRoundResult::Win => "1",
            RegularRoundResult::Draw => "=",
            RegularRoundResult::Loss => "0",
        }
        .to_string()
    }
}

/// Bye
#[derive(Debug, Copy, Clone)]
pub enum ByeRoundResult {
    /// Half-point-bye
    ///
    /// Not rated
    HalfPointBye,

    /// Full-point-bye
    ///
    /// Not rated
    FullPointBye,

    /// Pairing-allocated bye
    ///
    /// At most once for round
    /// Not rated
    ///
    /// (U for player unpaired by the system)
    PairingAllocatedBye,

    /// Zero-point-bye
    ///
    /// Known absence from round
    /// Not rated
    ZeroPointBye,
}

impl From<ByeRoundResult> for String {
    fn from(val: ByeRoundResult) -> Self {
        match val {
            ByeRoundResult::HalfPointBye => "H",
            ByeRoundResult::FullPointBye => "F",
            ByeRoundResult::PairingAllocatedBye => "U", // U for player unpaired by the system
            ByeRoundResult::ZeroPointBye => "Z",        // Equivalent to (blank)
        }
        .to_string()
    }
}

/// A player's single round result.
///
/// > **TODO:** Use Result
#[derive(Debug)]
pub struct PlayerRoundSection {
    /// Player or forfeit id.
    id: std::result::Result<Option<u16>, TRFError>,

    /// Scheduled color or forfeit.
    color: std::result::Result<Option<Color>, TRFError>,

    /// Result.
    result: std::result::Result<Option<Result>, TRFError>,
}

impl PlayerRoundSection {
    /// Get the player or forfeit id.
    ///
    /// # Errors
    ///
    /// This field is [`Err`] if the parsed value isn't a valid number.
    pub const fn id(&self) -> std::result::Result<&Option<u16>, &TRFError> {
        self.id.as_ref()
    }

    /// Get the scheduled color or forfeit.
    ///
    /// # Errors
    ///
    /// This field is [`Err`] if the parsed value isn't a valid color. See [`Color`].
    pub const fn color(&self) -> std::result::Result<&Option<Color>, &TRFError> {
        self.color.as_ref()
    }

    /// Get the round result.
    ///
    /// # Errors
    ///
    /// This field is [`Err`] if the parsed value isn't a valid result. See [`Result`].
    pub const fn result(&self) -> std::result::Result<&Option<Result>, &TRFError> {
        self.result.as_ref()
    }
}

impl TryFrom<&str> for PlayerRoundSection {
    type Error = TRFError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.len() < 7 {
            return Err(TRFError::PlayerRoundSectionTooShort(value.to_string()));
        };

        Ok(Self {
            id: match value[0..4].trim() {
                "0000" | "" => Ok(None),
                other => parse_number(other),
            },
            color: parse_into(&value[5..6]),
            result: parse_into(&value[7..8]),
        })
    }
}
