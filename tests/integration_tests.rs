use crate::common::{assertions::assert_that_expected_monitor_has_been_set, fuzzing::Fuzzer};
use common::assertions::{
    fail_because_an_error_occured, fail_because_primary_monitor_has_not_changed,
};
use convertible_couch::display_settings::DisplaySettings;

mod common;

#[test]
fn it_should_swap_the_primary_monitors_of_computer() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let display_settings =
        DisplaySettings::new(computer.win32_devices_display, computer.win32_graphics_gdi);

    unsafe {
        // Act
        let result = display_settings
            .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

        // Assert
        match result {
            Ok(response) => match response.new_primary {
                Some(actual_primary) => {
                    assert_that_expected_monitor_has_been_set(
                        &actual_primary,
                        &computer.secondary_monitor,
                    );

                    assert_eq!(
                        response.reboot_required, false,
                        "Reboot has been required whereas it was not expected"
                    )
                }
                None => fail_because_primary_monitor_has_not_changed(&computer.secondary_monitor),
            },
            Err(reason) => fail_because_an_error_occured(reason, &computer.secondary_monitor),
        }
    }
}

#[test]
fn it_should_swap_the_primary_monitors_of_computer_and_ask_for_reboot_when_required() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .which_requires_reboot()
        .build_computer();

    let display_settings =
        DisplaySettings::new(computer.win32_devices_display, computer.win32_graphics_gdi);

    unsafe {
        // Act
        let result = display_settings
            .swap_primary_monitors(&computer.primary_monitor, &computer.secondary_monitor);

        // Assert
        match result {
            Ok(response) => match response.new_primary {
                Some(actual_primary) => {
                    assert_that_expected_monitor_has_been_set(
                        &actual_primary,
                        &computer.secondary_monitor,
                    );

                    assert_eq!(
                        response.reboot_required, true,
                        "Reboot has not been required whereas it was expected"
                    )
                }
                None => fail_because_primary_monitor_has_not_changed(&computer.secondary_monitor),
            },
            Err(reason) => fail_because_an_error_occured(reason, &computer.secondary_monitor),
        }
    }
}

#[test]
fn it_should_validate_monitors() {
    // Arrange
    let mut fuzzer = new_fuzzer!();

    let wrong_desktop_monitor_name = fuzzer.generate_monitor_name();
    let wrong_couch_monitor_name = fuzzer.generate_monitor_name();
    let computer = fuzzer
        .generate_a_computer()
        .with_two_monitors_or_more()
        .build_computer();

    let display_settings =
        DisplaySettings::new(computer.win32_devices_display, computer.win32_graphics_gdi);

    unsafe {
        // Act
        let result = display_settings
            .swap_primary_monitors(&wrong_desktop_monitor_name, &wrong_couch_monitor_name);

        // Assert
        match result {
            Ok(_) => assert!(false, "Expected system to invalidate the desktop & couch monitor names but actually validate them"),
            Err(reason) => assert!(
                reason == format!(
                    "Desktop and/or couch monitors are invalid, possible values are [{}]",
                    computer
                        .monitors
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()
                        .join(", ")),
                "Expected swap to fail because monitors are invalid but failed because of '{}'",
                    reason
            ),
        }
    }
}
