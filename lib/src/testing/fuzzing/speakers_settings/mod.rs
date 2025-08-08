pub mod audio_endpoint_library;
pub mod speaker_id;
pub mod speaker_name;
pub mod speakers;

#[cfg(target_os = "windows")]
pub use audio_endpoint_library::FuzzedAudioEndpointLibrary as CurrentFuzzedSpeakersSettingsApi;
