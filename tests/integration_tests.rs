use common::assertions::assert_that_primary_monitors_have_been_swap_as_expected;
use common::fuzzing::Fuzzer;
use convertible_couch::display_settings::{DisplaySettings, SwapPrimaryMonitorsResponse};

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
        let actual_response = display_settings
            .swap_primary_monitors(&wrong_desktop_monitor_name, &wrong_couch_monitor_name);

        // Assert
        assert_eq!(
            actual_response,
            Err(format!(
                "Desktop and/or couch monitors are invalid, possible values are [{}]",
                computer
                    .monitors
                    .iter()
                    .map(|monitor_name| monitor_name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        );
    }
}
