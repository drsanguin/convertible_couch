use std::{
    ffi::OsString,
    os::{raw::c_ushort, windows::ffi::OsStringExt},
    ptr::null_mut,
    slice::from_raw_parts_mut,
};

use audio_endpoint_library::{AudioEndpoint, AudioEndpointLibrary};

use super::{SoundSettings, SwapDefaultOutputDeviceResponse};

pub mod audio_endpoint_library;

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

    fn swap_default_output_device(
        &mut self,
        desktop_sound_output_device_name: &str,
        couch_sound_output_device_name: &str,
    ) -> Result<SwapDefaultOutputDeviceResponse, String> {
        let audio_endpoints_count =
            unsafe { self.audio_endpoint_library.get_all_audio_endpoints_count() };

        if audio_endpoints_count == -1 {
            return Err(String::from(
                "Failed to get the number of sound output devices",
            ));
        }

        let audio_endpoints_count_as_usize = usize::try_from(audio_endpoints_count).unwrap();
        let mut audio_endpoints = vec![AudioEndpoint::default(); audio_endpoints_count_as_usize];

        if unsafe {
            self.audio_endpoint_library
                .get_all_audio_endpoints(audio_endpoints.as_mut_ptr(), audio_endpoints_count)
        } != 0
        {
            return Err(String::from("Failed to get the sound output devices"));
        }

        let mut desktop_sound_output_device_id: *mut u16 = null_mut();
        let mut couch_sound_output_device_id: *mut u16 = null_mut();
        let mut current_default_output_device_id: *mut u16 = null_mut();

        for audio_endpoint in &audio_endpoints {
            let name = to_string(audio_endpoint.name);
            let is_default = audio_endpoint.is_default == 1;

            if name == desktop_sound_output_device_name {
                desktop_sound_output_device_id = audio_endpoint.id;
            }

            if name == couch_sound_output_device_name {
                couch_sound_output_device_id = audio_endpoint.id;
            }

            if is_default {
                current_default_output_device_id = audio_endpoint.id;
            }
        }

        let possible_audio_endpoints = &audio_endpoints
            .iter()
            .map(|audio_endpoint| to_string(audio_endpoint.name))
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

        let (new_default_output_device_id, new_default_output_device_name) = if eq(
            current_default_output_device_id,
            desktop_sound_output_device_id,
        ) {
            (couch_sound_output_device_id, couch_sound_output_device_name)
        } else {
            (
                desktop_sound_output_device_id,
                desktop_sound_output_device_name,
            )
        };

        if unsafe {
            self.audio_endpoint_library
                .set_default_audio_endpoint(new_default_output_device_id)
        } != 0
        {
            return Err(String::from("Failed to set default audio endpoint"));
        }

        Ok(SwapDefaultOutputDeviceResponse {
            new_default_output_device: new_default_output_device_name.to_string(),
        })
    }
}

fn to_string(id: *mut c_ushort) -> String {
    let mut len = 0;

    while unsafe { *id.add(len) } != 0 {
        len += 1;
    }

    let slice = unsafe { from_raw_parts_mut(id, len) };

    OsString::from_wide(slice).to_string_lossy().into_owned()
}

fn eq(mut a: *mut u16, mut b: *mut u16) -> bool {
    loop {
        let va = unsafe { *a };
        let vb = unsafe { *b };

        if va != vb {
            return false;
        }

        if va == 0 && vb == 0 {
            return true;
        }

        a = unsafe { a.add(1) };
        b = unsafe { b.add(1) };
    }
}
