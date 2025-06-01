use clap::Parser;
use convertible_couch::{run_app, Args};
use convertible_couch_lib::{
    display_settings::{self, DisplaySettings},
    sound_settings::{self, SoundSettings},
};
use log::error;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let display_settings =
        display_settings::Current::new(display_settings::CurrentDisplaySettingsApi);
    let sound_settings = sound_settings::Current::new(sound_settings::CurrentSoundSettingsApi);

    match run_app(args, display_settings, sound_settings) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error!("{error}");

            ExitCode::FAILURE
        }
    }
}
