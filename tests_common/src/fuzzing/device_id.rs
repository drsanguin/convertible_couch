use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    Rng, RngCore, SeedableRng,
};

use super::{config_mod_info_id::ConfigModeInfoIdFuzzer, gsm_id::GsmIdFuzzer, guid::GuidFuzzer};

pub struct DeviceIdFuzzer {
    rand: StdRng,
    gsm_id_fuzzer: GsmIdFuzzer,
    guid_fuzzer: GuidFuzzer,
    config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer,
}

impl DeviceIdFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            rand,
            gsm_id_fuzzer: GsmIdFuzzer::new(StdRng::seed_from_u64(seed)),
            guid_fuzzer: GuidFuzzer::new(StdRng::seed_from_u64(seed)),
            config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer::new(StdRng::seed_from_u64(seed)),
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

    pub fn generate(&mut self) -> String {
        let gsm_id = self.gsm_id_fuzzer.generate_gsm_id();
        let monitors_id_common_part_1 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_2 =
            Alphanumeric.sample_string(&mut self.rand, 6).to_lowercase();
        let monitors_id_common_part_3 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_4 = self.guid_fuzzer.generate_uuid();
        let config_mode_info_id = self.config_mode_info_id_fuzzer.generate_config_mode_ids(1)[0];

        self.generate_using_common_parts(
            &gsm_id,
            monitors_id_common_part_1,
            &monitors_id_common_part_2,
            monitors_id_common_part_3,
            &monitors_id_common_part_4,
            config_mode_info_id,
        )
    }
}
