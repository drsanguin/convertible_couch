use convertible_couch_lib::{
    application_error::ApplicationError, application_result::ApplicationResult,
};

use convertible_couch::application::CommandResult;
use windows::Win32::Foundation::WIN32_ERROR;

pub fn assert_that_result_is_an_error_who_starts_with(
    actual_result: ApplicationResult<CommandResult>,
    expected_error_message_prefix: &str,
) {
    assert!(
        actual_result.as_ref().is_err_and(|error| match error {
            ApplicationError::Custom(message) => message.starts_with(expected_error_message_prefix),
        }),
        "  left: {:?}",
        actual_result
    );
}

pub fn assert_that_result_is_a_win32_error(
    actual_result: ApplicationResult<CommandResult>,
    expected_win32_error: WIN32_ERROR,
) {
    let expected = expected_win32_error.to_hresult().to_string();

    assert!(
        actual_result.as_ref().is_err_and(|error| match error {
            ApplicationError::Custom(message) => message.contains(&expected),
        }),
        " expected {:?} to be an error message containing {:?}",
        actual_result,
        expected
    );
}
