use crate::testing::fuzzing::speakers::{
    FuzzedSpeaker, settings_api::behaviour::CurrentFuzzedSpeakersSettingsApiBehaviour,
};

pub mod behaviour;

pub trait FuzzedSpeakersSettingsApi: Default {
    fn new(
        speakers: Vec<FuzzedSpeaker>,
        behaviour: CurrentFuzzedSpeakersSettingsApiBehaviour,
    ) -> Self;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWindowsCom as CurrentFuzzedSpeakersSettingsApi;
