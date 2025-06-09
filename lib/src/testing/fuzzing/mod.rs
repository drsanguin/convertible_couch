use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::testing::fuzzing::display_settings::{
    device_id::{DeviceIdFuzzer, FuzzedDeviceId},
    display_name::DisplayNameFuzzer,
};

use self::computer::ComputerFuzzer;

pub mod computer;
pub mod display_settings;
pub mod guid;
pub mod sound_settings;

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

    pub fn generate_computer(&mut self) -> ComputerFuzzer {
        ComputerFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
    }

    pub fn generate_display_name(&mut self) -> String {
        DisplayNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one()
    }

    pub fn generate_two_display_names(&mut self) -> (String, String) {
        DisplayNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_two()
    }

    pub fn generate_device_id(&mut self) -> FuzzedDeviceId {
        DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one()
    }
}
