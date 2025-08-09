#![cfg(target_os = "windows")]

use convertible_couch::testing::{arrangements::bootstrap_application, builders::ArgumentsBuilder};
use convertible_couch_lib::{func, testing::fuzzing::Fuzzer};

#[test]
fn it_should_return_an_error_if_getting_the_speakers_count_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .for_which_getting_the_speakers_count_fails()
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .speakers_only(
            default_speaker_name.clone(),
            alternative_speaker_name.clone(),
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from("Failed to get the number of speakers"))
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
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .for_which_getting_the_speakers_fails()
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .speakers_only(
            default_speaker_name.clone(),
            alternative_speaker_name.clone(),
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from("Failed to get the speakers"))
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
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .for_which_getting_the_default_speaker_fails()
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .speakers_only(
            default_speaker_name.clone(),
            alternative_speaker_name.clone(),
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from("Failed to get the current default speaker"))
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
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .for_which_setting_the_default_speaker_fails()
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
        .speakers_only(
            default_speaker_name.clone(),
            alternative_speaker_name.clone(),
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from("Failed to set default speaker"))
    );
}
