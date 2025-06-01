use clap::Parser;
use convertible_couch_lib::{
    display_settings::DisplaySettings,
    log::{configure_logger, LogLevel},
    sound_settings::SoundSettings,
};
use log::{error, info, warn};
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

pub fn run_app<
    TDisplaySettingsApi,
    TSoundSettingsApi,
    TDisplaySettings: DisplaySettings<TDisplaySettingsApi>,
    TSoundSettings: SoundSettings<TSoundSettingsApi>,
>(
    args: Args,
    mut display_settings: TDisplaySettings,
    mut sound_settings: TSoundSettings,
) -> Result<(), Box<dyn Error>> {
    configure_logger(args.log_level)?;

    display_settings
        .swap_primary_displays(&args.desktop_display_name, &args.couch_display_name)
        .and_then(
            |response| match (response.new_primary, response.reboot_required) {
                (None, _) => Ok(error!("Primary display has not been changed for an unknow reason")),
                (Some(_), true) => Ok(warn!("The settings change was successful but the computer must be restarted for the graphics mode to work.")),
                (Some(new_primary), false) => Ok(info!("Primary display set to {new_primary}")),
            },
        )?;

    sound_settings
        .swap_default_output_device(&args.desktop_speaker_name, &args.couch_speaker_name)?;

    Ok(())
}
