use std::collections::HashSet;

use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};

pub struct GsmIdFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> GsmIdFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> String {
        let hexa = Alphanumeric.sample_string(self.rand, 4).to_uppercase();
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
