use crate::{
    speakers_settings::windows::{
        win_32_based_windows_api::{
            immdevice::Win32ApiBasedIMMDevice,
            immdevice_collection::Win32ApiBasedIMMDeviceCollection,
        },
        windows_api::{
            IMMDevice as IMMDeviceTrait, IMMDeviceCollection as IMMDeviceCollectionTrait,
            IMMDeviceEnumerator as IMMDeviceEnumeratorTrait,
        },
    },
    trace_fn,
};
use windows::Win32::Media::Audio::{DEVICE_STATE, EDataFlow, ERole, IMMDeviceEnumerator};
use windows_core::Result;

pub struct Win32ApiBasedIMMDeviceEnumerator {
    immdevice_enumerator: IMMDeviceEnumerator,
}

impl Win32ApiBasedIMMDeviceEnumerator {
    pub fn new(immdevice_enumerator: IMMDeviceEnumerator) -> Self {
        trace_fn!();

        Self {
            immdevice_enumerator,
        }
    }
}

impl IMMDeviceEnumeratorTrait for Win32ApiBasedIMMDeviceEnumerator {
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<Box<dyn IMMDeviceTrait>> {
        trace_fn!();

        unsafe {
            let immdevice = self
                .immdevice_enumerator
                .GetDefaultAudioEndpoint(dataflow, role)?;
            let windows_api_based_immdevice = Win32ApiBasedIMMDevice::new(immdevice);
            let boxed_windows_api_based_immdevice = Box::new(windows_api_based_immdevice);

            Ok(boxed_windows_api_based_immdevice)
        }
    }

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollectionTrait>> {
        trace_fn!();

        unsafe {
            let immdevice_collection = self
                .immdevice_enumerator
                .EnumAudioEndpoints(dataflow, dwstatemask)?;
            let windows_api_based_immdevice_collection =
                Win32ApiBasedIMMDeviceCollection::new(immdevice_collection);
            let boxed_windows_api_based_immdevice_collection =
                Box::new(windows_api_based_immdevice_collection);

            Ok(boxed_windows_api_based_immdevice_collection)
        }
    }
}
