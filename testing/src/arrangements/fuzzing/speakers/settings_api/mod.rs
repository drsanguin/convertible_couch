use crate::arrangements::fuzzing::speakers::{
    FuzzedSpeaker, settings_api::behaviour::CurrentFuzzedSpeakersSettingsApiBehaviour,
};

pub mod behaviour;

pub trait FuzzedSpeakersSettingsApi: Default {
    fn new(
        speakers: Vec<FuzzedSpeaker>,
        behaviour: CurrentFuzzedSpeakersSettingsApiBehaviour,
    ) -> Self;
}

cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::FuzzedWindowsApi as CurrentFuzzedSpeakersSettingsApi;
    }
}
