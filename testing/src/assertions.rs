use convertible_couch_lib::application_error::ApplicationError;

use convertible_couch::application::ApplicationResult;

pub fn assert_that_result_is_an_error_who_starts_with(
    actual_result: Result<ApplicationResult, ApplicationError>,
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
