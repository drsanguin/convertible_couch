use rand::{rngs::StdRng, RngCore, SeedableRng};

use self::{computer::ComputerFuzzer, device_id::DeviceIdFuzzer, monitor_name::MonitorNameFuzzer};

pub mod computer;
pub mod config_mod_info_id;
pub mod device_id;
pub mod gsm_id;
pub mod guid;
pub mod monitor;
pub mod monitor_name;
pub mod position;
pub mod resolution;
pub mod video_output;
pub mod win32;

#[macro_export]
macro_rules! new_fuzzer {
    () => {{
        use convertible_couch_tests_common::func;
        use convertible_couch_tests_common::fuzzing::Fuzzer;

        Fuzzer::new(func!(), true)
    }};
}

#[macro_export]
macro_rules! new_fuzzer_no_seed_print {
    () => {{
        use convertible_couch_tests_common::func;
        use convertible_couch_tests_common::fuzzing::Fuzzer;

        Fuzzer::new(func!(), false)
    }};
}

#[macro_export]
macro_rules! func {
    () => {{
        use convertible_couch_tests_common::fuzzing::Fuzzer;
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        &name[19..name.len() - 3]
    }};
}

pub struct Fuzzer {
    rand: StdRng,
}

impl Fuzzer {
    pub fn new(test_name: &str, print_seed: bool) -> Self {
        let seed = StdRng::from_entropy().next_u64();

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

    pub fn generate_device_id(&mut self) -> String {
        DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one()
    }
}
