use crate::application::Application;
use convertible_couch_lib::{
    displays_settings::CurrentDisplaysSettings,
    speakers_settings::CurrentSpeakersSettings,
    testing::fuzzing::{
        computer::FuzzedComputer, displays_settings::CurrentFuzzedDisplaysSettingsApi,
        speakers_settings::CurrentFuzzedSpeakersSettingsApi,
    },
};

pub fn bootstrap_application(
    computer: FuzzedComputer,
) -> Application<
    CurrentFuzzedDisplaysSettingsApi,
    CurrentFuzzedSpeakersSettingsApi,
    CurrentDisplaysSettings<CurrentFuzzedDisplaysSettingsApi>,
    CurrentSpeakersSettings<CurrentFuzzedSpeakersSettingsApi>,
> {
    Application::<
        CurrentFuzzedDisplaysSettingsApi,
        CurrentFuzzedSpeakersSettingsApi,
        CurrentDisplaysSettings<CurrentFuzzedDisplaysSettingsApi>,
        CurrentSpeakersSettings<CurrentFuzzedSpeakersSettingsApi>,
    >::bootstrap(
        computer.displays_settings_api,
        computer.speakers_settings_api,
    )
}
