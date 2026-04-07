use convertible_couch_lib::func;
use convertible_couch_testing::arrangements::{
    builders::{
        application::ApplicationBuilder, arguments::ArgumentsBuilder,
        command_result::CommandResultBuilder,
    },
    fuzzing::{ComputerBuilder, Fuzzer},
};

#[test]
fn change_the_default_speaker() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::change().speakers_only(&alternative_speaker_name);

    assert_eq!(actual_result, expected_result);
}

#[test]
fn change_the_default_speaker_back_and_forth() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .speakers_only(&default_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    let expected_result = CommandResultBuilder::change().speakers_only(&default_speaker_name);

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_the_desktop_and_couch_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (
        invalid_speaker_name_1,
        invalid_speaker_name_2,
        default_speaker_name,
        alternative_speaker_name,
    ) = fuzzer.generate_four_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .speakers_only(&invalid_speaker_name_1, &invalid_speaker_name_2)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Desktop and couch speakers are invalid, possible values are [{default_speaker_name}, {alternative_speaker_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_the_desktop_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_speaker_name, default_speaker_name, alternative_speaker_name) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .speakers_only(&invalid_speaker_name, &alternative_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Desktop speaker is invalid, possible values are [{default_speaker_name}, {alternative_speaker_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}

#[test]
fn validate_the_couch_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_speaker_name, default_speaker_name, alternative_speaker_name) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(&default_speaker_name)
        .with_an_alternative_one_named(&alternative_speaker_name)
        .build_computer();

    let mut application = ApplicationBuilder::new(computer).build();

    let args = ArgumentsBuilder::change()
        .speakers_only(&default_speaker_name, &invalid_speaker_name)
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    let expected_result = CommandResultBuilder::custom_error(format!(
        "Couch speaker is invalid, possible values are [{default_speaker_name}, {alternative_speaker_name}]"
    ));

    assert_eq!(actual_result, expected_result);
}
