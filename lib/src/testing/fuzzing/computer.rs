use rand::rngs::StdRng;

use crate::testing::fuzzing::{
    displays::{settings_api::CurrentFuzzedDisplaysSettingsApi, DisplaysFuzzer},
    speakers::{settings_api::CurrentFuzzedSpeakersSettingsApi, SpeakersFuzzer},
};

pub struct FuzzedComputer {
    pub displays_settings_api: CurrentFuzzedDisplaysSettingsApi,
    pub speakers_settings_api: CurrentFuzzedSpeakersSettingsApi,
}

pub struct ComputerFuzzer {
    pub rand: StdRng,
    displays_settings_api: CurrentFuzzedDisplaysSettingsApi,
    speakers_settings_api: CurrentFuzzedSpeakersSettingsApi,
}

impl ComputerFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            displays_settings_api: CurrentFuzzedDisplaysSettingsApi::default(),
            speakers_settings_api: CurrentFuzzedSpeakersSettingsApi::default(),
        }
    }

    pub fn set_displays_settings_api(
        &mut self,
        displays_settings_api: CurrentFuzzedDisplaysSettingsApi,
    ) -> &mut Self {
        self.displays_settings_api = displays_settings_api;

        self
    }

    pub fn set_speakers_settings_api(
        &mut self,
        speakers_settings_api: CurrentFuzzedSpeakersSettingsApi,
    ) -> &mut Self {
        self.speakers_settings_api = speakers_settings_api;

        self
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer<'_> {
        DisplaysFuzzer::new(self)
    }

    pub fn with_speakers(&mut self) -> SpeakersFuzzer<'_> {
        SpeakersFuzzer::new(self)
    }

    pub fn build_computer(&mut self) -> FuzzedComputer {
        FuzzedComputer {
            displays_settings_api: self.displays_settings_api.clone(),
            speakers_settings_api: self.speakers_settings_api.clone(),
        }
    }
}
