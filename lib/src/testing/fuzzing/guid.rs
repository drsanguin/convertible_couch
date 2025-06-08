use std::collections::HashSet;

use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};

pub struct GuidFuzzer {
    rand: StdRng,
}

impl GuidFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> String {
        let low_time = Alphanumeric.sample_string(&mut self.rand, 8).to_lowercase();
        let mid_time = Alphanumeric.sample_string(&mut self.rand, 4).to_lowercase();
        let high_time_and_version = Alphanumeric.sample_string(&mut self.rand, 4).to_lowercase();
        let clock_sequence_and_variant =
            Alphanumeric.sample_string(&mut self.rand, 4).to_lowercase();
        let node = Alphanumeric
            .sample_string(&mut self.rand, 12)
            .to_lowercase();

        format!("{low_time}-{mid_time}-{high_time_and_version}-{clock_sequence_and_variant}-{node}")
    }

    pub fn generate_one_different_than(&mut self, forbidden_uuids: &HashSet<&str>) -> String {
        loop {
            let uuid = self.generate_one();

            if forbidden_uuids.contains(uuid.as_str()) {
                continue;
            }

            return uuid;
        }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<String> {
        let mut guids = HashSet::with_capacity(count);

        while guids.len() != count {
            let guid = self.generate_one();

            guids.insert(guid);
        }

        Vec::from_iter(guids)
    }
}
