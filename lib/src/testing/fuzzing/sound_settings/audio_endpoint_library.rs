use crate::{
    sound_settings::windows::{
        audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary},
        windows_sound_settings::map_c_ushort_to_string,
    },
    testing::fuzzing::sound_settings::audio_output_device::FuzzedAudioOutputDevice,
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
    audio_output_devices: Vec<FuzzedAudioOutputDevice>,
    getting_the_audio_outputs_count_fails: bool,
    getting_the_audio_outputs_fails: bool,
    getting_the_default_audio_output_fails: bool,
    setting_the_default_audio_output_fails: bool,
}

impl FuzzedAudioEndpointLibrary {
    pub fn default() -> Self {
        Self {
            audio_output_devices: vec![],
            getting_the_audio_outputs_count_fails: false,
            getting_the_audio_outputs_fails: false,
            getting_the_default_audio_output_fails: false,
            setting_the_default_audio_output_fails: false,
        }
    }

    pub fn new(
        audio_output_devices: Vec<FuzzedAudioOutputDevice>,
        getting_the_audio_outputs_count_fails: bool,
        getting_the_audio_outputs_fails: bool,
        getting_the_default_audio_output_fails: bool,
        setting_the_default_audio_output_fails: bool,
    ) -> Self {
        Self {
            audio_output_devices,
            getting_the_audio_outputs_count_fails,
            getting_the_audio_outputs_fails,
            getting_the_default_audio_output_fails,
            setting_the_default_audio_output_fails,
        }
    }
}

impl AudioEndpointLibrary for FuzzedAudioEndpointLibrary {
    fn get_all_audio_endpoints_count(&self) -> c_int {
        match self.getting_the_audio_outputs_count_fails {
            true => -1,
            false => self.audio_output_devices.len().try_into().unwrap_or(-1),
        }
    }

    fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int {
        if self.getting_the_audio_outputs_fails {
            return -1;
        }

        let audio_endpoints_count_as_usize = usize::try_from(audio_endpoints_count);

        if audio_endpoints_count_as_usize.is_err() {
            return -1;
        }

        for i in 0..audio_endpoints_count_as_usize.unwrap() {
            let out_audio_endpoint = unsafe { &mut *out_audio_endpoints.add(i) };
            let audio_output_device = &self.audio_output_devices[i];

            out_audio_endpoint.id = map_string_to_c_ushort(&audio_output_device.id);
            out_audio_endpoint.name = map_string_to_c_ushort(&audio_output_device.name);
            out_audio_endpoint.is_default = if self.getting_the_default_audio_output_fails {
                0
            } else if audio_output_device.is_default {
                1
            } else {
                0
            };
        }

        0
    }

    fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int {
        if self.setting_the_default_audio_output_fails {
            return -1;
        }

        let id_as_string = map_c_ushort_to_string(id);

        let audio_endpoint_exists = self
            .audio_output_devices
            .iter()
            .any(|x| x.id == id_as_string);

        if !audio_endpoint_exists {
            return -1;
        }

        for audio_endpoint in self.audio_output_devices.iter_mut() {
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
