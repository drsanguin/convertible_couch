use convertible_couch::commands::{
    Arguments, Commands,
    change::{ChangeCommands, DisplaysOptions, SpeakersOptions},
    shared::{SharedOptions, log_level_option::LogLevelOption},
};

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
