use rand::{rngs::StdRng, seq::index::sample, RngCore};

pub struct ConfigModeInfoIdFuzzer {
    rand: StdRng,
}

impl ConfigModeInfoIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> u32 {
        self.rand.next_u32()
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<u32> {
        sample(&mut self.rand, 99999, count)
            .iter()
            .map(|indice| u32::try_from(indice).unwrap())
            .collect()
    }
}
