use convertible_couch::ApplicationResult;

pub fn assert_that_result_is_an_error_who_starts_with(
    actual_result: Result<ApplicationResult, String>,
    expected_error_message_prefix: &str,
) {
    assert!(
        actual_result
            .as_ref()
            .is_err_and(|error_message| error_message.starts_with(expected_error_message_prefix)),
        "  left: {:?}",
        actual_result
    );
}
