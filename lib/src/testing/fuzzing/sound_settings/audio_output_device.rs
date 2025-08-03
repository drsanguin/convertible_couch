use std::collections::HashSet;

use rand::{rngs::StdRng, Rng};

use crate::testing::fuzzing::{
    computer::ComputerFuzzer,
    sound_settings::{
        audio_output_device_id::AudioOutputDeviceIdFuzzer,
        audio_output_device_name::AudioOutputDeviceNameFuzzer,
    },
};

#[derive(Clone)]
pub struct FuzzedAudioOutputDevice {
    pub name: String,
    pub id: String,
    pub is_default: bool,
}

pub struct AudioOutputDeviceFuzzer {
    rand: StdRng,
    computer_fuzzer: ComputerFuzzer,
    count: usize,
    default_audio_output_device_name: Option<String>,
    alternative_names: HashSet<String>,
}

impl AudioOutputDeviceFuzzer {
    pub fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            count: 0,
            default_audio_output_device_name: None,
            alternative_names: HashSet::new(),
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> &mut Self {
        self.count = count;

        self
    }

    pub fn whose_default_one_is_name(
        &mut self,
        default_audio_output_device_name: String,
    ) -> &mut Self {
        self.default_audio_output_device_name = Some(default_audio_output_device_name);

        self
    }

    pub fn with_an_alternative_one_named(
        &mut self,
        alternative_audio_output_device_name: String,
    ) -> &mut Self {
        self.alternative_names
            .insert(alternative_audio_output_device_name);

        self
    }

    pub fn build_audio_output_devices(&mut self) -> ComputerFuzzer {
        let mut names_already_taken = HashSet::new();

        if self.default_audio_output_device_name.is_some() {
            let default_audio_output_device_name =
                self.default_audio_output_device_name.clone().unwrap();
            names_already_taken.insert(default_audio_output_device_name);
        }

        names_already_taken.extend(self.alternative_names.clone());

        let names_not_taken = AudioOutputDeviceNameFuzzer::new(&mut self.rand).generate_several(
            self.count - names_already_taken.len(),
            names_already_taken.clone(),
        );

        let mut names = Vec::with_capacity(self.count);
        names.extend(names_already_taken);
        names.extend(names_not_taken);

        let ids = AudioOutputDeviceIdFuzzer::new(&mut self.rand).generate_several(self.count);

        let default_output_device_index = if self.default_audio_output_device_name.is_some() {
            let default_audio_output_device_name =
                self.default_audio_output_device_name.clone().unwrap();

            names
                .iter()
                .position(|name| name == &default_audio_output_device_name)
                .unwrap()
        } else {
            self.rand.random_range(0..self.count)
        };

        let audio_output_devices = (0..self.count)
            .map(|i| FuzzedAudioOutputDevice {
                name: names[i].clone(),
                id: ids[i].clone(),
                is_default: i == default_output_device_index,
            })
            .collect::<Vec<FuzzedAudioOutputDevice>>();

        ComputerFuzzer::new_with_audio_output_devices(
            &mut self.computer_fuzzer,
            audio_output_devices,
        )
    }
}
