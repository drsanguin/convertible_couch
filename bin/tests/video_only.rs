use crate::assertions::assert_that_response_is_an_error_who_starts_with;
use convertible_couch::{run_app, ApplicationResult, Arguments, Commands, SharedOpts, VideoOpts};
use convertible_couch_lib::{
    display_settings::{
        CurrentDisplaySettings, DisplaySettings, DisplaySettingsResult, INTERNAL_DISPLAY_NAME,
    },
    func,
    log::LogLevel,
    sound_settings::{CurrentSoundSettings, SoundSettings},
    testing::fuzzing::Fuzzer,
};
use std::collections::HashSet;

mod assertions;

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
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
        Ok(ApplicationResult::VideoOnly {
            display_settings: DisplaySettingsResult {
                new_primary: secondary_display_name,
                reboot_required: false
            }
        })
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
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
        Ok(ApplicationResult::VideoOnly {
            display_settings: DisplaySettingsResult {
                new_primary: primary_display_name,
                reboot_required: false
            }
        })
    );
}

#[test]
fn it_should_swap_the_desktop_display_with_the_couch_display_when_the_computer_has_an_internal_display(
) {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: String::from(INTERNAL_DISPLAY_NAME),
                couch_display_name: secondary_display_name.clone(),
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
        Ok(ApplicationResult::VideoOnly {
            display_settings: DisplaySettingsResult {
                new_primary: secondary_display_name,
                reboot_required: false,
            }
        })
    );
}

#[test]
fn it_should_swap_the_couch_display_with_the_desktop_display_has_an_internal_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let secondary_display_name = fuzzer.generate_display_name();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .including_an_internal_display()
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: String::from(INTERNAL_DISPLAY_NAME),
                couch_display_name: secondary_display_name.clone(),
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
        Ok(ApplicationResult::VideoOnly {
            display_settings: DisplaySettingsResult {
                new_primary: String::from(INTERNAL_DISPLAY_NAME),
                reboot_required: false,
            }
        })
    );
}

#[test]
fn it_should_validate_the_desktop_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (wrong_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let forbidden_display_names = HashSet::from([wrong_display_name.as_str()]);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_names_are_different_from(forbidden_display_names)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: wrong_display_name,
                couch_display_name: secondary_display_name.clone(),
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
        Err(format!(
            "Desktop display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        ))
    );
}

#[test]
fn it_should_validate_the_couch_display() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (wrong_display_name, primary_display_name, secondary_display_name) =
        fuzzer.generate_three_display_names();

    let forbidden_display_names = HashSet::from([wrong_display_name.as_str()]);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_names_are_different_from(forbidden_display_names)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: wrong_display_name,
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
        Err(format!(
            "Couch display is invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        ))
    );
}

#[test]
fn it_should_validate_both_desktop_and_couch_displays() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (wrong_display_name1, wrong_display_name2, primary_display_name, secondary_display_name) =
        fuzzer.generate_four_display_names();

    let forbidden_display_names =
        HashSet::from([wrong_display_name1.as_str(), wrong_display_name2.as_str()]);

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are(2)
        .whose_names_are_different_from(forbidden_display_names)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: wrong_display_name1,
                couch_display_name: wrong_display_name2,
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
        Err(format!(
            "Desktop and couch displays are invalid, possible values are [{primary_display_name}, {secondary_display_name}]"
        ))
    );
}

#[test]
fn it_should_handle_the_case_when_it_fails_to_get_the_primary_display_name() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .for_which_getting_the_primary_display_fails()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
            shared: SharedOpts {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_response = run_app(&args, &mut display_settings, &mut sound_settings);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve display configuration information about the device",
    );
}

#[test]
fn it_should_handle_the_case_when_querying_the_display_config_of_the_primary_display_fails() {
    // Arrange
    let mut fuzzer = Fuzzer::new(func!(), true);

    let (primary_display_name, secondary_display_name) = fuzzer.generate_two_display_names();

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .whose_primary_is_named(primary_display_name.clone())
        .with_a_secondary_named(secondary_display_name.clone())
        .build_displays()
        .for_which_querying_the_display_config_of_the_primary_display_fails()
        .build_computer();

    let mut display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let mut sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Arguments {
        command: Commands::VideoOnly {
            video: VideoOpts {
                desktop_display_name: primary_display_name.clone(),
                couch_display_name: secondary_display_name.clone(),
            },
            shared: SharedOpts {
                log_level: LogLevel::Off,
            },
        },
    };

    // Act
    let actual_response = run_app(&args, &mut display_settings, &mut sound_settings);

    // Assert
    assert_that_response_is_an_error_who_starts_with(
        actual_response,
        "Failed to retrieve the name of the display at the device path",
    );
}
