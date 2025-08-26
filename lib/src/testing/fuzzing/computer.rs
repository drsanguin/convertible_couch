use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::{
    displays::{
        settings_api::{CurrentFuzzedDisplaysSettingsApi, FuzzedDisplaysSettingsApi},
        DisplaysFuzzer,
    },
    speakers::{
        settings_api::{CurrentFuzzedSpeakersSettingsApi, FuzzedSpeakersSettingsApi},
        SpeakersFuzzer,
    },
};

pub struct FuzzedComputer {
    pub displays_settings_api: CurrentFuzzedDisplaysSettingsApi,
    pub speakers_settings_api: CurrentFuzzedSpeakersSettingsApi,
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
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

    pub fn new_with_display_settings_api(
        computer_fuzzer: &mut ComputerFuzzer,
        displays_settings_api: CurrentFuzzedDisplaysSettingsApi,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            displays_settings_api,
            speakers_settings_api: computer_fuzzer.speakers_settings_api.clone(),
        }
    }

    pub fn new_with_speakers(
        computer_fuzzer: &mut ComputerFuzzer,
        speakers_settings_api: CurrentFuzzedSpeakersSettingsApi,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            displays_settings_api: computer_fuzzer.displays_settings_api.clone(),
            speakers_settings_api,
        }
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer<'_> {
        DisplaysFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    pub fn with_speakers(&mut self) -> SpeakersFuzzer {
        SpeakersFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    pub fn build_computer(&mut self) -> FuzzedComputer {
        FuzzedComputer {
            displays_settings_api: self.displays_settings_api.clone(),
            speakers_settings_api: self.speakers_settings_api.clone(),
        }
    }
}
