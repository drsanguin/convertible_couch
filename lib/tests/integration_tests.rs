use std::collections::HashSet;

use convertible_couch_common::SwapPrimaryDisplaysResponse;
use convertible_couch_common_tests::{
    assertions::{
        assert_that_displays_have_been_validated,
        assert_that_primary_displays_have_been_swap_as_expected,
        assert_that_response_is_an_error_who_starts_with,
    },
    fuzzing::win32::FuzzedWin32,
    new_fuzzer,
};
use convertible_couch_lib::display_settings::DisplaySettings;
use test_case::test_case;
use windows::Win32::Graphics::Gdi::{DISP_CHANGE, DISP_CHANGE_RESTART};

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(computer.secondary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display)
        .and_then(|_| {
            display_settings
                .swap_primary_displays(&computer.primary_display, &computer.secondary_display)
        });

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(computer.primary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display_when_the_computer_has_an_internal_display(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings.swap_primary_displays(
        DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
        &computer.secondary_display,
    );

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(computer.secondary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display_has_an_internal_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(
            DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
            &computer.secondary_display,
        )
        .and_then(|_| {
            display_settings.swap_primary_displays(
                DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
                &computer.secondary_display,
            )
        });

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(String::from(
                DisplaySettings::<FuzzedWin32>::INTERNAL_DISPLAY,
            )),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_primary_displays_of_computer_and_ask_for_reboot_when_required_after_committing_display_changes(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_committing_the_display_changes_fails_with(DISP_CHANGE_RESTART)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(computer.secondary_display),
            reboot_required: true,
        }),
    );
}

#[test]
fn it_should_swap_the_primary_displays_of_computer_and_ask_for_reboot_when_required_after_changing_display_for_some_displays(
) {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_changing_the_display_settings_fails_for_some_displays(DISP_CHANGE_RESTART)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_primary_displays_have_been_swap_as_expected(
        actual_response,
        Ok(SwapPrimaryDisplaysResponse {
            new_primary: Some(computer.secondary_display),
            reboot_required: true,
        }),
    );
}

#[test]
fn it_should_validate_the_desktop_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_desktop_display_name = fuzzer.generate_display_name();
    let mut forbidden_display_names = HashSet::with_capacity(1);
    forbidden_display_names.insert(wrong_desktop_display_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_names_are_different_from(forbidden_display_names)
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&wrong_desktop_display_name, &computer.secondary_display);

    // Assert
    assert_that_displays_have_been_validated(
        actual_response,
        &computer.displays,
        "Desktop display is invalid",
    );
}

#[test]
fn it_should_validate_the_couch_display() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_couch_display_name = fuzzer.generate_display_name();
    let mut forbidden_display_names = HashSet::with_capacity(1);
    forbidden_display_names.insert(wrong_couch_display_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_names_are_different_from(forbidden_display_names)
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &wrong_couch_display_name);

    // Assert
    assert_that_displays_have_been_validated(
        actual_response,
        &computer.displays,
        "Couch display is invalid",
    );
}

#[test]
fn it_should_validate_both_desktop_and_couch_displays() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let (wrong_desktop_display_name, wrong_couch_display_name) =
        fuzzer.generate_two_display_names();

    let mut forbidden_display_names = HashSet::with_capacity(2);
    forbidden_display_names.insert(wrong_desktop_display_name.as_str());
    forbidden_display_names.insert(wrong_couch_display_name.as_str());

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_names_are_different_from(forbidden_display_names)
        .build_displays()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&wrong_desktop_display_name, &wrong_couch_display_name);

    // Assert
    assert_that_displays_have_been_validated(
        actual_response,
        &computer.displays,
        "Desktop and couch displays are invalid",
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
) -> Result<SwapPrimaryDisplaysResponse, String> {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_committing_the_display_changes_fails_with(disp_change)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    display_settings.swap_primary_displays(&computer.primary_display, &computer.secondary_display)
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_for_some_displays(
    disp_change: DISP_CHANGE,
) -> Result<SwapPrimaryDisplaysResponse, String> {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_changing_the_display_settings_fails_for_some_displays(disp_change)
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    display_settings.swap_primary_displays(&computer.primary_display, &computer.secondary_display)
}

#[test]
fn it_should_handle_the_case_when_it_fails_to_get_the_primary_display_name() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_getting_the_primary_display_fails()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve display configuration information about the device",
    );
}

#[test]
fn it_should_handle_the_case_when_querying_the_display_config_of_the_primary_display_fails() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_querying_the_display_config_of_the_primary_display_fails()
        .build_computer();

    let mut display_settings = DisplaySettings::new(computer.win32);

    // Act
    let actual_response = display_settings
        .swap_primary_displays(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve the name of the display at the device path",
    );
}
