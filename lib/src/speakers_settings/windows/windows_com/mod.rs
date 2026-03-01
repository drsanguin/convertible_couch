use core::ffi::c_void;

use windows::Win32::{
    Foundation::PROPERTYKEY,
    Media::Audio::{EDataFlow, ERole, DEVICE_STATE},
    System::Com::{StructuredStorage::PROPVARIANT, COINIT, STGM},
};
use windows_core::{Result, HRESULT, PCWSTR, PWSTR};

pub mod windows_api_based_windows_com;

pub trait WindowsCom {
    /// Initializes COM for the current thread.
    ///
    /// # Safety
    ///
    /// - Must be called at most once per thread unless balanced with `co_uninitialize`.
    /// - The calling thread must follow COM apartment threading rules.
    /// - `pvreserved` must be either `None` or a valid pointer as required by the Windows API.
    /// - `dwcoinit` must be a valid `COINIT` flag combination.
    /// - The caller must ensure COM is not already initialized with incompatible flags.
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT;

    /// Uninitializes COM on the current thread.
    ///
    /// # Safety
    ///
    /// - Must only be called if `co_initialize_ex` was previously called on the same thread.
    /// - Must not be called more times than `co_initialize_ex` was called.
    /// - No COM objects created on this thread may be used after this call.
    unsafe fn co_uninitialize(&self);

    /// Creates an `IMMDeviceEnumerator` COM object.
    ///
    /// # Safety
    ///
    /// - COM must be initialized on the current thread.
    /// - The returned object follows COM lifetime and threading rules.
    /// - The caller must ensure proper reference counting and release semantics.
    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumerator>>;

    /// Creates a `IPolicyConfigVista` COM object.
    ///
    /// # Safety
    ///
    /// - COM must be initialized on the current thread.
    /// - The returned interface must be used according to COM threading and lifetime rules.
    /// - The caller must ensure correct ownership and release of the COM object.
    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVista>>;
}

pub trait IMMDeviceEnumerator {
    /// Gets the default audio endpoint device.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid and properly initialized.
    /// - COM must be initialized on the calling thread.
    /// - `dataflow` and `role` must be valid enum values.
    /// - Returned device must be managed according to COM lifetime rules.
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<Box<dyn IMMDevice>>;

    /// Enumerates audio endpoint devices.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - `dwstatemask` must be a valid `DEVICE_STATE` flag combination.
    /// - Returned collection must follow COM lifetime and ownership rules.
    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollection>>;
}

pub trait IMMDevice {
    /// Retrieves the device ID string.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - The returned `PWSTR` must be freed according to Windows API memory rules.
    /// - The pointer must not be used after it is freed.
    unsafe fn get_id(&self) -> Result<PWSTR>;

    /// Opens the property store for the device.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - `stgmaccess` must be a valid `STGM` access flag.
    /// - The returned property store must follow COM lifetime and threading rules.
    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStore>>;
}

pub trait IMMDeviceCollection {
    /// Gets the number of devices in the collection.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - The returned count must not be used to index beyond the collection bounds.
    unsafe fn get_count(&self) -> Result<u32>;

    /// Retrieves a device by index.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - `ndevice` must be less than the value returned by `get_count`.
    /// - The returned device must follow COM lifetime rules.
    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDevice>>;
}

pub trait IPropertyStore {
    /// Retrieves a property value.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - `key` must be a valid, non-null pointer to a `PROPERTYKEY`.
    /// - The returned `PROPVARIANT` must be properly cleared/freed by the caller.
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT>;
}

pub trait IPolicyConfigVista {
    /// Sets the default audio endpoint device.
    ///
    /// # Safety
    ///
    /// - The COM object must be valid.
    /// - COM must be initialized on the calling thread.
    /// - `device_id` must be a valid, null-terminated UTF-16 string.
    /// - `role` must be a valid `ERole` value.
    /// - The caller must ensure the device ID remains valid for the duration of the call.
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()>;
}
