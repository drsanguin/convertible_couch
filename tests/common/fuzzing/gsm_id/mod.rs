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

    pub fn generate_gsm_id(&mut self) -> String {
        let hexa = Alphanumeric.sample_string(&mut self.rand, 4).to_uppercase();

        format!("GSM{}", hexa)
    }
}
