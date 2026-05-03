pub trait FuzzedDisplaysSettingsApiBehaviour: Clone + Default {}

cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::FuzzedWindowsDisplaysSettingsApiBehaviour as CurrentFuzzedDisplaysSettingsApiBehaviour;
    }
}
