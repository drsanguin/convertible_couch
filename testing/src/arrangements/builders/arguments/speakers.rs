use convertible_couch::commands::Arguments;

use crate::arrangements::builders::arguments::ArgumentsBuilder;

pub enum SpeakersCommand {
    ChangeDisplaysAndSpeakers,
    ChangeSpeakers,
    InfoDisplaysAndSpeakers,
    InfoSpeakers,
}

pub enum ChangeSpeakersCommand {
    ChangeDisplaysAndSpeakers,
    ChangeSpeakers,
}

pub struct SpeakersCommandBuilder;

impl SpeakersCommandBuilder {
    pub fn any(
        self,
        speakers_command: &SpeakersCommand,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Arguments {
        match speakers_command {
            SpeakersCommand::ChangeDisplaysAndSpeakers => ArgumentsBuilder::change()
                .displays_and_speakers(
                    desktop_display_name,
                    couch_display_name,
                    desktop_speaker_name,
                    couch_speaker_name,
                )
                .build(),
            SpeakersCommand::ChangeSpeakers => ArgumentsBuilder::change()
                .speakers_only(desktop_speaker_name, couch_speaker_name)
                .build(),
            SpeakersCommand::InfoDisplaysAndSpeakers => {
                ArgumentsBuilder::info().displays_and_speakers().build()
            }
            SpeakersCommand::InfoSpeakers => ArgumentsBuilder::info().speakers_only().build(),
        }
    }

    pub fn change(
        self,
        speakers_command: &ChangeSpeakersCommand,
        desktop_display_name: &str,
        couch_display_name: &str,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Arguments {
        match speakers_command {
            ChangeSpeakersCommand::ChangeDisplaysAndSpeakers => ArgumentsBuilder::change()
                .displays_and_speakers(
                    desktop_display_name,
                    couch_display_name,
                    desktop_speaker_name,
                    couch_speaker_name,
                )
                .build(),
            ChangeSpeakersCommand::ChangeSpeakers => ArgumentsBuilder::change()
                .speakers_only(desktop_speaker_name, couch_speaker_name)
                .build(),
        }
    }
}
