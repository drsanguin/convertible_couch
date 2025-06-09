use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

use crate::testing::fuzzing::sound_settings::{
    audio_endpoint_id::AudioEndpointIdFuzzer, audio_endpoint_name::AudioEndpointNameFuzzer,
};

#[derive(Clone)]
pub struct FuzzedAudioEndpoint {
    pub name: String,
    pub id: String,
    pub is_default: bool,
}

pub struct AudioEndpointFuzzer {
    rand: StdRng,
}

impl AudioEndpointFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<FuzzedAudioEndpoint> {
        let names = AudioEndpointNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(count);
        let ids = AudioEndpointIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(count);
        let default_output_device_index = self.rand.random_range(0..count);

        (0..count)
            .map(|i| FuzzedAudioEndpoint {
                name: names[i].clone(),
                id: ids[i].clone(),
                is_default: i == default_output_device_index,
            })
            .collect()
    }
}
