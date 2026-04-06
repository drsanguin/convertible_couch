use convertible_couch::application::{ApplicationChangeResult, CommandResult};
use convertible_couch_lib::{
    application_result::ApplicationResult, displays_settings::DisplaysSettingsResult,
    speakers_settings::SpeakersSettingsResult,
};

use crate::arrangements::builders::arguments::ChangeDisplaysCommand;

pub struct CommandResultBuilder;

impl CommandResultBuilder {
    pub fn change_displays(
        change_displays_command: &ChangeDisplaysCommand,
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        let result = match change_displays_command {
            ChangeDisplaysCommand::ChangeDisplaysAndSpeakers => {
                CommandResult::Change(ApplicationChangeResult::DisplaysAndSpeakers {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: new_primary_display.to_string(),
                        reboot_required: false,
                    },
                    speakers_result: SpeakersSettingsResult {
                        new_default_speaker: new_default_speaker.to_string(),
                    },
                })
            }
            ChangeDisplaysCommand::ChangeDisplays => {
                CommandResult::Change(ApplicationChangeResult::DisplaysOnly {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: new_primary_display.to_string(),
                        reboot_required: false,
                    },
                })
            }
        };

        Ok(result)
    }
}
