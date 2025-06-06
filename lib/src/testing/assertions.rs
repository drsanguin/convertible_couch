use crate::display_settings::DisplaySettingsResult;

pub fn assert_that_primary_display_have_been_changed_as_expected(
    actual_response: Result<DisplaySettingsResult, String>,
    expected_response: Result<DisplaySettingsResult, String>,
) {
    assert_eq!(
        actual_response, expected_response,
        "Primary display was not changed as expected"
    );
}

pub fn assert_that_displays_have_been_validated(
    actual_response: Result<DisplaySettingsResult, String>,
    actual_displays: &Vec<String>,
    expected_error_message_prefix: &str,
) {
    let possible_displays = actual_displays
        .iter()
        .map(|display_name| display_name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let expected_error_message =
        format!("{expected_error_message_prefix}, possible values are [{possible_displays}]");

    let expected_response = Err(expected_error_message);

    assert_eq!(actual_response, expected_response);
}

pub fn assert_that_response_is_an_error_who_starts_with(
    actual_response: Result<DisplaySettingsResult, String>,
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
