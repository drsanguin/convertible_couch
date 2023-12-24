use clap::Parser;

mod display;

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

    unsafe {
        let primary_monitor_name = display::get_primary_monitor_name().unwrap();
        let new_primary_monitor_name = if primary_monitor_name == args.desktop_monitor_name {
            args.couch_monitor_name
        } else {
            args.desktop_monitor_name
        };
        let new_primary_monitor_current_position =
            display::get_monitor_position(&new_primary_monitor_name).unwrap();
        let set_monitors_to_position_result =
            display::set_monitors_to_position(&new_primary_monitor_current_position);

        match set_monitors_to_position_result {
            Ok(()) => (),
            Err(message) => eprint!("Failed because of {0}", message),
        }
    }
}
