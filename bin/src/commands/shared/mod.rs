use clap::Args;

use crate::commands::shared::log_level_option::LogLevelOption;

pub mod log_level_option;

#[derive(Args, Debug)]
pub struct SharedOptions {
    /// Set the program's log level
    #[arg(short, long, value_enum, default_value_t = LogLevelOption::Warn)]
    pub log_level: LogLevelOption,
}
