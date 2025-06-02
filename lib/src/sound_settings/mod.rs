#[derive(Debug, PartialEq)]
pub struct SoundSettingsResult {
    pub new_default_output_device: String,
}

pub trait SoundSettings<TSoundSettingsApi> {
    fn new(sound_settings_api: TSoundSettingsApi) -> Self;

    fn change_default_output_device(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SoundSettingsResult, String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_sound_settings::WindowsSoundSettings as Current;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::DllBasedAudioEndpointLibrary as CurrentSoundSettingsApi;
