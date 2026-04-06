use crate::arrangements::fuzzing::computer::FuzzedComputer;

use convertible_couch::{
    application::{Application, ApplicationChangeResult, CommandResult},
    commands::{
        Arguments, Commands,
        change::{ChangeCommands, DisplaysOptions, SpeakersOptions},
        info::Device,
        shared::{SharedOptions, log_level_option::LogLevelOption},
    },
};
use convertible_couch_lib::{
    application_result::ApplicationResult, displays_settings::DisplaysSettingsResult,
    speakers_settings::SpeakersSettingsResult,
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

pub enum DisplaysCommand {
    ChangeDisplaysAndSpeakers,
    ChangeDisplays,
    InfoDisplaysAndSpeakers,
    InfoDisplays,
}

pub enum ChangeDisplaysCommand {
    ChangeDisplaysAndSpeakers,
    ChangeDisplays,
}

pub struct ArgumentsBuilder;

impl ArgumentsBuilder {
    pub fn change() -> ChangeCommandBuilder {
        ChangeCommandBuilder::default()
    }

    pub fn info() -> InfoCommandBuilder {
        InfoCommandBuilder::default()
    }

    pub fn display_command(
        displays_command: DisplaysCommand,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Arguments {
        match displays_command {
            DisplaysCommand::ChangeDisplaysAndSpeakers => ArgumentsBuilder::change()
                .displays_and_speakers(
                    desktop_display_name,
                    couch_display_name,
                    desktop_speaker_name,
                    couch_speaker_name,
                )
                .build(),
            DisplaysCommand::ChangeDisplays => ArgumentsBuilder::change()
                .displays_only(desktop_display_name, couch_display_name)
                .build(),
            DisplaysCommand::InfoDisplaysAndSpeakers => {
                ArgumentsBuilder::info().displays_and_speakers().build()
            }
            DisplaysCommand::InfoDisplays => ArgumentsBuilder::info().displays_only().build(),
        }
    }

    pub fn change_display_command(
        displays_command: &ChangeDisplaysCommand,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Arguments {
        match displays_command {
            ChangeDisplaysCommand::ChangeDisplaysAndSpeakers => ArgumentsBuilder::change()
                .displays_and_speakers(
                    desktop_display_name,
                    couch_display_name,
                    desktop_speaker_name,
                    couch_speaker_name,
                )
                .build(),
            ChangeDisplaysCommand::ChangeDisplays => ArgumentsBuilder::change()
                .displays_only(desktop_display_name, couch_display_name)
                .build(),
        }
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

pub struct CommandResultBuilder;

impl CommandResultBuilder {
    pub fn change_displays(
        change_displays_command: &ChangeDisplaysCommand,
        new_primary_display: &str,
        new_default_speaker: &str,
    ) -> ApplicationResult<CommandResult> {
        let commmand = match change_displays_command {
            ChangeDisplaysCommand::ChangeDisplaysAndSpeakers => {
                CommandResult::Change(ApplicationChangeResult::DisplaysAndSpeakers {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: new_primary_display.to_string(),
                        reboot_required: false,
                    },
                    speakers_result: SpeakersSettingsResult {
                        new_default_speaker: new_default_speaker.to_string(),
                    },
                })
            }
            ChangeDisplaysCommand::ChangeDisplays => {
                CommandResult::Change(ApplicationChangeResult::DisplaysOnly {
                    displays_result: DisplaysSettingsResult {
                        new_primary_display: new_primary_display.to_string(),
                        reboot_required: false,
                    },
                })
            }
        };

        Ok(commmand)
    }
}
