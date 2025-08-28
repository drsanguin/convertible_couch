use std::collections::HashSet;

use rand::{rngs::StdRng, Rng};

use crate::testing::fuzzing::{
    computer::ComputerFuzzer,
    speakers::{
        settings_api::{
            behaviour::CurrentFuzzedSpeakersSettingsApiBehaviour, CurrentFuzzedSpeakersSettingsApi,
            FuzzedSpeakersSettingsApi,
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
    rand: &'a mut StdRng,
    computer_fuzzer: ComputerFuzzer,
    min_count: usize,
    max_count: usize,
    default_speaker_name: Option<String>,
    alternative_names: HashSet<String>,
    behaviour: CurrentFuzzedSpeakersSettingsApiBehaviour,
}

impl<'a> SpeakersFuzzer<'a> {
    const MAX_SPEAKERS_COUNT: usize = 256;

    pub fn new(rand: &'a mut StdRng, computer_fuzzer: ComputerFuzzer) -> Self {
        Self {
            rand,
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

    pub fn whose_default_one_is_named(&mut self, default_speaker_name: String) -> &mut Self {
        self.default_speaker_name = Some(default_speaker_name);

        self
    }

    pub fn with_an_alternative_one_named(&mut self, alternative_speaker_name: String) -> &mut Self {
        self.alternative_names.insert(alternative_speaker_name);

        self
    }

    pub fn build_speakers(&mut self) -> &mut ComputerFuzzer {
        let mut names_already_taken = HashSet::new();

        if self.default_speaker_name.is_some() {
            let default_speaker_name = self.default_speaker_name.clone().unwrap();
            names_already_taken.insert(default_speaker_name);
        }

        names_already_taken.extend(self.alternative_names.clone());

        let count = self.rand.random_range(self.min_count..=self.max_count);

        let names_not_taken = SpeakerNameFuzzer::new(&mut self.rand)
            .generate_several(count - names_already_taken.len(), &names_already_taken);

        let mut names = Vec::with_capacity(count);
        names.extend(names_already_taken);
        names.extend(names_not_taken);

        let ids = SpeakerIdFuzzer::new(&mut self.rand).generate_several(count);

        let default_speaker_index = if self.default_speaker_name.is_some() {
            let default_speaker_name = self.default_speaker_name.clone().unwrap();

            names
                .iter()
                .position(|name| name == &default_speaker_name)
                .unwrap()
        } else {
            self.rand.random_range(0..count)
        };

        let speakers = (0..count)
            .map(|i| FuzzedSpeaker {
                name: names[i].clone(),
                id: ids[i].clone(),
                is_default: i == default_speaker_index,
            })
            .collect::<Vec<FuzzedSpeaker>>();

        let fuzzed_speakers_settings_api =
            CurrentFuzzedSpeakersSettingsApi::new(speakers, self.behaviour.clone());

        self.computer_fuzzer
            .set_speakers_settings_api(fuzzed_speakers_settings_api)
    }
}

#[cfg(target_os = "windows")]
impl<'a> SpeakersFuzzer<'a> {
    pub fn for_which_getting_the_speakers_count_fails(&mut self) -> &mut Self {
        self.behaviour.getting_the_speakers_count_fails = true;

        self
    }

    pub fn for_which_getting_the_speakers_fails(&mut self) -> &mut Self {
        self.behaviour.getting_the_speakers_fails = true;

        self
    }

    pub fn for_which_getting_the_default_speaker_fails(&mut self) -> &mut Self {
        self.behaviour.getting_the_default_speaker_fails = true;

        self
    }

    pub fn for_which_setting_the_default_speaker_fails(&mut self) -> &mut Self {
        self.behaviour.setting_the_default_speaker_fails = true;

        self
    }
}
