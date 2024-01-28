use std::collections::HashMap;

use rand::{rngs::StdRng, seq::SliceRandom, Rng, RngCore, SeedableRng};
use windows::Win32::Graphics::Gdi::DISP_CHANGE;

use super::{monitors::MonitorsFuzzer, video_output::FuzzedVideoOutput, win32::FuzzedWin32};

pub struct FuzzedComputer {
    pub win32: FuzzedWin32,
    pub primary_monitor: String,
    pub secondary_monitor: String,
    pub monitors: Vec<String>,
}

#[derive(Clone)]
pub struct ComputerFuzzer {
    rand: StdRng,
    video_outputs: Vec<FuzzedVideoOutput>,
    change_display_settings_error: Option<DISP_CHANGE>,
    change_display_settings_error_on_commit: Option<DISP_CHANGE>,
    getting_primary_monitor_name_fails: bool,
    querying_the_display_config_of_the_primary_monitor_fails: bool,
}

impl ComputerFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self {
            rand,
            video_outputs: vec![],
            change_display_settings_error: None,
            change_display_settings_error_on_commit: None,
            getting_primary_monitor_name_fails: false,
            querying_the_display_config_of_the_primary_monitor_fails: false,
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
            getting_primary_monitor_name_fails: computer_fuzzer.getting_primary_monitor_name_fails,
            querying_the_display_config_of_the_primary_monitor_fails: computer_fuzzer
                .querying_the_display_config_of_the_primary_monitor_fails,
        }
    }

    pub fn with_monitors(&mut self) -> MonitorsFuzzer {
        MonitorsFuzzer::new(StdRng::seed_from_u64(self.rand.next_u64()), self.clone())
    }

    pub fn for_which_committing_the_display_changes_fails_with(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: self.change_display_settings_error,
            change_display_settings_error_on_commit: Some(change_display_settings_error),
            getting_primary_monitor_name_fails: self.getting_primary_monitor_name_fails,
            querying_the_display_config_of_the_primary_monitor_fails: self
                .querying_the_display_config_of_the_primary_monitor_fails,
        }
    }

    pub fn for_which_changing_the_display_settings_fails_for_some_monitors(
        &mut self,
        change_display_settings_error: DISP_CHANGE,
    ) -> Self {
        Self {
            rand: StdRng::seed_from_u64(self.rand.next_u64()),
            video_outputs: self.video_outputs.clone(),
            change_display_settings_error: Some(change_display_settings_error),
            change_display_settings_error_on_commit: self.change_display_settings_error_on_commit,
            getting_primary_monitor_name_fails: self.getting_primary_monitor_name_fails,
            querying_the_display_config_of_the_primary_monitor_fails: self
                .querying_the_display_config_of_the_primary_monitor_fails,
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

    pub fn build_computer(&mut self) -> FuzzedComputer {
        let secondary_monitor = self.get_monitor(false);
        let primary_monitor = self.get_monitor(true);

        assert_ne!(
            secondary_monitor, primary_monitor,
            "Error during fuzzing ! Primary and secondary monitors are the same"
        );

        let mut change_display_settings_error_by_monitor = HashMap::new();

        if self.change_display_settings_error.is_some() {
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
                    change_display_settings_error_by_monitor.insert(
                        String::from(device_path),
                        self.change_display_settings_error.unwrap(),
                    );
                });
        }

        let win32 = FuzzedWin32::new(
            self.video_outputs.clone(),
            self.change_display_settings_error_on_commit,
            change_display_settings_error_by_monitor,
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
