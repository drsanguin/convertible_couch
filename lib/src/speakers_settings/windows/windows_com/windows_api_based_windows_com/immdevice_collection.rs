use crate::speakers_settings::windows::windows_com::{
    windows_api_based_windows_com::immdevice::WindowsApiBasedIMMDevice,
    IMMDevice as IMMDeviceTrait, IMMDeviceCollection as IMMDeviceCollectionTrait,
};
use windows::Win32::Media::Audio::IMMDeviceCollection;
use windows_core::Result;

pub struct WindowsApiBasedIMMDeviceCollection {
    immdevice_collection: IMMDeviceCollection,
}

impl WindowsApiBasedIMMDeviceCollection {
    pub fn new(immdevice_collection: IMMDeviceCollection) -> Self {
        Self {
            immdevice_collection,
        }
    }
}

impl IMMDeviceCollectionTrait for WindowsApiBasedIMMDeviceCollection {
    unsafe fn get_count(&self) -> Result<u32> {
        self.immdevice_collection.GetCount()
    }

    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDeviceTrait>> {
        let immdevice = self.immdevice_collection.Item(ndevice)?;
        let windows_api_based_immdevice = WindowsApiBasedIMMDevice::new(immdevice);
        let box_windows_api_based_immdevice = Box::new(windows_api_based_immdevice);

        Ok(box_windows_api_based_immdevice)
    }
}
