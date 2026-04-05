use crate::{
    speakers_settings::windows::{
        win_32_based_windows_api::immdevice::Win32ApiBasedIMMDevice,
        windows_api::{
            IMMDevice as IMMDeviceTrait, IMMDeviceCollection as IMMDeviceCollectionTrait,
        },
    },
    trace_fn,
};
use windows::Win32::Media::Audio::IMMDeviceCollection;
use windows_core::Result;

pub struct Win32ApiBasedIMMDeviceCollection {
    immdevice_collection: IMMDeviceCollection,
}

impl Win32ApiBasedIMMDeviceCollection {
    pub fn new(immdevice_collection: IMMDeviceCollection) -> Self {
        trace_fn!();

        Self {
            immdevice_collection,
        }
    }
}

impl IMMDeviceCollectionTrait for Win32ApiBasedIMMDeviceCollection {
    unsafe fn get_count(&self) -> Result<u32> {
        trace_fn!();

        unsafe { self.immdevice_collection.GetCount() }
    }

    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDeviceTrait>> {
        trace_fn!();

        unsafe {
            let immdevice = self.immdevice_collection.Item(ndevice)?;
            let windows_api_based_immdevice = Win32ApiBasedIMMDevice::new(immdevice);
            let boxed_windows_api_based_immdevice = Box::new(windows_api_based_immdevice);

            Ok(boxed_windows_api_based_immdevice)
        }
    }
}
