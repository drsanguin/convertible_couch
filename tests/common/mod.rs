use rand::{rngs::StdRng, RngCore, SeedableRng};

use self::fuzzing::{computer::ComputerFuzzer, monitor_name::MonitorNameFuzzer};

mod fuzzing;
mod utils;

#[macro_export]
macro_rules! new_fuzzer {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        Fuzzer::new(&name[19..name.len() - 3])
    }};
}

pub struct Fuzzer {
    computer_fuzzer: ComputerFuzzer,
    monitor_name_fuzzer: MonitorNameFuzzer,
}

impl Fuzzer {
    pub fn new(test_name: &str) -> Self {
        let mut seeder = StdRng::from_entropy();
        let seed = seeder.next_u64();

        println!("seed {} ... {}", test_name, seed);

        Self {
            computer_fuzzer: ComputerFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_name_fuzzer: MonitorNameFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn generate_a_computer(&mut self) -> &mut ComputerFuzzer {
        &mut self.computer_fuzzer
    }

    pub fn generate_monitor_name(&mut self) -> String {
        self.monitor_name_fuzzer.generate_name()
    }
}
