#![cfg(target_os = "windows")]

use convertible_couch::testing::arrangements::{ApplicationBuilder, ArgumentsBuilder};
use convertible_couch_lib::{
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
    ApplicationError,
};

#[test]
fn it_should_return_an_error_if_getting_the_speakers_count_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_getting_the_speakers_count_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "Failed to get the number of speakers"
        )))
    );
}

#[test]
fn it_should_return_an_error_if_getting_the_speakers_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_getting_the_speakers_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "Failed to get the speakers"
        )))
    );
}

#[test]
fn it_should_return_an_error_if_getting_the_current_default_speaker_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_getting_the_default_speaker_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "Failed to get the current default speaker"
        )))
    );
}

#[test]
fn it_should_return_an_error_if_setting_the_default_speaker_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_setting_the_default_speaker_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "Failed to set default speaker"
        )))
    );
}

#[test]
fn it_should_return_an_error_if_initializing_the_com_library_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_initializing_the_com_library_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert!(actual_result.is_err());
}
