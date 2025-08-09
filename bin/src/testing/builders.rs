use crate::commands::{Arguments, Commands, DisplaysOptions, SharedOptions, SpeakersOptions};
use convertible_couch_lib::log::LogLevel;

pub struct ArgumentsBuilder {
    argument_command_type: Option<ArgumentCommandType>,
    desktop_display_name: Option<String>,
    couch_display_name: Option<String>,
    desktop_speaker_name: Option<String>,
    couch_speaker_name: Option<String>,
}

impl ArgumentsBuilder {
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
        desktop_display_name: String,
        couch_display_name: String,
        desktop_speaker_name: String,
        couch_speaker_name: String,
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
        desktop_display_name: String,
        couch_display_name: String,
    ) -> &mut Self {
        self.argument_command_type = Some(ArgumentCommandType::DisplaysOnly);
        self.desktop_display_name = Some(desktop_display_name);
        self.couch_display_name = Some(couch_display_name);

        self
    }

    pub fn speakers_only(
        &mut self,
        desktop_speaker_name: String,
        couch_speaker_name: String,
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
                let desktop_display_name = self.desktop_display_name.clone().unwrap();
                let couch_display_name = self.couch_display_name.clone().unwrap();
                let desktop_speaker_name = self.desktop_speaker_name.clone().unwrap();
                let couch_speaker_name = self.couch_speaker_name.clone().unwrap();

                Arguments {
                    command: Commands::DisplaysAndSpeakers {
                        displays: DisplaysOptions {
                            desktop_display_name,
                            couch_display_name,
                        },
                        speakers: SpeakersOptions {
                            desktop_speaker_name,
                            couch_speaker_name,
                        },
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
            ArgumentCommandType::DisplaysOnly => {
                let desktop_display_name = self.desktop_display_name.clone().unwrap();
                let couch_display_name = self.couch_display_name.clone().unwrap();

                Arguments {
                    command: Commands::DisplaysOnly {
                        displays: DisplaysOptions {
                            desktop_display_name,
                            couch_display_name,
                        },
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
            ArgumentCommandType::SpeakersOnly => {
                let desktop_speaker_name = self.desktop_speaker_name.clone().unwrap();
                let couch_speaker_name = self.couch_speaker_name.clone().unwrap();

                Arguments {
                    command: Commands::SpeakersOnly {
                        speakers: SpeakersOptions {
                            desktop_speaker_name,
                            couch_speaker_name,
                        },
                        shared: SharedOptions {
                            log_level: LogLevel::Off,
                        },
                    },
                }
            }
        }
    }
}

enum ArgumentCommandType {
    DisplaysAndSpeakers,
    DisplaysOnly,
    SpeakersOnly,
}
