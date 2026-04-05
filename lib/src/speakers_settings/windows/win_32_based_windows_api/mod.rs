use core::ffi::c_void;

use crate::{
    speakers_settings::windows::{
        win_32_based_windows_api::{
            immdevice_enumerator::Win32ApiBasedIMMDeviceEnumerator,
            ipolicy_config_vista::{IPolicyConfigVista, Win32BasedIPolicyConfigVista},
        },
        windows_api::{
            IMMDeviceEnumerator as IMMDeviceEnumeratorTrait,
            IPolicyConfigVista as IPolicyConfigVistaTrait, WindowsApi,
        },
    },
    trace_fn,
};
use windows::Win32::{
    Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{CLSCTX_ALL, COINIT, CoCreateInstance, CoInitializeEx, CoUninitialize},
};
use windows_core::{GUID, HRESULT, Result};

pub mod immdevice;
pub mod immdevice_collection;
pub mod immdevice_enumerator;
pub mod ipolicy_config_vista;
pub mod iproperty_store;

const POLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x294935ce_f637_4e7c_a41b_ab255460b862);

pub struct Win32BasedWindowsApi;

impl WindowsApi for Win32BasedWindowsApi {
    unsafe fn co_initialize_ex(
        &mut self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT {
        trace_fn!();

        unsafe { CoInitializeEx(pvreserved, dwcoinit) }
    }

    unsafe fn co_uninitialize(&mut self) {
        trace_fn!();

        unsafe { CoUninitialize() }
    }

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumeratorTrait>> {
        trace_fn!();

        unsafe {
            let immdevice_enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let windows_api_based_immdevice_enumerator =
                Win32ApiBasedIMMDeviceEnumerator::new(immdevice_enumerator);
            let boxed_windows_api_based_immdevice_enumerator =
                Box::new(windows_api_based_immdevice_enumerator);

            Ok(boxed_windows_api_based_immdevice_enumerator)
        }
    }

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVistaTrait>> {
        trace_fn!();

        unsafe {
            let ipolicy_config_vista: IPolicyConfigVista =
                CoCreateInstance(&POLICY_CONFIG_VISTA, None, CLSCTX_ALL)?;
            let windows_api_based_ipolicy_config_vista =
                Win32BasedIPolicyConfigVista::new(ipolicy_config_vista);
            let boxed_windows_api_based_ipolicy_config_vista =
                Box::new(windows_api_based_ipolicy_config_vista);

            Ok(boxed_windows_api_based_ipolicy_config_vista)
        }
    }
}
