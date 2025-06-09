use crate::testing::fuzzing::{
    computer::ComputerFuzzer, display_settings::video_output::VideoOutputFuzzer,
};

use super::{
    device_id::{DeviceIdFuzzer, FuzzedDeviceId},
    display_name::DisplayNameFuzzer,
    position::{DisplayPositionFuzzer, FuzzedDisplayPosition},
    resolution::{FuzzedResolution, ResolutionFuzzer},
};

use rand::{rngs::StdRng, seq::IteratorRandom, Rng, RngCore, SeedableRng};

use std::collections::HashSet;

#[derive(Clone)]
pub struct FuzzedDisplay {
    pub name: String,
    pub primary: bool,
    pub config_mode_info_id: u32,
    pub device_id: String,
    pub resolution: FuzzedResolution,
    pub position: FuzzedDisplayPosition,
}

pub struct DisplaysFuzzer<'a> {
    rand: StdRng,
    computer_fuzzer: ComputerFuzzer,
    min_n_display: usize,
    max_n_display: usize,
    includes_an_internal_display: bool,
    forbidden_display_names: HashSet<&'a str>,
    forbidden_device_ids: HashSet<&'a FuzzedDeviceId>,
}

impl<'a> DisplaysFuzzer<'a> {
    /// According to the answer of [this question](https://learn.microsoft.com/en-us/answers/questions/1324305/what-is-the-maximum-horizontal-resolution-size-rec), Windows has a hard limit of 128 million pixels.
    /// Which implies that the theoretical maximum is 162 displays with a 1024x768 resolution.
    const MAX_N_DISPLAY: usize = 162;

    pub fn new(rand: StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
            computer_fuzzer,
            max_n_display: 0,
            min_n_display: 0,
            includes_an_internal_display: false,
            forbidden_display_names: HashSet::new(),
            forbidden_device_ids: HashSet::new(),
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_display: count,
            max_n_display: count,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_display_names: self.forbidden_display_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn of_which_there_are_at_least(&mut self, count: usize) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_display: count,
            max_n_display: Self::MAX_N_DISPLAY,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_display_names: self.forbidden_display_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn including_an_internal_display(&mut self) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_display: self.min_n_display,
            max_n_display: self.max_n_display,
            includes_an_internal_display: true,
            forbidden_display_names: self.forbidden_display_names.clone(),
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn whose_names_are_different_from(
        &mut self,
        forbidden_display_names: HashSet<&'a str>,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_display: self.min_n_display,
            max_n_display: self.max_n_display,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_display_names,
            forbidden_device_ids: self.forbidden_device_ids.clone(),
        }
    }

    pub fn whose_device_ids_are_different_from(
        &mut self,
        forbidden_device_ids: HashSet<&'a FuzzedDeviceId>,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            computer_fuzzer: self.computer_fuzzer.clone(),
            min_n_display: self.min_n_display,
            max_n_display: self.max_n_display,
            includes_an_internal_display: self.includes_an_internal_display,
            forbidden_display_names: self.forbidden_display_names.clone(),
            forbidden_device_ids,
        }
    }

    pub fn build_displays(&mut self) -> ComputerFuzzer {
        let n_video_output = self
            .rand
            .random_range(self.min_n_display..=self.max_n_display);
        let n_display = self.rand.random_range(self.min_n_display..=n_video_output);

        let displays = self.generate_several(n_display);

        assert_eq!(
            displays.iter().filter(|display| display.primary).count(),
            1,
            "More than one primary display has been generated"
        );

        let mut video_outputs = VideoOutputFuzzer::generate_several(n_video_output);

        let mut video_outputs_to_plug_in_indexes = video_outputs
            .iter()
            .enumerate()
            .map(|(index, _video_output)| index)
            .choose_multiple(&mut self.rand, n_display);

        video_outputs_to_plug_in_indexes.sort();

        video_outputs_to_plug_in_indexes
            .iter()
            .enumerate()
            .for_each(|(display_index, video_output_index)| {
                let display = displays[display_index].to_owned();

                video_outputs[*video_output_index] =
                    video_outputs[*video_output_index].plug_display(display);
            });

        ComputerFuzzer::new_with_video_outputs(&mut self.computer_fuzzer, video_outputs)
    }

    fn generate_several(&mut self, n_display: usize) -> Vec<FuzzedDisplay> {
        let displays_resolutions =
            ResolutionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(n_display);
        let positioned_resolutions =
            DisplayPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&displays_resolutions, self.includes_an_internal_display);
        let names = DisplayNameFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_display, &self.forbidden_display_names);
        let device_ids = DeviceIdFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(n_display, &self.forbidden_device_ids);

        (0..n_display)
            .map(|display_index| {
                let position = positioned_resolutions[display_index].position;
                let resolution = positioned_resolutions[display_index].resolution;
                let primary = position.is_positioned_at_origin();
                let name = if self.includes_an_internal_display && primary {
                    String::from("")
                } else {
                    names[display_index].to_owned()
                };
                let device_id = device_ids[display_index].clone();

                FuzzedDisplay {
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
