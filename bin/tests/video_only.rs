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
use windows::Win32::Graphics::Gdi::{DISP_CHANGE, DISP_CHANGE_RESTART};

use crate::assertions::assert_that_response_is_an_error_who_starts_with;

use test_case::test_case;

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

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_when_committing_changes(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, String> {
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
        .for_which_committing_the_display_changes_fails_with(disp_change)
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
    run_app(&args, &mut display_settings, &mut sound_settings)
}

#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADDUALVIEW => Err(String::from("The settings change was unsuccessful because the system is DualView capable.")); "when the error is BADDUALVIEW")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADFLAGS => Err(String::from("An invalid set of flags was passed in.")); "when the error is DISP_CHANGE_BADFLAGS")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADMODE => Err(String::from("The graphics mode is not supported.")); "when the error is DISP_CHANGE_BADMODE")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_BADPARAM => Err(String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags.")); "when the error is DISP_CHANGE_BADPARAM")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_FAILED => Err(String::from("The display driver failed the specified graphics mode.")); "when the error is DISP_CHANGE_FAILED")]
#[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_NOTUPDATED => Err(String::from("Unable to write settings to the registry.")); "when the error is DISP_CHANGE_NOTUPDATED")]
fn it_should_report_display_change_errors_that_happens_for_some_displays(
    disp_change: DISP_CHANGE,
) -> Result<ApplicationResult, String> {
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
        .for_which_changing_the_display_settings_fails_for_some_displays(disp_change)
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
    run_app(&args, &mut display_settings, &mut sound_settings)
}

#[test]
fn it_should_change_the_primary_display_of_computer_and_ask_for_reboot_when_required_after_committing_display_changes(
) {
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
        .for_which_committing_the_display_changes_fails_with(DISP_CHANGE_RESTART)
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
                reboot_required: true,
            }
        })
    );
}

#[test]
fn it_should_change_the_primary_display_of_computer_and_ask_for_reboot_when_required_after_changing_display_for_some_displays(
) {
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
        .for_which_changing_the_display_settings_fails_for_some_displays(DISP_CHANGE_RESTART)
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
                reboot_required: true,
            }
        })
    );
}
