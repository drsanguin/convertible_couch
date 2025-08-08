use clap::{Args, Parser, Subcommand};
use convertible_couch_lib::{
    displays_settings::{DisplaysSettings, DisplaysSettingsResult},
    log::{configure_logger, LogLevel},
    speakers_settings::{SpeakersSettings, SpeakersSettingsResult},
};

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

#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationResult {
    DisplaysAndSpeakers {
        displays_result: DisplaysSettingsResult,
        speakers_result: SpeakersSettingsResult,
    },
    DisplaysOnly {
        displays_result: DisplaysSettingsResult,
    },
    SpeakersOnly {
        speakers_result: SpeakersSettingsResult,
    },
}

pub fn run_app<
    TDisplaysSettingsApi,
    TSpeakersSettingsApi,
    TDisplaysSettings: DisplaysSettings<TDisplaysSettingsApi>,
    TSpeakersSettings: SpeakersSettings<TSpeakersSettingsApi>,
>(
    args: &Arguments,
    displays_settings: &mut TDisplaysSettings,
    speakers_settings: &mut TSpeakersSettings,
) -> Result<ApplicationResult, String> {
    match &args.command {
        Commands::DisplaysAndSpeakers {
            displays,
            speakers,
            shared,
        } => {
            configure_logger(&shared.log_level)?;

            let displays_result = displays_settings.change_primary_display(
                &displays.desktop_display_name,
                &displays.couch_display_name,
            )?;

            let speakers_result = speakers_settings.change_default_speaker(
                &speakers.desktop_speaker_name,
                &speakers.couch_speaker_name,
            )?;

            Ok(ApplicationResult::DisplaysAndSpeakers {
                displays_result,
                speakers_result,
            })
        }
        Commands::DisplaysOnly { displays, shared } => {
            configure_logger(&shared.log_level)?;

            let displays_result = displays_settings.change_primary_display(
                &displays.desktop_display_name,
                &displays.couch_display_name,
            )?;

            Ok(ApplicationResult::DisplaysOnly { displays_result })
        }
        Commands::SpeakersOnly { speakers, shared } => {
            configure_logger(&shared.log_level)?;

            let speakers_result = speakers_settings.change_default_speaker(
                &speakers.desktop_speaker_name,
                &speakers.couch_speaker_name,
            )?;

            Ok(ApplicationResult::SpeakersOnly { speakers_result })
        }
    }
}
