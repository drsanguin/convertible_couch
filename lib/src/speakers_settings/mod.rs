#[derive(Debug, PartialEq, Eq)]
pub struct SpeakersSettingsResult {
    pub new_default_speaker: String,
}

pub trait SpeakersSettings<TSpeakersSettingsApi> {
    fn new(speakers_settings_api: TSpeakersSettingsApi) -> Self;

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_speakers_settings::WindowsSoundSettings as CurrentSpeakersSettings;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::dll_based_audio_endpoint_library::DllBasedAudioEndpointLibrary as CurrentSpeakersSettingsApi;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::AudioEndpointLibrary as CurrentSpeakersSettingsApiTrait;
