use clap::Parser;
use convertible_couch::display_settings::{
    DisplaySettings, Win32DevicesDisplayImpl, Win32GraphicsGdiImpl,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    desktop_monitor_name: String,
    #[arg(short, long)]
    couch_monitor_name: String,
}

fn main() {
    let args: Args = Args::parse();

    let win32_devices_display = Win32DevicesDisplayImpl;
    let win32_graphics_gdi = Win32GraphicsGdiImpl;
    let display_settings = DisplaySettings::new(win32_devices_display, win32_graphics_gdi);

    unsafe {
        let set_monitors_to_position_result = display_settings
            .swap_primary_monitors(&args.desktop_monitor_name, &args.couch_monitor_name);

        match set_monitors_to_position_result {
            Ok(response) => {
                if response.reboot_required {
                    println!("The settings change was successful but the computer must be restarted for the graphics mode to work.");
                }
            }
            Err(message) => eprint!("Failed because of {0}", message),
        }
    }
}
