use convertible_couch::{run_app, Arguments, AudioOpts, Commands, SharedOpts};
use convertible_couch_lib::{
    display_settings::{CurrentDisplaySettings, DisplaySettings},
    func,
    log::LogLevel,
    sound_settings::{CurrentSoundSettings, SoundSettings},
    testing::fuzzing::Fuzzer,
};

#[test]
fn it_should_return_an_error_if_getting_the_audio_outputs_count_fails() {
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
        .for_which_getting_the_audio_outputs_count_fails()
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
        Err(String::from(
            "Failed to get the number of sound output devices"
        ))
    );
}

#[test]
fn it_should_return_an_error_if_getting_the_audio_outputs_fails() {
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
        .for_which_getting_the_audio_outputs_fails()
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
        Err(String::from("Failed to get the sound output devices"))
    );
}

#[test]
fn it_should_return_an_error_if_getting_the_current_default_audio_output_fails() {
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
        .for_which_getting_the_default_audio_output_fails()
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
        Err(String::from(
            "Failed to get the current default sound output device"
        ))
    );
}

#[test]
fn it_should_return_an_error_if_setting_the_default_audio_output_fails() {
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
        .for_which_setting_the_default_audio_output_fails()
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
        Err(String::from("Failed to set default audio endpoint"))
    );
}
