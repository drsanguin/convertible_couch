use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use crate::ApplicationError;

#[derive(Debug, PartialEq, Eq)]
pub struct SpeakersSettingsResult {
    pub new_default_speaker: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpeakerInfo {
    pub is_default: bool,
    pub name: String,
}

impl Ord for SpeakerInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .is_default
            .cmp(&self.is_default)
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for SpeakerInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for SpeakerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_default {
            write!(f, "[default] {}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub trait SpeakersSettings<TSpeakersSettingsApi> {
    fn new(speakers_settings_api: TSpeakersSettingsApi) -> Self;

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, ApplicationError>;

    fn get_speakers_infos(&self) -> Result<Vec<SpeakerInfo>, ApplicationError>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_speakers_settings::WindowsSoundSettings as CurrentSpeakersSettings;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::dll_based_audio_endpoint_library::DllBasedAudioEndpointLibrary as CurrentSpeakersSettingsApi;

#[cfg(target_os = "windows")]
pub use windows::audio_endpoint_library::AudioEndpointLibrary as CurrentSpeakersSettingsApiTrait;

#[cfg(test)]
mod tests {
    use crate::speakers_settings::SpeakerInfo;

    #[test]
    fn it_should_be_displayed_as_expected_when_default() {
        // Arrange
        let speaker_info = SpeakerInfo {
            is_default: true,
            name: String::from("Corsair Vengeance 1500"),
        };

        // Act
        let display = format!("{speaker_info}");

        // Assert
        assert_eq!(display, "[default] Corsair Vengeance 1500")
    }

    #[test]
    fn it_should_be_displayed_as_expected_when_alternative() {
        // Arrange
        let speaker_info = SpeakerInfo {
            is_default: false,
            name: String::from("Corsair Vengeance 1500"),
        };

        // Act
        let display = format!("{speaker_info}");

        // Assert
        assert_eq!(display, "Corsair Vengeance 1500")
    }
}
