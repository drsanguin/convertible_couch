use crate::speakers_settings::windows::windows_com::{
    windows_api_based_windows_com::iproperty_store::WindowsApiBasedIPropertyStore,
    IMMDevice as IMMDeviceTrait, IPropertyStore as IPropertyStoreTrait,
};
use windows::Win32::{Media::Audio::IMMDevice, System::Com::STGM};
use windows_core::{Result, PWSTR};

pub struct WindowsApiBasedIMMDevice {
    immdevice: IMMDevice,
}

impl WindowsApiBasedIMMDevice {
    pub fn new(immdevice: IMMDevice) -> Self {
        Self { immdevice }
    }
}

impl IMMDeviceTrait for WindowsApiBasedIMMDevice {
    unsafe fn get_id(&self) -> Result<PWSTR> {
        self.immdevice.GetId()
    }

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStoreTrait>> {
        let iproperty_store = self.immdevice.OpenPropertyStore(stgmaccess)?;
        let windows_api_based_iproperty_store = WindowsApiBasedIPropertyStore::new(iproperty_store);
        let box_windows_api_based_iproperty_store = Box::new(windows_api_based_iproperty_store);

        Ok(box_windows_api_based_iproperty_store)
    }
}
