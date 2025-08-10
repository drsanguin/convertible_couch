use std::{num::TryFromIntError, string::FromUtf16Error};

use ::log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors;
use thiserror::Error;

pub mod displays_settings;
pub mod log;
pub mod speakers_settings;
pub mod testing;

#[derive(Error, Debug, PartialEq)]
pub enum ApplicationError {
    #[error("{0}")]
    Custom(String),
}

impl From<String> for ApplicationError {
    fn from(value: String) -> Self {
        ApplicationError::Custom(value)
    }
}

impl From<ConfigErrors> for ApplicationError {
    fn from(value: ConfigErrors) -> Self {
        ApplicationError::Custom(value.to_string())
    }
}

impl From<SetLoggerError> for ApplicationError {
    fn from(value: SetLoggerError) -> Self {
        ApplicationError::Custom(value.to_string())
    }
}

impl From<FromUtf16Error> for ApplicationError {
    fn from(_: FromUtf16Error) -> Self {
        ApplicationError::Custom(String::from(
            "Failed to convert a String from a UTF-16 byte slice",
        ))
    }
}

impl From<TryFromIntError> for ApplicationError {
    fn from(_: TryFromIntError) -> Self {
        ApplicationError::Custom(String::from("Failed to convert an int"))
    }
}
