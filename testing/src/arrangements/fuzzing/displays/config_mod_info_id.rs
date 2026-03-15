use rand::{rngs::StdRng, seq::index::sample};

pub struct ConfigModeInfoIdFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> ConfigModeInfoIdFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<u32> {
        sample(self.rand, 99999, count)
            .iter()
            .map(|indice| u32::try_from(indice).unwrap())
            .collect()
    }
}
