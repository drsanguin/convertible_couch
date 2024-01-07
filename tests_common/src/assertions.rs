use convertible_couch_lib::display_settings::SwapPrimaryMonitorsResponse;

pub fn assert_that_primary_monitors_have_been_swap_as_expected(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
    expected_response: Result<SwapPrimaryMonitorsResponse, String>,
) {
    assert_eq!(
        actual_response, expected_response,
        "Primary monitors where not swapped as expected"
    )
}

pub fn assert_that_monitors_have_been_validated(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
    actual_monitors: &Vec<String>,
    expected_error_message_prefix: &str,
) {
    assert_eq!(
        actual_response,
        Err(format!(
            "{expected_error_message_prefix}, possible values are [{}]",
            actual_monitors
                .iter()
                .map(|monitor_name| monitor_name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    )
}
