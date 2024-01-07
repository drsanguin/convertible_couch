use convertible_couch_lib::display_settings::{DisplaySettings, SwapPrimaryMonitorsResponse};
use convertible_couch_tests_common::{
    assertions::assert_that_primary_monitors_have_been_swap_as_expected, new_fuzzer,
};
use test_case::test_case;
use windows::Win32::Graphics::Gdi::{DISP_CHANGE, DISP_CHANGE_RESTART};

#[test]
fn it_should_swap_the_desktop_monitor_with_the_couch_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    // Assert
    assert_that_primary_monitors_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryMonitorsResponse {
            new_primary: Some(computer.secondary_monitor),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_couch_monitor_with_the_desktop_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor)
        .and_then(|_| {
            display_settings
                .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor)
        });

    // Assert
    assert_that_primary_monitors_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryMonitorsResponse {
            new_primary: Some(computer.primary_monitor),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_primary_monitors_of_computer_and_ask_for_reboot_when_required_after_committing_display_changes(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .for_which_committing_the_display_changes_fails_with(DISP_CHANGE_RESTART)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    // Assert
    assert_that_primary_monitors_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryMonitorsResponse {
            new_primary: Some(computer.secondary_monitor),
            reboot_required: true,
        }),
    );
}

#[test]
fn it_should_swap_the_primary_monitors_of_computer_and_ask_for_reboot_when_required_after_changing_display_for_some_monitors(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .for_which_changing_the_display_settings_fails_for_some_monitors(DISP_CHANGE_RESTART)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    // Assert
    assert_that_primary_monitors_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryMonitorsResponse {
            new_primary: Some(computer.secondary_monitor),
            reboot_required: true,
        }),
    );
}

#[test]
fn it_should_validate_the_desktop_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_desktop_monitor_name = fuzzer.generate_monitor_name();
    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&wrong_desktop_monitor_name, &computer.secondary_monitor);

    // Assert
    assert_eq!(
        actual_response,
        Err(format!(
            "Desktop monitor is invalid, possible values are [{}]",
            computer
                .monitors
                .iter()
                .map(|monitor_name| monitor_name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    );
}

#[test]
fn it_should_validate_the_couch_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_couch_monitor_name = fuzzer.generate_monitor_name();
    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &wrong_couch_monitor_name);

    // Assert
    assert_eq!(
        actual_response,
        Err(format!(
            "Couch monitor is invalid, possible values are [{}]",
            computer
                .monitors
                .iter()
                .map(|monitor_name| monitor_name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    );
}

#[test]
fn it_should_validate_both_desktop_and_couch_monitors() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_desktop_monitor_name = fuzzer.generate_monitor_name();
    let wrong_couch_monitor_name = fuzzer.generate_monitor_name();
    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&wrong_desktop_monitor_name, &wrong_couch_monitor_name);

    // Assert
    assert_eq!(
        actual_response,
        Err(format!(
            "Desktop and couch monitors are invalid, possible values are [{}]",
            computer
                .monitors
                .iter()
                .map(|monitor_name| monitor_name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    );
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => "The settings change was unsuccessful because the system is DualView capable."; "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => "An invalid set of flags was passed in."; "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => "The graphics mode is not supported."; "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => "An invalid parameter was passed in. This can include an invalid flag or combination of flags."; "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => "The display driver failed the specified graphics mode."; "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => "Unable to write settings to the registry."; "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_when_committing_changes(
    disp_change: DISP_CHANGE,
) -> String {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .for_which_committing_the_display_changes_fails_with(disp_change)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    actual_response.unwrap_err()
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => "The settings change was unsuccessful because the system is DualView capable."; "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => "An invalid set of flags was passed in."; "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => "The graphics mode is not supported."; "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => "An invalid parameter was passed in. This can include an invalid flag or combination of flags."; "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => "The display driver failed the specified graphics mode."; "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => "Unable to write settings to the registry."; "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_for_some_monitors(
    disp_change: DISP_CHANGE,
) -> String {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .for_which_changing_the_display_settings_fails_for_some_monitors(disp_change)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    actual_response.unwrap_err()
}
