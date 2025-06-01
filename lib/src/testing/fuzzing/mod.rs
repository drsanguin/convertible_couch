use rand::{rngs::StdRng, RngCore, SeedableRng};

use self::{
    computer::ComputerFuzzer,
    device_id::{DeviceIdFuzzer, FuzzedDeviceId},
    display_name::DisplayNameFuzzer,
};

pub mod computer;
pub mod config_mod_info_id;
pub mod device_id;
pub mod display_name;
pub mod displays;
pub mod gsm_id;
pub mod guid;
pub mod position;
pub mod resolution;
pub mod video_output;
pub mod win32;

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
