use crate::speakers_settings::windows::windows_com::{
    windows_api_based_windows_com::{
        immdevice::WindowsApiBasedIMMDevice,
        immdevice_collection::WindowsApiBasedIMMDeviceCollection,
    },
    IMMDevice as IMMDeviceTrait, IMMDeviceCollection as IMMDeviceCollectionTrait,
    IMMDeviceEnumerator as IMMDeviceEnumeratorTrait,
};
use windows::Win32::Media::Audio::{EDataFlow, ERole, IMMDeviceEnumerator, DEVICE_STATE};
use windows_core::Result;

pub struct WindowsApiBasedIMMDeviceEnumerator {
    immdevice_enumerator: IMMDeviceEnumerator,
}

impl WindowsApiBasedIMMDeviceEnumerator {
    pub fn new(immdevice_enumerator: IMMDeviceEnumerator) -> Self {
        Self {
            immdevice_enumerator,
        }
    }
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
        let windows_api_based_immdevice = WindowsApiBasedIMMDevice::new(immdevice);
        let boxed_windows_api_based_immdevice = Box::new(windows_api_based_immdevice);

        Ok(boxed_windows_api_based_immdevice)
    }

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollectionTrait>> {
        let immdevice_collection = self
            .immdevice_enumerator
            .EnumAudioEndpoints(dataflow, dwstatemask)?;
        let windows_api_based_immdevice_collection =
            WindowsApiBasedIMMDeviceCollection::new(immdevice_collection);
        let boxed_windows_api_based_immdevice_collection =
            Box::new(windows_api_based_immdevice_collection);

        Ok(boxed_windows_api_based_immdevice_collection)
    }
}
