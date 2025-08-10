use clap::{Args, Parser, Subcommand};
use convertible_couch_lib::log::LogLevel;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct SharedOptions {
    /// Set the program's log level
    #[arg(short, long, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,
}

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
pub enum Commands {
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
    DisplaysOnly {
        #[command(flatten)]
        displays: DisplaysOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
    /// Change only default speaker
    SpeakersOnly {
        #[command(flatten)]
        speakers: SpeakersOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
}
