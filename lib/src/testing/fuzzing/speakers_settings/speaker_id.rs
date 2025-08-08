use rand::rngs::StdRng;

use crate::testing::fuzzing::guid::GuidFuzzer;

pub struct SpeakerIdFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> SpeakerIdFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<String> {
        GuidFuzzer::new(self.rand)
            .generate_several(count)
            .iter()
            .map(|guid| format!("{{0.0.0.00000000}}.{guid}"))
            .collect()
    }
}
