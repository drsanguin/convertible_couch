use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::{
    device_id::DeviceIdFuzzer,
    monitor_name::MonitorNameFuzzer,
    position::{FuzzedMonitorPosition, MonitorPositionFuzzer},
    resolution::{FuzzedResolution, ResolutionFuzzer},
};

#[derive(Clone)]
pub struct FuzzedMonitor {
    pub name: String,
    pub primary: bool,
    pub config_mode_info_id: u32,
    pub device_id: String,
    pub resolution: FuzzedResolution,
    pub position: FuzzedMonitorPosition,
}

pub struct MonitorFuzzer {
    rand: StdRng,
}

impl MonitorFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(
        &mut self,
        n_monitor: usize,
        has_an_internal_display: bool,
    ) -> Vec<FuzzedMonitor> {
        let monitors_resolutions =
            ResolutionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(n_monitor);
        let positioned_resolutions =
            MonitorPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&monitors_resolutions, has_an_internal_display);
        let names = MonitorNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_monitor);
        let device_ids = DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_monitor);

        (0..n_monitor)
            .map(|monitor_index| {
                let position = positioned_resolutions[monitor_index].position;
                let resolution = positioned_resolutions[monitor_index].resolution;
                let primary = position.is_positioned_at_origin();
                let name = if has_an_internal_display && primary {
                    String::from("")
                } else {
                    names[monitor_index].to_owned()
                };
                let device_id = device_ids[monitor_index].clone();

                FuzzedMonitor {
                    config_mode_info_id: device_id.config_mode_info_id,
                    device_id: device_id.full_id,
                    name,
                    position,
                    primary,
                    resolution,
                }
            })
            .collect()
    }
}
