use convertible_couch::commands::Arguments;

use crate::arrangements::builders::arguments::ArgumentsBuilder;

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

pub struct DisplaysCommandBuilder;

impl DisplaysCommandBuilder {
    pub fn any(
        self,
        displays_command: &DisplaysCommand,
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

    pub fn change(
        self,
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
