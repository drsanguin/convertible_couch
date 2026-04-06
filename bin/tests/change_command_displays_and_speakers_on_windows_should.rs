#![cfg(target_os = "windows")]

use convertible_couch_lib::{application_error::ApplicationError, func};
use convertible_couch_testing::arrangements::{
    builders::{application::ApplicationBuilder, arguments::ArgumentsBuilder},
    fuzzing::{ComputerBuilder, Fuzzer},
};

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

    let args = ArgumentsBuilder::change()
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
