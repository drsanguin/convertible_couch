use std::ffi::c_void;
use windows::Win32::System::Com::{CLSCTX, CLSCTX_ALL, COINIT, COINIT_MULTITHREADED};
use windows_core::{IUnknown, Interface, Param, GUID, HRESULT};

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

#[allow(dead_code)]
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
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT {
        if pvreserved != None || dwcoinit != COINIT_MULTITHREADED {
            return HRESULT(-1);
        }

        HRESULT(0)
    }

    unsafe fn co_uninitialize(&self) {}

    #[allow(unused_variables)]
    unsafe fn co_create_instance<P1, T>(
        &self,
        rclsid: *const GUID,
        punkouter: P1,
        dwclscontext: CLSCTX,
    ) -> windows_core::Result<T>
    where
        P1: Param<IUnknown>,
        T: Interface,
    {
        if !punkouter.param().abi().is_null() || dwclscontext != CLSCTX_ALL {
            return Err(windows_core::Error::empty());
        }

        todo!()
    }
}
