use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    Rng, RngCore, SeedableRng,
};

use super::{
    guid::GuidFuzzer,
    monitor::{FuzzedMonitor, MonitorFuzzer},
    position::MonitorPositionFuzzer,
    resolution::ResolutionFuzzer,
};

#[derive(Clone)]
pub struct FuzzedVideoOutput {
    pub id: String,
    pub monitor: Option<FuzzedMonitor>,
}

impl FuzzedVideoOutput {
    pub fn new(index: usize, monitor: Option<FuzzedMonitor>) -> Self {
        let id = match monitor {
            Some(_) => format!(r"\\.\DISPLAY{}\Monitor0", index),
            None => format!(r"\\.\DISPLAY{}", index),
        };

        Self { id, monitor }
    }
}

pub struct VideoOutputFuzzer {
    rand: StdRng,
    guid_fuzzer: GuidFuzzer,
    resolution_fuzzer: ResolutionFuzzer,
    monitor_fuzzer: MonitorFuzzer,
}

impl VideoOutputFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            rand,
            guid_fuzzer: GuidFuzzer::new(StdRng::seed_from_u64(seed)),
            monitor_fuzzer: MonitorFuzzer::new(StdRng::seed_from_u64(seed)),
            resolution_fuzzer: ResolutionFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn generate_video_outputs(&mut self, n_monitor: usize) -> Vec<FuzzedVideoOutput> {
        let n_video_output = self.rand.gen_range(n_monitor..=5);
        let n_monitor = self.rand.gen_range(n_monitor..=n_video_output);
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

        (1..=n_video_output)
            .map(|video_output_number| {
                if video_output_number > n_monitor {
                    FuzzedVideoOutput::new(video_output_number, None)
                } else {
                    let video_output_index = video_output_number - 1;

                    let position = monitors_positions[video_output_index];
                    let resolution = monitors_resolutions[video_output_index];
                    let primary = video_output_number == primary_monitor_number;

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

                    FuzzedVideoOutput::new(video_output_number, Some(monitor))
                }
            })
            .collect()
    }
}
