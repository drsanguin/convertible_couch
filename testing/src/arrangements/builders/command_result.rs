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

    pub fn info_displays_and_speakers() -> InfoDisplaysAndSpeakersResultBuilder {
        InfoDisplaysAndSpeakersResultBuilder::default()
    }

    pub fn info_displays_only() -> InfoDisplaysOnlyResultBuilder {
        InfoDisplaysOnlyResultBuilder::default()
    }

    pub fn info_speakers_only() -> InfoSpeakersOnlyResultBuilder {
        InfoSpeakersOnlyResultBuilder::default()
    }

    pub fn custom_error(expected_message: String) -> ApplicationResult<CommandResult> {
        Err(ApplicationError::Custom(expected_message))
    }
}

pub struct InfoDisplaysAndSpeakersResultBuilder {
    displays_result: Vec<DisplayInfo>,
    speakers_result: Vec<SpeakerInfo>,
}

impl Default for InfoDisplaysAndSpeakersResultBuilder {
    fn default() -> Self {
        Self {
            displays_result: vec![],
            speakers_result: vec![],
        }
    }
}

impl InfoDisplaysAndSpeakersResultBuilder {
    pub fn with_primary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: true,
            name: display_name.to_string(),
        });

        self
    }

    pub fn with_secondary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: false,
            name: display_name.to_string(),
        });

        self
    }

    pub fn with_default_speaker(mut self, speaker_name: &str) -> Self {
        self.speakers_result.push(SpeakerInfo {
            is_default: true,
            name: speaker_name.to_string(),
        });

        self
    }

    pub fn with_alternative_speaker(mut self, speaker_name: &str) -> Self {
        self.speakers_result.push(SpeakerInfo {
            is_default: false,
            name: speaker_name.to_string(),
        });

        self
    }

    pub fn build(self) -> ApplicationResult<CommandResult> {
        // CommandResultBuilder::info_displays_and_speakers(self.displays_result, self.speakers_result)
        Ok(CommandResult::Info(
            ApplicationInfoResult::DisplaysAndSpeakers {
                displays_result: self.displays_result,
                speakers_result: self.speakers_result,
            },
        ))
    }
}

pub struct InfoDisplaysOnlyResultBuilder {
    displays_result: Vec<DisplayInfo>,
}

impl Default for InfoDisplaysOnlyResultBuilder {
    fn default() -> Self {
        Self {
            displays_result: vec![],
        }
    }
}

impl InfoDisplaysOnlyResultBuilder {
    pub fn with_primary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: true,
            name: display_name.to_string(),
        });

        self
    }

    pub fn with_secondary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: false,
            name: display_name.to_string(),
        });

        self
    }

    pub fn build(self) -> ApplicationResult<CommandResult> {
        // CommandResultBuilder::info_displays_and_speakers(self.displays_result, self.speakers_result)
        Ok(CommandResult::Info(ApplicationInfoResult::DisplaysOnly {
            displays_result: self.displays_result,
        }))
    }
}

pub struct InfoSpeakersOnlyResultBuilder {
    speakers_result: Vec<SpeakerInfo>,
}

impl Default for InfoSpeakersOnlyResultBuilder {
    fn default() -> Self {
        Self {
            speakers_result: vec![],
        }
    }
}

impl InfoSpeakersOnlyResultBuilder {
    pub fn with_default_speaker(mut self, speaker_name: &str) -> Self {
        self.speakers_result.push(SpeakerInfo {
            is_default: true,
            name: speaker_name.to_string(),
        });

        self
    }

    pub fn with_alternative_speaker(mut self, speaker_name: &str) -> Self {
        self.speakers_result.push(SpeakerInfo {
            is_default: false,
            name: speaker_name.to_string(),
        });

        self
    }

    pub fn build(self) -> ApplicationResult<CommandResult> {
        // CommandResultBuilder::info_displays_and_speakers(self.displays_result, self.speakers_result)
        Ok(CommandResult::Info(ApplicationInfoResult::SpeakersOnly {
            speakers_result: self.speakers_result,
        }))
    }
}
