use clap::{Args, Parser, Subcommand};
use convertible_couch_lib::{
    display_settings::{DisplaySettings, DisplaySettingsResult},
    log::{configure_logger, LogLevel},
    sound_settings::{SoundSettings, SoundSettingsResult},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct SharedOpts {
    #[arg(short, long, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,
}

#[derive(Args, Debug)]
pub struct VideoOpts {
    #[arg(long)]
    pub desktop_display_name: String,
    #[arg(long)]
    pub couch_display_name: String,
}

#[derive(Args, Debug)]
pub struct AudioOpts {
    #[arg(long)]
    pub desktop_speaker_name: String,
    #[arg(long)]
    pub couch_speaker_name: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    VideoAndAudio {
        #[command(flatten)]
        video: VideoOpts,
        #[command(flatten)]
        audio: AudioOpts,
        #[command(flatten)]
        shared: SharedOpts,
    },
    VideoOnly {
        #[command(flatten)]
        video: VideoOpts,
        #[command(flatten)]
        shared: SharedOpts,
    },
    AudioOnly {
        #[command(flatten)]
        audio: AudioOpts,
        #[command(flatten)]
        shared: SharedOpts,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationResult {
    VideoAndAudio {
        display_settings: DisplaySettingsResult,
        sound_settings: SoundSettingsResult,
    },
    VideoOnly {
        display_settings: DisplaySettingsResult,
    },
    AudioOnly {
        sound_settings: SoundSettingsResult,
    },
}

pub fn run_app<
    TDisplaySettingsApi,
    TSoundSettingsApi,
    TDisplaySettings: DisplaySettings<TDisplaySettingsApi>,
    TSoundSettings: SoundSettings<TSoundSettingsApi>,
>(
    args: Arguments,
    mut display_settings: TDisplaySettings,
    mut sound_settings: TSoundSettings,
) -> Result<ApplicationResult, String> {
    match args.command {
        Commands::VideoAndAudio {
            video,
            audio,
            shared,
        } => {
            configure_logger(shared.log_level)?;

            let display_settings_result = display_settings
                .change_primary_display(&video.desktop_display_name, &video.couch_display_name)?;

            let sound_settings_result = sound_settings.change_default_output_device(
                &audio.desktop_speaker_name,
                &audio.couch_speaker_name,
            )?;

            Ok(ApplicationResult::VideoAndAudio {
                display_settings: display_settings_result,
                sound_settings: sound_settings_result,
            })
        }
        Commands::VideoOnly { video, shared } => {
            configure_logger(shared.log_level)?;

            let display_settings_result = display_settings
                .change_primary_display(&video.desktop_display_name, &video.couch_display_name)?;

            Ok(ApplicationResult::VideoOnly {
                display_settings: display_settings_result,
            })
        }
        Commands::AudioOnly { audio, shared } => {
            configure_logger(shared.log_level)?;

            let sound_settings_result = sound_settings.change_default_output_device(
                &audio.desktop_speaker_name,
                &audio.couch_speaker_name,
            )?;

            Ok(ApplicationResult::AudioOnly {
                sound_settings: sound_settings_result,
            })
        }
    }
}
