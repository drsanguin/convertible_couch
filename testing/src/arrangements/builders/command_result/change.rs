use convertible_couch::application::{ApplicationChangeResult, CommandResult};
use convertible_couch_lib::{
    application_result::ApplicationResult, displays_settings::DisplaysSettingsResult,
    speakers_settings::SpeakersSettingsResult,
};

use crate::arrangements::builders::arguments::ChangeDisplaysCommand;

#[derive(Default)]
pub struct ChangeResultBuilder;

impl ChangeResultBuilder {
    pub fn displays_and_speakers(
        self,
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        let speakers_result = SpeakersSettingsResult {
            new_default_speaker: new_default_speaker.to_string(),
        };
        let displays_result = DisplaysSettingsResult {
            new_primary_display: new_primary_display.to_string(),
        };
        let application_change_result = ApplicationChangeResult::DisplaysAndSpeakers {
            displays_result,
            speakers_result,
        };
        let command_result = CommandResult::Change(application_change_result);

        Ok(command_result)
    }

    pub fn displays_only(self, new_primary_display: &str) -> ApplicationResult<CommandResult> {
        let displays_result = DisplaysSettingsResult {
            new_primary_display: new_primary_display.to_string(),
        };
        let application_change_result = ApplicationChangeResult::DisplaysOnly { displays_result };
        let command_result = CommandResult::Change(application_change_result);

        Ok(command_result)
    }

    pub fn speakers_only(self, new_default_speaker: &str) -> ApplicationResult<CommandResult> {
        let speakers_result = SpeakersSettingsResult {
            new_default_speaker: new_default_speaker.to_string(),
        };
        let application_change_result = ApplicationChangeResult::SpeakersOnly { speakers_result };
        let command_result = CommandResult::Change(application_change_result);

        Ok(command_result)
    }

    pub fn displays(
        self,
        change_displays_command: &ChangeDisplaysCommand,
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        match change_displays_command {
            ChangeDisplaysCommand::ChangeDisplaysAndSpeakers => {
                self.displays_and_speakers(new_primary_display, new_default_speaker)
            }
            ChangeDisplaysCommand::ChangeDisplays => self.displays_only(new_primary_display),
        }
    }
}
