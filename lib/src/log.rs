use clap::{builder::PossibleValue, ValueEnum};
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

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
pub fn configure_logger(log_level: LogLevel) {
    let level = map_to_level_filter(log_level);
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("| {({l}):5.5} | {m}\r\n")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();

    let _ = init_config(config).unwrap();
}

fn map_to_level_filter(log_level: LogLevel) -> LevelFilter {
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
    use clap::ValueEnum;
    use log::LevelFilter;
    use test_case::test_case;

    use super::{configure_logger, map_to_level_filter, LogLevel};

    #[test]
    fn if_should_configure_the_logger() {
        // Act
        configure_logger(LogLevel::Off);
    }

    #[test_case(LogLevel::Off => LevelFilter::Off; "when log level is off")]
    #[test_case(LogLevel::Error => LevelFilter::Error; "when log level is error")]
    #[test_case(LogLevel::Warn => LevelFilter::Warn; "when log level is warn")]
    #[test_case(LogLevel::Info => LevelFilter::Info; "when log level is info")]
    #[test_case(LogLevel::Debug => LevelFilter::Debug; "when log level is debug")]
    #[test_case(LogLevel::Trace => LevelFilter::Trace; "when log level is trace")]
    fn it_should_map_a_log_level_to_a_log_filter(log_level: LogLevel) -> LevelFilter {
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

    #[test_case(LogLevel::Off => "off"; "when log level is off")]
    #[test_case(LogLevel::Error => "error"; "when log level is error")]
    #[test_case(LogLevel::Warn => "warn"; "when log level is warn")]
    #[test_case(LogLevel::Info => "info"; "when log level is info")]
    #[test_case(LogLevel::Debug => "debug"; "when log level is debug")]
    #[test_case(LogLevel::Trace => "trace"; "when log level is trace")]
    fn it_should_provide_the_canonical_argument_value(log_level: LogLevel) -> String {
        // Act
        let possible_value = log_level.to_possible_value().unwrap();
        let actual_name = possible_value.get_name();

        String::from(actual_name)
    }
}
