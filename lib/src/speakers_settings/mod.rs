use std::cmp::Ordering;

use crate::application_error::ApplicationError;
use crate::trace_fn;

#[derive(Debug, PartialEq, Eq)]
pub struct SpeakersSettingsResult {
    pub new_default_speaker: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpeakerInfo {
    pub is_default: bool,
    pub name: String,
}

impl Ord for SpeakerInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        trace_fn!();

        other
            .is_default
            .cmp(&self.is_default)
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for SpeakerInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        trace_fn!();
        
        Some(self.cmp(other))
    }
}

pub trait SpeakersSettings {
    fn new(speakers_settings_api: Box<dyn CurrentSpeakersSettingsApiTrait>) -> Self;

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, ApplicationError>;

    fn get_speakers_infos(&mut self) -> Result<Vec<SpeakerInfo>, ApplicationError>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsSoundSettings as CurrentSpeakersSettings;

#[cfg(target_os = "windows")]
pub use windows::windows_com::windows_api_based_windows_com::WindowsApiBasedWindowsCom as CurrentSpeakersSettingsApi;

#[cfg(target_os = "windows")]
pub use windows::windows_com::WindowsCom as CurrentSpeakersSettingsApiTrait;
