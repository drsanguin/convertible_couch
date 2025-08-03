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
}

impl AudioOutputDeviceFuzzer {
    pub fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            count: 0,
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> &mut Self {
        self.count = count;

        self
    }

    pub fn build_audio_output_devices(&mut self) -> ComputerFuzzer {
        let names = AudioOutputDeviceNameFuzzer::new(&mut self.rand).generate_several(self.count);
        let ids = AudioOutputDeviceIdFuzzer::new(&mut self.rand).generate_several(self.count);
        let default_output_device_index = self.rand.random_range(0..self.count);

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
