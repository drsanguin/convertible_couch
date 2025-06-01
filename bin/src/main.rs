use clap::Parser;
use convertible_couch_lib::{
    display_settings::{self, DisplaySettings},
    log::{configure_logger, LogLevel},
    sound_settings::{self, SoundSettings},
};
use log::{error, info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    desktop_display_name: String,
    #[arg(long)]
    couch_display_name: String,
    #[arg(long)]
    desktop_speaker_name: String,
    #[arg(long)]
    couch_speaker_name: String,
    #[arg(short, long, value_enum, default_value_t = LogLevel::Off)]
    log_level: LogLevel,
}

fn main() {
    let args = Args::parse();

    match configure_logger(args.log_level).and_then(|_| {
        display_settings::Current::new(display_settings::CurrentDisplaySettingsApi)
            .swap_primary_displays(&args.desktop_display_name, &args.couch_display_name)
    }) {
        Ok(response) => {
            match response.new_primary {
                Some(new_primary) => info!("Primary display set to {new_primary}"),
                None => error!("Primary display has not been changed for an unknow reason"),
            }

            if response.reboot_required {
                warn!("The settings change was successful but the computer must be restarted for the graphics mode to work.");
            }
        }
        Err(message) => error!("{message}"),
    }

    match sound_settings::Current::new(sound_settings::CurrentSoundSettingsApi)
        .swap_default_output_device(&args.desktop_speaker_name, &args.couch_speaker_name)
    {
        Ok(_) => info!("Switched sound output device"),
        Err(message) => error!("{message}"),
    }
}
