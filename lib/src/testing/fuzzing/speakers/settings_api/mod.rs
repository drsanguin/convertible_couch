use crate::testing::fuzzing::speakers::{
    settings_api::behaviour::CurrentFuzzedSpeakersSettingsApiBehaviour, FuzzedSpeaker,
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
pub use windows::FuzzedAudioEndpointLibrary as CurrentFuzzedSpeakersSettingsApi;
