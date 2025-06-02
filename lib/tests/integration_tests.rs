use convertible_couch_lib::{
    display_settings::{self, DisplaySettings, DisplaySettingsResult},
    func,
    testing::{
        assertions::{
            assert_that_displays_have_been_validated,
            assert_that_primary_display_have_been_changed_as_expected,
            assert_that_response_is_an_error_who_starts_with,
        },
        fuzzing::Fuzzer,
    },
};
use std::collections::HashSet;

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_primary_display_have_been_changed_as_expected(
        actual_response,
        Ok(DisplaySettingsResult {
            new_primary: Some(computer.secondary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&computer.primary_display, &computer.secondary_display)
        .and_then(|_| {
            display_settings
                .change_primary_display(&computer.primary_display, &computer.secondary_display)
        });

    // Assert
    assert_that_primary_display_have_been_changed_as_expected(
        actual_response,
        Ok(DisplaySettingsResult {
            new_primary: Some(computer.primary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display_when_the_computer_has_an_internal_display(
) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .build_displays()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings.change_primary_display(
        display_settings::INTERNAL_DISPLAY_NAME,
        &computer.secondary_display,
    );

    // Assert
    assert_that_primary_display_have_been_changed_as_expected(
        actual_response,
        Ok(DisplaySettingsResult {
            new_primary: Some(computer.secondary_display),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display_has_an_internal_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .build_displays()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(
            display_settings::INTERNAL_DISPLAY_NAME,
            &computer.secondary_display,
        )
        .and_then(|_| {
            display_settings.change_primary_display(
                display_settings::INTERNAL_DISPLAY_NAME,
                &computer.secondary_display,
            )
        });

    // Assert
    assert_that_primary_display_have_been_changed_as_expected(
        actual_response,
        Ok(DisplaySettingsResult {
            new_primary: Some(String::from(display_settings::INTERNAL_DISPLAY_NAME)),
            reboot_required: false,
        }),
    );
}

#[test]
fn it_should_validate_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

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

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&wrong_desktop_display_name, &computer.secondary_display);

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
    let mut fuzzer = Fuzzer::new(func!(), true);

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

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&computer.primary_display, &wrong_couch_display_name);

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
    let mut fuzzer = Fuzzer::new(func!(), true);

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

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&wrong_desktop_display_name, &wrong_couch_display_name);

    // Assert
    assert_that_displays_have_been_validated(
        actual_response,
        &computer.displays,
        "Desktop and couch displays are invalid",
    );
}

#[test]
fn it_should_handle_the_case_when_it_fails_to_get_the_primary_display_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_getting_the_primary_display_fails()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve display configuration information about the device",
    );
}

#[test]
fn it_should_handle_the_case_when_querying_the_display_config_of_the_primary_display_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .for_which_querying_the_display_config_of_the_primary_display_fails()
        .build_computer();

    let mut display_settings = display_settings::Current::new(computer.display_settings_api);

    // Act
    let actual_response = display_settings
        .change_primary_display(&computer.primary_display, &computer.secondary_display);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve the name of the display at the device path",
    );
}
