use crate::application::Application;
use crate::commands::{Arguments, Commands, DisplaysOptions, SharedOptions, SpeakersOptions};
use convertible_couch_lib::{
    displays_settings::CurrentDisplaysSettings,
    log::LogLevel,
    speakers_settings::CurrentSpeakersSettings,
    testing::fuzzing::{
        computer::FuzzedComputer, displays_settings::CurrentFuzzedDisplaysSettingsApi,
        speakers::settings_api::CurrentFuzzedSpeakersSettingsApi,
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

pub struct ArgumentsBuilder<'a> {
    argument_command_type: Option<ArgumentCommandType>,
    desktop_display_name: Option<&'a str>,
    couch_display_name: Option<&'a str>,
    desktop_speaker_name: Option<&'a str>,
    couch_speaker_name: Option<&'a str>,
}

impl<'a> ArgumentsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            argument_command_type: None,
            desktop_display_name: None,
            couch_display_name: None,
            desktop_speaker_name: None,
            couch_speaker_name: None,
        }
    }

    pub fn displays_and_speakers(
        &mut self,
        desktop_display_name: &'a str,
        couch_display_name: &'a str,
        desktop_speaker_name: &'a str,
        couch_speaker_name: &'a str,
    ) -> &mut Self {
        self.argument_command_type = Some(ArgumentCommandType::DisplaysAndSpeakers);
        self.desktop_display_name = Some(desktop_display_name);
        self.couch_display_name = Some(couch_display_name);
        self.desktop_speaker_name = Some(desktop_speaker_name);
        self.couch_speaker_name = Some(couch_speaker_name);

        self
    }

    pub fn displays_only(
        &mut self,
        desktop_display_name: &'a str,
        couch_display_name: &'a str,
    ) -> &mut Self {
        self.argument_command_type = Some(ArgumentCommandType::DisplaysOnly);
        self.desktop_display_name = Some(desktop_display_name);
        self.couch_display_name = Some(couch_display_name);

        self
    }

    pub fn speakers_only(
        &mut self,
        desktop_speaker_name: &'a str,
        couch_speaker_name: &'a str,
    ) -> &mut Self {
        self.argument_command_type = Some(ArgumentCommandType::SpeakersOnly);
        self.desktop_speaker_name = Some(desktop_speaker_name);
        self.couch_speaker_name = Some(couch_speaker_name);

        self
    }

    pub fn build(&mut self) -> Arguments {
        let argument_command_type = self.argument_command_type.as_ref().unwrap();

        match argument_command_type {
            ArgumentCommandType::DisplaysAndSpeakers => {
                let displays = self.build_displays_options();
                let speakers = self.build_speakers_options();

                Arguments {
                    command: Commands::DisplaysAndSpeakers {
                        displays,
                        speakers,
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
            ArgumentCommandType::DisplaysOnly => {
                let displays = self.build_displays_options();

                Arguments {
                    command: Commands::DisplaysOnly {
                        displays,
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
            ArgumentCommandType::SpeakersOnly => {
                let speakers = self.build_speakers_options();

                Arguments {
                    command: Commands::SpeakersOnly {
                        speakers,
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
        }
    }

    fn build_displays_options(&mut self) -> DisplaysOptions {
        let desktop_display_name = self.desktop_display_name.unwrap().to_string();
        let couch_display_name = self.couch_display_name.unwrap().to_string();

        DisplaysOptions {
            desktop_display_name,
            couch_display_name,
        }
    }

    fn build_speakers_options(&mut self) -> SpeakersOptions {
        let desktop_speaker_name = self.desktop_speaker_name.unwrap().to_string();
        let couch_speaker_name = self.couch_speaker_name.unwrap().to_string();

        SpeakersOptions {
            desktop_speaker_name,
            couch_speaker_name,
        }
    }
}

enum ArgumentCommandType {
    DisplaysAndSpeakers,
    DisplaysOnly,
    SpeakersOnly,
}
