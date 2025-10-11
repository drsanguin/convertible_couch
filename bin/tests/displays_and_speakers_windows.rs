use convertible_couch::testing::arrangements::{bootstrap_application, ArgumentsBuilder};
use convertible_couch_lib::{
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
    ApplicationError,
};
use windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM;

#[test]
fn it_should_report_a_displays_settings_error() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .for_which_changing_the_display_settings_fails_with(DISP_CHANGE_BADPARAM)
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
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
        Err(ApplicationError::Custom(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")))
    );
}

#[test]
fn it_should_report_a_speakers_settings_error() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .for_which_setting_the_default_speaker_fails()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::new()
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
            "Failed to set default speaker"
        )))
    );
}
