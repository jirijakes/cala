use chrono::ParseError;
use thiserror::Error;

use crate::cel_type::*;

#[derive(Error, Debug)]
pub enum ResultCoersionError {
    #[error("Error evaluating expression '{0}' - Could not coerce {1:?} into {2:?}")]
    BadCoreTypeCoersion(String, CelType, CelType),
    #[error("Error evaluating expression '{0}' - Could not coerce {1:?} into {2:?}")]
    BadExternalTypeCoersion(String, CelType, &'static str),
    #[error("Error evaluating expression '{0}' - Could not coerce {1:?} into {2:?} - Reason: {3}")]
    ExternalTypeCoersionError(String, String, &'static str, String),
}

#[derive(Error, Debug)]
pub enum CelError {
    #[error("CelError - CelParseError: {0}")]
    CelParseError(String),
    #[error("CelError - BadType: expected {0:?} found {1:?}")]
    BadType(CelType, CelType),
    #[error("CelError - UnknownIdentifier: {0}")]
    UnknownIdent(String),
    #[error("CelError - IllegalTarget")]
    IllegalTarget,
    #[error("CelError - MissingArgument")]
    MissingArgument,
    #[error("CelError - WrongArgumentType: {0:?} instead of {1:?}")]
    WrongArgumentType(CelType, CelType),
    #[error("CelError - ChronoParseError: {0}")]
    ChronoParseError(#[from] ParseError),
    #[error("CelError - UuidError: {0}")]
    UuidError(String),
    #[error("CelError - DecimalError: {0}")]
    DecimalError(String),
    #[error("CelError - NoMatchingOverload: {0}")]
    NoMatchingOverload(String),
    #[error("CelError - Unexpected: {0}")]
    Unexpected(String),

    #[error("CelError - {0}")]
    ResultCoersionError(#[from] ResultCoersionError),

    #[error("Error evaluating cell expression '{0}' - {1}")]
    EvaluationError(String, Box<Self>),
}
