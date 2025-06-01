use clap::Parser;
use convertible_couch::{run_app, Args};
use convertible_couch_lib::{
    display_settings::{self, DisplaySettings},
    sound_settings::{self, SoundSettings},
};
use log::{error, info, warn};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let display_settings =
        display_settings::Current::new(display_settings::CurrentDisplaySettingsApi);
    let sound_settings = sound_settings::Current::new(sound_settings::CurrentSoundSettingsApi);

    match run_app(args, display_settings, sound_settings) {
        Ok((display_settings_response, sound_settings_response)) => {
            match (
                display_settings_response.new_primary,
                display_settings_response.reboot_required,
            ) {
                (None, _) => error!("Primary display has not been changed for an unknow reason"),
                (Some(_), true) => warn!("The settings change was successful but the computer must be restarted for the graphics mode to work."),
                (Some(new_primary), false) => info!("Primary display set to {new_primary}"),
            }

            info!(
                "Default output device set to {0}",
                sound_settings_response.new_default_output_device
            );

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");

            ExitCode::FAILURE
        }
    }
}
