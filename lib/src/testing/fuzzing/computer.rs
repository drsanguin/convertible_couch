use crate::testing::fuzzing::audio_endpoint::{AudioEndpointFuzzer, FuzzedAudioEndpoint};
#[cfg(target_os = "windows")]
use crate::testing::fuzzing::audio_endpoint_library::FuzzedAudioEndpointLibrary;

use super::{displays::DisplaysFuzzer, video_output::FuzzedVideoOutput, win_32::FuzzedWin32};
use rand::{rngs::StdRng, seq::IndexedRandom, Rng, RngCore, SeedableRng};
use std::collections::HashMap;
use windows::Win32::Graphics::Gdi::DISP_CHANGE;

pub struct FuzzedComputer {
    #[cfg(target_os = "windows")]
    pub display_settings_api: FuzzedWin32,
    pub primary_display: String,
    pub secondary_display: String,
    pub displays: Vec<String>,
    #[cfg(target_os = "windows")]
    pub audio_settings_api: FuzzedAudioEndpointLibrary,
    pub default_audio_endpoint: String,
    pub non_default_audio_endpoint: String,
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
    video_outputs: Vec<FuzzedVideoOutput>,
    change_display_settings_error: Option<DISP_CHANGE>,
    change_display_settings_error_on_commit: Option<DISP_CHANGE>,
    getting_primary_display_name_fails: bool,
    querying_the_display_config_of_the_primary_display_fails: bool,
    audio_endpoints: Vec<FuzzedAudioEndpoint>,
}

impl ComputerFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            video_outputs: vec![],
            change_display_settings_error: None,
            change_display_settings_error_on_commit: None,
            getting_primary_display_name_fails: false,
            querying_the_display_config_of_the_primary_display_fails: false,
            audio_endpoints: vec![],
        }
    }

    pub fn new_with_video_outputs(
        computer_fuzzer: &mut ComputerFuzzer,
        video_outputs: Vec<FuzzedVideoOutput>,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(computer_fuzzer.rand.next_u64()),
            video_outputs,
            change_display_settings_error: computer_fuzzer.change_display_settings_error,
            change_display_settings_error_on_commit: computer_fuzzer
                .change_display_settings_error_on_commit,
            getting_primary_display_name_fails: computer_fuzzer.getting_primary_display_name_fails,
            querying_the_display_config_of_the_primary_display_fails: computer_fuzzer
                .querying_the_display_config_of_the_primary_display_fails,
            audio_endpoints: computer_fuzzer.audio_endpoints.clone(),
        }
    }

    pub fn with_displays(&mut self) -> DisplaysFuzzer {
        DisplaysFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_committing_the_display_changes_fails_with(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: self.change_display_settings_error,
            change_display_settings_error_on_commit: Some(change_display_settings_error),
            getting_primary_display_name_fails: self.getting_primary_display_name_fails,
            querying_the_display_config_of_the_primary_display_fails: self
                .querying_the_display_config_of_the_primary_display_fails,
            audio_endpoints: self.audio_endpoints.clone(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn for_which_changing_the_display_settings_fails_for_some_displays(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: Some(change_display_settings_error),
            change_display_settings_error_on_commit: self.change_display_settings_error_on_commit,
            getting_primary_display_name_fails: self.getting_primary_display_name_fails,
            querying_the_display_config_of_the_primary_display_fails: self
                .querying_the_display_config_of_the_primary_display_fails,
            audio_endpoints: self.audio_endpoints.clone(),
        }
    }

    pub fn for_which_getting_the_primary_display_fails(&mut self) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: self.change_display_settings_error,
            change_display_settings_error_on_commit: self.change_display_settings_error_on_commit,
            getting_primary_display_name_fails: true,
            querying_the_display_config_of_the_primary_display_fails: self
                .querying_the_display_config_of_the_primary_display_fails,
            audio_endpoints: self.audio_endpoints.clone(),
        }
    }

    pub fn for_which_querying_the_display_config_of_the_primary_display_fails(&mut self) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: self.change_display_settings_error,
            change_display_settings_error_on_commit: self.change_display_settings_error_on_commit,
            getting_primary_display_name_fails: self.getting_primary_display_name_fails,
            querying_the_display_config_of_the_primary_display_fails: true,
            audio_endpoints: self.audio_endpoints.clone(),
        }
    }

    pub fn with_audio_output_devices(&mut self, count: usize) -> Self {
        let audio_endpoints = AudioEndpointFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()))
            .generate_several(count);

        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: self.change_display_settings_error,
            change_display_settings_error_on_commit: self.change_display_settings_error_on_commit,
            getting_primary_display_name_fails: self.getting_primary_display_name_fails,
            querying_the_display_config_of_the_primary_display_fails: self
                .querying_the_display_config_of_the_primary_display_fails,
            audio_endpoints: audio_endpoints,
        }
    }

    pub fn build_computer(&mut self) -> FuzzedComputer {
        let secondary_display = self.get_display(false);
        let primary_display = self.get_display(true);

        assert_ne!(
            secondary_display, primary_display,
            "Error during fuzzing ! Primary and secondary displays are the same"
        );

        let mut change_display_settings_error_by_display = HashMap::new();

        if self.change_display_settings_error.is_some() {
            let possible_devices_paths = self
                .video_outputs
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

        let display_settings_api =
            self.get_display_settings_api(change_display_settings_error_by_display);

        let mut displays = self.get_all_displays();

        displays.sort();

        let default_audio_endpoint = self
            .audio_endpoints
            .iter()
            .find(|audio_endpoint| audio_endpoint.is_default)
            .unwrap_or(&FuzzedAudioEndpoint {
                name: String::from(""),
                id: String::from(""),
                is_default: false,
            })
            .name
            .clone();

        let non_default_audio_endpoint = self
            .audio_endpoints
            .iter()
            .find(|audio_endpoint| !audio_endpoint.is_default)
            .unwrap_or(&FuzzedAudioEndpoint {
                name: String::from(""),
                id: String::from(""),
                is_default: false,
            })
            .name
            .clone();

        FuzzedComputer {
            secondary_display,
            primary_display,
            display_settings_api,
            displays,
            audio_settings_api: FuzzedAudioEndpointLibrary::new(self.audio_endpoints.clone()),
            default_audio_endpoint,
            non_default_audio_endpoint,
        }
    }

    fn get_display(&self, primary: bool) -> String {
        self.video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.display {
                Some(display) => match display.primary {
                    p if p == primary => Some(display.name.clone()),
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

    #[cfg(target_os = "windows")]
    fn get_display_settings_api(
        &mut self,
        change_display_settings_error_by_display: HashMap<String, DISP_CHANGE>,
    ) -> FuzzedWin32 {
        FuzzedWin32::new(
            self.video_outputs.clone(),
            self.change_display_settings_error_on_commit,
            change_display_settings_error_by_display,
            self.getting_primary_display_name_fails,
            self.querying_the_display_config_of_the_primary_display_fails,
        )
    }

    fn get_all_displays(&self) -> Vec<String> {
        self.video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.display {
                Some(display) => Some(display.name.clone()),
                None => None,
            })
            .collect::<Vec<String>>()
    }
}
