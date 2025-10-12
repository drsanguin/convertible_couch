use clap::Parser;
use convertible_couch::{
    application::{Application, ApplicationChangeResult, ApplicationInfoResult, ApplicationResult},
    commands::Arguments,
};
use convertible_couch_lib::{
    displays_settings::{
        CurrentDisplaysSettings, CurrentDisplaysSettingsApi, DisplaysSettingsResult,
    },
    speakers_settings::{
        CurrentSpeakersSettings, CurrentSpeakersSettingsApi, SpeakersSettingsResult,
    },
    DeviceInfo,
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
                ApplicationResult::Change(application_change_result) => {
                    match application_change_result {
                        ApplicationChangeResult::DisplaysAndSpeakers {
                            displays_result,
                            speakers_result,
                        } => {
                            log_change_displays_settings_result(displays_result);
                            log_change_speakers_settings_result(speakers_result);
                        }
                        ApplicationChangeResult::DisplaysOnly { displays_result } => {
                            log_change_displays_settings_result(displays_result)
                        }
                        ApplicationChangeResult::SpeakersOnly { speakers_result } => {
                            log_change_speakers_settings_result(speakers_result)
                        }
                    }
                }
                ApplicationResult::Info(application_info_result) => match application_info_result {
                    ApplicationInfoResult::DisplaysAndSpeakers {
                        displays_result,
                        speakers_result,
                    } => {
                        log_info_displays_settings_result(displays_result);
                        log_info_speakers_settings_result(speakers_result);
                    }
                    ApplicationInfoResult::DisplaysOnly { displays_result } => {
                        log_info_displays_settings_result(displays_result)
                    }
                    ApplicationInfoResult::SpeakersOnly { speakers_result } => {
                        log_info_speakers_settings_result(speakers_result)
                    }
                },
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");

            ExitCode::FAILURE
        }
    }
}

fn log_change_speakers_settings_result(speakers_result: SpeakersSettingsResult) {
    info!(
        "Default speaker set to {0}",
        speakers_result.new_default_speaker
    );
}

fn log_change_displays_settings_result(displays_result: DisplaysSettingsResult) {
    match (
        displays_result.new_primary_display,
        displays_result.reboot_required,
    ) {
        (new_primary, true) => warn!("Primary display set to {new_primary} but the computer must be restarted for the graphics mode to work."),
        (new_primary, false) => info!("Primary display set to {new_primary}"),
    }
}

fn log_info_displays_settings_result(displays_result: Vec<DeviceInfo>) {
    let displays_list = displays_result
        .iter()
        .map(|device| device.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    info!("Displays: {displays_list}")
}

fn log_info_speakers_settings_result(speakers_result: Vec<DeviceInfo>) {
    let speakers_list = speakers_result
        .iter()
        .map(|device| device.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    info!("Speakers: {speakers_list}")
}
