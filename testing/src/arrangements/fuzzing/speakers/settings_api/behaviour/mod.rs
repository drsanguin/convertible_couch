pub trait FuzzedSpeakersSettingsApiBehaviour: Clone + Default {}

cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::FuzzedWindowsSpeakersSettingsApiBehaviour as CurrentFuzzedSpeakersSettingsApiBehaviour;
    }
}
