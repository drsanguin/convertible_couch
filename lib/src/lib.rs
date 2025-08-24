use std::{num::TryFromIntError, string::FromUtf16Error};

use ::log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors;
use thiserror::Error;

pub mod displays_settings;
pub mod log;
pub mod speakers_settings;
#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl From<ConfigErrors> for ApplicationError {
    fn from(value: ConfigErrors) -> Self {
        ApplicationError::Custom(value.to_string())
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::ApplicationError;

    #[test]
    fn it_should_be_converted_from_a_string() {
        // Arrange
        let initial_error = String::from("Something wrong happened");

        // Act
        let error = ApplicationError::from(initial_error.clone());

        // Assert
        assert_eq!(error, ApplicationError::Custom(initial_error));
    }

    #[test]
    fn it_should_be_converted_from_a_from_utf16_error() {
        // Arrange
        let bad_data = vec![0xD800];
        let initial_error = String::from_utf16(&bad_data).unwrap_err();

        // Act
        let error = ApplicationError::from(initial_error);

        // Assert
        assert_eq!(
            error,
            ApplicationError::Custom(String::from(
                "Failed to convert a String from a UTF-16 byte slice"
            ))
        );
    }

    #[test]
    fn it_should_be_converted_from_a_try_from_int_error() {
        // Arrange
        let bad_data: i32 = 256;
        let initial_error = u8::try_from(bad_data).unwrap_err();

        // Act
        let error = ApplicationError::from(initial_error);

        // Assert
        assert_eq!(
            error,
            ApplicationError::Custom(String::from("Failed to convert an int"))
        );
    }
}
