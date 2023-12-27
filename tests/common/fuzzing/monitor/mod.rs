use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::{
    config_mod_info_id::ConfigModeInfoIdFuzzer, gsm_id::GsmIdFuzzer,
    monitor_name::MonitorNameFuzzer, position::FuzzedMonitorPosition, resolution::FuzzedResolution,
};

#[derive(Clone)]
pub struct FuzzedMonitor {
    pub name: String,
    pub primary: bool,
    pub config_mode_info_id: u32,
    pub id: String,
    pub resolution: FuzzedResolution,
    pub position: FuzzedMonitorPosition,
}

pub struct MonitorFuzzer {
    config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer,
    gsm_id_fuzzer: GsmIdFuzzer,
    monitor_name_fuzzer: MonitorNameFuzzer,
}

impl MonitorFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            config_mode_info_id_fuzzer: ConfigModeInfoIdFuzzer::new(StdRng::seed_from_u64(seed)),
            gsm_id_fuzzer: GsmIdFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_name_fuzzer: MonitorNameFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn generate_monitor(
        &mut self,
        monitors_id_common_part_1: i32,
        monitors_id_common_part_2: &str,
        monitors_id_common_part_3: i32,
        monitors_id_common_part_4: &str,
        position: FuzzedMonitorPosition,
        resolution: FuzzedResolution,
        primary: bool,
    ) -> FuzzedMonitor {
        let monitor_config_mode_info_id = self.config_mode_info_id_fuzzer.generate_config_mode_id();
        let monitor_id_gsm_part = self.gsm_id_fuzzer.generate_gsm_id();
        let monitor_id = format!(
            r"\\?\DISPLAY#{}#{}&{}&{}&UID{}#{{{}}}",
            monitor_id_gsm_part,
            monitors_id_common_part_1,
            monitors_id_common_part_2,
            monitors_id_common_part_3,
            monitor_config_mode_info_id,
            monitors_id_common_part_4
        );

        let monitor_name = self.monitor_name_fuzzer.generate_name();

        FuzzedMonitor {
            config_mode_info_id: monitor_config_mode_info_id,
            id: monitor_id,
            name: monitor_name,
            position,
            primary,
            resolution,
        }
    }
}
