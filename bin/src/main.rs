use clap::Parser;
use convertible_couch::{run_app, ApplicationResult, Arguments};
use convertible_couch_lib::{
    displays_settings::{CurrentDisplaysSettings, CurrentDisplaysSettingsApi, DisplaysSettings},
    speakers_settings::{CurrentSpeakersSettings, CurrentSpeakersSettingsApi, SpeakersSettings},
};
use log::{error, info, warn};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Arguments::parse();
    let mut displays_settings = CurrentDisplaysSettings::new(CurrentDisplaysSettingsApi);
    let mut speakers_settings = CurrentSpeakersSettings::new(CurrentSpeakersSettingsApi);

    let application_result = run_app(&args, &mut displays_settings, &mut speakers_settings);

    match application_result {
        Ok(result) => {
            match result {
                ApplicationResult::DisplaysAndSpeakers {
                    displays_result,
                    speakers_result,
                } => {
                    match (
                        displays_result.new_primary_display,
                        displays_result.reboot_required,
                    ) {
                        (new_primary, true) => warn!("Primary display set to {new_primary} but the computer must be restarted for the graphics mode to work."),
                        (new_primary, false) => info!("Primary display set to {new_primary}"),
                    }

                    info!("Default speaker set to {0}", speakers_result.new_default_speaker);
                }
                ApplicationResult::DisplaysOnly { displays_result } => match (
                        displays_result.new_primary_display,
                        displays_result.reboot_required,
                    ) {
                        (new_primary, true) => warn!("Primary display set to {new_primary} but the computer must be restarted for the graphics mode to work."),
                        (new_primary, false) => info!("Primary display set to {new_primary}"),
                    },
                ApplicationResult::SpeakersOnly { speakers_result } => info!("Default speaker set to {0}", speakers_result.new_default_speaker),
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");

            ExitCode::FAILURE
        }
    }
}
