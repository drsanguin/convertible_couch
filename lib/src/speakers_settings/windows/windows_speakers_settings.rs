use std::{
    ffi::OsString,
    os::{raw::c_ushort, windows::ffi::OsStringExt},
    ptr::null_mut,
    slice::from_raw_parts_mut,
    usize,
};

use crate::speakers_settings::{SpeakersSettings, SpeakersSettingsResult};

use super::audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary};

pub struct WindowsSoundSettings<TAudioEndpointLibrary: AudioEndpointLibrary> {
    audio_endpoint_library: TAudioEndpointLibrary,
}

impl<TAudioEndpointLibrary: AudioEndpointLibrary> SpeakersSettings<TAudioEndpointLibrary>
    for WindowsSoundSettings<TAudioEndpointLibrary>
{
    fn new(speakers_settings_api: TAudioEndpointLibrary) -> Self {
        Self {
            audio_endpoint_library: speakers_settings_api,
        }
    }

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, String> {
        let speakers_count = self.audio_endpoint_library.get_all_audio_endpoints_count();

        if speakers_count == -1 {
            return Err(String::from("Failed to get the number of speakers"));
        }

        let speakers_count_as_usize = usize::try_from(speakers_count).unwrap();
        let mut speakers = vec![AudioEndpoint::default(); speakers_count_as_usize];

        let get_all_speakers_result = self
            .audio_endpoint_library
            .get_all_audio_endpoints(speakers.as_mut_ptr(), speakers_count);

        if get_all_speakers_result != 0 {
            return Err(String::from("Failed to get the speakers"));
        }

        let mut desktop_speaker_id: *mut u16 = null_mut();
        let mut couch_speaker_id: *mut u16 = null_mut();
        let mut current_speaker_id: *mut u16 = null_mut();

        for speaker in &speakers {
            let name = map_c_ushort_to_string(speaker.name);
            let is_default = speaker.is_default == 1;

            if name == desktop_speaker_name {
                desktop_speaker_id = speaker.id;
            }

            if name == couch_speaker_name {
                couch_speaker_id = speaker.id;
            }

            if is_default {
                current_speaker_id = speaker.id;
            }
        }

        if current_speaker_id.is_null() {
            return Err(format!("Failed to get the current default speaker",));
        }

        if desktop_speaker_id.is_null() {
            let possible_speakers_message_fragment =
                get_possible_speakers_message_fragment(&speakers);
            let error_message = format!("Desktop speaker is invalid, possible values are are {possible_speakers_message_fragment}");

            return Err(error_message);
        }

        if couch_speaker_id.is_null() {
            let possible_speakers_message_fragment =
                get_possible_speakers_message_fragment(&speakers);

            return Err(format!("Couch speaker is invalid, possible values are {possible_speakers_message_fragment}"));
        }

        let is_current_default_speaker_the_desktop_one =
            are_pointers_equals(current_speaker_id, desktop_speaker_id);

        let (new_default_speaker_id, new_default_speaker_name) =
            if is_current_default_speaker_the_desktop_one {
                (couch_speaker_id, couch_speaker_name)
            } else {
                (desktop_speaker_id, desktop_speaker_name)
            };

        let set_speaker_result = self
            .audio_endpoint_library
            .set_default_audio_endpoint(new_default_speaker_id);

        if set_speaker_result != 0 {
            return Err(String::from("Failed to set default speaker"));
        }

        Ok(SpeakersSettingsResult {
            new_default_speaker: new_default_speaker_name.to_string(),
        })
    }
}

pub fn map_c_ushort_to_string(id: *mut c_ushort) -> String {
    let mut len = 0;

    for i in 0..=usize::MAX {
        if unsafe { *id.add(i) } != 0 {
            continue;
        }

        len = i;
        break;
    }

    let slice = unsafe { from_raw_parts_mut(id, len) };

    OsString::from_wide(slice).to_string_lossy().into_owned()
}

fn are_pointers_equals(mut p1: *mut u16, mut p2: *mut u16) -> bool {
    loop {
        let v1 = unsafe { *p1 };
        let v2 = unsafe { *p2 };

        if v1 != v2 {
            return false;
        }

        if v1 == 0 {
            return v2 == 0;
        }

        if v2 == 0 {
            return false;
        }

        p1 = unsafe { p1.add(1) };
        p2 = unsafe { p2.add(1) };
    }
}

fn get_possible_speakers_message_fragment(audio_endpoints: &Vec<AudioEndpoint>) -> String {
    let mut possible_audio_endpoints = audio_endpoints
        .iter()
        .map(|audio_endpoint| map_c_ushort_to_string(audio_endpoint.name))
        .collect::<Vec<String>>();

    possible_audio_endpoints.sort();

    possible_audio_endpoints.join(", ")
}
