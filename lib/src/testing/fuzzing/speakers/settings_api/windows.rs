use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
    Foundation::{E_FAIL, PROPERTYKEY},
    Media::Audio::{eConsole, eRender, EDataFlow, ERole, DEVICE_STATE, DEVICE_STATE_ACTIVE},
    System::{
        Com::{
            StructuredStorage::{PROPVARIANT, PROPVARIANT_0_0},
            COINIT, COINIT_MULTITHREADED, STGM, STGM_READ,
        },
        Variant::VARENUM,
    },
};
use windows_core::{Error, Result, HRESULT, PCWSTR, PWSTR};

use crate::{
    speakers_settings::windows::windows_com::{
        IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, IPolicyConfigVista, IPropertyStore,
        WindowsCom,
    },
    testing::fuzzing::speakers::{
        settings_api::{
            behaviour::windows::FuzzedWindowsSpeakersSettingsApiBehaviour,
            FuzzedSpeakersSettingsApi,
        },
        FuzzedSpeaker,
    },
};

use std::{cell::RefCell, ffi::c_void, mem::ManuallyDrop, rc::Rc};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct FuzzedWindowsCom {
    speakers: Rc<RefCell<Vec<FuzzedSpeaker>>>,
    behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
}

impl FuzzedSpeakersSettingsApi for FuzzedWindowsCom {
    fn new(
        speakers: Vec<FuzzedSpeaker>,
        behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
    ) -> Self {
        Self {
            speakers: Rc::new(RefCell::new(speakers)),
            behaviour,
        }
    }
}

impl WindowsCom for FuzzedWindowsCom {
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> windows_core::HRESULT {
        if pvreserved.is_some() || dwcoinit != COINIT_MULTITHREADED {
            return HRESULT(-1);
        }

        HRESULT(0)
    }

    unsafe fn co_uninitialize(&self) {}

    unsafe fn co_create_immdevice_enumerator(&self) -> Result<Box<dyn IMMDeviceEnumerator>> {
        let fuzzed_immdevice_enumerator = FuzzedIMMDeviceEnumerator {
            speakers: self.speakers.borrow().clone(),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_immdevice_enumerator = Box::new(fuzzed_immdevice_enumerator);

        Ok(boxed_fuzzed_immdevice_enumerator)
    }

    unsafe fn co_create_ipolicy_config_vista(&self) -> Result<Box<dyn IPolicyConfigVista>> {
        let fuzzed_ipolicy_config_vista = FuzzedIPolicyConfigVista {
            speakers: Rc::clone(&self.speakers),
            behaviour: self.behaviour.clone(),
        };
        let boxed_fuzzed_ipolicy_config_vista = Box::new(fuzzed_ipolicy_config_vista);

        Ok(boxed_fuzzed_ipolicy_config_vista)
    }
}

pub struct FuzzedIMMDeviceEnumerator {
    speakers: Vec<FuzzedSpeaker>,
    behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
}

impl IMMDeviceEnumerator for FuzzedIMMDeviceEnumerator {
    unsafe fn get_default_audio_endpoint(
        &self,
        dataflow: EDataFlow,
        role: ERole,
    ) -> Result<Box<dyn IMMDevice>> {
        if self.behaviour.getting_the_default_speaker_fails {
            return Err(Error::new(
                E_FAIL,
                "Failed to get the current default speaker",
            ));
        }

        if dataflow != eRender || role != eConsole {
            return Err(Error::empty());
        }

        let default_speaker_option = self.speakers.iter().find(|s| s.is_default);

        if default_speaker_option.is_none() {
            return Err(Error::empty());
        }

        let default_speaker = default_speaker_option.unwrap();
        let fuzzed_immdevice = FuzzedIMMDevice {
            speaker: default_speaker.clone(),
        };
        let boxed_fuzzed_immdevice = Box::new(fuzzed_immdevice);

        Ok(boxed_fuzzed_immdevice)
    }

