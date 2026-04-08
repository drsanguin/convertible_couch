use convertible_couch_lib::{displays_settings::INTERNAL_DISPLAY_NAME, func};
use convertible_couch_testing::arrangements::{
    builders::{
        application::ApplicationBuilder, arguments::ArgumentsBuilder,
        command_result::CommandResultBuilder,
    },
    fuzzing::{ComputerBuilder, Fuzzer},
};

#[test]
fn swap_the_desktop_display_with_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::change().displays_only(&secondary_display_name);

    assert_eq!(actual_result, expected_result);
}

#[test]
fn swap_the_couch_display_with_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(&primary_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    let expected_result = CommandResultBuilder::change().displays_only(&primary_display_name);

    assert_eq!(actual_result, expected_result);
}

#[test]
fn swap_the_desktop_display_with_the_couch_display_when_the_computer_has_an_internal_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(INTERNAL_DISPLAY_NAME, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::change().displays_only(&secondary_display_name);

    assert_eq!(actual_result, expected_result);
}

#[test]
fn swap_the_couch_display_with_the_desktop_display_has_an_internal_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(INTERNAL_DISPLAY_NAME, &secondary_display_name)
        .build();

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    let expected_result =
        CommandResultBuilder::change().displays_only(&String::from(INTERNAL_DISPLAY_NAME));

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(&invalid_display_name, &secondary_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Desktop display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(&primary_display_name, &invalid_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Couch display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_both_desktop_and_couch_displays() {
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
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .displays_only(&invalid_desktop_display_name, &invalid_couch_display_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Desktop and couch displays are invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}
