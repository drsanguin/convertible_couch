use convertible_couch::{run_app, ApplicationResult, Args};
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

    let computer = fuzzer
        .generate_computer()
        .with_displays()
        .of_which_there_are_at_least(2)
        .build_displays()
        .with_audio_output_devices()
        .of_which_there_are(2)
        .build_audio_output_devices()
        .build_computer();

    let display_settings = CurrentDisplaySettings::new(computer.display_settings_api);
    let sound_settings = CurrentSoundSettings::new(computer.audio_settings_api);

    let args = Args {
        desktop_display_name: computer.primary_display,
        couch_display_name: computer.secondary_display.clone(),
        desktop_speaker_name: computer.default_audio_endpoint,
        couch_speaker_name: computer.non_default_audio_endpoint.clone(),
        log_level: LogLevel::Off,
    };

    // Act
    let actual_response = run_app(args, display_settings, sound_settings);

    // Assert
    assert_eq!(
        actual_response,
        Ok(ApplicationResult {
            display_settings: DisplaySettingsResult {
                new_primary: computer.secondary_display,
                reboot_required: false
            },
            sound_settings: SoundSettingsResult {
                new_default_output_device: computer.non_default_audio_endpoint
            }
        })
    );
}
