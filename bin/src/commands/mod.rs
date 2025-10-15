use clap::{Parser, Subcommand};

use crate::commands::{change::ChangeCommands, info::Device, shared::SharedOptions};

pub mod change;
pub mod info;
pub mod shared;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Change primary display and/or default speaker
    #[command(subcommand)]
    Change(ChangeCommands),
    /// Get informations about displays and/or speakers
    Info {
        /// Which device(s) to get informations about
        #[arg(short, long, value_enum, default_value_t = Device::DisplaysAndSpeakers)]
        device: Device,
        #[command(flatten)]
        shared: SharedOptions,
    },
}
