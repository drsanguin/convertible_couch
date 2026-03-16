use crate::{
    speakers_settings::windows::windows_com::{
        IMMDevice as IMMDeviceTrait, IPropertyStore as IPropertyStoreTrait,
        windows_api_based_windows_com::iproperty_store::WindowsApiBasedIPropertyStore,
    },
    trace_fn,
};
use windows::Win32::{Media::Audio::IMMDevice, System::Com::STGM};
use windows_core::{PWSTR, Result};

pub struct WindowsApiBasedIMMDevice {
    immdevice: IMMDevice,
}

impl WindowsApiBasedIMMDevice {
    pub fn new(immdevice: IMMDevice) -> Self {
        trace_fn!();
        Self { immdevice }
    }
}

impl IMMDeviceTrait for WindowsApiBasedIMMDevice {
    unsafe fn get_id(&self) -> Result<PWSTR> {
        trace_fn!();
        unsafe { self.immdevice.GetId() }
    }

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStoreTrait>> {
        trace_fn!();
        unsafe {
            let iproperty_store = self.immdevice.OpenPropertyStore(stgmaccess)?;
            let windows_api_based_iproperty_store =
                WindowsApiBasedIPropertyStore::new(iproperty_store);
            let boxed_windows_api_based_iproperty_store =
                Box::new(windows_api_based_iproperty_store);

            Ok(boxed_windows_api_based_iproperty_store)
        }
    }
}
