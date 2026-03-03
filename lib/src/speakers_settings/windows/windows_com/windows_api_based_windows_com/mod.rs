use core::ffi::c_void;

use crate::speakers_settings::windows::windows_com::{
    windows_api_based_windows_com::{
        immdevice_enumerator::WindowsApiBasedIMMDeviceEnumerator,
        ipolicy_config_vista::{IPolicyConfigVista, WindowsApiBasedIPolicyConfigVista},
    },
    IMMDeviceEnumerator as IMMDeviceEnumeratorTrait, IPolicyConfigVista as IPolicyConfigVistaTrait,
    WindowsCom as WindowsComTrait,
};
use windows::Win32::{
    Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL},
};
use windows_core::{Result, GUID, HRESULT};

pub mod immdevice;
pub mod immdevice_collection;
pub mod immdevice_enumerator;
pub mod ipolicy_config_vista;
pub mod iproperty_store;

const POLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x294935ce_f637_4e7c_a41b_ab255460b862);

pub struct WindowsApiBasedWindowsCom;

impl WindowsComTrait for WindowsApiBasedWindowsCom {
    unsafe fn co_initialize_ex(
        &mut self,
        pvreserved: Option<*const c_void>,
        dwcoinit: windows::Win32::System::Com::COINIT,
    ) -> HRESULT {
        CoInitializeEx(pvreserved, dwcoinit)
    }

    unsafe fn co_uninitialize(&mut self) {
        CoUninitialize()
    }

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumeratorTrait>> {
        let immdevice_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let windows_api_based_immdevice_enumerator =
            WindowsApiBasedIMMDeviceEnumerator::new(immdevice_enumerator);
        let boxed_windows_api_based_immdevice_enumerator =
            Box::new(windows_api_based_immdevice_enumerator);

        Ok(boxed_windows_api_based_immdevice_enumerator)
    }

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVistaTrait>> {
        let ipolicy_config_vista: IPolicyConfigVista =
            CoCreateInstance(&POLICY_CONFIG_VISTA, None, CLSCTX_ALL)?;
        let windows_api_based_ipolicy_config_vista =
            WindowsApiBasedIPolicyConfigVista::new(ipolicy_config_vista);
        let boxed_windows_api_based_ipolicy_config_vista =
            Box::new(windows_api_based_ipolicy_config_vista);

        Ok(boxed_windows_api_based_ipolicy_config_vista)
    }
}
