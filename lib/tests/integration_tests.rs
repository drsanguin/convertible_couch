use std::collections::HashSet;

use convertible_couch_common::SwapPrimaryMonitorsResponse;
use convertible_couch_lib::display_settings::DisplaySettings;
use convertible_couch_tests_common::{
    assertions::{
        assert_that_monitors_have_been_validated,
        assert_that_primary_monitors_have_been_swap_as_expected,
        assert_that_response_is_an_error_who_starts_with,
    },
    fuzzing::win32::FuzzedWin32,
    new_fuzzer,
};
use test_case::test_case;
use windows::Win32::Graphics::Gdi::{DISP_CHANGE, DISP_CHANGE_RESTART};

#[test]
fn it_should_swap_the_desktop_monitor_with_the_couch_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more()
        .build();

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
        .generate_computer()
        .with_two_monitors_or_more()
        .build();

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
fn it_should_swap_the_desktop_monitor_with_the_couch_monitor_when_the_computer_has_an_internal_display(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_an_internal_display_and_at_least_one_more_monitor()
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings.swap_primary_monitors(
        DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
        &computer.secondary_monitor,
    );

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
fn it_should_swap_the_couch_monitor_with_the_desktop_monitor_has_an_internal_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_an_internal_display_and_at_least_one_more_monitor()
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(
            DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
            &computer.secondary_monitor,
        )
        .and_then(|_| {
            display_settings.swap_primary_monitors(
                DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
                &computer.secondary_monitor,
            )
        });

    // Assert
    assert_that_primary_monitors_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryMonitorsResponse {
            new_primary: Some(String::from(
                DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
            )),
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
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_committing_the_display_changes_fails_with(DISP_CHANGE_RESTART)
        .build();

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
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_changing_the_display_settings_fails_for_some_monitors(DISP_CHANGE_RESTART)
        .build();

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
    let mut forbidden_monitor_names = HashSet::with_capacity(1);
    forbidden_monitor_names.insert(wrong_desktop_monitor_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more_with_names_different_than(&forbidden_monitor_names)
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&wrong_desktop_monitor_name, &computer.secondary_monitor);

    // Assert
    assert_that_monitors_have_been_validated(
        actual_response,
        &computer.monitors,
        "Desktop monitor is invalid",
    );
}

#[test]
fn it_should_validate_the_couch_monitor() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_couch_monitor_name = fuzzer.generate_monitor_name();
    let mut forbidden_monitor_names = HashSet::with_capacity(1);
    forbidden_monitor_names.insert(wrong_couch_monitor_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more_with_names_different_than(&forbidden_monitor_names)
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &wrong_couch_monitor_name);

    // Assert
    assert_that_monitors_have_been_validated(
        actual_response,
        &computer.monitors,
        "Couch monitor is invalid",
    );
}

#[test]
fn it_should_validate_both_desktop_and_couch_monitors() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let (wrong_desktop_monitor_name, wrong_couch_monitor_name) =
        fuzzer.generate_two_monitor_names();

    let mut forbidden_monitor_names = HashSet::with_capacity(2);
    forbidden_monitor_names.insert(wrong_desktop_monitor_name.as_str());
    forbidden_monitor_names.insert(wrong_couch_monitor_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more_with_names_different_than(&forbidden_monitor_names)
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&wrong_desktop_monitor_name, &wrong_couch_monitor_name);

    // Assert
    assert_that_monitors_have_been_validated(
        actual_response,
        &computer.monitors,
        "Desktop and couch monitors are invalid",
    );
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_when_committing_changes(
    disp_change: DISP_CHANGE,
) -> Result<SwapPrimaryMonitorsResponse, String> {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_committing_the_display_changes_fails_with(disp_change)
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    display_settings.swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor)
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_for_some_monitors(
    disp_change: DISP_CHANGE,
) -> Result<SwapPrimaryMonitorsResponse, String> {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_changing_the_display_settings_fails_for_some_monitors(disp_change)
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    display_settings.swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor)
}

#[test]
fn it_should_handle_the_case_when_it_fails_to_get_the_primary_monitor_name() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_getting_the_primary_monitor_fails()
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve display configuration information about the device",
    );
}

#[test]
fn it_should_handle_the_case_when_querying_the_display_config_of_the_primary_monitor_fails() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_two_monitors_or_more()
        .for_which_querying_the_display_config_of_the_primary_monitor_fails()
        .build();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve the name of the monitor at the device path",
    );
}
