use clap::Parser;
use convertible_couch_lib::{
    display_settings::{DisplaySettings, DisplaySettingsResult},
    log::{configure_logger, LogLevel},
    sound_settings::{SoundSettings, SoundSettingsResult},
};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub desktop_display_name: String,
    #[arg(long)]
    pub couch_display_name: String,
    #[arg(long)]
    pub desktop_speaker_name: String,
    #[arg(long)]
    pub couch_speaker_name: String,
    #[arg(short, long, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,
}

pub struct ApplicationResult {
    pub display_settings: DisplaySettingsResult,
    pub sound_settings: SoundSettingsResult,
}

pub fn run_app<
    TDisplaySettingsApi,
    TSoundSettingsApi,
    TDisplaySettings: DisplaySettings<TDisplaySettingsApi>,
    TSoundSettings: SoundSettings<TSoundSettingsApi>,
>(
    args: Args,
    mut display_settings: TDisplaySettings,
    mut sound_settings: TSoundSettings,
) -> Result<ApplicationResult, Box<dyn Error>> {
    configure_logger(args.log_level)?;

    let display_settings_result = display_settings
        .change_primary_displays(&args.desktop_display_name, &args.couch_display_name)?;

    let sound_settings_result = sound_settings
        .change_default_output_device(&args.desktop_speaker_name, &args.couch_speaker_name)?;

    Ok(ApplicationResult {
        display_settings: display_settings_result,
        sound_settings: sound_settings_result,
    })
}
