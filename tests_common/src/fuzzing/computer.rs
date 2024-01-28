use std::collections::HashMap;

use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, RngCore, SeedableRng,
};
use windows::Win32::Graphics::Gdi::DISP_CHANGE;

use super::{
    device_id::DeviceIdFuzzer,
    monitor::MonitorFuzzer,
    position::MonitorPositionFuzzer,
    resolution::ResolutionFuzzer,
    video_output::{FuzzedVideoOutput, VideoOutputFuzzer},
    win32::FuzzedWin32,
};

pub struct FuzzedComputer {
    pub win32: FuzzedWin32,
    pub primary_monitor: String,
    pub secondary_monitor: String,
    pub monitors: Vec<String>,
}

pub struct ComputerFuzzer {
    rand: StdRng,
    video_outputs: Vec<FuzzedVideoOutput>,
    change_display_settings_error_on_commit: Option<DISP_CHANGE>,
    change_display_settings_error_by_monitor: HashMap<String, DISP_CHANGE>,
    has_an_internal_display: bool,
    getting_primary_monitor_name_fails: bool,
    querying_the_display_config_of_the_primary_monitor_fails: bool,
}

impl ComputerFuzzer {
    /// According to the answer of [this question](https://learn.microsoft.com/en-us/answers/questions/1324305/what-is-the-maximum-horizontal-resolution-size-rec), Windows has a hard limit of 128 million pixels.
    /// Which implies that the theoretical maximum is 162 monitors with a 1024x768 resolution.
    const MAX_VIDEO_OUTPUTS: usize = 162;

    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            video_outputs: vec![],
            change_display_settings_error_on_commit: None,
            change_display_settings_error_by_monitor: HashMap::new(),
            has_an_internal_display: false,
            getting_primary_monitor_name_fails: false,
            querying_the_display_config_of_the_primary_monitor_fails: false,
        }
    }

    pub fn with_two_monitors_or_more(&mut self) -> &mut Self {
        self.with_a_range_of_monitors(2, Self::MAX_VIDEO_OUTPUTS, &[], &[])
    }

    pub fn with_n_monitors(&mut self, n_monitor: usize) -> &mut Self {
        self.with_a_range_of_monitors(n_monitor, n_monitor, &[], &[])
    }

    pub fn with_an_internal_display_and_at_least_one_more_monitor(&mut self) -> &mut Self {
        self.has_an_internal_display = true;
        self.with_two_monitors_or_more()
    }

    pub fn for_which_committing_the_display_changes_fails_with(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> &mut Self {
        self.change_display_settings_error_on_commit = Some(change_display_settings_error);
        self
    }

    pub fn for_which_changing_the_display_settings_fails_for_some_monitors(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> &mut Self {
        let possible_devices_paths = self
            .video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.monitor {
                Some(_) => Some(video_output.device_name.clone()),
                None => None,
            })
            .collect::<Vec<String>>();

        let n_monitor_on_error = self.rand.gen_range(1..possible_devices_paths.len());

        possible_devices_paths
            .choose_multiple(&mut self.rand, n_monitor_on_error)
            .for_each(|device_path| {
                self.change_display_settings_error_by_monitor
                    .insert(String::from(device_path), change_display_settings_error);
            });

        self
    }

    pub fn build(&self) -> FuzzedComputer {
        let secondary_monitor = self.get_monitor(false);
        let primary_monitor = self.get_monitor(true);

        assert_ne!(
            secondary_monitor, primary_monitor,
            "Error during fuzzing ! Primary and secondary monitors are the same"
        );

        let win32 = FuzzedWin32::new(
            self.video_outputs.clone(),
            self.change_display_settings_error_on_commit,
            self.change_display_settings_error_by_monitor.clone(),
            self.getting_primary_monitor_name_fails,
            self.querying_the_display_config_of_the_primary_monitor_fails,
        );

        let mut monitors = self.get_all_monitors();

        monitors.sort();

        FuzzedComputer {
            secondary_monitor,
            primary_monitor,
            win32,
            monitors,
        }
    }

    pub fn for_which_getting_the_primary_monitor_fails(&mut self) -> &mut Self {
        self.getting_primary_monitor_name_fails = true;

        self
    }

    pub fn for_which_querying_the_display_config_of_the_primary_monitor_fails(
        &mut self,
    ) -> &mut Self {
        self.querying_the_display_config_of_the_primary_monitor_fails = true;

        self
    }

    pub fn with_two_monitors_or_more_with_device_ids_different_than(
        &mut self,
        non_existing_monitor_device_path: &str,
    ) -> &mut Self {
        self.with_a_range_of_monitors(
            2,
            Self::MAX_VIDEO_OUTPUTS,
            &[non_existing_monitor_device_path],
            &[],
        )
    }

    pub fn with_two_monitors_or_more_with_names_different_than(
        &mut self,
        forbidden_monitor_names: &[&str],
    ) -> &mut Self {
        self.with_a_range_of_monitors(2, Self::MAX_VIDEO_OUTPUTS, &[], forbidden_monitor_names)
    }

    fn with_a_range_of_monitors(
        &mut self,
        min: usize,
        max: usize,
        forbidden_device_ids: &[&str],
        forbidden_monitor_names: &[&str],
    ) -> &mut Self {
        let mut monitor_fuzzer = MonitorFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()));

        let n_video_output = self.rand.gen_range(min..=max);
        let n_monitor = self.rand.gen_range(min..=n_video_output);

        let monitors_id_common_parts =
            DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_computer_common_parts();

        let monitors_resolutions =
            ResolutionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(n_monitor);

        let positioned_resolutions =
            MonitorPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&monitors_resolutions, self.has_an_internal_display);

        let mut monitors_opt = None;

        while monitors_opt.is_none() {
            let possible_monitors = monitor_fuzzer.generate_several(
                monitors_id_common_parts.part_1,
                &monitors_id_common_parts.part_2,
                monitors_id_common_parts.part_3,
                &monitors_id_common_parts.part_4,
                self.has_an_internal_display,
                &positioned_resolutions,
            );

            monitors_opt = if possible_monitors.iter().any(|possible_monitor| {
                forbidden_device_ids.contains(&possible_monitor.device_id.as_str())
                    || forbidden_monitor_names.contains(&possible_monitor.name.as_str())
            }) {
                None
            } else {
                Some(possible_monitors)
            };
        }

        let monitors = monitors_opt.unwrap();

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

        self.video_outputs = video_outputs;

        self
    }

    fn get_monitor(&self, primary: bool) -> String {
        self.video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.monitor {
                Some(monitor) => match monitor.primary {
                    p if p == primary => Some(monitor.name.clone()),
                    _ => None,
                },
                None => None,
            })
            .nth(0)
            .unwrap_or(if primary {
                String::from("<primary>")
            } else {
                String::from("<secondary>")
            })
    }

    fn get_all_monitors(&self) -> Vec<String> {
        self.video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.monitor {
                Some(monitor) => Some(monitor.name.clone()),
                None => None,
            })
            .collect::<Vec<String>>()
    }
}
