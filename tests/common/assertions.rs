use convertible_couch::display_settings::SwapPrimaryMonitorsResponse;

pub fn assert_that_primary_monitors_have_been_swap_as_expected(
    actual_response: Result<SwapPrimaryMonitorsResponse, String>,
    expected_response: Result<SwapPrimaryMonitorsResponse, String>,
) {
    assert_eq!(
        actual_response, expected_response,
        "Primary monitors where not swapped as expected"
    )
}
