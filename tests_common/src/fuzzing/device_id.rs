use std::fmt::{Display, Formatter};

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

#[derive(Clone)]
pub struct DeviceId {
    pub config_mode_info_id: u32,
    pub full_id: String,
}

pub struct DeviceIdFuzzer {
    rand: StdRng,
}

impl DeviceId {
    pub fn new(
        gsm_id: &str,
        monitors_id_part_1: i32,
        monitors_id_part_2: &str,
        monitors_id_part_3: i32,
        monitors_id_part_4: &str,
        config_mode_info_id: u32,
    ) -> Self {
        Self {
            config_mode_info_id,
            full_id: format!(
                r"\\?\DISPLAY#{gsm_id}#{monitors_id_part_1}&{monitors_id_part_2}&{monitors_id_part_3}&UID{:0>5}#{{{monitors_id_part_4}}}",
                config_mode_info_id
            ),
        }
    }
}

impl Display for DeviceId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.full_id)
    }
}

impl DeviceIdFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> DeviceId {
        self.generate_several(1)[0].clone()
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<DeviceId> {
        let monitor_id_gsm_parts =
            GsmIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_several(count);

        let monitors_id_common_parts =
            DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_computer_common_parts();

        let config_mode_info_ids =
            ConfigModeInfoIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(count);

        (0..count)
            .map(|monitor_index| {
                let config_mode_info_id = config_mode_info_ids[monitor_index];
                let monitor_id_gsm_part = &monitor_id_gsm_parts[monitor_index];

                Self::generate_from_parts(
                    monitor_id_gsm_part,
                    monitors_id_common_parts.part_1,
                    &monitors_id_common_parts.part_2,
                    monitors_id_common_parts.part_3,
                    &monitors_id_common_parts.part_4,
                    config_mode_info_id,
                )
            })
            .collect()
    }

    fn generate_from_parts(
        gsm_id: &str,
        monitors_id_part_1: i32,
        monitors_id_part_2: &str,
        monitors_id_part_3: i32,
        monitors_id_part_4: &str,
        config_mode_info_id: u32,
    ) -> DeviceId {
        DeviceId::new(
            gsm_id,
            monitors_id_part_1,
            monitors_id_part_2,
            monitors_id_part_3,
            monitors_id_part_4,
            config_mode_info_id,
        )
    }

    fn generate_computer_common_parts(&mut self) -> CommonDeviceIdPartsByComputer {
        let part_1 = self.rand.gen_range(0..=9);
        let part_2 = Alphanumeric.sample_string(&mut self.rand, 6).to_lowercase();
        let part_3 = self.rand.gen_range(0..=9);
        let part_4 = GuidFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64())).generate_one();

        CommonDeviceIdPartsByComputer {
            part_1,
            part_2,
            part_3,
            part_4,
        }
    }
}
