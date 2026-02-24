use core::ffi::c_void;

use windows::{
    core::{GUID, HRESULT},
    Win32::System::Com::{CLSCTX, COINIT},
};

use windows_core::{IUnknown, Interface, Param, Result};

pub mod windows_api_based_windows_com;

pub trait WindowsCom {
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT;
    unsafe fn co_uninitialize(&self);
    unsafe fn co_create_instance<P1, T>(
        &self,
        rclsid: *const GUID,
        punkouter: P1,
        dwclscontext: CLSCTX,
    ) -> Result<T>
    where
        P1: Param<IUnknown>,
        T: Interface;
}
