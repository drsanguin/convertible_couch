use core::ffi::c_void;

use crate::speakers_settings::windows::windows_com::{
    IMMDevice as IMMDeviceTrait, IMMDeviceCollection as IMMDeviceCollectionTrait,
    IMMDeviceEnumerator as IMMDeviceEnumeratorTrait, IPolicyConfigVista as IPolicyConfigVistaTrait,
    IPropertyStore as IPropertyStoreTrait, WindowsCom as WindowsComTrait,
};
use windows::{
    core::{define_interface, interface_hierarchy},
    Win32::{
        Foundation::PROPERTYKEY,
        Media::Audio::{
            EDataFlow, ERole, IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator,
            MMDeviceEnumerator, DEVICE_STATE, WAVEFORMATEX,
        },
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, StructuredStorage::PROPVARIANT,
            CLSCTX_ALL, STGM,
        },
        UI::Shell::PropertiesSystem::IPropertyStore,
    },
};
use windows_core::{IUnknown, IUnknown_Vtbl, Interface, Result, GUID, HRESULT, PCWSTR, PWSTR};

const POLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x294935ce_f637_4e7c_a41b_ab255460b862);

pub struct WindowsApiBasedWindowsCom;

impl WindowsComTrait for WindowsApiBasedWindowsCom {
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: windows::Win32::System::Com::COINIT,
    ) -> HRESULT {
        CoInitializeEx(pvreserved, dwcoinit)
    }

    unsafe fn co_uninitialize(&self) {
        CoUninitialize()
    }

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumeratorTrait>> {
        let immdevice_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        Ok(Box::new(WindowsApiBasedIMMDeviceEnumerator {
            immdevice_enumerator,
        }))
    }

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVistaTrait>> {
        let ipolicy_config_vista: IPolicyConfigVista =
            CoCreateInstance(&POLICY_CONFIG_VISTA, None, CLSCTX_ALL)?;

        Ok(Box::new(WindowsApiBasedIPolicyConfigVista {
            ipolicy_config_vista,
        }))
    }
}

pub struct WindowsApiBasedIMMDeviceEnumerator {
    immdevice_enumerator: IMMDeviceEnumerator,
}

impl IMMDeviceEnumeratorTrait for WindowsApiBasedIMMDeviceEnumerator {
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<Box<dyn IMMDeviceTrait>> {
        let immdevice = self
            .immdevice_enumerator
            .GetDefaultAudioEndpoint(dataflow, role)?;

        Ok(Box::new(WindowsApiBasedIMMDevice { immdevice }))
    }

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollectionTrait>> {
        let immdevice_collection = self
            .immdevice_enumerator
            .EnumAudioEndpoints(dataflow, dwstatemask)?;

        Ok(Box::new(WindowsApiBasedIMMDeviceCollection {
            immdevice_collection,
        }))
    }
}

pub struct WindowsApiBasedIMMDeviceCollection {
    immdevice_collection: IMMDeviceCollection,
}

impl IMMDeviceCollectionTrait for WindowsApiBasedIMMDeviceCollection {
    unsafe fn get_count(&self) -> Result<u32> {
        self.immdevice_collection.GetCount()
    }

    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDeviceTrait>> {
        let immdevice = self.immdevice_collection.Item(ndevice)?;

        Ok(Box::new(WindowsApiBasedIMMDevice { immdevice }))
    }
}

pub struct WindowsApiBasedIMMDevice {
    immdevice: IMMDevice,
}

impl IMMDeviceTrait for WindowsApiBasedIMMDevice {
    unsafe fn get_id(&self) -> Result<PWSTR> {
        self.immdevice.GetId()
    }

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStoreTrait>> {
        let iproperty_store = self.immdevice.OpenPropertyStore(stgmaccess)?;

        Ok(Box::new(WindowsApiBasedIPropertyStore { iproperty_store }))
    }
}

pub struct WindowsApiBasedIPropertyStore {
    iproperty_store: IPropertyStore,
}

impl IPropertyStoreTrait for WindowsApiBasedIPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        self.iproperty_store.GetValue(key)
    }
}

pub struct WindowsApiBasedIPolicyConfigVista {
    ipolicy_config_vista: IPolicyConfigVista,
}

impl IPolicyConfigVistaTrait for WindowsApiBasedIPolicyConfigVista {
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()> {
        self.ipolicy_config_vista
            .SetDefaultEndpoint(device_id, role)
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
