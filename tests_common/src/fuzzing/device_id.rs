use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    Rng, RngCore, SeedableRng,
};

use super::{config_mod_info_id::ConfigModeInfoIdFuzzer, gsm_id::GsmIdFuzzer, guid::GuidFuzzer};

pub struct CommonDeviceIdPartsByComputer {
    pub part_1: i32,
    pub part_2: String,
    pub part_3: i32,
    pub part_4: String,
}

pub struct DeviceIdFuzzer {
    rand: StdRng,
    gsm_id_fuzzer: GsmIdFuzzer,
    guid_fuzzer: GuidFuzzer,
    config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer,
}

impl DeviceIdFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let gsm_id_fuzzer = GsmIdFuzzer::new(StdRng::seed_from_u64(rand.next_u64()));
        let guid_fuzzer = GuidFuzzer::new(StdRng::seed_from_u64(rand.next_u64()));
        let config_mode_info_id_fuzzer =
            ConfigModeInfoIdFuzzer::new(StdRng::seed_from_u64(rand.next_u64()));

        Self {
            rand,
            gsm_id_fuzzer,
            guid_fuzzer,
            config_mode_info_id_fuzzer,
        }
    }

    pub fn generate_using_common_parts(
        &mut self,
        gsm_id: &str,
        monitors_id_part_1: i32,
        monitors_id_part_2: &str,
        monitors_id_part_3: i32,
        monitors_id_part_4: &str,
        config_mode_info_id: u32,
    ) -> String {
        format!(
            r"\\?\DISPLAY#{gsm_id}#{monitors_id_part_1}&{monitors_id_part_2}&{monitors_id_part_3}&UID{:0>5}#{{{monitors_id_part_4}}}",
            config_mode_info_id
        )
    }

    pub fn generate_one(&mut self) -> String {
        let gsm_id = self.gsm_id_fuzzer.generate_one();
        let monitors_id_common_parts = self.generate_computer_common_parts();
        let config_mode_info_id = self.config_mode_info_id_fuzzer.generate_one();

        self.generate_using_common_parts(
            &gsm_id,
            monitors_id_common_parts.part_1,
            &monitors_id_common_parts.part_2,
            monitors_id_common_parts.part_3,
            &monitors_id_common_parts.part_4,
            config_mode_info_id,
        )
    }

    pub fn generate_computer_common_parts(&mut self) -> CommonDeviceIdPartsByComputer {
        let part_1 = self.rand.gen_range(0..=9);
        let part_2 = Alphanumeric.sample_string(&mut self.rand, 6).to_lowercase();
        let part_3 = self.rand.gen_range(0..=9);
        let part_4 = self.guid_fuzzer.generate_one();

        CommonDeviceIdPartsByComputer {
            part_1,
            part_2,
            part_3,
            part_4,
        }
    }
}
