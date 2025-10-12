use convertible_couch::{
    application::{ApplicationInfoResult, ApplicationResult},
    testing::arrangements::{bootstrap_application, ArgumentsBuilder},
};
use convertible_couch_lib::{
    func,
    testing::fuzzing::{ComputerBuilder, Fuzzer},
    DeviceInfo,
};

#[test]
fn it_should_get_informations_about_displays_and_speakers() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default()
        .info()
        .displays_and_speakers()
        .build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::DisplaysAndSpeakers {
                displays_result: vec![
                    DeviceInfo {
                        name: primary_display_name
                    },
                    DeviceInfo {
                        name: secondary_display_name
                    }
                ],
                speakers_result: vec![
                    DeviceInfo {
                        name: default_speaker_name
                    },
                    DeviceInfo {
                        name: alternative_speaker_name
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

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default().info().displays_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::DisplaysOnly {
                displays_result: vec![
                    DeviceInfo {
                        name: primary_display_name
                    },
                    DeviceInfo {
                        name: secondary_display_name
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

    let (default_speaker_name, alternative_speaker_name) = fuzzer.generate_two_speakers_names();

    let computer = fuzzer
        .generate_computer()
        .with_speakers()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_speaker_name.clone())
        .with_an_alternative_one_named(alternative_speaker_name.clone())
        .build_computer();

    let mut application = bootstrap_application(computer);

    let args = ArgumentsBuilder::default().info().speakers_only().build();

    // Act
    let actual_result = application.execute(&args);

    // Assert
    assert_eq!(
        actual_result,
        Ok(ApplicationResult::Info(
            ApplicationInfoResult::SpeakersOnly {
                speakers_result: vec![
                    DeviceInfo {
                        name: default_speaker_name
                    },
                    DeviceInfo {
                        name: alternative_speaker_name
                    }
                ]
            }
        ))
    );
}
