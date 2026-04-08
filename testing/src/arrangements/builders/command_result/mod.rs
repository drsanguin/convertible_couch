use convertible_couch::application::{
    ApplicationChangeResult, ApplicationInfoResult, CommandResult,
};
use convertible_couch_lib::{
    application_error::ApplicationError,
    application_result::ApplicationResult,
    displays_settings::{DisplayInfo, DisplaysSettingsResult},
    speakers_settings::{SpeakerInfo, SpeakersSettingsResult},
};

use crate::arrangements::builders::{
    arguments::DisplaysCommand,
    command_result::{change::ChangeResultBuilder, info::InfoResultBuilder},
};

pub mod change;
pub mod info;

pub struct CommandResultBuilder;

impl CommandResultBuilder {
    pub fn change() -> ChangeResultBuilder {
        ChangeResultBuilder
    }

    pub fn info() -> InfoResultBuilder {
        InfoResultBuilder
    }

    pub fn displays(
        displays_command: &DisplaysCommand,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> ApplicationResult<CommandResult> {
        let command_result = match displays_command {
            DisplaysCommand::ChangeDisplaysAndSpeakers => {
                CommandResult::Change(ApplicationChangeResult::DisplaysAndSpeakers {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: couch_display_name.to_string(),
                    },
                    speakers_result: SpeakersSettingsResult {
                        new_default_speaker: couch_speaker_name.to_string(),
                    },
                })
            }
            DisplaysCommand::ChangeDisplays => {
                CommandResult::Change(ApplicationChangeResult::DisplaysOnly {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: couch_display_name.to_string(),
                    },
                })
            }
            DisplaysCommand::InfoDisplaysAndSpeakers => {
                CommandResult::Info(ApplicationInfoResult::DisplaysAndSpeakers {
                    displays_result: vec![
                        DisplayInfo {
                            name: desktop_display_name.to_string(),
                            is_primary: true,
                        },
                        DisplayInfo {
                            name: couch_display_name.to_string(),
                            is_primary: false,
                        },
                    ],
                    speakers_result: vec![
                        SpeakerInfo {
                            name: desktop_speaker_name.to_string(),
                            is_default: true,
                        },
                        SpeakerInfo {
                            name: couch_speaker_name.to_string(),
                            is_default: false,
                        },
                    ],
                })
            }
            DisplaysCommand::InfoDisplays => {
                CommandResult::Info(ApplicationInfoResult::DisplaysOnly {
                    displays_result: vec![
                        DisplayInfo {
                            name: desktop_display_name.to_string(),
                            is_primary: true,
                        },
                        DisplayInfo {
                            name: couch_display_name.to_string(),
                            is_primary: false,
                        },
                    ],
                })
            }
        };

        Ok(command_result)
    }

    pub fn custom_error(expected_message: String) -> ApplicationResult<CommandResult> {
        let application_error = ApplicationError::Custom(expected_message);

        Err(application_error)
    }
}
