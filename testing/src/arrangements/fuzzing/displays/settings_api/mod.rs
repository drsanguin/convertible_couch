use crate::arrangements::fuzzing::displays::{
    FuzzedDisplay, settings_api::behaviour::CurrentFuzzedDisplaysSettingsApiBehaviour,
};

pub mod behaviour;

pub trait FuzzedDisplaysSettingsApi: Default {
    fn new(
        displays: Vec<FuzzedDisplay>,
        behaviour: CurrentFuzzedDisplaysSettingsApiBehaviour,
    ) -> Self;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWindowsApi as CurrentFuzzedDisplaysSettingsApi;
