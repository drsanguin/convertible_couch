use clap::{Args, Subcommand};

use crate::commands::shared::SharedOptions;

#[derive(Args, Debug)]
pub struct DisplaysOptions {
    /// The name of the display to use on your dekstop
    #[arg(long)]
    pub desktop_display_name: String,
    /// The name of the display to use on your couch
    #[arg(long)]
    pub couch_display_name: String,
}

#[derive(Args, Debug)]
pub struct SpeakersOptions {
    /// The name of the speaker to use on your desktop
    #[arg(long)]
    pub desktop_speaker_name: String,
    /// The name of the speaker to use on your couch
    #[arg(long)]
    pub couch_speaker_name: String,
}

#[derive(Debug, Subcommand)]
pub enum ChangeCommands {
    /// Change primary display and default speaker
    DisplaysAndSpeakers {
        #[command(flatten)]
        displays: DisplaysOptions,
        #[command(flatten)]
        speakers: SpeakersOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
    /// Change only primary display
    Displays {
        #[command(flatten)]
        displays: DisplaysOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
    /// Change only default speaker
    Speakers {
        #[command(flatten)]
        speakers: SpeakersOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
}
