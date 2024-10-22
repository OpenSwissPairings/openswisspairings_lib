//! # TRF files reader
//!
//! This is a set of tools to read TRF - Tournament Report Files.
//!
//! It is based on the official reference: C.04.A.10 Annex-2 TRF16  
//! <https://www.fide.com/FIDE/handbook/C04Annex2_TRF16.pdf>
//!
//! ## Status
//!
//! Currently only reading player data is supported.
//!
//! ## Usage
//!
//! Do not use this directly. Instead, use [`crate::Situation`].

use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

pub mod player;

/// An error in the TRF parser.
///
/// Most of the time those don't stop the parser, but they are stored inside of
/// [`Result`] in the struct instead.
#[derive(Error, Debug, Clone)]
pub enum TRFError {
    /// Error when the gender is invalid (neither "m" nor "w" nor empty).
    #[error("Expected a valid gender, found string {0}")]
    InvalidGenderError(String),

    /// Error when the title is invalid (not a standard FIDE title).
    #[error("Expected a valid title, found string {0}")]
    InvalidTitleError(String),

    /// Error when the name is incorectly formatted (should be Lastname, Firstname).
    #[error("Expected a string of format 'Lastname, Firstname', found string {0}")]
    InvalidPlayerNameError(String),

    /// Error when the birth date is invalid (not in YYYY/MM/DD format).
    #[error("Expected a valid date, found string {0}")]
    InvalidDateError(String),

    /// Error when the player round color is invalid (neither "w" nor "b" nor "-" nor
    /// empty).
    #[error("Expected a valid color, found string {0}")]
    InvalidColorError(String),

    /// Error when a round result is invalid.
    ///
    /// See [`crate::trf::player::round::Result`].
    #[error("Expected a valid round result, found string {0}")]
    InvalidRoundResultError(String),

    /// Error when a player's round section isn't the right length.
    #[error("A player round section must be 7 characters long, found string {0}")]
    PlayerRoundSectionTooShort(String),

    /// Error when the parser can't parse an integer.
    #[error("Expected a valid integer: {0}")]
    ParseIntError(#[from] ParseIntError),

    /// Error when the parser can't parse a floating point number.
    #[error("Expected a valid floating point number: {0}")]
    ParseFloatError(#[from] ParseFloatError),

    /// Error when we somehow ran into the end of a string after checking it was long
    /// enough
    #[error("Unexpectedly ran into the end of a string")]
    UnexpectedEndOfString(),
}
