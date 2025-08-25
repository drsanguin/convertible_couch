pub mod behaviour;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::FuzzedWin32 as CurrentFuzzedDisplaysSettingsApi;
