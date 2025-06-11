use rand::{rngs::StdRng, RngCore, SeedableRng};

#[cfg(target_os = "windows")]
use crate::testing::fuzzing::{
    display_settings::win_32::FuzzedWin32,
    sound_settings::audio_endpoint_library::FuzzedAudioEndpointLibrary,
};

pub struct FuzzerNew {
    rand: StdRng,
}

impl FuzzerNew {
    pub fn new(test_name: &str, print_seed: bool) -> Self {
        let seed = StdRng::from_os_rng().next_u64();

        if print_seed {
            println!("seed {test_name} ... {seed}");
        }

        Self {
            rand: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_two_display_names(&self) -> (String, String) {
        todo!()
    }

    pub fn generate_two_audio_output_devices_names(&self) -> (String, String) {
        todo!()
    }

    pub fn generate_computer(&mut self) -> ComputerFuzzer {
        todo!()
    }
}

pub struct ComputerFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> ComputerFuzzer<'a> {
    pub fn with_displays(&mut self) -> DisplaysFuzzer {
        todo!()
    }

    pub fn with_audio_output_devices(&mut self) -> AudioOutputDevicesFuzzer {
        todo!()
    }

    pub fn build_computer(&mut self) -> FuzzedComputer {
        todo!()
    }
}

pub struct FuzzedComputer {
    #[cfg(target_os = "windows")]
    pub display_settings_api: FuzzedWin32,
    #[cfg(target_os = "windows")]
    pub audio_settings_api: FuzzedAudioEndpointLibrary,
}

pub struct DisplaysFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> DisplaysFuzzer<'a> {
    pub fn of_which_there_are(&mut self, displays_count: u32) -> Self {
        todo!()
    }

    pub fn whose_primary_is_name(&mut self, primary_display_name: &str) -> Self {
        todo!()
    }

    pub fn with_a_secondary_named(&mut self, secondary_display_name: &str) -> Self {
        todo!()
    }

    pub fn build_displays(&mut self) -> ComputerFuzzer {
        todo!()
    }
}

pub struct AudioOutputDevicesFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> AudioOutputDevicesFuzzer<'a> {
    pub fn of_which_there_are(&mut self, audio_output_devices_count: u32) -> Self {
        todo!()
    }

    pub fn whose_default_one_is_name(&mut self, default_audio_output_device_name: &str) -> Self {
        todo!()
    }

    pub fn with_an_alternative_one_named(
        &mut self,
        alternative_audio_output_device_name: &str,
    ) -> Self {
        todo!()
    }

    pub fn build_audio_output_devices(&mut self) -> ComputerFuzzer {
        todo!()
    }
}
