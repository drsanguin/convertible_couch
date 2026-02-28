use std::marker::PhantomData;

use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Media::Audio::{eConsole, eRender, EDataFlow, DEVICE_STATE_ACTIVE},
    System::Com::{COINIT_MULTITHREADED, STGM_READ},
};
use windows_core::{PCWSTR, PWSTR};

use crate::{
    speakers_settings::{
        windows::windows_com::{
            IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, IPolicyConfigVista,
            IPropertyStore, WindowsCom,
        },
        SpeakerInfo, SpeakersSettings, SpeakersSettingsResult,
    },
    ApplicationError,
};

pub struct WindowsSoundSettings<
    TWindowsCom: WindowsCom<
        TIMMDeviceEnumerator,
        TIMMDevice,
        TIMMDeviceCollection,
        TIPropertyStore,
        TIPolicyConfigVista,
    >,
    TIMMDeviceEnumerator: IMMDeviceEnumerator<TIMMDevice, TIMMDeviceCollection, TIPropertyStore>,
    TIMMDevice: IMMDevice<TIPropertyStore>,
    TIMMDeviceCollection: IMMDeviceCollection<TIMMDevice, TIPropertyStore>,
    TIPropertyStore: IPropertyStore,
    TIPolicyConfigVista: IPolicyConfigVista,
> {
    windows_com: TWindowsCom,
    immdevice_enumerator: PhantomData<TIMMDeviceEnumerator>,
    immdevice: PhantomData<TIMMDevice>,
    immdevice_collection: PhantomData<TIMMDeviceCollection>,
    iproperty_store: PhantomData<TIPropertyStore>,
    ipolicy_config_vista: PhantomData<TIPolicyConfigVista>,
}

// impl<TWindowsCom: WindowsCom> WindowsSoundSettings<TWindowsCom> {}

impl<
        TWindowsCom: WindowsCom<
            TIMMDeviceEnumerator,
            TIMMDevice,
            TIMMDeviceCollection,
            TIPropertyStore,
            TIPolicyConfigVista,
        >,
        TIMMDeviceEnumerator: IMMDeviceEnumerator<TIMMDevice, TIMMDeviceCollection, TIPropertyStore>,
        TIMMDevice: IMMDevice<TIPropertyStore>,
        TIMMDeviceCollection: IMMDeviceCollection<TIMMDevice, TIPropertyStore>,
        TIPropertyStore: IPropertyStore,
        TIPolicyConfigVista: IPolicyConfigVista,
    > SpeakersSettings<TWindowsCom>
    for WindowsSoundSettings<
        TWindowsCom,
        TIMMDeviceEnumerator,
        TIMMDevice,
        TIMMDeviceCollection,
        TIPropertyStore,
        TIPolicyConfigVista,
    >
{
    fn new(speakers_settings_api: TWindowsCom) -> Self {
        Self {
            windows_com: speakers_settings_api,
            immdevice_enumerator: PhantomData,
            immdevice: PhantomData,
            immdevice_collection: PhantomData,
            iproperty_store: PhantomData,
            ipolicy_config_vista: PhantomData,
        }
    }

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<SpeakersSettingsResult, ApplicationError> {
        let co_initialize_ex_result = unsafe {
            self.windows_com
                .co_initialize_ex(None, COINIT_MULTITHREADED)
        };

        if co_initialize_ex_result.is_err() {
            return Err(ApplicationError::Custom(
                "co_initialize_ex failed".to_string(),
            ));
        }

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
                let error_message = format!("{invalid_params_error_message_fragment}, possible values are [{possible_values_fragment}]");
                let error = ApplicationError::Custom(error_message);

                return Err(error);
            }

            let new_default_speaker_id: PWSTR;

            if unsafe { pwstr_eq(default_speaker_id, desktop_speaker_id) } {
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

    fn get_speakers_infos(&self) -> Result<Vec<SpeakerInfo>, ApplicationError> {
        let co_initialize_ex_result = unsafe {
            self.windows_com
                .co_initialize_ex(None, COINIT_MULTITHREADED)
        };

        if co_initialize_ex_result.is_err() {
            return Err(ApplicationError::Custom(
                "co_initialize_ex failed".to_string(),
            ));
        }

        let mut speakers_infos: Vec<SpeakerInfo>;

        {
            let immdevice_enumerator =
                unsafe { self.windows_com.co_create_immdevice_enumerator() }?;

            let get_default_audio_endpoint_result =
                unsafe { immdevice_enumerator.get_default_audio_endpoint(eRender, eConsole) };

            let mut default_speaker_id_option: Option<PWSTR> = None;

            if let Ok(default_speaker) = get_default_audio_endpoint_result {
                default_speaker_id_option = Some(unsafe { default_speaker.get_id() }?);
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
                    unsafe { pwstr_eq(default_speaker_id, immdevice_id) }
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

unsafe fn pwstr_eq(a: PWSTR, b: PWSTR) -> bool {
    let mut pa = a.0;
    let mut pb = b.0;

    if pa.is_null() || pb.is_null() {
        return pa == pb;
    }

    loop {
        let ca = *pa;
        let cb = *pb;

        if ca != cb {
            return false;
        }

        if ca == 0 {
            // both null-terminated
            return true;
        }

        pa = pa.add(1);
        pb = pb.add(1);
    }
}
