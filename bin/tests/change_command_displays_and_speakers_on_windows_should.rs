#![cfg(target_os = "windows")]

use convertible_couch::application::{ApplicationChangeResult, ApplicationResult};
use convertible_couch_lib::{
    application_error::ApplicationError, displays_settings::DisplaysSettingsResult, func,
    speakers_settings::SpeakersSettingsResult,
};
use convertible_couch_testing::{
    arrangements::{
        builders::{ApplicationBuilder, ArgumentsBuilder},
        fuzzing::{ComputerBuilder, Fuzzer},
    },
    assertions::assert_that_result_is_a_win32_error,
};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER};

#[test]
fn report_get_display_config_buffer_sizes_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_get_display_config_buffer_fails_with(ERROR_INVALID_PARAMETER)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_query_display_config_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_query_display_config_fails_with(vec![ERROR_INVALID_PARAMETER])
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_display_config_get_device_info_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_display_config_get_device_info_fails_with(ERROR_INVALID_PARAMETER)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_set_display_config_errors() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_set_display_config_fails_with(ERROR_INVALID_PARAMETER)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_that_result_is_a_win32_error(actual_result, ERROR_INVALID_PARAMETER);
}

#[test]
fn report_a_speakers_settings_error() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .for_which_setting_the_default_speaker_fails()
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(ApplicationError::Custom(String::from(
            "Failed to set default speaker (0x80004005)"
        )))
    );
}

#[test]
fn overcome_query_display_config_returning_an_insufficient_buffer_error() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .for_which_query_display_config_fails_with(vec![ERROR_INSUFFICIENT_BUFFER])
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder
        .change()
        .displays_and_speakers(
            &primary_display_name,
            &secondary_display_name,
            &default_speaker_name,
            &alternative_speaker_name,
        )
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Change(
            ApplicationChangeResult::DisplaysAndSpeakers {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: secondary_display_name,
                    reboot_required: false
                },
                speakers_result: SpeakersSettingsResult {
                    new_default_speaker: alternative_speaker_name
                }
            }
        ))
    );
}
