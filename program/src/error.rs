//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    program_error::{PrintProgramError, ProgramError},
};

use num_traits::FromPrimitive;
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Error {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// Already in use
    #[error("Already in use")]
    AlreadyInUse,
    /// Not rent exempt
    #[error("Not rent exempt")]
    NotRentExempt,
    /// Not found aggregator
    #[error("Not found aggregator")]
    NotFoundAggregator,
    /// Oracle exist
    #[error("Oracle exist")]
    OracleExist,
    /// Owner mismatch
    #[error("Owner mismatch")]
    OwnerMismatch,
    /// Not found oracle
    #[error("Not found oracle")]
    NotFoundOracle,
    /// Not found oracle
    #[error("Submission value out of range")]
    SubmissonValueOutOfRange,
    /// Submit cooling
    #[error("Submission cooling")]
    SubmissonCooling,
    /// Insufficient withdrawable
    #[error("Insufficient withdrawable")]
    InsufficientWithdrawable,
    /// Aggregator key not match
    #[error("Aggregator key not match")]
    AggregatorMismatch,

    #[error("Invalid round id")]
    InvalidRoundID,

    #[error("Cannot start new round until cooldown")]
    OracleNewRoundCooldown,

    #[error("Max number of submissions reached for this round")]
    MaxSubmissionsReached,

    #[error("Each oracle may only submit once per round")]
    OracleAlreadySubmitted,

    #[error("Unknown error")]
    UnknownError,
}

// impl PrintProgramError for Error {
//     fn print<E>(&self)
//     where
//         E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
//     {
//         match self {
//             Error::InvalidInstruction => msg!("Error: Invalid instruction"),
//             Error::AlreadyInUse => msg!("Error: Already in use"),
//             Error::NotRentExempt => msg!("Error: No rent exempt"),
//             Error::NotFoundAggregator => msg!("Error: no found aggregator"),
//             Error::OracleExist => msg!("Error: Oracle exist"),
//             Error::OwnerMismatch => msg!("Error: Owner mismatch"),
//             Error::NotFoundOracle => msg!("Error: Not found oracle"),
//             Error::SubmissonValueOutOfRange => msg!("Error: Submisson value out of range"),
//             Error::SubmissonCooling => msg!("Submission cooling"),
//             Error::InsufficientWithdrawable => msg!("Insufficient withdrawable"),
//             Error::AggregatorMismatch => msg!("Aggregator key not match"),
//             Error::MaxOralcesReached => msg!("Max oracles reached"),
//         }
//     }
// }

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl From<ProgramError> for Error {
    fn from(err: ProgramError) -> Self {
        match err {
            ProgramError::Custom(code) => Error::from_u32(code).unwrap_or(Error::UnknownError),
            _ => Error::UnknownError,
        }
    }
}
