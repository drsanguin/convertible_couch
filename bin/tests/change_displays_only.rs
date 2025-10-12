use convertible_couch::{
    application::{ApplicationChangeResult, ApplicationResult},
    testing::{
        arrangements::{bootstrap_application, ArgumentsBuilder},
        assertions::assert_that_result_is_an_error_who_starts_with,
    },
};
use convertible_couch_lib::{
    displays_settings::{DisplaysSettingsResult, INTERNAL_DISPLAY_NAME},
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
    ApplicationError,
};

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: secondary_display_name,
                    reboot_required: false
                }
            }
        ))
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: primary_display_name,
                    reboot_required: false
                }
            }
        ))
    );
}

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display_when_the_computer_has_an_internal_display(
) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(INTERNAL_DISPLAY_NAME, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: secondary_display_name,
                    reboot_required: false,
                }
            }
        ))
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display_has_an_internal_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(INTERNAL_DISPLAY_NAME, &secondary_display_name)
        .build();

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: String::from(INTERNAL_DISPLAY_NAME),
                    reboot_required: false,
                }
            }
        ))
    );
}

#[test]
fn it_should_validate_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&invalid_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(format!(
            "Desktop display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        )))
    );
}

#[test]
fn it_should_validate_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&primary_display_name, &invalid_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(format!(
            "Couch display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        )))
    );
}

#[test]
fn it_should_validate_both_desktop_and_couch_displays() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (
        invalid_desktop_display_name,
        invalid_couch_display_name,
        primary_display_name,
        secondary_display_name,
    ) = fuzzer.generate_four_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&invalid_desktop_display_name, &invalid_couch_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(format!(
            "Desktop and couch displays are invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        )))
    );
}

#[test]
fn it_should_handle_the_case_when_it_fails_to_get_the_primary_display_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_getting_the_primary_display_fails()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_an_error_who_starts_with(
        actual_result,
        "Failed to retrieve display configuration information about the device",
    );
}
