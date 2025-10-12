use std::marker::PhantomData;

use convertible_couch_lib::{
    displays_settings::{
        CurrentDisplaysSettingsApiTrait, DisplaysSettings, DisplaysSettingsResult,
    },
    log::{configure_logger, LogLevel},
    speakers_settings::{
        CurrentSpeakersSettingsApiTrait, SpeakersSettings, SpeakersSettingsResult,
    },
    ApplicationError,
};

use crate::commands::{
    change::ChangeCommands, shared::log_level_option::LogLevelOption, Arguments,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationResult {
    DisplaysAndSpeakers {
        displays_result: DisplaysSettingsResult,
        speakers_result: SpeakersSettingsResult,
    },
    DisplaysOnly {
        displays_result: DisplaysSettingsResult,
    },
    SpeakersOnly {
        speakers_result: SpeakersSettingsResult,
    },
}

pub struct Application<
    TDisplaysSettingsApi: CurrentDisplaysSettingsApiTrait,
    TSpeakersSettingsApi: CurrentSpeakersSettingsApiTrait,
    TDisplaysSettings: DisplaysSettings<TDisplaysSettingsApi>,
    TSpeakersSettings: SpeakersSettings<TSpeakersSettingsApi>,
> {
    displays_settings: TDisplaysSettings,
    speakers_settings: TSpeakersSettings,
    displays_settings_api: PhantomData<TDisplaysSettingsApi>,
    speakers_settings_api: PhantomData<TSpeakersSettingsApi>,
}

impl<
        TDisplaysSettingsApi: CurrentDisplaysSettingsApiTrait,
        TSpeakersSettingsApi: CurrentSpeakersSettingsApiTrait,
        TDisplaysSettings: DisplaysSettings<TDisplaysSettingsApi>,
        TSpeakersSettings: SpeakersSettings<TSpeakersSettingsApi>,
    >
    Application<TDisplaysSettingsApi, TSpeakersSettingsApi, TDisplaysSettings, TSpeakersSettings>
{
    pub fn bootstrap(
        displays_settings_api: TDisplaysSettingsApi,
        speakers_settings_api: TSpeakersSettingsApi,
    ) -> Self {
        Self {
            displays_settings: TDisplaysSettings::new(displays_settings_api),
            speakers_settings: TSpeakersSettings::new(speakers_settings_api),
            displays_settings_api: PhantomData,
            speakers_settings_api: PhantomData,
        }
    }

    pub fn execute(&mut self, args: &Arguments) -> Result<ApplicationResult, ApplicationError> {
        match &args.command {
            crate::commands::Commands::Change(change_commands) => match change_commands {
                ChangeCommands::DisplaysAndSpeakers {
                    displays,
                    speakers,
                    shared,
                } => {
                    let log_level = map_to_log_level(&shared.log_level);

                    configure_logger(&log_level)?;

                    let displays_result = self.displays_settings.change_primary_display(
                        &displays.desktop_display_name,
                        &displays.couch_display_name,
                    )?;

                    let speakers_result = self.speakers_settings.change_default_speaker(
                        &speakers.desktop_speaker_name,
                        &speakers.couch_speaker_name,
                    )?;

                    Ok(ApplicationResult::DisplaysAndSpeakers {
                        displays_result,
                        speakers_result,
                    })
                }
                ChangeCommands::DisplaysOnly { displays, shared } => {
                    let log_level = map_to_log_level(&shared.log_level);

                    configure_logger(&log_level)?;

                    let displays_result = self.displays_settings.change_primary_display(
                        &displays.desktop_display_name,
                        &displays.couch_display_name,
                    )?;

                    Ok(ApplicationResult::DisplaysOnly { displays_result })
                }
                ChangeCommands::SpeakersOnly { speakers, shared } => {
                    let log_level = map_to_log_level(&shared.log_level);

                    configure_logger(&log_level)?;

                    let speakers_result = self.speakers_settings.change_default_speaker(
                        &speakers.desktop_speaker_name,
                        &speakers.couch_speaker_name,
                    )?;

                    Ok(ApplicationResult::SpeakersOnly { speakers_result })
                }
            },
            crate::commands::Commands::Info { device, shared } => todo!(),
        }
    }
}

fn map_to_log_level(log_level_option: &LogLevelOption) -> LogLevel {
    match log_level_option {
        LogLevelOption::Off => LogLevel::Off,
        LogLevelOption::Error => LogLevel::Error,
        LogLevelOption::Warn => LogLevel::Warn,
        LogLevelOption::Info => LogLevel::Info,
        LogLevelOption::Debug => LogLevel::Debug,
        LogLevelOption::Trace => LogLevel::Trace,
    }
}

#[cfg(test)]
mod tests {
    use convertible_couch_lib::log::LogLevel;
    use test_case::test_case;

    use crate::{
        application::map_to_log_level, commands::shared::log_level_option::LogLevelOption,
    };

    #[test_case(LogLevelOption::Off => LogLevel::Off)]
    #[test_case(LogLevelOption::Error => LogLevel::Error)]
    #[test_case(LogLevelOption::Warn => LogLevel::Warn)]
    #[test_case(LogLevelOption::Info => LogLevel::Info)]
    #[test_case(LogLevelOption::Debug => LogLevel::Debug)]
    #[test_case(LogLevelOption::Trace => LogLevel::Trace)]
    fn it_should_map_a_log_level_option_to_the_expected_log_level(
        log_level_option: LogLevelOption,
    ) -> LogLevel {
        map_to_log_level(&log_level_option)
    }
}
