use rand::{rngs::StdRng, seq::index::sample};

pub struct ConfigModeInfoIdFuzzer {
    rand: StdRng,
}

impl ConfigModeInfoIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_config_mode_ids(&mut self, count: usize) -> Vec<u32> {
        sample(&mut self.rand, 99999, count)
            .iter()
            .map(|indice| u32::try_from(indice).unwrap())
            .collect()
    }
}
