use core::ffi::c_void;

use crate::speakers_settings::windows::windows_com::IPolicyConfigVista as IPolicyConfigVistaTrait;
use windows::{
    Win32::{
        Foundation::PROPERTYKEY,
        Media::Audio::{ERole, WAVEFORMATEX},
        System::Com::StructuredStorage::PROPVARIANT,
    },
    core::{define_interface, interface_hierarchy},
};
use windows_core::{HRESULT, IUnknown, IUnknown_Vtbl, Interface, PCWSTR, Result};

pub struct WindowsApiBasedIPolicyConfigVista {
    ipolicy_config_vista: IPolicyConfigVista,
}

impl WindowsApiBasedIPolicyConfigVista {
    pub fn new(ipolicy_config_vista: IPolicyConfigVista) -> Self {
        Self {
            ipolicy_config_vista,
        }
    }
}

impl IPolicyConfigVistaTrait for WindowsApiBasedIPolicyConfigVista {
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()> {
        unsafe {
            self.ipolicy_config_vista
                .SetDefaultEndpoint(device_id, role)
        }
    }
}

define_interface!(
    IPolicyConfigVista,
    IPolicyConfigVista_Vtbl,
    0x568b9108_44bf_40b4_9006_86afe5b5a620
);
interface_hierarchy!(IPolicyConfigVista, IUnknown);

impl IPolicyConfigVista {
    #[allow(non_snake_case)]
    /// Sets the default audio endpoint device for the specified role.
    ///
    /// This function forwards the call to the underlying COM interface method
    /// and changes the system-wide default audio endpoint.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `device_id` is a valid, non-null, properly null-terminated wide string (`PCWSTR`)
    ///   that points to readable memory for the duration of the call.
    /// - The `device_id` pointer originates from a trusted source (e.g., a valid Windows API
    ///   return value) and is not dangling or freed.
    /// - `self` is a valid, properly initialized COM interface pointer with a correct vtable.
    /// - The current thread is in a valid COM apartment state required by the underlying API.
    ///
    /// Violating any of these conditions may result in undefined behavior, including
    /// memory corruption or process crashes.
    pub unsafe fn SetDefaultEndpoint(&self, device_id: PCWSTR, role: ERole) -> Result<()> {
        unsafe {
            (Interface::vtable(self).SetDefaultEndpoint)(Interface::as_raw(self), device_id, role)
                .and_then(|| Ok(()))
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
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
        mode: *mut c_void,
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

    pub SetDefaultEndpoint:
        unsafe extern "system" fn(this: *mut c_void, device_id: PCWSTR, role: ERole) -> HRESULT,

    pub SetEndpointVisibility:
        unsafe extern "system" fn(this: *mut c_void, device_id: PCWSTR, visible: i32) -> HRESULT,
}
