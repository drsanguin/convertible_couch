use convertible_couch::commands::{
    Arguments, Commands,
    info::Device,
    shared::{SharedOptions, log_level_option::LogLevelOption},
};

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
