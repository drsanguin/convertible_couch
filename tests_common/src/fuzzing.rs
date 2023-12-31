use rand::{rngs::StdRng, RngCore, SeedableRng};

use self::computer::ComputerFuzzer;

pub mod computer;
pub mod config_mod_info_id;
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
    ($expression:expr) => {{
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
    computer_fuzzer: ComputerFuzzer,
}

impl Fuzzer {
    pub fn new(test_name: &str, print_seed: bool) -> Self {
        let mut seeder = StdRng::from_entropy();
        let seed = seeder.next_u64();

        if print_seed {
            println!("seed {test_name} ... {seed}");
        }

        Self {
            computer_fuzzer: ComputerFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn generate_a_computer(&mut self) -> &mut ComputerFuzzer {
        &mut self.computer_fuzzer
    }

    pub fn generate_monitor_name(&mut self) -> String {
        self.computer_fuzzer
            .monitor_fuzzer
            .monitor_name_fuzzer
            .generate_name()
    }
}
