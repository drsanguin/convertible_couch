use std::ffi::c_void;

use windows::{
    core::{define_interface, interface_hierarchy},
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Foundation::PROPERTYKEY,
        Media::Audio::{
            eConsole, eRender, EDataFlow, IMMDeviceEnumerator, MMDeviceEnumerator,
            DEVICE_STATE_ACTIVE, WAVEFORMATEX,
        },
        System::Com::{
            StructuredStorage::PROPVARIANT, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
        },
    },
};
use windows_core::{IUnknown_Vtbl, GUID, HRESULT, PCWSTR, PWSTR};

use crate::speakers_settings::{windows::windows_com::WindowsCom, SpeakersSettings};

pub struct WindowsSoundSettings2<TWindowsCom: WindowsCom> {
    windows_com: TWindowsCom,
}

impl<TWindowsCom: WindowsCom> WindowsSoundSettings2<TWindowsCom> {}

impl<TWindowsCom: WindowsCom> SpeakersSettings<TWindowsCom> for WindowsSoundSettings2<TWindowsCom> {
    fn new(speakers_settings_api: TWindowsCom) -> Self {
        Self {
            windows_com: speakers_settings_api,
        }
    }

    fn change_default_speaker(
        &mut self,
        desktop_speaker_name: &str,
        couch_speaker_name: &str,
    ) -> Result<crate::speakers_settings::SpeakersSettingsResult, crate::ApplicationError> {
        let co_initialize_ex_result = unsafe {
            self.windows_com
                .co_initialize_ex(None, COINIT_MULTITHREADED)
        };

        if co_initialize_ex_result.is_err() {
            panic!("co_initialize_ex failed")
        }

        {
            let immdevice_enumerator: IMMDeviceEnumerator = unsafe {
                self.windows_com
                    .co_create_instance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            }?;

            let default_speaker =
                unsafe { immdevice_enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }?;

            let default_speaker_id = unsafe { default_speaker.GetId() }?;

            let immdevice_collection = unsafe {
                immdevice_enumerator.EnumAudioEndpoints(EDataFlow::default(), DEVICE_STATE_ACTIVE)
            }?;

            let speaker_count = unsafe { immdevice_collection.GetCount() }?;

            let mut desktop_speaker_id: PWSTR = PWSTR::default();
            let mut couch_speaker_id: PWSTR = PWSTR::default();
            let mut current_speaker_id: PWSTR = PWSTR::default();

            for speaker_index in 0..speaker_count {
                let immdevice = unsafe { immdevice_collection.Item(speaker_index) }?;
                let immdevice_id = unsafe { immdevice.GetId() }?;
                let property_store = unsafe { immdevice.OpenPropertyStore(STGM_READ) }?;
                let propvariant = unsafe { property_store.GetValue(&PKEY_Device_FriendlyName) }?;
                let pwsz_val = unsafe { propvariant.Anonymous.Anonymous.Anonymous.pwszVal };
                let friendly_name = String::from_utf16(unsafe { pwsz_val.as_wide() })?;

                if friendly_name == desktop_speaker_name {
                    desktop_speaker_id = immdevice_id;
                } else if friendly_name == couch_speaker_name {
                    couch_speaker_id = immdevice_id;
                }
            }

            let new_default_speaker_id =
                if unsafe { pwstr_eq(default_speaker_id, desktop_speaker_id) } {
                    couch_speaker_id
                } else {
                    desktop_speaker_id
                };

            let policy: IPolicyConfigVista = unsafe {
                self.windows_com.co_create_instance(
                    &GUID::from_u128(0x294935ce_f637_4e7c_a41b_ab255460b862),
                    None,
                    CLSCTX_ALL,
                )
            }?;

            (unsafe {
                policy
                    .SetDefaultEndpoint(PCWSTR(new_default_speaker_id.0 as *const u16), eConsole.0)
            })?;
        }

        unsafe { self.windows_com.co_uninitialize() };

        Ok(crate::speakers_settings::SpeakersSettingsResult {
            new_default_speaker: "".to_string(),
        })
    }

    fn get_speakers_infos(
        &self,
    ) -> Result<Vec<crate::speakers_settings::SpeakerInfo>, crate::ApplicationError> {
        todo!()
    }
}

define_interface!(
    IPolicyConfigVista,
    IPolicyConfigVista_Vtbl,
    0x568b9108_44bf_40b4_9006_86afe5b5a620
);
interface_hierarchy!(IPolicyConfigVista, windows_core::IUnknown);

impl IPolicyConfigVista {
    pub unsafe fn SetDefaultEndpoint(
        &self,
        device_id: PCWSTR,
        role: i32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultEndpoint)(
                windows_core::Interface::as_raw(self),
                device_id,
                role,
            )
            .and_then(|| Ok(()))
        }
    }
}

#[repr(C)]
pub struct IPolicyConfigVista_Vtbl {
    pub base__: IUnknown_Vtbl,

    pub GetMixFormat: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        format: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,

    pub GetDeviceFormat: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        mode: i32,
        format: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,

    pub SetDeviceFormat: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        format: *mut WAVEFORMATEX,
        mix: *mut WAVEFORMATEX,
    ) -> HRESULT,

    pub GetProcessingPeriod: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        mode: i32,
        def_period: *mut i64,
        min_period: *mut i64,
    ) -> HRESULT,

    pub SetProcessingPeriod: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        period: *mut i64,
    ) -> HRESULT,

    pub GetShareMode: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        mode: *mut c_void, // DeviceShareMode struct (undocumented)
    ) -> HRESULT,

    pub SetShareMode: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        mode: *mut c_void,
    ) -> HRESULT,

    pub GetPropertyValue: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        key: *const PROPERTYKEY,
        value: *mut PROPVARIANT,
    ) -> HRESULT,

    pub SetPropertyValue: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        key: *const PROPERTYKEY,
        value: *const PROPVARIANT,
    ) -> HRESULT,

    pub SetDefaultEndpoint: unsafe extern "system" fn(
        this: *mut c_void,
        device_id: PCWSTR,
        role: i32, // ERole
    ) -> HRESULT,

    pub SetEndpointVisibility:
        unsafe extern "system" fn(this: *mut c_void, device_id: PCWSTR, visible: i32) -> HRESULT,
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
