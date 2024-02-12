use convertible_couch_common::SwapPrimaryMonitorsResponse;

pub fn assert_that_primary_monitors_have_been_swap_as_expected(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
    expected_response: Result<SwapPrimaryMonitorsResponse, String>,
) {
    assert_eq!(
        actual_response, expected_response,
        "Primary monitors where not swapped as expected"
    );
}

pub fn assert_that_monitors_have_been_validated(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
    actual_monitors: &Vec<String>,
    expected_error_message_prefix: &str,
) {
    let possible_monitors = actual_monitors
        .iter()
        .map(|monitor_name| monitor_name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let expected_error_message =
        format!("{expected_error_message_prefix}, possible values are [{possible_monitors}]");

    let expected_response = Err(expected_error_message);

    assert_eq!(actual_response, expected_response);
}

pub fn assert_that_response_is_an_error_who_starts_with(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
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
