use convertible_couch::{
    run_app, ApplicationResult, Arguments, AudioOpts, Commands, SharedOpts, VideoOpts,
};
use convertible_couch_lib::{
    display_settings::{CurrentDisplaySettings, DisplaySettings, DisplaySettingsResult},
    func,
    log::LogLevel,
    sound_settings::{CurrentSoundSettings, SoundSettings, SoundSettingsResult},
    testing::fuzzing::Fuzzer,
};

#[test]
fn it_should_change_primary_display_and_default_output_device() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_audio_output_device_name, alternative_audio_output_device_name) =
        fuzzer.generate_two_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .with_audio_output_devices()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoAndAudio {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
            audio: AudioOpts {
                desktop_speaker_name: default_audio_output_device_name.clone(),
                couch_speaker_name: alternative_audio_output_device_name.clone(),
            },
            shared: SharedOpts {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_response = run_app(&args, &mut display_settings, &mut sound_settings);

    // Assert
    assert_eq!(
        actual_response,
        Ok(ApplicationResult::VideoAndAudio {
            display_settings: DisplaySettingsResult {
                new_primary: secondary_display_name,
                reboot_required: false
            },
            sound_settings: SoundSettingsResult {
                new_default_output_device: alternative_audio_output_device_name
            }
        })
    );
}

#[test]
fn it_should_change_primary_display_and_default_output_device_back_and_forth() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();
    let (default_audio_output_device_name, alternative_audio_output_device_name) =
        fuzzer.generate_two_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .with_audio_output_devices()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoAndAudio {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
            audio: AudioOpts {
                desktop_speaker_name: default_audio_output_device_name.clone(),
                couch_speaker_name: alternative_audio_output_device_name.clone(),
            },
            shared: SharedOpts {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_response = run_app(&args, &mut display_settings, &mut sound_settings)
        .and_then(|_| run_app(&args, &mut display_settings, &mut sound_settings));

    // Assert
    assert_eq!(
        actual_response,
        Ok(ApplicationResult::VideoAndAudio {
            display_settings: DisplaySettingsResult {
                new_primary: primary_display_name,
                reboot_required: false
            },
            sound_settings: SoundSettingsResult {
                new_default_output_device: default_audio_output_device_name
            }
        })
    );
}
