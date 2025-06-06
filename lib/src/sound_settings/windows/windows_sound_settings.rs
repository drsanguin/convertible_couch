use std::{
    ffi::{OsStr, OsString},
    mem::transmute_copy,
    os::{
        raw::{c_int, c_ushort},
        windows::ffi::{OsStrExt, OsStringExt},
    },
    ptr::null_mut,
    slice::from_raw_parts_mut,
};

use windows::{
    core::{Error, IUnknownImpl, IUnknown_Vtbl},
    Win32::Media::Speech::ISpVoice,
};
use windows::{
    core::{IUnknown, Interface, GUID, HRESULT, PCWSTR},
    Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
    },
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
        // let audio_endpoints_count =
        //     unsafe { self.audio_endpoint_library.get_all_audio_endpoints_count() };

        // if audio_endpoints_count == -1 {
        //     return Err(String::from(
        //         "Failed to get the number of sound output devices",
        //     ));
        // }

        // let audio_endpoints_count_as_usize = usize::try_from(audio_endpoints_count).unwrap();
        // let mut audio_endpoints = vec![AudioEndpoint::default(); audio_endpoints_count_as_usize];

        // if unsafe {
        //     self.audio_endpoint_library
        //         .get_all_audio_endpoints(audio_endpoints.as_mut_ptr(), audio_endpoints_count)
        // } != 0
        // {
        //     return Err(String::from("Failed to get the sound output devices"));
        // }

        // let mut desktop_sound_output_device_id: *mut u16 = null_mut();
        // let mut couch_sound_output_device_id: *mut u16 = null_mut();
        // let mut current_default_output_device_id: *mut u16 = null_mut();

        // for audio_endpoint in &audio_endpoints {
        //     let name = to_string(audio_endpoint.name);
        //     let is_default = audio_endpoint.is_default == 1;

        //     if name == desktop_speaker_name {
        //         desktop_sound_output_device_id = audio_endpoint.id;
        //     }

        //     if name == couch_speaker_name {
        //         couch_sound_output_device_id = audio_endpoint.id;
        //     }

        //     if is_default {
        //         current_default_output_device_id = audio_endpoint.id;
        //     }
        // }

        // let possible_audio_endpoints = &audio_endpoints
        //     .iter()
        //     .map(|audio_endpoint| to_string(audio_endpoint.name))
        //     .collect::<Vec<String>>()
        //     .join(", ");

        // if current_default_output_device_id.is_null() {
        //     return Err(format!(
        //             "Failed to get the default sound output device, possible values are {possible_audio_endpoints}",
        //         ));
        // }

        // if desktop_sound_output_device_id.is_null() {
        //     return Err(format!(
        //             "Failed to get the desktop sound output device, possible values are {possible_audio_endpoints}",
        //         ));
        // }

        // if couch_sound_output_device_id.is_null() {
        //     return Err(format!("Failed to get the couch sound output device, possible values are {possible_audio_endpoints}"));
        // }

        // let (new_default_output_device_id, new_default_output_device_name) = if eq(
        //     current_default_output_device_id,
        //     desktop_sound_output_device_id,
        // ) {
        //     (couch_sound_output_device_id, couch_speaker_name)
        // } else {
        //     (desktop_sound_output_device_id, desktop_speaker_name)
        // };

        // if unsafe {
        //     self.audio_endpoint_library
        //         .set_default_audio_endpoint(new_default_output_device_id)
        // } != 0
        // {
        //     return Err(String::from("Failed to set default audio endpoint"));
        // }

        // Ok(SoundSettingsResult {
        //     new_default_output_device: new_default_output_device_name.to_string(),
        // })

        unsafe {
            let co_initialize_ex_result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            if co_initialize_ex_result.is_err() {
                panic!("CoInitializeEx failed");
            }

            let co_create_instance_result: Result<IPolicyConfig, windows::core::Error> =
                CoCreateInstance(
                    &GUID::from_u128(0x870AF99C_171D_4F9E_AF0D_E63DF40C2BC9),
                    None,
                    CLSCTX_ALL,
                );

            if co_create_instance_result.is_err() {
                panic!("CoCreateInstance failed");
            }

            let instance = co_create_instance_result.unwrap();

            let (pcwstr, buffer) =
                get_pcwstr("{0.0.0.00000000}.{74142d58-9711-41bc-a9fe-c86b07aca84c}".to_string());
            let set_default_endpoint_result = instance.SetDefaultEndpoint(pcwstr, 0);

            if set_default_endpoint_result.is_err() {
                panic!(
                    "SetDefaultEndpoint failed : {0}",
                    set_default_endpoint_result.err().unwrap()
                );
            }

            CoUninitialize();
        }

        Err(format!("Something wrong happened"))
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone)]
pub struct IPolicyConfig(IUnknown);

unsafe impl Interface for IPolicyConfig {
    type Vtable = IPolicyConfig_Vtbl;
    const IID: GUID = GUID::from_u128(0xF8679F50_850A_41CF_9C72_430F290290C8);
}

impl IPolicyConfig {
    pub unsafe fn SetDefaultEndpoint(
        &self,
        pszDeviceName: PCWSTR,
        role: c_int,
    ) -> Result<(), Error> {
        unsafe {
            (Interface::vtable(self).SetDefaultEndpoint)(
                Interface::as_raw(self),
                pszDeviceName,
                role,
            )
            .ok()
        }
    }
}

#[repr(C)]
pub struct IPolicyConfig_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub SetDefaultEndpoint:
        unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR, c_int) -> HRESULT,
}

pub trait IPolicyConfig_Impl: IUnknownImpl {
    fn SetDefaultEndpoint(&self, pszDeviceName: PCWSTR, role: c_int) -> Result<(), Error>;
}

impl IPolicyConfig_Vtbl {
    pub const fn new<Identity: IPolicyConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDefaultEndpoint<
            Identity: IPolicyConfig_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pszDeviceName: PCWSTR,
            role: c_int,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);

                IPolicyConfig_Impl::SetDefaultEndpoint(
                    this,
                    transmute_copy(&pszDeviceName),
                    transmute_copy(&role),
                )
                .into()
            }
        }

        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultEndpoint: SetDefaultEndpoint::<Identity, OFFSET>,
        }
    }
}

// #[repr(C)]
// pub struct IPolicyConfigVista_Vtbl {
//     pub base__: IUnknown,
//     pub SetDefaultEndpoint: unsafe extern "system" fn(
//         this: *mut core::ffi::c_void,
//         device_id: PCWSTR,
//         role: c_int,
//     ) -> HRESULT,
// }

// #[repr(transparent)]
// #[derive(Clone)]
// pub struct IPolicyConfigVista {
//     vtbl: *const IPolicyConfigVista_Vtbl,
// }

// unsafe impl Interface for IPolicyConfigVista {
//     type Vtable = IPolicyConfigVista_Vtbl;

//     const IID: GUID = GUID::from_u128(0xF8679F50_850A_41CF_9C72_430F290290C8);
// }

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

pub fn string_to_pcwstr(input: String) -> Vec<u16> {
    // Convert the String to a wide string (Vec<u16>) and null-terminate it
    let wide: Vec<u16> = OsStr::new(&input)
        .encode_wide()
        .chain(std::iter::once(0)) // null terminator
        .collect();

    wide
}

/// Returns a PCWSTR from a String. You must keep the returned Vec<u16> alive.
pub fn get_pcwstr(input: String) -> (PCWSTR, Vec<u16>) {
    let wide = string_to_pcwstr(input);
    let pcwstr = PCWSTR(wide.as_ptr());
    (pcwstr, wide) // Return both so the buffer lives long enough
}
