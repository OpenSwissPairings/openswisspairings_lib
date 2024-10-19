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
use std::error::Error;

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
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            "w" => Self::White,
            "b" => Self::Black,
            "-" | " " => Self::None,

            other => return Err(Box::from(format!("Invalid round color: {other}"))),
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
    type Error = Box<dyn Error>;

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

            other => return Err(Box::from(format!("Invalid round result: {other}"))),
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
#[derive(Debug, Copy, Clone)]
pub struct PlayerRoundSection {
    /// Player or forfeit id.
    id: Option<u16>,

    /// Scheduled color or forfeit.
    color: Option<Color>,

    /// Result.
    result: Option<Result>,
}

impl PlayerRoundSection {
    /// Get the player id or forfeit
    #[must_use]
    pub const fn id(&self) -> Option<u16> {
        self.id
    }

    /// Get the scheduled color or forfeit
    #[must_use]
    pub const fn color(&self) -> Option<Color> {
        self.color
    }

    /// Get the round result
    #[must_use]
    pub const fn result(&self) -> Option<Result> {
        self.result
    }
}

impl TryFrom<&str> for PlayerRoundSection {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.len() < 7 {
            return Err(Box::from(
                "At least 7 characters are required to parse a player round section",
            ));
        };

        Ok(Self {
            id: match value[0..4].trim() {
                "0000" | "" => None,
                other => other.parse::<u16>().ok(),
            },
            color: Color::try_from(&value[5..6]).ok(),
            result: Result::try_from(&value[7..8]).ok(),
        })
    }
}
