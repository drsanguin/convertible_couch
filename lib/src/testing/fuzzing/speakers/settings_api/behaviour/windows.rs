#![cfg(target_os = "windows")]

use crate::testing::fuzzing::speakers::settings_api::behaviour::FuzzedSpeakersSettingsApiBehaviour;

#[derive(Clone, Default)]
pub struct FuzzedWindowsSpeakersSettingsApiBehaviour {
    pub getting_the_speakers_count_fails: bool,
    pub getting_the_speakers_fails: bool,
    pub getting_the_default_speaker_fails: bool,
    pub setting_the_default_speaker_fails: bool,
}

impl FuzzedSpeakersSettingsApiBehaviour for FuzzedWindowsSpeakersSettingsApiBehaviour {}
