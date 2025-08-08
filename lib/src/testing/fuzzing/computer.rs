use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::{
    displays_settings::displays::DisplaysFuzzer, speakers_settings::speakers::SpeakersFuzzer,
};
#[cfg(target_os = "windows")]
use crate::testing::fuzzing::{
    displays_settings::win_32::FuzzedWin32,
    speakers_settings::audio_endpoint_library::FuzzedAudioEndpointLibrary,
};

pub struct FuzzedComputer {
    #[cfg(target_os = "windows")]
    pub displays_settings_api: FuzzedWin32,
    #[cfg(target_os = "windows")]
    pub speakers_settings_api: FuzzedAudioEndpointLibrary,
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
    displays_settings_api: FuzzedWin32,
    speakers_settings_api: FuzzedAudioEndpointLibrary,
}

impl ComputerFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            displays_settings_api: FuzzedWin32::default(),
            speakers_settings_api: FuzzedAudioEndpointLibrary::default(),
        }
    }

    pub fn new_with_display_settings_api(
        computer_fuzzer: &mut ComputerFuzzer,
        displays_settings_api: FuzzedWin32,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            displays_settings_api,
            speakers_settings_api: computer_fuzzer.speakers_settings_api.clone(),
        }
    }

    pub fn new_with_speakers(
        computer_fuzzer: &mut ComputerFuzzer,
        speakers_settings_api: FuzzedAudioEndpointLibrary,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            displays_settings_api: computer_fuzzer.displays_settings_api.clone(),
            speakers_settings_api,
        }
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer {
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
