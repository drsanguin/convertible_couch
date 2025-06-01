#[derive(Debug, PartialEq)]
pub struct SwapDefaultOutputDeviceResponse {
    pub new_default_output_device: String,
}

pub trait SoundSettings<TSoundSettingsApi> {
    fn new(sound_settings_api: TSoundSettingsApi) -> Self;

    fn swap_default_output_device(
        &mut self,
        desktop_sound_output_device_name: &str,
        couch_sound_output_device_name: &str,
    ) -> Result<SwapDefaultOutputDeviceResponse, String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsSoundSettings as Current;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::DllBasedAudioEndpointLibrary as CurrentSoundSettingsApi;