    unsafe fn enum_audio_endpoints(
        &self,
        dataflow: EDataFlow,
        dwstatemask: DEVICE_STATE,
    ) -> Result<Box<dyn IMMDeviceCollection>> {
        if self.behaviour.getting_the_speakers_fails {
            return Err(Error::new(E_FAIL, "Failed to get the speakers"));
        }

        if dataflow != eRender || dwstatemask != DEVICE_STATE_ACTIVE {
            return Err(Error::empty());
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
}

impl IMMDevice for FuzzedIMMDevice {
    unsafe fn get_id(&self) -> Result<PWSTR> {
        let mut id_utf16 = self.speaker.id.encode_utf16().collect::<Vec<_>>();

        id_utf16.push(0);

        let boxed = id_utf16.into_boxed_slice();
        let leaked = Box::leak(boxed);

        Ok(PWSTR(leaked.as_mut_ptr()))
    }

    unsafe fn open_property_store(&self, stgmaccess: STGM) -> Result<Box<dyn IPropertyStore>> {
        if stgmaccess != STGM_READ {
            return Err(Error::empty());
        }

        let fuzzed_iproperty_store = FuzzedIPropertyStore {
            speaker: self.speaker.clone(),
        };
        let boxed_fuzzed_iproperty_store = Box::new(fuzzed_iproperty_store);

        Ok(boxed_fuzzed_iproperty_store)
    }
}

pub struct FuzzedIMMDeviceCollection {
    speakers: Vec<FuzzedSpeaker>,
    behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
}

impl IMMDeviceCollection for FuzzedIMMDeviceCollection {
    unsafe fn get_count(&self) -> Result<u32> {
        if self.behaviour.getting_the_speakers_count_fails {
            return Err(Error::new(E_FAIL, "Failed to get the number of speakers"));
        }

        Ok(self.speakers.len().try_into().unwrap())
    }

    unsafe fn item(&self, ndevice: u32) -> Result<Box<dyn IMMDevice>> {
        let index: usize = ndevice.try_into().unwrap();
        let speaker_option = self.speakers.get(index);

        if speaker_option.is_none() {
            return Err(Error::empty());
        }

        let fuzzed_immdevice = FuzzedIMMDevice {
            speaker: speaker_option.unwrap().clone(),
        };
        let boxed_fuzzed_immdevice = Box::new(fuzzed_immdevice);

        Ok(boxed_fuzzed_immdevice)
    }
}

pub struct FuzzedIPropertyStore {
    speaker: FuzzedSpeaker,
}

impl IPropertyStore for FuzzedIPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        if *key != PKEY_Device_FriendlyName {
            return Err(Error::empty());
        }

        let mut name_utf16 = self.speaker.name.encode_utf16().collect::<Vec<_>>();

        name_utf16.push(0);

        let boxed = name_utf16.into_boxed_slice();
        let leaked = Box::leak(boxed);

        Ok(PROPVARIANT {
            Anonymous: windows::Win32::System::Com::StructuredStorage::PROPVARIANT_0 {
                Anonymous: ManuallyDrop::<PROPVARIANT_0_0>::new(PROPVARIANT_0_0 {
                    vt: VARENUM::default(),
                    wReserved1: u16::default(),
                    wReserved2: u16::default(),
                    wReserved3: u16::default(),
                    Anonymous: windows::Win32::System::Com::StructuredStorage::PROPVARIANT_0_0_0 {
                        pwszVal: PWSTR(leaked.as_mut_ptr()),
                    },
                }),
            },
        })
    }
}

pub struct FuzzedIPolicyConfigVista {
    speakers: Rc<RefCell<Vec<FuzzedSpeaker>>>,
    behaviour: FuzzedWindowsSpeakersSettingsApiBehaviour,
}

impl IPolicyConfigVista for FuzzedIPolicyConfigVista {
    unsafe fn set_default_endpoint(&mut self, device_id: PCWSTR, role: ERole) -> Result<()> {
        if self.behaviour.setting_the_default_speaker_fails {
            return Err(Error::new(E_FAIL, "Failed to set default speaker"));
        }

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
