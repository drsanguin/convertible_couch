use crate::{
    speakers_settings::windows::{
        audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary},
        windows_speakers_settings::map_c_ushort_to_string,
    },
    testing::fuzzing::speakers_settings::speakers::FuzzedSpeaker,
};

use std::{
    ffi::OsStr,
    iter::once,
    mem::forget,
    os::{
        raw::{c_int, c_ushort},
        windows::ffi::OsStrExt,
    },
};

#[derive(Clone)]
pub struct FuzzedAudioEndpointLibrary {
    speakers: Vec<FuzzedSpeaker>,
    getting_the_speakers_count_fails: bool,
    getting_the_speakers_fails: bool,
    getting_the_default_speaker_fails: bool,
    setting_the_default_speaker_fails: bool,
}

impl FuzzedAudioEndpointLibrary {
    pub fn default() -> Self {
        Self {
            speakers: vec![],
            getting_the_speakers_count_fails: false,
            getting_the_speakers_fails: false,
            getting_the_default_speaker_fails: false,
            setting_the_default_speaker_fails: false,
        }
    }

    pub fn new(
        speakers: Vec<FuzzedSpeaker>,
        getting_the_speakers_count_fails: bool,
        getting_the_speakers_fails: bool,
        getting_the_default_speaker_fails: bool,
        setting_the_default_speaker_fails: bool,
    ) -> Self {
        Self {
            speakers,
            getting_the_speakers_count_fails,
            getting_the_speakers_fails,
            getting_the_default_speaker_fails,
            setting_the_default_speaker_fails,
        }
    }
}

impl AudioEndpointLibrary for FuzzedAudioEndpointLibrary {
    fn get_all_audio_endpoints_count(&self) -> c_int {
        match self.getting_the_speakers_count_fails {
            true => -1,
            false => self.speakers.len().try_into().unwrap_or(-1),
        }
    }

    fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int {
        if self.getting_the_speakers_fails {
            return -1;
        }

        let speakers_count_as_usize = usize::try_from(audio_endpoints_count);

        if speakers_count_as_usize.is_err() {
            return -1;
        }

        for i in 0..speakers_count_as_usize.unwrap() {
            let out_audio_endpoint = unsafe { &mut *out_audio_endpoints.add(i) };
            let speaker = &self.speakers[i];

            out_audio_endpoint.id = map_string_to_c_ushort(&speaker.id);
            out_audio_endpoint.name = map_string_to_c_ushort(&speaker.name);
            out_audio_endpoint.is_default = if self.getting_the_default_speaker_fails {
                0
            } else if speaker.is_default {
                1
            } else {
                0
            };
        }

        0
    }

    fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int {
        if self.setting_the_default_speaker_fails {
            return -1;
        }

        let id_as_string = map_c_ushort_to_string(id);

        let speaker_exists = self.speakers.iter().any(|x| x.id == id_as_string);

        if !speaker_exists {
            return -1;
        }

        for speaker in self.speakers.iter_mut() {
            if speaker.is_default {
                speaker.is_default = false;
            }

            if speaker.id == id_as_string {
                speaker.is_default = true;
            }
        }

        0
    }
}

pub fn map_string_to_c_ushort(string: &str) -> *mut c_ushort {
    let wide: Vec<c_ushort> = OsStr::new(string)
        .encode_wide()
        .chain(once(0)) // Null terminator
        .collect();

    // Allocate memory and copy the data into it
    let mut boxed_slice = wide.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();

    // Leak the boxed slice so it isn't deallocated when it goes out of scope
    forget(boxed_slice);

    ptr
}
