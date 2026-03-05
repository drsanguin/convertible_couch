use clap::Parser;
use convertible_couch::{
    application::{Application, ApplicationChangeResult, ApplicationInfoResult, ApplicationResult},
    commands::Arguments,
};
use convertible_couch_lib::{
    displays_settings::{CurrentDisplaysSettingsApi, DisplayInfo, DisplaysSettingsResult},
    speakers_settings::{CurrentSpeakersSettingsApi, SpeakerInfo, SpeakersSettingsResult},
};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Arguments::parse();

    let displays_settings_api = Box::new(CurrentDisplaysSettingsApi);
    let speakers_settings_api = Box::new(CurrentSpeakersSettingsApi);
    let mut application = Application::bootstrap(displays_settings_api, speakers_settings_api);

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
            eprintln!("{error}");

            ExitCode::FAILURE
        }
    }
}

fn log_change_speakers_settings_result(speakers_result: SpeakersSettingsResult) {
    println!(
        "Default speaker set to {0}",
        speakers_result.new_default_speaker
    );
}

fn log_change_displays_settings_result(displays_result: DisplaysSettingsResult) {
    match (
        displays_result.new_primary_display,
        displays_result.reboot_required,
    ) {
        (new_primary, true) => println!("Primary display set to {new_primary} but the computer must be restarted for the graphics mode to work."),
        (new_primary, false) => println!("Primary display set to {new_primary}"),
    }
}

fn log_info_displays_settings_result(displays_result: Vec<DisplayInfo>) {
    let max_name_length = displays_result
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(4);

    println!("Displays");
    println!("PRIMARY NAME");

    println!("{}", "-".repeat(8 + max_name_length));

    for display_result in displays_result {
        println!(
            "{:<7} {:<max_name_length$}",
            display_result.is_primary,
            display_result.name,
            max_name_length = max_name_length
        )
    }
    println!();
}

fn log_info_speakers_settings_result(speakers_result: Vec<SpeakerInfo>) {
    let max_name_length = speakers_result
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(4);

    println!("Speakers");
    println!("DEFAULT NAME");

    println!("{}", "-".repeat(8 + max_name_length));

    for display_result in speakers_result {
        println!(
            "{:<7} {:<max_name_length$}",
            display_result.is_default,
            display_result.name,
            max_name_length = max_name_length
        )
    }
    println!();
}
