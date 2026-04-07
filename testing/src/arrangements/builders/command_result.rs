use convertible_couch::application::{
    ApplicationChangeResult, ApplicationInfoResult, CommandResult,
};
use convertible_couch_lib::{
    application_error::ApplicationError,
    application_result::ApplicationResult,
    displays_settings::{DisplayInfo, DisplaysSettingsResult},
    speakers_settings::{SpeakerInfo, SpeakersSettingsResult},
};

use crate::arrangements::builders::arguments::ChangeDisplaysCommand;

pub struct CommandResultBuilder;

impl CommandResultBuilder {
    pub fn change_displays_and_speakers(
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Change(
            ApplicationChangeResult::DisplaysAndSpeakers {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: new_primary_display.to_string(),
                },
                speakers_result: SpeakersSettingsResult {
                    new_default_speaker: new_default_speaker.to_string(),
                },
            },
        ))
    }

    pub fn change_displays_only(new_primary_display: &str) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Change(
            ApplicationChangeResult::DisplaysOnly {
                displays_result: DisplaysSettingsResult {
                    new_primary_display: new_primary_display.to_string(),
                },
            },
        ))
    }

    pub fn change_speakers_only(new_default_speaker: &str) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Change(
            ApplicationChangeResult::SpeakersOnly {
                speakers_result: SpeakersSettingsResult {
                    new_default_speaker: new_default_speaker.to_string(),
                },
            },
        ))
    }

    pub fn change_displays(
        change_displays_command: &ChangeDisplaysCommand,
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        match change_displays_command {
            ChangeDisplaysCommand::ChangeDisplaysAndSpeakers => {
                Self::change_displays_and_speakers(new_primary_display, new_default_speaker)
            }
            ChangeDisplaysCommand::ChangeDisplays => {
                Self::change_displays_only(new_primary_display)
            }
        }
    }

    pub fn info_displays_and_speakers(
        displays_result: Vec<DisplayInfo>,
        speakers_result: Vec<SpeakerInfo>,
    ) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Info(
            ApplicationInfoResult::DisplaysAndSpeakers {
                displays_result,
                speakers_result,
            },
        ))
    }

    pub fn info_displays_only(
        displays_result: Vec<DisplayInfo>,
    ) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Info(ApplicationInfoResult::DisplaysOnly {
            displays_result,
        }))
    }

    pub fn info_speakers_only(
        speakers_result: Vec<SpeakerInfo>,
    ) -> ApplicationResult<CommandResult> {
        Ok(CommandResult::Info(ApplicationInfoResult::SpeakersOnly {
            speakers_result,
        }))
    }

    pub fn custom_error(expected_message: String) -> ApplicationResult<CommandResult> {
        Err(ApplicationError::Custom(expected_message))
    }
}
