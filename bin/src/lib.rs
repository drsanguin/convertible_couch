use convertible_couch_lib::{
    displays_settings::{
        CurrentDisplaysSettingsApiTrait, DisplaysSettings, DisplaysSettingsResult,
    },
    log::configure_logger,
    speakers_settings::{
        CurrentSpeakersSettingsApiTrait, SpeakersSettings, SpeakersSettingsResult,
    },
};

use crate::commands::{Arguments, Commands};

pub mod commands;

#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationResult {
    DisplaysAndSpeakers {
        displays_result: DisplaysSettingsResult,
        speakers_result: SpeakersSettingsResult,
    },
    DisplaysOnly {
        displays_result: DisplaysSettingsResult,
    },
    SpeakersOnly {
        speakers_result: SpeakersSettingsResult,
    },
}

pub fn run_app<
    TDisplaysSettingsApi: CurrentDisplaysSettingsApiTrait,
    TSpeakersSettingsApi: CurrentSpeakersSettingsApiTrait,
    TDisplaysSettings: DisplaysSettings<TDisplaysSettingsApi>,
    TSpeakersSettings: SpeakersSettings<TSpeakersSettingsApi>,
>(
    args: &Arguments,
    displays_settings: &mut TDisplaysSettings,
    speakers_settings: &mut TSpeakersSettings,
) -> Result<ApplicationResult, String> {
    match &args.command {
        Commands::DisplaysAndSpeakers {
            displays,
            speakers,
            shared,
        } => {
            configure_logger(&shared.log_level)?;

            let displays_result = displays_settings.change_primary_display(
                &displays.desktop_display_name,
                &displays.couch_display_name,
            )?;

            let speakers_result = speakers_settings.change_default_speaker(
                &speakers.desktop_speaker_name,
                &speakers.couch_speaker_name,
            )?;

            Ok(ApplicationResult::DisplaysAndSpeakers {
                displays_result,
                speakers_result,
            })
        }
        Commands::DisplaysOnly { displays, shared } => {
            configure_logger(&shared.log_level)?;

            let displays_result = displays_settings.change_primary_display(
                &displays.desktop_display_name,
                &displays.couch_display_name,
            )?;

            Ok(ApplicationResult::DisplaysOnly { displays_result })
        }
        Commands::SpeakersOnly { speakers, shared } => {
            configure_logger(&shared.log_level)?;

            let speakers_result = speakers_settings.change_default_speaker(
                &speakers.desktop_speaker_name,
                &speakers.couch_speaker_name,
            )?;

            Ok(ApplicationResult::SpeakersOnly { speakers_result })
        }
    }
}
