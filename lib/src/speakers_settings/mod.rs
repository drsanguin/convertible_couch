use std::cmp::Ordering;

use crate::application_result::ApplicationResult;
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

#[allow(clippy::non_canonical_partial_ord_impl)]
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
    ) -> ApplicationResult<SpeakersSettingsResult>;

    fn get_speakers_infos(&mut self) -> ApplicationResult<Vec<SpeakerInfo>>;
}

cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::windows_sound_settings::WindowsSoundSettings as CurrentSpeakersSettings;
        pub use windows::win_32_based_windows_api::Win32BasedWindowsApi as CurrentSpeakersSettingsApi;
        pub use windows::windows_api::WindowsApi as CurrentSpeakersSettingsApiTrait;
    }
}
