use core::ffi::c_void;

use windows::Win32::{
    Foundation::PROPERTYKEY,
    Media::Audio::{EDataFlow, ERole, DEVICE_STATE},
    System::Com::{StructuredStorage::PROPVARIANT, COINIT, STGM},
};
use windows_core::{Result, HRESULT, PCWSTR, PWSTR};

pub mod windows_api_based_windows_com;

pub trait WindowsCom<
    TIMMDeviceEnumerator,
    TIMMDevice,
    TIMMDeviceCollection,
    TIPropertyStore,
    TIPolicyConfigVista,
> where
    TIMMDeviceEnumerator: IMMDeviceEnumerator<TIMMDevice, TIMMDeviceCollection, TIPropertyStore>,
    TIMMDevice: IMMDevice<TIPropertyStore>,
    TIMMDeviceCollection: IMMDeviceCollection<TIMMDevice, TIPropertyStore>,
    TIPropertyStore: IPropertyStore,
    TIPolicyConfigVista: IPolicyConfigVista,
{
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT;

    unsafe fn co_uninitialize(&self);

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<TIMMDeviceEnumerator>;

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<TIPolicyConfigVista>;
}

pub trait IMMDeviceEnumerator<TIMMDevice, TIMMDeviceCollection, TIPropertyStore>
where
    TIMMDevice: IMMDevice<TIPropertyStore>,
    TIMMDeviceCollection: IMMDeviceCollection<TIMMDevice, TIPropertyStore>,
    TIPropertyStore: IPropertyStore,
{
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<TIMMDevice>;

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<TIMMDeviceCollection>;
}

pub trait IMMDevice<TIPropertyStore>
where
    TIPropertyStore: IPropertyStore,
{
    unsafe fn get_id(&self) -> Result<PWSTR>;

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<TIPropertyStore>;
}

pub trait IMMDeviceCollection<TIMMDevice, TIPropertyStore>
where
    TIMMDevice: IMMDevice<TIPropertyStore>,
    TIPropertyStore: IPropertyStore,
{
    unsafe fn get_count(&self) -> Result<u32>;

    unsafe fn item(&self, ndevice: u32) -> Result<TIMMDevice>;
}

pub trait IPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT>;
}

pub trait IPolicyConfigVista {
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()>;
}
