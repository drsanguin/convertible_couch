pub trait FuzzedDisplaysSettingsApiBehaviour: Clone + Default {}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWindowsDisplaysSettingsApiBehaviour as CurrentFuzzedDisplaysSettingsApiBehaviour;
