use convertible_couch::ApplicationResult;

pub fn assert_that_response_is_an_error_who_starts_with(
    actual_response: Result<ApplicationResult, String>,
    expected_error_message_prefix: &str,
) {
    assert!(
        actual_response
            .as_ref()
            .is_err_and(|error_message| error_message.starts_with(expected_error_message_prefix)),
        "  left: {:?}",
        actual_response
    );
}
