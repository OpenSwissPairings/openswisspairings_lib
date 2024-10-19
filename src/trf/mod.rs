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

use std::num::ParseIntError;

use thiserror::Error;

pub mod player;

/// An error in the TRF parser.
///
/// Most of the time those don't stop the parser, but they are stored inside of
/// [`Result`] in the struct instead.
#[derive(Error, Debug)]
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

    /// Error when the parser can't parse a number.
    #[error("Expected a valid number: {0}")]
    ParseIntError(#[from] ParseIntError),
}
