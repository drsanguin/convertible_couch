use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
};

pub struct GuidFuzzer {
    rand: StdRng,
}

impl GuidFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_uuid(&mut self) -> String {
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
}
