use crate::testing::fuzzing::{
    computer::ComputerFuzzer,
    displays_settings::{video_output::VideoOutputFuzzer, win_32::FuzzedWin32},
};

use super::{
    device_id::{DeviceIdFuzzer, FuzzedDeviceId},
    display_name::DisplayNameFuzzer,
    position::{DisplayPositionFuzzer, FuzzedDisplayPosition},
    resolution::{FuzzedResolution, ResolutionFuzzer},
};

use rand::{
    rngs::StdRng,
    seq::{IndexedRandom, IteratorRandom},
    Rng, RngCore, SeedableRng,
};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::DISP_CHANGE;

use std::collections::{HashMap, HashSet};

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
    primary_display_name: Option<String>,
    secondary_display_names: HashSet<String>,
    change_display_settings_error_on_commit: Option<DISP_CHANGE>,
    change_display_settings_error: Option<DISP_CHANGE>,
    getting_primary_display_name_fails: bool,
    querying_the_display_config_of_the_primary_display_fails: bool,
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
            primary_display_name: None,
            secondary_display_names: HashSet::new(),
            change_display_settings_error_on_commit: None,
            change_display_settings_error: None,
            getting_primary_display_name_fails: false,
            querying_the_display_config_of_the_primary_display_fails: false,
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> &mut Self {
        self.min_n_display = count;
        self.max_n_display = count;

        self
    }

    pub fn of_which_there_are_at_least(&mut self, count: usize) -> &mut Self {
        self.min_n_display = count;
        self.max_n_display = Self::MAX_N_DISPLAY;

        self
    }

    pub fn including_an_internal_display(&mut self) -> &mut Self {
        self.includes_an_internal_display = true;

        self
    }

    pub fn whose_device_ids_are_different_from(
        &mut self,
        forbidden_device_ids: HashSet<&'a FuzzedDeviceId>,
    ) -> &mut Self {
        self.forbidden_device_ids = forbidden_device_ids;

        self
    }

    pub fn whose_primary_is_named(&mut self, primary_display_name: String) -> &mut Self {
        self.primary_display_name = Some(primary_display_name);

        self
    }

    pub fn with_a_secondary_named(&mut self, secondary_display_name: String) -> &mut Self {
        self.secondary_display_names.insert(secondary_display_name);

        self
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_committing_the_display_changes_fails_with(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> &mut Self {
        self.change_display_settings_error_on_commit = Some(change_display_settings_error);

        self
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_changing_the_display_settings_fails_for_some_displays(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> &mut Self {
        self.change_display_settings_error = Some(change_display_settings_error);

        self
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_getting_the_primary_display_fails(&mut self) -> &mut Self {
        self.getting_primary_display_name_fails = true;

        self
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_querying_the_display_config_of_the_primary_display_fails(
        &mut self,
    ) -> &mut Self {
        self.querying_the_display_config_of_the_primary_display_fails = true;

        self
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

        let mut change_display_settings_error_by_display = HashMap::new();

        if self.change_display_settings_error.is_some() {
            let possible_devices_paths = video_outputs
                .iter()
                .filter_map(|video_output| match &video_output.display {
                    Some(_) => Some(video_output.device_name.clone()),
                    None => None,
                })
                .collect::<Vec<String>>();

            let n_display_on_error = self.rand.random_range(1..possible_devices_paths.len());

            possible_devices_paths
                .choose_multiple(&mut self.rand, n_display_on_error)
                .for_each(|device_path| {
                    change_display_settings_error_by_display.insert(
                        String::from(device_path),
                        self.change_display_settings_error.unwrap(),
                    );
                });
        }

        let fuzzed_win_32 = FuzzedWin32::new(
            video_outputs,
            self.change_display_settings_error_on_commit,
            change_display_settings_error_by_display,
            self.getting_primary_display_name_fails,
            self.querying_the_display_config_of_the_primary_display_fails,
        );

        ComputerFuzzer::new_with_display_settings_api(&mut self.computer_fuzzer, fuzzed_win_32)
    }

    fn generate_several(&mut self, n_display: usize) -> Vec<FuzzedDisplay> {
        let mut forbidden_display_names = HashSet::from_iter(self.forbidden_display_names.clone());

        let mut names_already_taken_count = self.secondary_display_names.len();

        if self.primary_display_name.is_some() {
            let primary_display_name = self.primary_display_name.as_mut().unwrap().as_str();
            forbidden_display_names.insert(primary_display_name);
            names_already_taken_count += 1;
        }

        forbidden_display_names.extend(
            self.secondary_display_names
                .iter()
                .map(|secondary_name| secondary_name.as_str()),
        );

        let displays_resolutions =
            ResolutionFuzzer::new(&mut self.rand).generate_several(n_display);
        let positioned_resolutions =
            DisplayPositionFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
                .generate_several(&displays_resolutions, self.includes_an_internal_display);
        let mut names = DisplayNameFuzzer::new(&mut self.rand).generate_several(
            n_display - names_already_taken_count,
            &forbidden_display_names,
        );

        names.extend(self.secondary_display_names.clone());

        if self.primary_display_name.is_some() {
            let primary_position_target_index = positioned_resolutions
                .iter()
                .position(|x| x.position.is_positioned_at_origin())
                .unwrap();

            names.push(self.primary_display_name.clone().unwrap());

            let primary_position_source_index = names.len() - 1;

            names.swap(primary_position_source_index, primary_position_target_index);
        }

        let device_ids = DeviceIdFuzzer::new(&mut self.rand)
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
