use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::guid::GuidFuzzer;

pub struct AudioOutputDeviceIdFuzzer {
    rand: StdRng,
}

impl AudioOutputDeviceIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<String> {
        GuidFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(count)
            .iter()
            .map(|guid| format!("{{0.0.0.00000000}}.{guid}"))
            .collect()
    }
}
