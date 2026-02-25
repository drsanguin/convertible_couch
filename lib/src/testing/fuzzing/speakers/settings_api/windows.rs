use crate::{
    speakers_settings::windows::windows_com::WindowsCom,
    testing::fuzzing::speakers::{
        settings_api::{
            behaviour::windows::FuzzedWindowsSpeakersSettingsApiBehaviour,
            FuzzedSpeakersSettingsApi,
        },
        FuzzedSpeaker,
    },
};

#[derive(Clone, Default)]
pub struct FuzzedWindowsCom {
    speakers: Vec<FuzzedSpeaker>,
    behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
}

impl FuzzedSpeakersSettingsApi for FuzzedWindowsCom {
    fn new(
        speakers: Vec<FuzzedSpeaker>,
        behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
    ) -> Self {
        Self {
            speakers,
            behaviour,
        }
    }
}

impl WindowsCom for FuzzedWindowsCom {
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const std::ffi::c_void>,
        dwcoinit: windows::Win32::System::Com::COINIT,
    ) -> windows_core::HRESULT {
        todo!()
    }

    unsafe fn co_uninitialize(&self) {
        todo!()
    }

    unsafe fn co_create_instance<P1, T>(
        &self,
        rclsid: *const windows_core::GUID,
        punkouter: P1,
        dwclscontext: windows::Win32::System::Com::CLSCTX,
    ) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        todo!()
    }
}
