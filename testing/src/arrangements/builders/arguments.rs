use convertible_couch::commands::{
    Arguments, Commands,
    change::{ChangeCommands, DisplaysOptions, SpeakersOptions},
    info::Device,
    shared::{SharedOptions, log_level_option::LogLevelOption},
};

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
