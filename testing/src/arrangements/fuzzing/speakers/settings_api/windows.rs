use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Foundation::{E_INVALIDARG, PROPERTYKEY, S_FALSE, S_OK},
    Media::Audio::{DEVICE_STATE, DEVICE_STATE_ACTIVE, EDataFlow, ERole, eConsole, eRender},
    System::Com::{
        COINIT, COINIT_MULTITHREADED, STGM, STGM_READ,
        StructuredStorage::{PROPVARIANT, PROPVARIANT_0, PROPVARIANT_0_0, PROPVARIANT_0_0_0},
    },
};
use windows_core::{Error, HRESULT, PCWSTR, PWSTR, Result};

use crate::arrangements::fuzzing::speakers::{
    FuzzedSpeaker,
    settings_api::{
        FuzzedSpeakersSettingsApi, behaviour::windows::FuzzedWindowsSpeakersSettingsApiBehaviour,
    },
};
use convertible_couch_lib::speakers_settings::windows::windows_api::{
    IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, IPolicyConfigVista, IPropertyStore,
    WindowsApi,
};

use std::{cell::RefCell, ffi::c_void, mem::ManuallyDrop, rc::Rc};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct FuzzedWindowsApi {
    speakers: Rc<RefCell<Vec<FuzzedSpeaker>>>,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
    com_library_initialized: bool,
}

impl FuzzedSpeakersSettingsApi for FuzzedWindowsApi {
    fn new(
        speakers: Vec<FuzzedSpeaker>,
        behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
    ) -> Self {
        Self {
            speakers: Rc::new(RefCell::new(speakers)),
            behaviour: Rc::new(behaviour),
            com_library_initialized: false,
        }
    }
}

