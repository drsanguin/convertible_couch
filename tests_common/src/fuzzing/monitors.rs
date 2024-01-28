use std::collections::HashSet;

use rand::{rngs::StdRng, seq::IteratorRandom, Rng, RngCore, SeedableRng};

use crate::fuzzing::video_output::VideoOutputFuzzer;

use super::{
    computer::ComputerFuzzer,
    device_id::{DeviceId, DeviceIdFuzzer},
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

pub struct MonitorsFuzzer<'a> {
    rand: StdRng,
    computer_fuzzer: ComputerFuzzer,
    min_n_monitor: usize,
    max_n_monitor: usize,
    includes_an_internal_display: bool,
    forbidden_monitor_names: HashSet<&'a str>,
    forbidden_device_ids: HashSet<&'a DeviceId>,
}

impl<'a> MonitorsFuzzer<'a> {
    /// According to the answer of [this question](https://learn.microsoft.com/en-us/answers/questions/1324305/what-is-the-maximum-horizontal-resolution-size-rec), Windows has a hard limit of 128 million pixels.
    /// Which implies that the theoretical maximum is 162 monitors with a 1024x768 resolution.
    const MAX_N_MONITOR: usize = 162;

    pub fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            max_n_monitor: 0,
            min_n_monitor: 0,
            includes_an_internal_display: false,
            forbidden_monitor_names: HashSet::new(),
            forbidden_device_ids: HashSet::new(),
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_monitor: count,
            max_n_monitor: count,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_monitor_names: self.forbidden_monitor_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn of_which_there_are_at_least(&mut self, count: usize) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_monitor: count,
            max_n_monitor: Self::MAX_N_MONITOR,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_monitor_names: self.forbidden_monitor_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn including_an_internal_display(&mut self) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_monitor: self.min_n_monitor,
            max_n_monitor: self.max_n_monitor,
            includes_an_internal_display: true,
            forbidden_monitor_names: self.forbidden_monitor_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn whose_names_are_different_from(
        &mut self,
        forbidden_monitor_names: HashSet<&'a str>,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_monitor: self.min_n_monitor,
            max_n_monitor: self.max_n_monitor,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_monitor_names,
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn whose_device_ids_are_different_from(
        &mut self,
        forbidden_device_ids: HashSet<&'a DeviceId>,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_monitor: self.min_n_monitor,
            max_n_monitor: self.max_n_monitor,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_monitor_names: self.forbidden_monitor_names.clone(),
            forbidden_device_ids,
        }
    }

    pub fn build_monitors(&mut self) -> ComputerFuzzer {
        let n_video_output = self.rand.gen_range(self.min_n_monitor..=self.max_n_monitor);
        let n_monitor = self.rand.gen_range(self.min_n_monitor..=n_video_output);

        let monitors = self.generate_several(n_monitor);

        assert_eq!(
            monitors.iter().filter(|monitor| monitor.primary).count(),
            1,
            "More than one primary monitor has been generated"
        );

        let mut video_outputs = VideoOutputFuzzer::generate_several(n_video_output);

        let mut video_outputs_to_plug_in_indexes = video_outputs
            .iter()
            .enumerate()
            .map(|(index, _video_output)| index)
            .choose_multiple(&mut self.rand, n_monitor);

        video_outputs_to_plug_in_indexes.sort();

        video_outputs_to_plug_in_indexes
            .iter()
            .enumerate()
            .for_each(|(monitor_index, video_output_index)| {
                let monitor = monitors[monitor_index].to_owned();

                video_outputs[*video_output_index] =
                    video_outputs[*video_output_index].plug_monitor(monitor);
            });

        ComputerFuzzer::new_with_video_outputs(&mut self.computer_fuzzer, video_outputs)
    }

    fn generate_several(&mut self, n_monitor: usize) -> Vec<FuzzedMonitor> {
        let monitors_resolutions =
            ResolutionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(n_monitor);
        let positioned_resolutions =
            MonitorPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&monitors_resolutions, self.includes_an_internal_display);
        let names = MonitorNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_monitor, &self.forbidden_monitor_names);
        let device_ids = DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_monitor, &self.forbidden_device_ids);

        (0..n_monitor)
            .map(|monitor_index| {
                let position = positioned_resolutions[monitor_index].position;
                let resolution = positioned_resolutions[monitor_index].resolution;
                let primary = position.is_positioned_at_origin();
                let name = if self.includes_an_internal_display && primary {
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
