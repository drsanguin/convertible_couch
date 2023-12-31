use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    seq::IteratorRandom,
    Rng, RngCore, SeedableRng,
};
use windows::Win32::Graphics::Gdi::DISP_CHANGE;

use super::{
    guid::GuidFuzzer,
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
    pub video_outputs: Vec<FuzzedVideoOutput>,
    pub monitor_fuzzer: MonitorFuzzer,
    change_display_settings_error: Option<DISP_CHANGE>,
    rand: StdRng,
    guid_fuzzer: GuidFuzzer,
    resolution_fuzzer: ResolutionFuzzer,
    monitor_position_fuzzer: MonitorPositionFuzzer,
}

impl ComputerFuzzer {
    /// According to the answer of this question https://learn.microsoft.com/en-us/answers/questions/1324305/what-is-the-maximum-horizontal-resolution-size-rec Windows has a hard limit of 128 million pixels.
    /// Which implies that the theoretical maximum is 162 monitors with a 1024x768 resolution.
    const MAX_VIDEO_OUTPUTS: usize = 162;

    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            rand,
            video_outputs: vec![],
            change_display_settings_error: None,
            guid_fuzzer: GuidFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_fuzzer: MonitorFuzzer::new(StdRng::seed_from_u64(seed)),
            resolution_fuzzer: ResolutionFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_position_fuzzer: MonitorPositionFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn with_two_monitors_or_more(&mut self) -> &mut Self {
        self.with_a_range_of_monitors(2, Self::MAX_VIDEO_OUTPUTS)
    }

    pub fn with_n_monitors(&mut self, n_monitor: usize) -> &mut Self {
        self.with_a_range_of_monitors(n_monitor, n_monitor)
    }

    pub fn for_which_changing_the_display_settings_fails_with(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> &mut Self {
        self.change_display_settings_error = Some(change_display_settings_error);
        self
    }

    pub fn build_computer(&self) -> FuzzedComputer {
        let secondary_monitor = self.get_monitor(false);
        let primary_monitor = self.get_monitor(true);

        assert_ne!(
            secondary_monitor, primary_monitor,
            "Error during fuzzing ! Primary and secondary monitors are the same"
        );

        let win32 = FuzzedWin32::new(
            self.video_outputs.clone(),
            self.change_display_settings_error,
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

    fn with_a_range_of_monitors(&mut self, min: usize, max: usize) -> &mut Self {
        let n_video_output = self.rand.gen_range(min..=max);
        let n_monitor = self.rand.gen_range(min..=n_video_output);

        let monitors_id_common_part_1 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_2 =
            Alphanumeric.sample_string(&mut self.rand, 6).to_lowercase();
        let monitors_id_common_part_3 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_4 = self.guid_fuzzer.generate_uuid();

        let monitors_resolutions = self.resolution_fuzzer.generate_resolutions(n_monitor);

        let positioned_resolutions = self
            .monitor_position_fuzzer
            .generate_positions(&monitors_resolutions);

        let monitors = self.monitor_fuzzer.generate_monitors(
            monitors_id_common_part_1,
            &monitors_id_common_part_2,
            monitors_id_common_part_3,
            &monitors_id_common_part_4,
            &positioned_resolutions,
        );

        assert_eq!(
            monitors.iter().filter(|monitor| monitor.primary).count(),
            1,
            "More than one primary monitor has been generated"
        );

        let mut video_outputs = VideoOutputFuzzer::generate_video_outputs(n_video_output);

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
            .unwrap()
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