impl WindowsApi for FuzzedWindowsApi {
    unsafe fn co_initialize_ex(
        &mut self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT {
        if let Some(error) = self.behaviour.co_initialize_ex_error {
            return error.into();
        }

        if pvreserved.is_some() || dwcoinit != COINIT_MULTITHREADED {
            return E_INVALIDARG;
        }

        if self.com_library_initialized {
            return S_FALSE;
        }

        self.com_library_initialized = true;

        S_OK
    }

    unsafe fn co_uninitialize(&mut self) {
        self.com_library_initialized = false;
    }

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumerator>> {
        if let Some(error) = self.behaviour.co_create_immdevice_enumerator_error {
            return Err(error.into());
        }

        if !self.com_library_initialized {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let fuzzed_immdevice_enumerator = FuzzedIMMDeviceEnumerator {
            speakers: self.speakers.borrow().clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_immdevice_enumerator = Box::new(fuzzed_immdevice_enumerator);

        Ok(boxed_fuzzed_immdevice_enumerator)
    }

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVista>> {
        if let Some(error) = self.behaviour.co_create_ipolicy_config_vista_error {
            return Err(error.into());
        }

        if !self.com_library_initialized {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let fuzzed_ipolicy_config_vista = FuzzedIPolicyConfigVista {
            speakers: self.speakers.clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_ipolicy_config_vista = Box::new(fuzzed_ipolicy_config_vista);

        Ok(boxed_fuzzed_ipolicy_config_vista)
    }
}

pub struct FuzzedIMMDeviceEnumerator {
    speakers: Vec<FuzzedSpeaker>,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
}

impl IMMDeviceEnumerator for FuzzedIMMDeviceEnumerator {
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<Box<dyn IMMDevice>> {
        if let Some(error) = self
            .behaviour
            .immdevice_enumerator_get_default_audio_endpoint_error
        {
            return Err(error.into());
        }

        if dataflow != eRender || role != eConsole {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let default_speaker_option = self.speakers.iter().find(|s| s.is_default);

        if default_speaker_option.is_none() {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let default_speaker = default_speaker_option.unwrap();
        let fuzzed_immdevice = FuzzedIMMDevice {
            speaker: default_speaker.clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_immdevice = Box::new(fuzzed_immdevice);

        Ok(boxed_fuzzed_immdevice)
    }

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollection>> {
        if let Some(error) = self
            .behaviour
            .immdevice_enumerator_enum_audio_endpoints_error
        {
            return Err(error.into());
        }

        if dataflow != eRender || dwstatemask != DEVICE_STATE_ACTIVE {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let fuzzed_immdevice_collection = FuzzedIMMDeviceCollection {
            speakers: self.speakers.clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_immdevice_collection = Box::new(fuzzed_immdevice_collection);

        Ok(boxed_fuzzed_immdevice_collection)
    }
}

pub struct FuzzedIMMDevice {
    speaker: FuzzedSpeaker,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
}

impl IMMDevice for FuzzedIMMDevice {
    unsafe fn get_id(&self) -> Result<PWSTR> {
        if let Some(error) = self.behaviour.immdevice_get_id_error {
            return Err(error.into());
        }

        let mut id_utf16 = self.speaker.id.encode_utf16().collect::<Vec<_>>();

        id_utf16.push(0);

        let boxed = id_utf16.into_boxed_slice();
        let leaked = Box::leak(boxed);

        Ok(PWSTR(leaked.as_mut_ptr()))
    }

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStore>> {
        if let Some(error) = self.behaviour.immdevice_open_property_store_error {
            return Err(error.into());
        }

        if stgmaccess != STGM_READ {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let fuzzed_iproperty_store = FuzzedIPropertyStore {
            speaker: self.speaker.clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_iproperty_store = Box::new(fuzzed_iproperty_store);

        Ok(boxed_fuzzed_iproperty_store)
    }
}

pub struct FuzzedIMMDeviceCollection {
    speakers: Vec<FuzzedSpeaker>,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
}

impl IMMDeviceCollection for FuzzedIMMDeviceCollection {
    unsafe fn get_count(&self) -> Result<u32> {
        if let Some(error) = self.behaviour.immdevice_collection_get_count_error {
            return Err(error.into());
        }

        Ok(self.speakers.len().try_into().unwrap())
    }

    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDevice>> {
        if let Some(error) = self.behaviour.immdevice_collection_item_error {
            return Err(error.into());
        }

        let index: usize = ndevice.try_into().unwrap();
        let speaker_option = self.speakers.get(index);

        if speaker_option.is_none() {
            let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

            return Err(error);
        }

        let fuzzed_immdevice = FuzzedIMMDevice {
            speaker: speaker_option.unwrap().clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_immdevice = Box::new(fuzzed_immdevice);

        Ok(boxed_fuzzed_immdevice)
    }
}

pub struct FuzzedIPropertyStore {
    speaker: FuzzedSpeaker,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
}

impl IPropertyStore for FuzzedIPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        if let Some(error) = self.behaviour.property_store_get_value_error {
            return Err(error.into());
        }

        unsafe {
            if *key != PKEY_Device_FriendlyName {
                let error = Error::new(E_INVALIDARG, "One or more arguments are not valid");

                return Err(error);
            }

            let mut name_utf16 = self.speaker.name.encode_utf16().collect::<Vec<_>>();
            name_utf16.push(0);

            let boxed = name_utf16.into_boxed_slice();
            let leaked = Box::leak(boxed);
            let propvariant = PROPVARIANT {
                Anonymous: PROPVARIANT_0 {
                    Anonymous: ManuallyDrop::<PROPVARIANT_0_0>::new(PROPVARIANT_0_0 {
                        Anonymous: PROPVARIANT_0_0_0 {
                            pwszVal: PWSTR(leaked.as_mut_ptr()),
                        },
                        ..Default::default()
                    }),
                },
            };

            Ok(propvariant)
        }
    }
}

pub struct FuzzedIPolicyConfigVista {
    speakers: Rc<RefCell<Vec<FuzzedSpeaker>>>,
    behaviour: Rc<FuzzedWindowsSpeakersSettingsApiBehaviour>,
}

impl IPolicyConfigVista for FuzzedIPolicyConfigVista {
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()> {
        if let Some(error) = self
            .behaviour
            .ipolicy_config_vista_set_default_endpoint_error
        {
            return Err(error.into());
        }

        unsafe {
            if role != eConsole {
                return Err(Error::empty());
            }

            let speaker_id = String::from_utf16(device_id.as_wide())?;

            for speaker in self.speakers.borrow_mut().iter_mut() {
                if speaker.is_default {
                    speaker.is_default = false;
                }

                if speaker.id == speaker_id {
                    speaker.is_default = true;
                }
            }

            Ok(())
        }
    }
}
