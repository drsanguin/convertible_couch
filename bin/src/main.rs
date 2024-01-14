use clap::Parser;
use convertible_couch_common::win32::Win32Impl;
use convertible_couch_lib::{
    display_settings::DisplaySettings,
    log::{configure_logger, LogLevel},
};
use log::{error, info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    desktop_monitor_name: String,
    #[arg(short, long)]
    couch_monitor_name: String,
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

fn main() {
    let args: Args = Args::parse();

    match configure_logger(args.log_level).and_then(|_| {
        DisplaySettings::new(Win32Impl)
            .swap_primary_monitors(&args.desktop_monitor_name, &args.couch_monitor_name)
    }) {
        Ok(response) => {
            match response.new_primary {
                Some(new_primary) => info!("Primary monitor set to {new_primary}"),
                None => error!("Primary monitor has not been changed for an unknow reason"),
            }

            if response.reboot_required {
                warn!("The settings change was successful but the computer must be restarted for the graphics mode to work.");
            }
        }
        Err(message) => error!("{message}"),
    }
}
