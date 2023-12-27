use rand::{rngs::StdRng, Rng};

pub struct ConfigModeInfoIdFuzzer {
    rand: StdRng,
}

impl ConfigModeInfoIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_config_mode_id(&mut self) -> u32 {
        self.rand.gen_range(10000..=99999)
    }
}
