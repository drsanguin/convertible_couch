use std::marker::PhantomData;

use convertible_couch_lib::{
    displays_settings::{
        CurrentDisplaysSettingsApiTrait, DisplaysSettings, DisplaysSettingsResult,
    },
    log::configure_logger,
    speakers_settings::{
        CurrentSpeakersSettingsApiTrait, SpeakersSettings, SpeakersSettingsResult,
    },
    ApplicationError,
};

use crate::commands::{Arguments, Commands};

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
            Commands::DisplaysAndSpeakers {
                displays,
                speakers,
                shared,
            } => {
                configure_logger(&shared.log_level)?;

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
            Commands::DisplaysOnly { displays, shared } => {
                configure_logger(&shared.log_level)?;

                let displays_result = self.displays_settings.change_primary_display(
                    &displays.desktop_display_name,
                    &displays.couch_display_name,
                )?;

                Ok(ApplicationResult::DisplaysOnly { displays_result })
            }
            Commands::SpeakersOnly { speakers, shared } => {
                configure_logger(&shared.log_level)?;

                let speakers_result = self.speakers_settings.change_default_speaker(
                    &speakers.desktop_speaker_name,
                    &speakers.couch_speaker_name,
                )?;

                Ok(ApplicationResult::SpeakersOnly { speakers_result })
            }
        }
    }
}
