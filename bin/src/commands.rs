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
    #[arg(short, long, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,
}

#[derive(Args, Debug)]
pub struct DisplaysOptions {
    #[arg(long)]
    pub desktop_display_name: String,
    #[arg(long)]
    pub couch_display_name: String,
}

#[derive(Args, Debug)]
pub struct SpeakersOptions {
    #[arg(long)]
    pub desktop_speaker_name: String,
    #[arg(long)]
    pub couch_speaker_name: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    DisplaysAndSpeakers {
        #[command(flatten)]
        displays: DisplaysOptions,
        #[command(flatten)]
        speakers: SpeakersOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
    DisplaysOnly {
        #[command(flatten)]
        displays: DisplaysOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
    SpeakersOnly {
        #[command(flatten)]
        speakers: SpeakersOptions,
        #[command(flatten)]
        shared: SharedOptions,
    },
}
