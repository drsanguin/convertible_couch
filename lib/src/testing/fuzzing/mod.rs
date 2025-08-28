use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::{
    computer::FuzzedComputer,
    displays::{
        device_id::{DeviceIdFuzzer, FuzzedDeviceId},
        display_name::DisplayNameFuzzer,
    },
    speakers::speaker_name::SpeakerNameFuzzer,
};

use self::computer::ComputerFuzzer;

pub mod computer;
pub mod displays;
pub mod guid;
pub mod speakers;

pub trait ComputerBuilder<'a> {
    fn build_computer(&'a mut self) -> FuzzedComputer;
}

pub struct Fuzzer {
    rand: StdRng,
}

impl Fuzzer {
    pub fn new(test_name: &str, print_seed: bool) -> Self {
        let seed = StdRng::from_os_rng().next_u64();

        if print_seed {
            println!("seed {test_name} ... {seed}");
        }

        Self {
            rand: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_computer(&mut self) -> ComputerFuzzer<'_> {
        ComputerFuzzer::new(&mut self.rand)
    }

    pub fn generate_display_name(&mut self) -> String {
        DisplayNameFuzzer::new(&mut self.rand).generate_one()
    }

    pub fn generate_two_display_names(&mut self) -> (String, String) {
        DisplayNameFuzzer::new(&mut self.rand).generate_two()
    }

    pub fn generate_three_display_names(&mut self) -> (String, String, String) {
        DisplayNameFuzzer::new(&mut self.rand).generate_three()
    }

    pub fn generate_four_display_names(&mut self) -> (String, String, String, String) {
        DisplayNameFuzzer::new(&mut self.rand).generate_four()
    }

    pub fn generate_device_id(&mut self) -> FuzzedDeviceId {
        DeviceIdFuzzer::new(&mut self.rand).generate_one()
    }

    pub fn generate_two_speakers_names(&mut self) -> (String, String) {
        SpeakerNameFuzzer::new(&mut self.rand).generate_two()
    }

    pub fn generate_three_speakers_names(&mut self) -> (String, String, String) {
        SpeakerNameFuzzer::new(&mut self.rand).generate_three()
    }

    pub fn generate_four_speakers_names(&mut self) -> (String, String, String, String) {
        SpeakerNameFuzzer::new(&mut self.rand).generate_four()
    }
}
