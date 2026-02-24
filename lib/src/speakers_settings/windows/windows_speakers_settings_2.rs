use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Media::Audio::{
        eConsole, eRender, EDataFlow, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    },
    System::Com::{CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ},
};

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

            let immdevice_collection = unsafe {
                immdevice_enumerator.EnumAudioEndpoints(EDataFlow::default(), DEVICE_STATE_ACTIVE)
            }?;

            let speaker_count = unsafe { immdevice_collection.GetCount() }?;

            for speaker_index in 0..speaker_count {
                let immdevice = unsafe { immdevice_collection.Item(speaker_index) }?;
                let property_store = unsafe { immdevice.OpenPropertyStore(STGM_READ) }?;
                let propvariant = unsafe { property_store.GetValue(&PKEY_Device_FriendlyName) }?;
                let pwsz_val = unsafe { propvariant.Anonymous.Anonymous.Anonymous.pwszVal };
                let friendly_name = String::from_utf16(unsafe { pwsz_val.as_wide() })?;
            }
        }

        unsafe { self.windows_com.co_uninitialize() };

        todo!()
    }

    fn get_speakers_infos(
        &self,
    ) -> Result<Vec<crate::speakers_settings::SpeakerInfo>, crate::ApplicationError> {
        todo!()
    }
}
