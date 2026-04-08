use convertible_couch_lib::func;
use convertible_couch_testing::arrangements::{
    builders::{
        application::ApplicationBuilder, arguments::ArgumentsBuilder,
        command_result::CommandResultBuilder,
    },
    fuzzing::{ComputerBuilder, Fuzzer},
};

#[test]
fn get_informations_about_displays_and_speakers() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name, secondary_display_name_2) =
        fuzzer.generate_three_display_names();
    let (default_speaker_name, alternative_speaker_name, alternative_speaker_name_2) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(3)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .with_a_secondary_named(&secondary_display_name_2)
        .build_displays()
        .with_speakers()
        .of_which_there_are(3)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name_2)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::info().displays_and_speakers().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::info()
        .displays_and_speakers()
        .with_primary_display(&primary_display_name)
        .with_secondary_display(&secondary_display_name)
        .with_secondary_display(&secondary_display_name_2)
        .with_default_speaker(&default_speaker_name)
        .with_alternative_speaker(&alternative_speaker_name)
        .with_alternative_speaker(&alternative_speaker_name_2)
        .build();

    assert_eq!(actual_result, expected_result);
}

#[test]
fn get_informations_about_displays_and_speakers_when_the_computer_has_no_displays_and_no_speakers()
{
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let computer = fuzzer.generate_computer().build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::info().displays_and_speakers().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::info().displays_and_speakers().build();

    assert_eq!(actual_result, expected_result);
}

#[test]
fn get_informations_about_displays_only() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name, secondary_display_name_2) =
        fuzzer.generate_three_display_names();
    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(3)
        .whose_primary_is_named(&primary_display_name)
        .with_a_secondary_named(&secondary_display_name)
        .with_a_secondary_named(&secondary_display_name_2)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::info().displays_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::info()
        .displays_only()
        .with_primary_display(&primary_display_name)
        .with_secondary_display(&secondary_display_name)
        .with_secondary_display(&secondary_display_name_2)
        .build();

    assert_eq!(actual_result, expected_result);
}

#[test]
fn get_informations_about_speakers_only() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name, alternative_speaker_name_2) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(3)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name_2)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::info().speakers_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::info()
        .speakers_only()
        .with_default_speaker(&default_speaker_name)
        .with_alternative_speaker(&alternative_speaker_name)
        .with_alternative_speaker(&alternative_speaker_name_2)
        .build();

    assert_eq!(actual_result, expected_result);
}

#[test]
fn get_informations_about_speakers_only_even_if_there_if_no_default_one() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (alternative_speaker_name, alternative_speaker_name_2) =
        fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name_2)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::info().speakers_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::info()
        .speakers_only()
        .with_alternative_speaker(&alternative_speaker_name)
        .with_alternative_speaker(&alternative_speaker_name_2)
        .build();

    assert_eq!(actual_result, expected_result);
}
