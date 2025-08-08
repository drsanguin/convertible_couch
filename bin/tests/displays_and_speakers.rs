use convertible_couch::{
    run_app, ApplicationResult, Arguments, Commands, DisplaysOptions, SharedOptions,
    SpeakersOptions,
};
use convertible_couch_lib::{
    displays_settings::{CurrentDisplaysSettings, DisplaysSettings, DisplaysSettingsResult},
    func,
    log::LogLevel,
    speakers_settings::{CurrentSpeakersSettings, SpeakersSettings, SpeakersSettingsResult},
    testing::fuzzing::Fuzzer,
};

#[test]
fn it_should_change_primary_display_and_default_speaker() {
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
        .build_speakers()
        .build_computer();

    let mut displays_settings = CurrentDisplaysSettings::new(computer.displays_settings_api);
    let mut speakers_settings = CurrentSpeakersSettings::new(computer.speakers_settings_api);

    let args = Arguments {
        command: Commands::DisplaysAndSpeakers {
            displays: DisplaysOptions {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
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
    let actual_response = run_app(&args, &mut displays_settings, &mut speakers_settings);

    // Assert
    assert_eq!(
        actual_response,
        Ok(ApplicationResult::DisplaysAndSpeakers {
            displays_result: DisplaysSettingsResult {
                new_primary_display: secondary_display_name,
                reboot_required: false
            },
            speakers_result: SpeakersSettingsResult {
                new_default_speaker: alternative_speaker_name
            }
        })
    );
}

#[test]
fn it_should_change_primary_display_and_default_speaker_back_and_forth() {
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
        .build_speakers()
        .build_computer();

    let mut displays_settings = CurrentDisplaysSettings::new(computer.displays_settings_api);
    let mut speakers_settings = CurrentSpeakersSettings::new(computer.speakers_settings_api);

    let args = Arguments {
        command: Commands::DisplaysAndSpeakers {
            displays: DisplaysOptions {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
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
    let actual_response = run_app(&args, &mut displays_settings, &mut speakers_settings)
        .and_then(|_| run_app(&args, &mut displays_settings, &mut speakers_settings));

    // Assert
    assert_eq!(
        actual_response,
        Ok(ApplicationResult::DisplaysAndSpeakers {
            displays_result: DisplaysSettingsResult {
                new_primary_display: primary_display_name,
                reboot_required: false
            },
            speakers_result: SpeakersSettingsResult {
                new_default_speaker: default_speaker_name
            }
        })
    );
}
