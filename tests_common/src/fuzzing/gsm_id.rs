use std::collections::HashSet;

use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
};

pub struct GsmIdFuzzer {
    rand: StdRng,
    gsm_ids: HashSet<String>,
}

impl GsmIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            gsm_ids: HashSet::new(),
        }
    }

    pub fn generate_gsm_id(&mut self) -> String {
        let mut gsm_id_opt = None;

        while gsm_id_opt.is_none() {
            let hexa = Alphanumeric.sample_string(&mut self.rand, 4).to_uppercase();
            let gsm_id = format!("GSM{hexa}");

            gsm_id_opt = if !self.gsm_ids.contains(&gsm_id) {
                Some(gsm_id)
            } else {
                None
            };
        }

        let gsm_id = gsm_id_opt.unwrap();
        self.gsm_ids.insert(gsm_id.clone());

        gsm_id
    }

    pub fn generate_gsm_ids(&mut self, count: usize) -> Vec<String> {
        (0..count).map(|_| self.generate_gsm_id()).collect()
    }
}
