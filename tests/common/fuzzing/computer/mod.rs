use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    seq::IteratorRandom,
    Rng, RngCore, SeedableRng,
};

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
    pub reboot_required: bool,
    rand: StdRng,
    guid_fuzzer: GuidFuzzer,
    resolution_fuzzer: ResolutionFuzzer,
    monitor_fuzzer: MonitorFuzzer,
}

impl ComputerFuzzer {
    const MAX_VIDEO_OUTPUTS: usize = 5;

    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            rand,
            video_outputs: vec![],
            reboot_required: false,
            guid_fuzzer: GuidFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_fuzzer: MonitorFuzzer::new(StdRng::seed_from_u64(seed)),
            resolution_fuzzer: ResolutionFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn with_two_monitors_or_more(&mut self) -> &mut Self {
        let min_n_monitor = 2;
        let n_video_output = self.rand.gen_range(min_n_monitor..=Self::MAX_VIDEO_OUTPUTS);
        let n_monitor = self.rand.gen_range(min_n_monitor..=n_video_output);
        let primary_monitor_number = self.rand.gen_range(1..=n_monitor);

        let monitors_id_common_part_1 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_2 =
            Alphanumeric.sample_string(&mut self.rand, 6).to_lowercase();
        let monitors_id_common_part_3 = self.rand.gen_range(0..=9);
        let monitors_id_common_part_4 = self.guid_fuzzer.generate_uuid();

        let monitors_resolutions = self.resolution_fuzzer.generate_resolutions(n_monitor);

        let monitors_positions = MonitorPositionFuzzer::generate_positions(
            &monitors_resolutions,
            primary_monitor_number,
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
                let position = monitors_positions[monitor_index];
                let resolution = monitors_resolutions[monitor_index];
                let primary = monitor_index + 1 == primary_monitor_number;

                if primary {
                    assert!(
                        position.x == 0 && position.y == 0,
                        "Error during fuzzing ! A primary monitor has been positioned to {}.",
                        position
                    );
                } else {
                    assert!(
                        position.x != 0 || position.y != 0,
                        "Error during fuzzing ! A non primary monitor has been positioned to {}",
                        position
                    );
                }

                let monitor = self.monitor_fuzzer.generate_monitor(
                    monitors_id_common_part_1,
                    &monitors_id_common_part_2,
                    monitors_id_common_part_3,
                    &monitors_id_common_part_4,
                    position,
                    resolution,
                    primary,
                );

                video_outputs[*video_output_index] =
                    video_outputs[*video_output_index].plug_monitor(monitor);
            });

        self.video_outputs = video_outputs;

        self
    }

    pub fn which_requires_reboot(&mut self) -> &mut Self {
        self.reboot_required = true;

        self
    }

    pub fn build_computer(&self) -> FuzzedComputer {
        let secondary_monitor = self.get_monitor(false);
        let primary_monitor = self.get_monitor(true);

        assert_ne!(
            secondary_monitor, primary_monitor,
            "Error during fuzzing ! Primary and secondary monitors are the same"
        );

        let win32 = FuzzedWin32 {
            video_outputs: self.video_outputs.clone(),
            reboot_required: self.reboot_required,
        };

        let mut monitors = self.get_all_monitors();

        monitors.sort();

        FuzzedComputer {
            secondary_monitor,
            primary_monitor,
            win32,
            monitors,
        }
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
