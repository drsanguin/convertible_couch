pub trait FuzzedSpeakersSettingsApiBehaviour: Clone + Default {}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWindowsSpeakersSettingsApiBehaviour as CurrentFuzzedSpeakersSettingsApiBehaviour;
