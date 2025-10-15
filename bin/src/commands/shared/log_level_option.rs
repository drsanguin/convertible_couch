use clap::{builder::PossibleValue, ValueEnum};

/// An enum representing the available verbosity level filters of the logger.
#[derive(Clone, Debug, PartialEq)]
pub enum LogLevelOption {
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

impl ValueEnum for LogLevelOption {
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
        let canonical_argument_value = match self {
            Self::Off => PossibleValue::new("off"),
            Self::Error => PossibleValue::new("error"),
            Self::Warn => PossibleValue::new("warn"),
            Self::Info => PossibleValue::new("info"),
            Self::Debug => PossibleValue::new("debug"),
            Self::Trace => PossibleValue::new("trace"),
        };

        Some(canonical_argument_value)
    }
}

#[cfg(test)]
mod tests {
    use clap::{builder::PossibleValue, ValueEnum};
    use test_case::test_case;

    use crate::commands::shared::log_level_option::LogLevelOption;

    #[test]
    fn it_should_provide_all_possible_argument_values() {
        // Act
        let value_variants = LogLevelOption::value_variants();

        // Assert
        assert_eq!(
            value_variants,
            [
                LogLevelOption::Off,
                LogLevelOption::Error,
                LogLevelOption::Warn,
                LogLevelOption::Info,
                LogLevelOption::Debug,
                LogLevelOption::Trace
            ]
        );
    }

    #[test_case(LogLevelOption::Off => Some(PossibleValue::new("off")); "when log level is off")]
    #[test_case(LogLevelOption::Error => Some(PossibleValue::new("error")); "when log level is error")]
    #[test_case(LogLevelOption::Warn => Some(PossibleValue::new("warn")); "when log level is warn")]
    #[test_case(LogLevelOption::Info => Some(PossibleValue::new("info")); "when log level is info")]
    #[test_case(LogLevelOption::Debug => Some(PossibleValue::new("debug")); "when log level is debug")]
    #[test_case(LogLevelOption::Trace => Some(PossibleValue::new("trace")); "when log level is trace")]
    fn it_should_provide_the_canonical_argument_value(
        log_level: LogLevelOption,
    ) -> Option<PossibleValue> {
        // Act
        log_level.to_possible_value()
    }
}
