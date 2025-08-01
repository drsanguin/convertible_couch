use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
    Rng,
};

use crate::testing::fuzzing::guid::GuidFuzzer;

use super::{config_mod_info_id::ConfigModeInfoIdFuzzer, gsm_id::GsmIdFuzzer};

pub struct CommonDeviceIdPartsByComputer {
    pub part_1: i32,
    pub part_2: String,
    pub part_3: i32,
    pub uuid: String,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct FuzzedDeviceId {
    pub uuid: String,
    pub config_mode_info_id: u32,
    pub full_id: String,
}

pub struct DeviceIdFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl FuzzedDeviceId {
    pub fn new(
        gsm_id: &str,
        displays_id_part_1: i32,
        displays_id_part_2: &str,
        displays_id_part_3: i32,
        uuid: &str,
        config_mode_info_id: u32,
    ) -> Self {
        Self {
            uuid: String::from(uuid),
            config_mode_info_id,
            full_id: format!(
                r"\\?\DISPLAY#{gsm_id}#{displays_id_part_1}&{displays_id_part_2}&{displays_id_part_3}&UID{:0>5}#{{{uuid}}}",
                config_mode_info_id
            ),
        }
    }
}

impl Display for FuzzedDeviceId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.full_id)
    }
}

impl<'a> DeviceIdFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> FuzzedDeviceId {
        self.generate_several(1, &HashSet::new()).remove(0)
    }

    pub fn generate_several(
        &mut self,
        count: usize,
        forbidden_device_ids: &HashSet<&FuzzedDeviceId>,
    ) -> Vec<FuzzedDeviceId> {
        let display_id_gsm_parts = GsmIdFuzzer::new(self.rand).generate_several(count);

        let displays_id_common_parts =
            DeviceIdFuzzer::new(self.rand).generate_computer_common_parts(forbidden_device_ids);

        let config_mode_info_ids = ConfigModeInfoIdFuzzer::new(self.rand).generate_several(count);

        (0..count)
            .map(|display_index| {
                let config_mode_info_id = config_mode_info_ids[display_index];
                let display_id_gsm_part = &display_id_gsm_parts[display_index];

                FuzzedDeviceId::new(
                    display_id_gsm_part,
                    displays_id_common_parts.part_1,
                    &displays_id_common_parts.part_2,
                    displays_id_common_parts.part_3,
                    &displays_id_common_parts.uuid,
                    config_mode_info_id,
                )
            })
            .collect()
    }

    fn generate_computer_common_parts(
        &mut self,
        forbidden_device_ids: &HashSet<&FuzzedDeviceId>,
    ) -> CommonDeviceIdPartsByComputer {
        let forbidden_uuids = HashSet::<&str>::from_iter(
            forbidden_device_ids
                .iter()
                .map(|forbidden_device_id| forbidden_device_id.uuid.as_str()),
        );

        let part_1 = self.rand.random_range(0..=9);
        let part_2 = Alphanumeric.sample_string(self.rand, 6).to_lowercase();
        let part_3 = self.rand.random_range(0..=9);
        let uuid = GuidFuzzer::new(self.rand).generate_one_different_than(&forbidden_uuids);

        CommonDeviceIdPartsByComputer {
            part_1,
            part_2,
            part_3,
            uuid,
        }
    }
}
