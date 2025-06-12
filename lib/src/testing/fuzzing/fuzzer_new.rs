use std::collections::HashSet;

use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

#[cfg(target_os = "windows")]
use crate::testing::fuzzing::{
    display_settings::win_32::FuzzedWin32,
    sound_settings::audio_endpoint_library::FuzzedAudioEndpointLibrary,
};
use crate::testing::fuzzing::{
    display_settings::{
        device_id::DeviceIdFuzzer, display_name::DisplayNameFuzzer, displays::FuzzedDisplay,
        position::DisplayPositionFuzzer, resolution::ResolutionFuzzer,
        video_output::VideoOutputFuzzer,
    },
    sound_settings::audio_output_device_name::AudioOutputDeviceNameFuzzer,
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
    count: usize,
    primary_name: Option<&'a str>,
    secondaries_names: HashSet<&'a str>,
    includes_an_internal_display: bool,
}

impl<'a> DisplaysFuzzer<'a> {
    fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            count: 0,
            primary_name: None,
            secondaries_names: HashSet::new(),
            includes_an_internal_display: false,
        }
    }

    pub fn of_which_there_are(self, count: usize) -> Self {
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
        let mut video_outputs = VideoOutputFuzzer::generate_several(self.count);
        let displays = self.generateDisplays();

        todo!()
    }

    fn generateDisplays(&mut self) -> Vec<FuzzedDisplay> {
        let displays_resolutions =
            ResolutionFuzzer::new(&mut self.rand).generate_several(self.count);
        let positioned_resolutions =
            DisplayPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&displays_resolutions, self.includes_an_internal_display);
        let names_to_generate_count = self.count
            - self.secondaries_names.len()
            - if self.primary_name.is_some() { 1 } else { 0 };
        let mut names = DisplayNameFuzzer::new(&mut self.rand)
            .generate_several(names_to_generate_count, &self.secondaries_names);
        let device_ids =
            DeviceIdFuzzer::new(&mut self.rand).generate_several(self.count, &HashSet::new());

        for secondary_name in self.secondaries_names.iter() {
            names.push(secondary_name.to_string());
        }

        if self.primary_name.is_some() {
            let primary_index = positioned_resolutions
                .iter()
                .position(|positioned_resolution| {
                    positioned_resolution.position.is_positioned_at_origin()
                })
                .unwrap();

            let primary_name = self.primary_name.unwrap().to_string();

            if primary_index == (names.len() - 1) {
                names.push(primary_name);
            } else {
                names.insert(primary_index, primary_name);
            }
        }

        (0..self.count)
            .map(|display_index| {
                let position = positioned_resolutions[display_index].position;
                let resolution = positioned_resolutions[display_index].resolution;
                let primary = position.is_positioned_at_origin();
                let name = if self.includes_an_internal_display && primary {
                    String::from("")
                } else {
                    names[display_index].to_owned()
                };
                let device_id = device_ids[display_index].clone();

                FuzzedDisplay {
                    config_mode_info_id: device_id.config_mode_info_id,
                    device_id: device_id.full_id,
                    name,
                    position,
                    primary,
                    resolution,
                }
            })
            .collect()
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
