use common::Fuzzer;
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
                Some(new_primary) => {
                    assert_eq!(
                        new_primary, computer.secondary_monitor,
                        "Expected primary monitor to have been set to {0} but it has been set to {1}",
                        computer.secondary_monitor, new_primary
                    );

                    assert_eq!(response.reboot_required, false, "Reboot has been required whereas it was not expected")
                },
                None => assert!(
                    false,
                    "Expected primary monitor to have been set to {0} but it has not been changed",
                    computer.secondary_monitor
                ),
            },
            Err(reason) => assert!(
                false,
                "Expected primary monitor to have been set to {0} but if failed because of the error {1}",
                    computer.secondary_monitor,
                    reason
            ),
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
                Some(new_primary) => {
                    assert_eq!(
                        new_primary, computer.secondary_monitor,
                        "Expected primary monitor to have been set to {0} but it has been set to {1}",
                        computer.secondary_monitor, new_primary
                    );

                    assert_eq!(response.reboot_required, true, "Reboot has not been required whereas it was expected")
                },
                None => assert!(
                    false,
                    "Expected primary monitor to have been set to {0} but it has not been changed",
                    computer.secondary_monitor
                ),
            },
            Err(reason) => assert!(
                false,
                "Expected primary monitor to have been set to {0} but if failed because of the error {1}",
                    computer.secondary_monitor,
                    reason
            ),
        }
    }
}
