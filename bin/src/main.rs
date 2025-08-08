use clap::Parser;
use convertible_couch::{commands::Arguments, Application, ApplicationResult};
use convertible_couch_lib::{
    displays_settings::{CurrentDisplaysSettings, CurrentDisplaysSettingsApi},
    speakers_settings::{CurrentSpeakersSettings, CurrentSpeakersSettingsApi},
};
use log::{error, info, warn};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Arguments::parse();
    let mut application = Application::<
        CurrentDisplaysSettingsApi,
        CurrentSpeakersSettingsApi,
        CurrentDisplaysSettings<CurrentDisplaysSettingsApi>,
        CurrentSpeakersSettings<CurrentSpeakersSettingsApi>,
    >::bootstrap(CurrentDisplaysSettingsApi, CurrentSpeakersSettingsApi);

    let application_result = application.execute(&args);

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
