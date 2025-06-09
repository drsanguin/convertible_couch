use rand::{rngs::StdRng, Rng};

use crate::testing::fuzzing::sound_settings::{
    audio_output_device_id::AudioOutputDeviceIdFuzzer,
    audio_output_device_name::AudioOutputDeviceNameFuzzer,
};

#[derive(Clone)]
pub struct FuzzedAudioOutputDevice {
    pub name: String,
    pub id: String,
    pub is_default: bool,
}

pub struct AudioOutputDeviceFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> AudioOutputDeviceFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<FuzzedAudioOutputDevice> {
        let names = AudioOutputDeviceNameFuzzer::new(self.rand).generate_several(count);
        let ids = AudioOutputDeviceIdFuzzer::new(self.rand).generate_several(count);
        let default_output_device_index = self.rand.random_range(0..count);

        (0..count)
            .map(|i| FuzzedAudioOutputDevice {
                name: names[i].clone(),
                id: ids[i].clone(),
                is_default: i == default_output_device_index,
            })
            .collect()
    }
}
