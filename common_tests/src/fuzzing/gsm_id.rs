use std::collections::HashSet;

use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
};

pub struct GsmIdFuzzer {
    rand: StdRng,
}

impl GsmIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> String {
        let hexa = Alphanumeric.sample_string(&mut self.rand, 4).to_uppercase();
        format!("GSM{hexa}")
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<String> {
        let mut gsm_ids = HashSet::with_capacity(count);

        while gsm_ids.len() != count {
            let gsm_id = self.generate_one();
            gsm_ids.insert(gsm_id);
        }

        Vec::from_iter(gsm_ids)
    }
}
