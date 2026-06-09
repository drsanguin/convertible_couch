use convertible_couch::application::{ApplicationInfoResult, CommandResult};
use convertible_couch_lib::{
    application_result::ApplicationResult, displays_settings::DisplayInfo,
    speakers_settings::SpeakerInfo,
};

#[derive(Default)]
pub struct InfoResultBuilder;

impl InfoResultBuilder {
    pub fn displays_and_speakers(self) -> InfoDisplaysAndSpeakersResultBuilder {
        InfoDisplaysAndSpeakersResultBuilder::default()
    }

    pub fn displays_only(self) -> InfoDisplaysOnlyResultBuilder {
        InfoDisplaysOnlyResultBuilder::default()
    }

    pub fn speakers_only(self) -> InfoSpeakersOnlyResultBuilder {
        InfoSpeakersOnlyResultBuilder::default()
    }
}

#[derive(Default)]
pub struct InfoDisplaysAndSpeakersResultBuilder {
    displays_result: Vec<DisplayInfo>,
    speakers_result: Vec<SpeakerInfo>,
}

impl InfoDisplaysAndSpeakersResultBuilder {
    pub fn with_primary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: true,
            name: display_name.to_string(),
            number: 1,
        });

        self
    }

    pub fn with_secondary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: false,
            name: display_name.to_string(),
            number: 2,
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
        let application_info_result = ApplicationInfoResult::DisplaysAndSpeakers {
            displays_result: self.displays_result,
            speakers_result: self.speakers_result,
        };
        let command_result = CommandResult::Info(application_info_result);

        Ok(command_result)
    }
}

#[derive(Default)]
pub struct InfoDisplaysOnlyResultBuilder {
    displays_result: Vec<DisplayInfo>,
}

impl InfoDisplaysOnlyResultBuilder {
    pub fn with_primary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: true,
            name: display_name.to_string(),
            number: 1,
        });

        self
    }

    pub fn with_secondary_display(mut self, display_name: &str) -> Self {
        self.displays_result.push(DisplayInfo {
            is_primary: false,
            name: display_name.to_string(),
            number: 2,
        });

        self
    }

    pub fn build(self) -> ApplicationResult<CommandResult> {
        let application_info_result = ApplicationInfoResult::DisplaysOnly {
            displays_result: self.displays_result,
        };
        let command_result = CommandResult::Info(application_info_result);

        Ok(command_result)
    }
}

#[derive(Default)]
pub struct InfoSpeakersOnlyResultBuilder {
    speakers_result: Vec<SpeakerInfo>,
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
        let application_info_result = ApplicationInfoResult::SpeakersOnly {
            speakers_result: self.speakers_result,
        };
        let command_result = CommandResult::Info(application_info_result);

        Ok(command_result)
    }
}
