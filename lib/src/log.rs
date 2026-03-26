use log::{LevelFilter, trace};
use log4rs::{
    Config,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config,
};

use crate::application_error::ApplicationError;

#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        name.rsplit("::").nth(1).unwrap_or(name)
    }};
}

pub struct Tracer {
    fn_name: String,
}

impl Tracer {
    pub fn new(fn_name: String) -> Self {
        trace!("→ ENTERING: {fn_name}");

        Self { fn_name }
    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        trace!("← EXITING: {}", self.fn_name);
    }
}

#[macro_export]
macro_rules! trace_fn {
    () => {
        #[allow(unused_variables)]
        let tracer =
            $crate::log::Tracer::new(format!("{}::{}", std::module_path!(), $crate::func!()));
    };
}

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

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

#[cfg(test)]
mod should {
    use log::LevelFilter;
    use test_case::test_case;

    use super::{LogLevel, map_to_level_filter};

    #[test_case(&LogLevel::Off => LevelFilter::Off; "when log level is off")]
    #[test_case(&LogLevel::Error => LevelFilter::Error; "when log level is error")]
    #[test_case(&LogLevel::Warn => LevelFilter::Warn; "when log level is warn")]
    #[test_case(&LogLevel::Info => LevelFilter::Info; "when log level is info")]
    #[test_case(&LogLevel::Debug => LevelFilter::Debug; "when log level is debug")]
    #[test_case(&LogLevel::Trace => LevelFilter::Trace; "when log level is trace")]
    fn map_a_log_level_to_a_log_filter(log_level: &LogLevel) -> LevelFilter {
        // Act
        map_to_level_filter(log_level)
    }
}
