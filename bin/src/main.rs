#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use clap::Parser;
use convertible_couch::{
    application::{Application, ApplicationResult},
    commands::Arguments,
};
use convertible_couch_lib::{
    displays_settings::{
        CurrentDisplaysSettings, CurrentDisplaysSettingsApi, DisplaysSettingsResult,
    },
    speakers_settings::{
        CurrentSpeakersSettings, CurrentSpeakersSettingsApi, SpeakersSettingsResult,
    },
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
                    log_displays_settings_result(displays_result);
                    log_speakers_settings_result(speakers_result);
                }
                ApplicationResult::DisplaysOnly { displays_result } => {
                    log_displays_settings_result(displays_result)
                }
                ApplicationResult::SpeakersOnly { speakers_result } => {
                    log_speakers_settings_result(speakers_result)
                }
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");

            ExitCode::FAILURE
        }
    }
}

fn log_speakers_settings_result(speakers_result: SpeakersSettingsResult) {
    info!(
        "Default speaker set to {0}",
        speakers_result.new_default_speaker
    );
}

fn log_displays_settings_result(displays_result: DisplaysSettingsResult) {
    match (
        displays_result.new_primary_display,
        displays_result.reboot_required,
    ) {
        (new_primary, true) => warn!("Primary display set to {new_primary} but the computer must be restarted for the graphics mode to work."),
        (new_primary, false) => info!("Primary display set to {new_primary}"),
    }
}
