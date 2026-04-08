use std::collections::HashSet;

use rand::RngExt;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::WIN32_ERROR;

use crate::arrangements::fuzzing::{
    ComputerBuilder,
    computer::{ComputerFuzzer, FuzzedComputer},
    speakers::{
        settings_api::{
            CurrentFuzzedSpeakersSettingsApi, FuzzedSpeakersSettingsApi,
            behaviour::CurrentFuzzedSpeakersSettingsApiBehaviour,
        },
        speaker_id::SpeakerIdFuzzer,
        speaker_name::SpeakerNameFuzzer,
    },
};

pub mod settings_api;
pub mod speaker_id;
pub mod speaker_name;

#[derive(Clone)]
pub struct FuzzedSpeaker {
    pub name: String,
    pub id: String,
    pub is_default: bool,
}

pub struct SpeakersFuzzer<'a> {
    computer_fuzzer: &'a mut ComputerFuzzer<'a>,
    min_count: usize,
    max_count: usize,
    default_speaker_name: Option<String>,
    alternative_names: HashSet<String>,
    behaviour: CurrentFuzzedSpeakersSettingsApiBehaviour,
}

impl<'a> SpeakersFuzzer<'a> {
    const MAX_SPEAKERS_COUNT: usize = 256;

    pub fn new(computer_fuzzer: &'a mut ComputerFuzzer<'a>) -> Self {
        Self {
            computer_fuzzer,
            min_count: 0,
            max_count: 0,
            default_speaker_name: None,
            alternative_names: HashSet::new(),
            behaviour: CurrentFuzzedSpeakersSettingsApiBehaviour::default(),
        }
    }

    pub fn of_which_there_are(&mut self, count: usize) -> &mut Self {
        self.min_count = count;
        self.max_count = count;

        self
    }

    pub fn of_which_there_are_at_least(&mut self, min_count: usize) -> &mut Self {
        self.min_count = min_count;
        self.max_count = Self::MAX_SPEAKERS_COUNT;

        self
    }

    pub fn whose_default_one_is_named(&mut self, default_speaker_name: &str) -> &mut Self {
        self.default_speaker_name = Some(default_speaker_name.to_string());

        self
    }

    pub fn with_an_alternative_one_named(&mut self, alternative_speaker_name: &str) -> &mut Self {
        self.alternative_names
            .insert(alternative_speaker_name.to_string());

        self
    }

    pub fn build_speakers(&'a mut self) -> &'a mut ComputerFuzzer<'a> {
        let mut names_already_taken = HashSet::new();

        if let Some(default_speaker_name) = &self.default_speaker_name {
            names_already_taken.insert(default_speaker_name.to_string());
        }

        names_already_taken.extend(self.alternative_names.clone());

        let count = self
            .computer_fuzzer
            .rand
            .random_range(self.min_count..=self.max_count);

        let names_not_taken = SpeakerNameFuzzer::new(self.computer_fuzzer.rand)
            .generate_several(count - names_already_taken.len(), &names_already_taken);

        let mut names = Vec::with_capacity(count);
        names.extend(names_already_taken);
        names.extend(names_not_taken);

        let ids = SpeakerIdFuzzer::new(self.computer_fuzzer.rand).generate_several(count);

        let default_speaker_index =
            self.default_speaker_name
                .as_ref()
                .map(|default_speaker_name| {
                    names
                        .iter()
                        .position(|name| name == default_speaker_name)
                        .unwrap()
                });

        let speakers = (0..count)
            .map(|i| FuzzedSpeaker {
                name: names[i].clone(),
                id: ids[i].clone(),
                is_default: default_speaker_index.is_some_and(|x| x == i),
            })
            .collect::<Vec<FuzzedSpeaker>>();

        let fuzzed_speakers_settings_api =
            CurrentFuzzedSpeakersSettingsApi::new(speakers, self.behaviour.clone());

        self.computer_fuzzer
            .set_speakers_settings_api(fuzzed_speakers_settings_api)
    }
}

impl<'a> ComputerBuilder<'a> for SpeakersFuzzer<'a> {
    fn build_computer(&'a mut self) -> FuzzedComputer {
        self.build_speakers().build_computer()
    }
}

