use clap::{builder::PossibleValue, ValueEnum};
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

use crate::ApplicationError;

/// An enum representing the available verbosity level filters of the logger.
#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

/// Initializes the global logger with the provided level.
pub fn configure_logger(log_level: &LogLevel) -> Result<(), ApplicationError> {
    if log_level == &LogLevel::Off {
        return Ok(());
    }

    let encoder = PatternEncoder::new("| {({l}):5.5} | {m}\r\n");
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();
    let appender = Appender::builder().build("stdout", Box::new(stdout));
    let level = map_to_level_filter(log_level);
    let root = Root::builder().appender("stdout").build(level);
    let config = Config::builder().appender(appender).build(root)?;

    init_config(config)?;

    Ok(())
}

fn map_to_level_filter(log_level: &LogLevel) -> LevelFilter {
    match log_level {
        LogLevel::Off => LevelFilter::Off,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    }
}

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Off,
            Self::Error,
            Self::Warn,
            Self::Info,
            Self::Debug,
            Self::Trace,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Off => PossibleValue::new("off"),
            Self::Error => PossibleValue::new("error"),
            Self::Warn => PossibleValue::new("warn"),
            Self::Info => PossibleValue::new("info"),
            Self::Debug => PossibleValue::new("debug"),
            Self::Trace => PossibleValue::new("trace"),
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::{builder::PossibleValue, ValueEnum};
    use log::LevelFilter;
    use test_case::test_case;

    use super::{configure_logger, map_to_level_filter, LogLevel};

    #[test]
    fn if_should_configure_the_logger() {
        // Act
        let result = configure_logger(&LogLevel::Off);

        assert_eq!(result, Ok(()));
    }

    #[test_case(&LogLevel::Off => LevelFilter::Off; "when log level is off")]
    #[test_case(&LogLevel::Error => LevelFilter::Error; "when log level is error")]
    #[test_case(&LogLevel::Warn => LevelFilter::Warn; "when log level is warn")]
    #[test_case(&LogLevel::Info => LevelFilter::Info; "when log level is info")]
    #[test_case(&LogLevel::Debug => LevelFilter::Debug; "when log level is debug")]
    #[test_case(&LogLevel::Trace => LevelFilter::Trace; "when log level is trace")]
    fn it_should_map_a_log_level_to_a_log_filter(log_level: &LogLevel) -> LevelFilter {
        // Act
        map_to_level_filter(log_level)
    }

    #[test]
    fn it_should_provide_all_possible_argument_values() {
        // Act
        let value_variants = LogLevel::value_variants();

        // Assert
        assert_eq!(
            value_variants,
            [
                LogLevel::Off,
                LogLevel::Error,
                LogLevel::Warn,
                LogLevel::Info,
                LogLevel::Debug,
                LogLevel::Trace
            ]
        );
    }

    #[test_case(LogLevel::Off => Some(PossibleValue::new("off")); "when log level is off")]
    #[test_case(LogLevel::Error => Some(PossibleValue::new("error")); "when log level is error")]
    #[test_case(LogLevel::Warn => Some(PossibleValue::new("warn")); "when log level is warn")]
    #[test_case(LogLevel::Info => Some(PossibleValue::new("info")); "when log level is info")]
    #[test_case(LogLevel::Debug => Some(PossibleValue::new("debug")); "when log level is debug")]
    #[test_case(LogLevel::Trace => Some(PossibleValue::new("trace")); "when log level is trace")]
    fn it_should_provide_the_canonical_argument_value(
        log_level: LogLevel,
    ) -> Option<PossibleValue> {
        // Act
        log_level.to_possible_value()
    }
}
