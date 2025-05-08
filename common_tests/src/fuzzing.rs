use rand::{rngs::StdRng, RngCore, SeedableRng};

use self::{
    computer::ComputerFuzzer,
    device_id::{DeviceIdFuzzer, FuzzedDeviceId},
    monitor_name::MonitorNameFuzzer,
};

pub mod computer;
pub mod config_mod_info_id;
pub mod device_id;
pub mod gsm_id;
pub mod guid;
pub mod monitor_name;
pub mod monitors;
pub mod position;
pub mod resolution;
pub mod video_output;
pub mod win32;

#[macro_export]
macro_rules! new_fuzzer {
    () => {
        convertible_couch_common_tests::fuzzing::Fuzzer::new(
            convertible_couch_common_tests::func!(),
            true,
        )
    };
}

#[macro_export]
macro_rules! new_fuzzer_no_seed_print {
    () => {
        convertible_couch_common_tests::fuzzing::Fuzzer::new(
            convertible_couch_common_tests::func!(),
            false,
        )
    };
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

    pub fn generate_computer(&mut self) -> ComputerFuzzer {
        ComputerFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
    }

    pub fn generate_monitor_name(&mut self) -> String {
        MonitorNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one()
    }

    pub fn generate_two_monitor_names(&mut self) -> (String, String) {
        MonitorNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_two()
    }

    pub fn generate_device_id(&mut self) -> FuzzedDeviceId {
        DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one()
    }
}
