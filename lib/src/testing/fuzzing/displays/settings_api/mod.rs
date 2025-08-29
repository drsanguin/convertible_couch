use crate::testing::fuzzing::displays::{
    settings_api::behaviour::CurrentFuzzedDisplaysSettingsApiBehaviour,
    video_output::FuzzedVideoOutput,
};

pub mod behaviour;

pub trait FuzzedDisplaysSettingsApi: Default {
    fn new(
        video_outputs: Vec<FuzzedVideoOutput>,
        behaviour: CurrentFuzzedDisplaysSettingsApiBehaviour,
    ) -> Self;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWin32 as CurrentFuzzedDisplaysSettingsApi;
