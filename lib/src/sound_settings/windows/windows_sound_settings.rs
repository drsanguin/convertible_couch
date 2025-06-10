use std::{
    ffi::OsString,
    os::{raw::c_ushort, windows::ffi::OsStringExt},
    ptr::null_mut,
    slice::from_raw_parts_mut,
};

use crate::sound_settings::{SoundSettings, SoundSettingsResult};

use super::audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary};

pub struct WindowsSoundSettings<TAudioEndpointLibrary: AudioEndpointLibrary> {
    audio_endpoint_library: TAudioEndpointLibrary,
}

impl<TAudioEndpointLibrary: AudioEndpointLibrary> SoundSettings<TAudioEndpointLibrary>
    for WindowsSoundSettings<TAudioEndpointLibrary>
{
    fn new(sound_settings_api: TAudioEndpointLibrary) -> Self {
        Self {
            audio_endpoint_library: sound_settings_api,
        }
    }

    fn change_default_output_device(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SoundSettingsResult, String> {
        let audio_endpoints_count = self.audio_endpoint_library.get_all_audio_endpoints_count();

        if audio_endpoints_count == -1 {
            return Err(String::from(
                "Failed to get the number of sound output devices",
            ));
        }

        let audio_endpoints_count_as_usize = usize::try_from(audio_endpoints_count).unwrap();
        let mut audio_endpoints = vec![AudioEndpoint::default(); audio_endpoints_count_as_usize];

        let get_all_audio_endpoints_result = self
            .audio_endpoint_library
            .get_all_audio_endpoints(audio_endpoints.as_mut_ptr(), audio_endpoints_count);

        if get_all_audio_endpoints_result != 0 {
            return Err(String::from("Failed to get the sound output devices"));
        }

        let mut desktop_sound_output_device_id: *mut u16 = null_mut();
        let mut couch_sound_output_device_id: *mut u16 = null_mut();
        let mut current_default_output_device_id: *mut u16 = null_mut();

        for audio_endpoint in &audio_endpoints {
            let name = map_c_ushort_to_string(audio_endpoint.name);
            let is_default = audio_endpoint.is_default == 1;

            if name == desktop_speaker_name {
                desktop_sound_output_device_id = audio_endpoint.id;
            }

            if name == couch_speaker_name {
                couch_sound_output_device_id = audio_endpoint.id;
            }

            if is_default {
                current_default_output_device_id = audio_endpoint.id;
            }
        }

        let possible_audio_endpoints = &audio_endpoints
            .iter()
            .map(|audio_endpoint| map_c_ushort_to_string(audio_endpoint.name))
            .collect::<Vec<String>>()
            .join(", ");

        if current_default_output_device_id.is_null() {
            return Err(format!(
                    "Failed to get the default sound output device, possible values are {possible_audio_endpoints}",
                ));
        }

        if desktop_sound_output_device_id.is_null() {
            return Err(format!(
                    "Failed to get the desktop sound output device, possible values are {possible_audio_endpoints}",
                ));
        }

        if couch_sound_output_device_id.is_null() {
            return Err(format!("Failed to get the couch sound output device, possible values are {possible_audio_endpoints}"));
        }

        let is_current_default_output_device_the_desktop_one = are_pointers_equals(
            current_default_output_device_id,
            desktop_sound_output_device_id,
        );

        let (new_default_output_device_id, new_default_output_device_name) =
            if is_current_default_output_device_the_desktop_one {
                (couch_sound_output_device_id, couch_speaker_name)
            } else {
                (desktop_sound_output_device_id, desktop_speaker_name)
            };

        let set_audio_endpoint_result = self
            .audio_endpoint_library
            .set_default_audio_endpoint(new_default_output_device_id);

        if set_audio_endpoint_result != 0 {
            return Err(String::from("Failed to set default audio endpoint"));
        }

        Ok(SoundSettingsResult {
            new_default_output_device: new_default_output_device_name.to_string(),
        })
    }
}

pub fn map_c_ushort_to_string(id: *mut c_ushort) -> String {
    let mut len = 0;

    while unsafe { *id.add(len) } != 0 {
        len += 1;
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

        if v1 == 0 || v2 == 0 {
            return v1 == v2;
        }

        p1 = unsafe { p1.add(1) };
        p2 = unsafe { p2.add(1) };
    }
}
