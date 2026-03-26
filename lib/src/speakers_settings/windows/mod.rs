use log::info;
use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Media::Audio::{DEVICE_STATE_ACTIVE, EDataFlow, eConsole, eRender},
    System::Com::{COINIT_MULTITHREADED, STGM_READ},
};
use windows_core::{PCWSTR, PWSTR};

use crate::{
    application_error::ApplicationError,
    speakers_settings::{
        SpeakerInfo, SpeakersSettings, SpeakersSettingsResult, windows::windows_com::WindowsCom,
    },
    trace_fn,
};

pub mod windows_com;

pub struct WindowsSoundSettings {
    windows_com: Box<dyn WindowsCom>,
}

impl SpeakersSettings for WindowsSoundSettings {
    fn new(speakers_settings_api: Box<dyn WindowsCom>) -> Self {
        trace_fn!();

        Self {
            windows_com: speakers_settings_api,
        }
    }

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, ApplicationError> {
        trace_fn!();
        info!("Changing default speaker");

        (unsafe {
            self.windows_com
                .co_initialize_ex(None, COINIT_MULTITHREADED)
                .ok()
        })?;

        let new_default_speaker_name: String;

        {
            let immdevice_enumerator =
                unsafe { self.windows_com.co_create_immdevice_enumerator() }?;

            let default_speaker =
                unsafe { immdevice_enumerator.get_default_audio_endpoint(eRender, eConsole) }?;

            let default_speaker_id = unsafe { default_speaker.get_id() }?;

            let immdevice_collection =
                unsafe { immdevice_enumerator.enum_audio_endpoints(eRender, DEVICE_STATE_ACTIVE) }?;

            let speaker_count = unsafe { immdevice_collection.get_count() }?;

            let mut desktop_speaker_id: PWSTR = PWSTR::default();
            let mut couch_speaker_id: PWSTR = PWSTR::default();
            let mut speaker_names = Vec::with_capacity(speaker_count as usize);

            for speaker_index in 0..speaker_count {
                let immdevice = unsafe { immdevice_collection.item(speaker_index) }?;
                let immdevice_id = unsafe { immdevice.get_id() }?;
                let property_store = unsafe { immdevice.open_property_store(STGM_READ) }?;
                let propvariant = unsafe { property_store.get_value(&PKEY_Device_FriendlyName) }?;
                let pwsz_val = unsafe { propvariant.Anonymous.Anonymous.Anonymous.pwszVal };
                let friendly_name = String::from_utf16(unsafe { pwsz_val.as_wide() })?;

                if friendly_name == desktop_speaker_name {
                    desktop_speaker_id = immdevice_id;
                } else if friendly_name == couch_speaker_name {
                    couch_speaker_id = immdevice_id;
                }

                speaker_names.push(friendly_name);
            }

            speaker_names.sort();

            let invalid_params_error_message =
                match (desktop_speaker_id.is_null(), couch_speaker_id.is_null()) {
                    (true, true) => Some("Desktop and couch speakers are invalid"),
                    (true, _) => Some("Desktop speaker is invalid"),
                    (_, true) => Some("Couch speaker is invalid"),
                    _ => None,
                };

            if let Some(invalid_params_error_message_fragment) = invalid_params_error_message {
                let possible_values_fragment = speaker_names.join(", ");
                let error_message = format!(
                    "{invalid_params_error_message_fragment}, possible values are [{possible_values_fragment}]"
                );
                let error = ApplicationError::Custom(error_message);

                return Err(error);
            }

            let new_default_speaker_id: PWSTR;

            if pwstr_eq(default_speaker_id, desktop_speaker_id) {
                new_default_speaker_id = couch_speaker_id;
                new_default_speaker_name = couch_speaker_name.to_string();
            } else {
                new_default_speaker_id = desktop_speaker_id;
                new_default_speaker_name = desktop_speaker_name.to_string();
            }

            let mut policy = unsafe { self.windows_com.co_create_ipolicy_config_vista() }?;

            (unsafe {
                policy
                    .set_default_endpoint(PCWSTR(new_default_speaker_id.0 as *const u16), eConsole)
            })?;
        }

        unsafe { self.windows_com.co_uninitialize() };

        Ok(SpeakersSettingsResult {
            new_default_speaker: new_default_speaker_name,
        })
    }

