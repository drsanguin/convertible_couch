use convertible_couch::{
    application::ApplicationResult,
    commands::{Arguments, Commands, SharedOptions, SpeakersOptions},
    testing::bootstrap_application,
};
use convertible_couch_lib::{
    func, log::LogLevel, speakers_settings::SpeakersSettingsResult, testing::fuzzing::Fuzzer,
};

#[test]
fn it_should_change_the_default_speaker() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = Arguments {
        command: Commands::SpeakersOnly {
            speakers: SpeakersOptions {
                desktop_speaker_name: default_speaker_name.clone(),
                couch_speaker_name: alternative_speaker_name.clone(),
            },
            shared: SharedOptions {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::SpeakersOnly {
            speakers_result: SpeakersSettingsResult {
                new_default_speaker: alternative_speaker_name
            }
        })
    );
}

#[test]
fn it_should_change_the_default_speaker_back_and_forth() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = Arguments {
        command: Commands::SpeakersOnly {
            speakers: SpeakersOptions {
                desktop_speaker_name: default_speaker_name.clone(),
                couch_speaker_name: alternative_speaker_name.clone(),
            },
            shared: SharedOptions {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_result = application
        .execute(&args)
        .and_then(|_| application.execute(&args));

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::SpeakersOnly {
            speakers_result: SpeakersSettingsResult {
                new_default_speaker: default_speaker_name
            }
        })
    );
}

#[test]
fn it_should_validate_the_desktop_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_speaker_name, default_speaker_name, alternative_speaker_name) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = Arguments {
        command: Commands::SpeakersOnly {
            speakers: SpeakersOptions {
                desktop_speaker_name: invalid_speaker_name,
                couch_speaker_name: alternative_speaker_name.clone(),
            },
            shared: SharedOptions {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from(
            format!("Desktop speaker is invalid, possible values are are {default_speaker_name}, {alternative_speaker_name}")
        ))
    );
}

#[test]
fn it_should_validate_the_couch_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (invalid_speaker_name, default_speaker_name, alternative_speaker_name) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_speakers()
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = Arguments {
        command: Commands::SpeakersOnly {
            speakers: SpeakersOptions {
                desktop_speaker_name: default_speaker_name.clone(),
                couch_speaker_name: invalid_speaker_name.clone(),
            },
            shared: SharedOptions {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Err(String::from(
            format!("Couch speaker is invalid, possible values are {default_speaker_name}, {alternative_speaker_name}")
        ))
    );
}
