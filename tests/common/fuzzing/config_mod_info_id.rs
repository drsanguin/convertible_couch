use std::collections::HashSet;

use rand::{rngs::StdRng, Rng};

pub struct ConfigModeInfoIdFuzzer {
    rand: StdRng,
    config_mode_ids: HashSet<u32>,
}

impl ConfigModeInfoIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            config_mode_ids: HashSet::new(),
        }
    }

    pub fn generate_config_mode_id(&mut self) -> u32 {
        let mut config_mode_id_opt = None;

        while config_mode_id_opt.is_none() {
            let config_mode_id = self.rand.gen_range(0..99999);

            config_mode_id_opt = if !self.config_mode_ids.contains(&config_mode_id) {
                Some(config_mode_id)
            } else {
                None
            };
        }

        let config_mode_id = config_mode_id_opt.unwrap();
        self.config_mode_ids.insert(config_mode_id);

        config_mode_id
    }
}
