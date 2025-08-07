use convertible_couch::{run_app, ApplicationResult, Arguments, AudioOpts, Commands, SharedOpts};
use convertible_couch_lib::{
    display_settings::{CurrentDisplaySettings, DisplaySettings},
    func,
    log::LogLevel,
    sound_settings::{CurrentSoundSettings, SoundSettings, SoundSettingsResult},
    testing::fuzzing::Fuzzer,
};

#[test]
fn it_should_change_the_default_output_device() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_audio_output_device_name, alternative_audio_output_device_name) =
        fuzzer.generate_two_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_audio_output_devices()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::AudioOnly {
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
        Ok(ApplicationResult::AudioOnly {
            sound_settings: SoundSettingsResult {
                new_default_output_device: alternative_audio_output_device_name
            }
        })
    );
}

#[test]
fn it_should_change_the_default_output_device_back_and_forth() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (default_audio_output_device_name, alternative_audio_output_device_name) =
        fuzzer.generate_two_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_audio_output_devices()
        .of_which_there_are_at_least(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::AudioOnly {
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
        Ok(ApplicationResult::AudioOnly {
            sound_settings: SoundSettingsResult {
                new_default_output_device: default_audio_output_device_name
            }
        })
    );
}

#[test]
fn it_should_validate_the_desktop_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (
        invalid_audio_output_device,
        default_audio_output_device_name,
        alternative_audio_output_device_name,
    ) = fuzzer.generate_three_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_audio_output_devices()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::AudioOnly {
            audio: AudioOpts {
                desktop_speaker_name: invalid_audio_output_device,
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
        Err(String::from(
            format!("Desktop sound output device is invalid, possible values are are {default_audio_output_device_name}, {alternative_audio_output_device_name}")
        ))
    );
}

#[test]
fn it_should_validate_the_couch_speaker_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (
        invalid_audio_output_device,
        default_audio_output_device_name,
        alternative_audio_output_device_name,
    ) = fuzzer.generate_three_audio_output_devices_names();

    let computer = fuzzer
        .generate_computer()
        .with_audio_output_devices()
        .of_which_there_are(2)
        .whose_default_one_is_named(default_audio_output_device_name.clone())
        .with_an_alternative_one_named(alternative_audio_output_device_name.clone())
        .build_audio_output_devices()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::AudioOnly {
            audio: AudioOpts {
                desktop_speaker_name: default_audio_output_device_name.clone(),
                couch_speaker_name: invalid_audio_output_device.clone(),
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
        Err(String::from(
            format!("Couch sound output device is invalid, possible values are {default_audio_output_device_name}, {alternative_audio_output_device_name}")
        ))
    );
}