    fn get_speakers_infos(&mut self) -> Result<Vec<SpeakerInfo>, ApplicationError> {
        trace_fn!();
        info!("Getting speakers informations");

        (unsafe {
            self.windows_com
                .co_initialize_ex(None, COINIT_MULTITHREADED)
                .ok()
        })?;

        let mut speakers_infos: Vec<SpeakerInfo>;

        {
            let immdevice_enumerator =
                unsafe { self.windows_com.co_create_immdevice_enumerator() }?;

            let get_default_audio_endpoint_result =
                unsafe { immdevice_enumerator.get_default_audio_endpoint(eRender, eConsole) };

            let mut default_speaker_id_option: Option<PWSTR> = None;

            if let Ok(default_speaker) = get_default_audio_endpoint_result {
                let default_speaker_id = unsafe { default_speaker.get_id() }?;

                default_speaker_id_option = Some(default_speaker_id);
            }

            let immdevice_collection = unsafe {
                immdevice_enumerator.enum_audio_endpoints(EDataFlow::default(), DEVICE_STATE_ACTIVE)
            }?;

            let speaker_count = unsafe { immdevice_collection.get_count() }?;

            speakers_infos = Vec::with_capacity(speaker_count.try_into().unwrap());

            for speaker_index in 0..speaker_count {
                let immdevice = unsafe { immdevice_collection.item(speaker_index) }?;
                let immdevice_id = unsafe { immdevice.get_id() }?;
                let property_store = unsafe { immdevice.open_property_store(STGM_READ) }?;
                let propvariant = unsafe { property_store.get_value(&PKEY_Device_FriendlyName) }?;
                let pwsz_val = unsafe { propvariant.Anonymous.Anonymous.Anonymous.pwszVal };
                let friendly_name = String::from_utf16(unsafe { pwsz_val.as_wide() })?;

                let is_default = if let Some(default_speaker_id) = default_speaker_id_option {
                    pwstr_eq(default_speaker_id, immdevice_id)
                } else {
                    false
                };

                speakers_infos.push(SpeakerInfo {
                    is_default,
                    name: friendly_name,
                });
            }
        }

        speakers_infos.sort();

        unsafe { self.windows_com.co_uninitialize() };

        Ok(speakers_infos)
    }
}

fn pwstr_eq(a: PWSTR, b: PWSTR) -> bool {
    trace_fn!();
    
    let mut pa = a.0;
    let mut pb = b.0;

    if pa.is_null() || pb.is_null() {
        return pa == pb;
    }

    loop {
        let ca = unsafe { *pa };
        let cb = unsafe { *pb };

        if ca != cb {
            return false;
        }

        if ca == 0 {
            return true;
        }

        pa = unsafe { pa.add(1) };
        pb = unsafe { pb.add(1) };
    }
}

#[cfg(test)]
mod should {
    use std::ptr::null_mut;

    use windows_core::PWSTR;

    use test_case::test_case;

    use crate::speakers_settings::windows::pwstr_eq;

    #[test_case(None, None => true; "when both pointers are null")]
    #[test_case(None, Some("") => false; "when first pointer is null")]
    #[test_case(Some(""), None => false; "when second pointer is null")]
    fn check_equality_of_two_pwstr(a_content: Option<&str>, b_content: Option<&str>) -> bool {
        // Arrange
        let mut a_str_buffer = Vec::new();
        let a = a_content
            .map(|x| {
                a_str_buffer = x.encode_utf16().collect::<Vec<u16>>();

                PWSTR::from_raw(a_str_buffer.as_mut_ptr())
            })
            .unwrap_or(PWSTR::from_raw(null_mut()));

        let mut b_str_buffer = Vec::new();
        let b = b_content
            .map(|x| {
                b_str_buffer = x.encode_utf16().collect::<Vec<u16>>();

                PWSTR::from_raw(b_str_buffer.as_mut_ptr())
            })
            .unwrap_or(PWSTR::from_raw(null_mut()));

        // Act
        pwstr_eq(a, b)
    }
}
