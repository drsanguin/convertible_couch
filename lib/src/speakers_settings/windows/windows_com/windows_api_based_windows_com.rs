use std::ffi::c_void;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX, COINIT,
};
use windows_core::{IUnknown, Interface, Param, Result, GUID, HRESULT};

use crate::speakers_settings::windows::windows_com::WindowsCom;

pub struct WindowsApiBasedWindowsCom;

impl WindowsCom for WindowsApiBasedWindowsCom {
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT {
        CoInitializeEx(pvreserved, dwcoinit)
    }

    unsafe fn co_uninitialize(&self) {
        CoUninitialize();
    }

    unsafe fn co_create_instance<P1, T>(
        &self,
        rclsid: *const GUID,
        punkouter: P1,
        dwclscontext: CLSCTX,
    ) -> Result<T>
    where
        P1: Param<IUnknown>,
        T: Interface,
    {
        CoCreateInstance(rclsid, punkouter, dwclscontext)
    }
}
