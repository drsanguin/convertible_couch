use crate::{
    sound_settings::windows::{
        audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary},
        windows_sound_settings::to_string,
    },
    testing::fuzzing::sound_settings::audio_endpoint::FuzzedAudioEndpoint,
};

use std::{
    ffi::OsStr,
    iter::once,
    os::{
        raw::{c_int, c_ushort},
        windows::ffi::OsStrExt,
    },
};

pub struct FuzzedAudioEndpointLibrary {
    audio_endpoints: Vec<FuzzedAudioEndpoint>,
}

impl FuzzedAudioEndpointLibrary {
    pub fn new(audio_endpoints: Vec<FuzzedAudioEndpoint>) -> Self {
        Self { audio_endpoints }
    }
}

impl AudioEndpointLibrary for FuzzedAudioEndpointLibrary {
    fn get_all_audio_endpoints_count(&self) -> c_int {
        self.audio_endpoints.len().try_into().unwrap_or(-1)
    }

    fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int {
        let audio_endpoints_count_as_usize = usize::try_from(audio_endpoints_count);

        if audio_endpoints_count_as_usize.is_err() {
            return -1;
        }

        for i in 0..audio_endpoints_count_as_usize.unwrap() {
            let out_audio_endpoint = unsafe { &mut *out_audio_endpoints.add(i) };
            let audio_endpoint = &self.audio_endpoints[i];

            out_audio_endpoint.id = string_to_c_ushort(&audio_endpoint.id);
            out_audio_endpoint.name = string_to_c_ushort(&audio_endpoint.name);
            out_audio_endpoint.is_default = if audio_endpoint.is_default { 1 } else { 0 };
        }

        0
    }

    fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int {
        let id_as_string = to_string(id);

        let audio_endpoint = self.audio_endpoints.iter().any(|x| x.id == id_as_string);

        if !audio_endpoint {
            return -1;
        }

        for audio_endpoint in self.audio_endpoints.iter_mut() {
            if audio_endpoint.is_default {
                audio_endpoint.is_default = false;
            }

            if audio_endpoint.id == id_as_string {
                audio_endpoint.is_default = true;
            }
        }

        0
    }
}

pub fn string_to_c_ushort(s: &str) -> *mut c_ushort {
    let wide: Vec<c_ushort> = OsStr::new(s)
        .encode_wide()
        .chain(once(0)) // Null terminator
        .collect();

    // Allocate memory and copy the data into it
    let mut boxed_slice = wide.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();

    // Leak the boxed slice so it isn't deallocated when it goes out of scope
    std::mem::forget(boxed_slice);

    ptr
}