#[cfg(target_os = "windows")]
pub enum Function {
    CoInitializeEx,
    CoCreateIMMDeviceEnumerator,
    IMMDeviceEnumeratorGetDefaultAudioEndpoint,
    IMMDeviceEnumeratorEnumAudioEndpoints,
    IMMDeviceGetId,
    IMMDeviceCollectionGetCount,
    IMMDeviceCollectionItem,
    IMMDeviceOpenPropertyStore,
    PropertyStoreGetValue,
    CoCreateIPolicyConfigVista,
    IPolicyConfigVistaSetDefaultEndpoint,
}

#[cfg(target_os = "windows")]
impl<'a> SpeakersFuzzer<'a> {
    pub fn for_which_function_fails_with(
        &mut self,
        function: Function,
        error: WIN32_ERROR,
    ) -> &mut Self {
        match function {
            Function::CoInitializeEx => self.for_which_co_initialize_ex_fails_with(error),
            Function::CoCreateIMMDeviceEnumerator => {
                self.for_which_co_create_immdevice_enumerator_fails_with(error)
            }
            Function::IMMDeviceEnumeratorGetDefaultAudioEndpoint => {
                self.for_which_immdevice_enumerator_get_default_audio_endpoint_fails_with(error)
            }
            Function::IMMDeviceEnumeratorEnumAudioEndpoints => {
                self.for_which_immdevice_enumerator_enum_audio_endpoints_fails_with(error)
            }
            Function::IMMDeviceGetId => self.for_which_immdevice_get_id_fails_with(error),
            Function::IMMDeviceCollectionGetCount => {
                self.for_which_immdevice_collection_get_count_fails_with(error)
            }
            Function::IMMDeviceCollectionItem => {
                self.for_which_immdevice_collection_item_fails_with(error)
            }
            Function::IMMDeviceOpenPropertyStore => {
                self.for_which_immdevice_open_property_store_fails_with(error)
            }
            Function::PropertyStoreGetValue => {
                self.for_which_property_store_get_value_fails_with(error)
            }
            Function::CoCreateIPolicyConfigVista => {
                self.for_which_co_create_ipolicy_config_vista_fails_with(error)
            }
            Function::IPolicyConfigVistaSetDefaultEndpoint => {
                self.for_which_ipolicy_config_vista_set_default_endpoint_fails_with(error)
            }
        }
    }

    pub fn for_which_co_initialize_ex_fails_with(&mut self, error: WIN32_ERROR) -> &mut Self {
        self.behaviour.co_initialize_ex_error = Some(error);

        self
    }

    pub fn for_which_co_create_immdevice_enumerator_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.co_create_immdevice_enumerator_error = Some(error);

        self
    }

    pub fn for_which_immdevice_enumerator_get_default_audio_endpoint_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour
            .immdevice_enumerator_get_default_audio_endpoint_error = Some(error);

        self
    }

    pub fn for_which_immdevice_enumerator_enum_audio_endpoints_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour
            .immdevice_enumerator_enum_audio_endpoints_error = Some(error);

        self
    }

    pub fn for_which_immdevice_get_id_fails_with(&mut self, error: WIN32_ERROR) -> &mut Self {
        self.behaviour.immdevice_get_id_error = Some(error);

        self
    }

    pub fn for_which_immdevice_collection_get_count_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.immdevice_collection_get_count_error = Some(error);

        self
    }

    pub fn for_which_immdevice_collection_item_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.immdevice_collection_item_error = Some(error);

        self
    }

    pub fn for_which_immdevice_open_property_store_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.immdevice_open_property_store_error = Some(error);

        self
    }

    pub fn for_which_property_store_get_value_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.property_store_get_value_error = Some(error);

        self
    }

    pub fn for_which_co_create_ipolicy_config_vista_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour.co_create_ipolicy_config_vista_error = Some(error);

        self
    }

    pub fn for_which_ipolicy_config_vista_set_default_endpoint_fails_with(
        &mut self,
        error: WIN32_ERROR,
    ) -> &mut Self {
        self.behaviour
            .ipolicy_config_vista_set_default_endpoint_error = Some(error);

        self
    }
}
