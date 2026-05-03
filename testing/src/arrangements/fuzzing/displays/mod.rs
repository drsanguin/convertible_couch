use crate::arrangements::fuzzing::{
    ComputerBuilder,
    computer::{ComputerFuzzer, FuzzedComputer},
    displays::{
        device_id::DeviceIdFuzzer,
        display_name::DisplayNameFuzzer,
        position::{DisplayPositionFuzzer, FuzzedDisplayPosition},
        resolution::{FuzzedResolution, ResolutionFuzzer},
        settings_api::{
            CurrentFuzzedDisplaysSettingsApi, FuzzedDisplaysSettingsApi,
            behaviour::CurrentFuzzedDisplaysSettingsApiBehaviour,
        },
        video_output::VideoOutputFuzzer,
    },
};

use rand::{RngExt, seq::IteratorRandom};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::WIN32_ERROR;

use std::collections::HashSet;

pub mod config_mod_info_id;
pub mod device_id;
pub mod display_name;
pub mod gsm_id;
pub mod position;
pub mod resolution;
pub mod settings_api;
pub mod video_output;

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
    computer_fuzzer: &'a mut ComputerFuzzer<'a>,
    min_n_display: usize,
    max_n_display: usize,
    includes_an_internal_display: bool,
    primary_display_name: Option<String>,
    secondary_display_names: HashSet<String>,
    behaviour: CurrentFuzzedDisplaysSettingsApiBehaviour,
}

impl<'a> DisplaysFuzzer<'a> {
    /// According to the answer of [this question](https://learn.microsoft.com/en-us/answers/questions/1324305/what-is-the-maximum-horizontal-resolution-size-rec), Windows has a hard limit of 128 million pixels.
    /// Which implies that the theoretical maximum is 162 displays with a 1024x768 resolution.
    const MAX_N_DISPLAY: usize = 162;

    pub fn new(computer_fuzzer: &'a mut ComputerFuzzer<'a>) -> Self {
        Self {
            computer_fuzzer,
            max_n_display: 0,
            min_n_display: 0,
            includes_an_internal_display: false,
            primary_display_name: None,
            secondary_display_names: HashSet::new(),
            behaviour: CurrentFuzzedDisplaysSettingsApiBehaviour::default(),
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

    pub fn whose_primary_is_named(&mut self, primary_display_name: &str) -> &mut Self {
        self.primary_display_name = Some(primary_display_name.to_string());

        self
    }

    pub fn with_a_secondary_named(&mut self, secondary_display_name: &str) -> &mut Self {
        self.secondary_display_names
            .insert(secondary_display_name.to_string());

        self
    }

    pub fn build_displays(&'a mut self) -> &'a mut ComputerFuzzer<'a> {
        let n_video_output = self
            .computer_fuzzer
            .rand
            .random_range(self.min_n_display..=self.max_n_display);

        let n_display = self
            .computer_fuzzer
            .rand
            .random_range(self.min_n_display..=n_video_output);

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
            .sample(&mut self.computer_fuzzer.rand, n_display);

        video_outputs_to_plug_in_indexes.sort();

        video_outputs_to_plug_in_indexes
            .iter()
            .enumerate()
            .for_each(|(display_index, video_output_index)| {
                let display = displays[display_index].to_owned();

                video_outputs[*video_output_index] =
                    video_outputs[*video_output_index].plug_display(display);
            });

        let displays_settings_api =
            CurrentFuzzedDisplaysSettingsApi::new(video_outputs, self.behaviour.clone());

        self.computer_fuzzer
            .set_displays_settings_api(displays_settings_api)
    }

    fn generate_several(&mut self, n_display: usize) -> Vec<FuzzedDisplay> {
        let mut forbidden_display_names = HashSet::new();
        let mut names_already_taken_count = self.secondary_display_names.len();

        if let Some(primary_display_name) = &self.primary_display_name {
            forbidden_display_names.insert(primary_display_name);
            names_already_taken_count += 1;
        };

        forbidden_display_names.extend(&self.secondary_display_names);

        let displays_resolutions =
            ResolutionFuzzer::new(self.computer_fuzzer.rand).generate_several(n_display);
        let positioned_resolutions = DisplayPositionFuzzer::new(self.computer_fuzzer.rand)
            .generate_several(&displays_resolutions, self.includes_an_internal_display);
        let mut names = DisplayNameFuzzer::new(self.computer_fuzzer.rand).generate_several(
            n_display - names_already_taken_count,
            &forbidden_display_names,
        );

        names.extend(self.secondary_display_names.clone());

        if let Some(primary_display_name) = &self.primary_display_name {
            let primary_position_target_index = positioned_resolutions
                .iter()
                .position(|x| x.position.is_positioned_at_origin())
                .unwrap();

            names.push(primary_display_name.to_string());

            let primary_position_source_index = names.len() - 1;

            names.swap(primary_position_source_index, primary_position_target_index);
        }

        let device_ids = DeviceIdFuzzer::new(self.computer_fuzzer.rand).generate_several(n_display);

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

impl<'a> ComputerBuilder<'a> for DisplaysFuzzer<'a> {
    fn build_computer(&'a mut self) -> FuzzedComputer {
        self.build_displays().build_computer()
    }
}

#[cfg(target_os = "windows")]
pub enum Function {
    GetDisplayConfigBufferSizes,
    QueryDisplayConfig,
    DisplayConfigGetDeviceInfo,
    SetDisplayConfig,
}

#[cfg(target_os = "windows")]
impl<'a> DisplaysFuzzer<'a> {
    pub fn for_which_function_fails_with(
        &mut self,
        function: Function,
        error: WIN32_ERROR,
    ) -> &mut Self {
        match function {
            Function::GetDisplayConfigBufferSizes => {
                self.for_which_get_display_config_buffer_fails_with(error)
            }
            Function::QueryDisplayConfig => {
                self.for_which_query_display_config_fails_with(vec![error])
            }
            Function::DisplayConfigGetDeviceInfo => {
                self.for_which_display_config_get_device_info_fails_with(error)
            }
            Function::SetDisplayConfig => self.for_which_set_display_config_fails_with(error),
        }
    }

    pub fn for_which_get_display_config_buffer_fails_with(
        &mut self,
        get_display_config_buffer_sizes_error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.get_display_config_buffer_sizes_error =
            Some(get_display_config_buffer_sizes_error);

        self
    }

    pub fn for_which_query_display_config_fails_with(
        &mut self,
        query_display_config_errors: Vec<WIN32_ERROR>,
    ) -> &mut Self {
        self.behaviour.query_display_config_errors = query_display_config_errors;

        self
    }

    pub fn for_which_display_config_get_device_info_fails_with(
        &mut self,
        display_config_get_device_info_error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.display_config_get_device_info_error =
            Some(display_config_get_device_info_error);

        self
    }

    pub fn for_which_set_display_config_fails_with(
        &mut self,
        set_display_config_error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.set_display_config_error = Some(set_display_config_error);

        self
    }
}
