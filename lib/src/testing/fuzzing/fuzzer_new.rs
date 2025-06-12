use std::{collections::HashSet, ptr::null};

use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::{
    display_settings::display_name::DisplayNameFuzzer,
    sound_settings::audio_output_device_name::AudioOutputDeviceNameFuzzer,
};
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

    pub fn generate_two_display_names(&mut self) -> (String, String) {
        DisplayNameFuzzer::new(&mut self.rand).generate_two()
    }

    pub fn generate_two_audio_output_devices_names(&mut self) -> (String, String) {
        AudioOutputDeviceNameFuzzer::new(&mut self.rand).generate_two()
    }

    pub fn generate_computer(&mut self) -> ComputerFuzzer {
        let seed = self.rand.next_u64();
        let rand = StdRng::seed_from_u64(seed);

        ComputerFuzzer::new(rand)
    }
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
}

impl ComputerFuzzer {
    fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer {
        let seed = self.rand.next_u64();
        let rand = StdRng::seed_from_u64(seed);
        let computer_fuzzer = self.clone();

        DisplaysFuzzer::new(rand, computer_fuzzer)
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
    rand: StdRng,
    computer_fuzzer: ComputerFuzzer,
    count: u32,
    primary_name: Option<&'a str>,
    secondaries_names: HashSet<&'a str>,
}

impl<'a> DisplaysFuzzer<'a> {
    fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            count: 0,
            primary_name: None,
            secondaries_names: HashSet::new(),
        }
    }

    pub fn of_which_there_are(self, count: u32) -> Self {
        Self { count, ..self }
    }

    pub fn whose_primary_is_name(self, primary_name: &'a str) -> Self {
        Self {
            primary_name: Some(primary_name),
            ..self
        }
    }

    pub fn with_a_secondary_named(self, secondary_name: &'a str) -> Self {
        let mut secondaries_names = HashSet::from_iter(self.secondaries_names);
        secondaries_names.insert(secondary_name);

        Self {
            secondaries_names,
            ..self
        }
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
