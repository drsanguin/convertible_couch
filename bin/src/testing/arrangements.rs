use convertible_couch_lib::testing::fuzzing::computer::FuzzedComputer;

use crate::{
    application::Application,
    commands::{
        Arguments, Commands,
        change::{ChangeCommands, DisplaysOptions, SpeakersOptions},
        info::Device,
        shared::{SharedOptions, log_level_option::LogLevelOption},
    },
};

pub struct ApplicationBuilder {
    computer: FuzzedComputer,
}

impl ApplicationBuilder {
    pub fn new(computer: FuzzedComputer) -> Self {
        Self { computer }
    }

    pub fn build(self) -> Application {
        let displays_settings_api = Box::new(self.computer.displays_settings_api);
        let speakers_settings_api = Box::new(self.computer.speakers_settings_api);

        Application::bootstrap(displays_settings_api, speakers_settings_api)
    }
}

pub struct ArgumentsBuilder;

impl Default for ArgumentsBuilder {
    fn default() -> Self {
        Self
    }
}

impl ArgumentsBuilder {
    pub fn change(self) -> ChangeCommandBuilder {
        ChangeCommandBuilder::default()
    }

    pub fn info(self) -> InfoCommandBuilder {
        InfoCommandBuilder::default()
    }
}

#[derive(Default)]
pub struct ChangeCommandBuilder {
    arguments: Option<Arguments>,
}

impl ChangeCommandBuilder {
    pub fn displays_and_speakers(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Change(ChangeCommands::DisplaysAndSpeakers {
                displays: DisplaysOptions {
                    desktop_display_name: desktop_display_name.to_string(),
                    couch_display_name: couch_display_name.to_string(),
                },
                speakers: SpeakersOptions {
                    desktop_speaker_name: desktop_speaker_name.to_string(),
                    couch_speaker_name: couch_speaker_name.to_string(),
                },
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            }),
        });

        self
    }

    pub fn displays_only(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Change(ChangeCommands::Displays {
                displays: DisplaysOptions {
                    desktop_display_name: desktop_display_name.to_string(),
                    couch_display_name: couch_display_name.to_string(),
                },
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            }),
        });

        self
    }

    pub fn speakers_only(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Change(ChangeCommands::Speakers {
                speakers: SpeakersOptions {
                    desktop_speaker_name: desktop_speaker_name.to_string(),
                    couch_speaker_name: couch_speaker_name.to_string(),
                },
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            }),
        });

        self
    }

    pub fn build(&mut self) -> Arguments {
        self.arguments.take().unwrap()
    }
}

#[derive(Default)]
pub struct InfoCommandBuilder {
    arguments: Option<Arguments>,
}

impl InfoCommandBuilder {
    pub fn displays_and_speakers(&mut self) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Info {
                device: Device::DisplaysAndSpeakers,
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            },
        });

        self
    }

    pub fn displays_only(&mut self) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Info {
                device: Device::Displays,
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            },
        });

        self
    }

    pub fn speakers_only(&mut self) -> &mut Self {
        self.arguments = Some(Arguments {
            command: Commands::Info {
                device: Device::Speakers,
                shared: SharedOptions {
                    log_level: LogLevelOption::Off,
                },
            },
        });

        self
    }

    pub fn build(&mut self) -> Arguments {
        self.arguments.take().unwrap()
    }
}
