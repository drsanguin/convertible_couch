use convertible_couch::{
    application::{ApplicationInfoResult, ApplicationResult},
    testing::arrangements::{bootstrap_application, ArgumentsBuilder},
};
use convertible_couch_lib::{
    displays_settings::DisplayInfo,
    func,
    speakers_settings::SpeakerInfo,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
};

#[test]
fn it_should_get_informations_about_displays_and_speakers() {
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
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .with_a_secondary_named(secondary_display_name_2.clone())
        .build_displays()
        .with_speakers()
        .of_which_there_are(3)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name_2.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder.info().displays_and_speakers().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::DisplaysAndSpeakers {
                displays_result: vec![
                    DisplayInfo {
                        is_primary: true,
                        name: primary_display_name
                    },
                    DisplayInfo {
                        is_primary: false,
                        name: secondary_display_name
                    },
                    DisplayInfo {
                        is_primary: false,
                        name: secondary_display_name_2
                    }
                ],
                speakers_result: vec![
                    SpeakerInfo {
                        is_default: true,
                        name: default_speaker_name
                    },
                    SpeakerInfo {
                        is_default: false,
                        name: alternative_speaker_name
                    },
                    SpeakerInfo {
                        is_default: false,
                        name: alternative_speaker_name_2
                    }
                ]
            }
        ))
    );
}

#[test]
fn it_should_get_informations_about_displays_only() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name, secondary_display_name_2) =
        fuzzer.generate_three_display_names();
    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(3)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .with_a_secondary_named(secondary_display_name_2.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder.info().displays_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::DisplaysOnly {
                displays_result: vec![
                    DisplayInfo {
                        is_primary: true,
                        name: primary_display_name
                    },
                    DisplayInfo {
                        is_primary: false,
                        name: secondary_display_name
                    },
                    DisplayInfo {
                        is_primary: false,
                        name: secondary_display_name_2
                    }
                ]
            }
        ))
    );
}

#[test]
fn it_should_get_informations_about_speakers_only() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_speaker_name, alternative_speaker_name, alternative_speaker_name_2) =
        fuzzer.generate_three_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(3)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name_2.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder.info().speakers_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::SpeakersOnly {
                speakers_result: vec![
                    SpeakerInfo {
                        is_default: true,
                        name: default_speaker_name
                    },
                    SpeakerInfo {
                        is_default: false,
                        name: alternative_speaker_name
                    },
                    SpeakerInfo {
                        is_default: false,
                        name: alternative_speaker_name_2
                    }
                ]
            }
        ))
    );
}
