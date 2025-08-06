use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::{
    display_settings::displays::DisplaysFuzzer,
    sound_settings::audio_output_device::AudioOutputDeviceFuzzer,
};
#[cfg(target_os = "windows")]
use crate::testing::fuzzing::{
    display_settings::win_32::FuzzedWin32,
    sound_settings::audio_endpoint_library::FuzzedAudioEndpointLibrary,
};

pub struct FuzzedComputer {
    #[cfg(target_os = "windows")]
    pub display_settings_api: FuzzedWin32,
    #[cfg(target_os = "windows")]
    pub audio_settings_api: FuzzedAudioEndpointLibrary,
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
    display_settings_api: FuzzedWin32,
    audio_settings_api: FuzzedAudioEndpointLibrary,
}

impl ComputerFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            display_settings_api: FuzzedWin32::default(),
            audio_settings_api: FuzzedAudioEndpointLibrary::default(),
        }
    }

    pub fn new_with_display_settings_api(
        computer_fuzzer: &mut ComputerFuzzer,
        display_settings_api: FuzzedWin32,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            display_settings_api,
            audio_settings_api: computer_fuzzer.audio_settings_api.clone(),
        }
    }

    pub fn new_with_audio_output_devices(
        computer_fuzzer: &mut ComputerFuzzer,
        audio_settings_api: FuzzedAudioEndpointLibrary,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            display_settings_api: computer_fuzzer.display_settings_api.clone(),
            audio_settings_api: audio_settings_api,
        }
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer {
        DisplaysFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    pub fn with_audio_output_devices(&mut self) -> AudioOutputDeviceFuzzer {
        AudioOutputDeviceFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    pub fn build_computer(&mut self) -> FuzzedComputer {
        FuzzedComputer {
            display_settings_api: self.display_settings_api.clone(),
            audio_settings_api: self.audio_settings_api.clone(),
        }
    }
}
