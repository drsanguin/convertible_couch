use crate::arrangements::fuzzing::displays::{
    settings_api::{
        FuzzedDisplaysSettingsApi, behaviour::windows::FuzzedWindowsDisplaysSettingsApiBehaviour,
    },
    video_output::FuzzedVideoOutput,
};
use convertible_couch_lib::displays_settings::windows::win_32::Win32;
use std::collections::HashMap;
use windows::Win32::{
    Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_0,
        DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET,
        DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_PATH_SOURCE_INFO, DISPLAYCONFIG_PATH_SOURCE_INFO_0,
        DISPLAYCONFIG_PATH_TARGET_INFO, DISPLAYCONFIG_PATH_TARGET_INFO_0,
        DISPLAYCONFIG_SOURCE_MODE, DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID,
        QDC_ONLY_ACTIVE_PATHS, QUERY_DISPLAY_CONFIG_FLAGS, SDC_ALLOW_CHANGES, SDC_APPLY,
        SDC_SAVE_TO_DATABASE, SDC_USE_SUPPLIED_DISPLAY_CONFIG, SET_DISPLAY_CONFIG_FLAGS,
    },
    Foundation::{ERROR_INVALID_PARAMETER, ERROR_SUCCESS, LUID, POINTL, WIN32_ERROR},
};

#[derive(Clone, Default)]
pub struct FuzzedWin32 {
    patharray: Vec<DISPLAYCONFIG_PATH_INFO>,
    modeinfoarray: Vec<DISPLAYCONFIG_MODE_INFO>,
    displays_names: HashMap<(i32, u32, u32), String>,
    behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
}

impl FuzzedDisplaysSettingsApi for FuzzedWin32 {
    fn new(
        video_outputs: Vec<FuzzedVideoOutput>,
        behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
    ) -> Self {
        let mut patharray: Vec<DISPLAYCONFIG_PATH_INFO> = Vec::new();
        let mut modeinfoarray: Vec<DISPLAYCONFIG_MODE_INFO> = Vec::new();
        let mut displays_names = HashMap::new();

        let adapter_id = LUID {
            LowPart: 62504,
            HighPart: 0,
        };

        let displays = video_outputs
            .iter()
            .filter_map(|video_output| match &video_output.display {
                Some(display) => Some(display),
                None => None,
            })
            .collect::<Vec<_>>();

        for (i, display) in displays.iter().enumerate() {
            patharray.push(DISPLAYCONFIG_PATH_INFO {
                sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO {
                    adapterId: adapter_id,
                    id: i as u32,
                    Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
                        modeInfoIdx: (modeinfoarray.len() + 1) as u32,
                    },
                    ..Default::default()
                },
                targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO {
                    adapterId: adapter_id,
                    id: display.config_mode_info_id,
                    Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0 {
                        modeInfoIdx: modeinfoarray.len() as u32,
                    },
                    ..Default::default()
                },
                ..Default::default()
            });

            modeinfoarray.push(DISPLAYCONFIG_MODE_INFO {
                infoType: DISPLAYCONFIG_MODE_INFO_TYPE_TARGET,
                id: display.config_mode_info_id,
                adapterId: adapter_id,
                ..Default::default()
            });

            modeinfoarray.push(
                DISPLAYCONFIG_MODE_INFO {
                    infoType: DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE,
                    id: 0,
                    adapterId: adapter_id,
                    Anonymous: DISPLAYCONFIG_MODE_INFO_0 {
                        sourceMode: DISPLAYCONFIG_SOURCE_MODE {
                            width: display.resolution.width,
                            height: display.resolution.height,
                            position: POINTL {
                                x: display.position.x,
                                y: display.position.y,
                            },
                            ..Default::default()
                        },
                    },
                }, // ..Default::default()
            );

            displays_names.insert(
                (
                    adapter_id.HighPart,
                    adapter_id.LowPart,
                    display.config_mode_info_id,
                ),
                display.name.clone(),
            );
        }

        Self {
            patharray,
            modeinfoarray,
            displays_names,
            behaviour,
        }
    }
}

impl Win32 for FuzzedWin32 {
    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR {
        if !flags.contains(QDC_ONLY_ACTIVE_PATHS) {
            return ERROR_INVALID_PARAMETER;
        }

        unsafe {
            *numpatharrayelements = self.patharray.len() as u32;
            *nummodeinfoarrayelements = self.modeinfoarray.len() as u32;
        }

        ERROR_SUCCESS
    }

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR {
        if !flags.contains(QDC_ONLY_ACTIVE_PATHS) || currenttopologyid.is_some() {
            return ERROR_INVALID_PARAMETER;
        }

        for i in 0..unsafe { *numpatharrayelements } {
            unsafe {
                *patharray.add(i as usize) = self.patharray[i as usize].clone();
            }
        }

        for i in 0..unsafe { *nummodeinfoarrayelements } {
            unsafe {
                *modeinfoarray.add(i as usize) = self.modeinfoarray[i as usize].clone();
            }
        }

        ERROR_SUCCESS
    }

    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        let request_packet = requestpacket.cast::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();

        let name = self
            .displays_names
            .get(&(
                unsafe { (*request_packet).header.adapterId.HighPart },
                unsafe { (*request_packet).header.adapterId.LowPart },
                unsafe { (*request_packet).header.id },
            ))
            .unwrap();

        let monitor_friendly_device_name = encode_utf16::<64>(&name);

        unsafe { (*request_packet).monitorFriendlyDeviceName = monitor_friendly_device_name };

        ERROR_SUCCESS.0 as i32
    }

    unsafe fn set_display_config(
        &mut self,
        patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>,
        modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32 {
        if !flags.contains(SDC_APPLY)
            || !flags.contains(SDC_USE_SUPPLIED_DISPLAY_CONFIG)
            || !flags.contains(SDC_ALLOW_CHANGES)
            || !flags.contains(SDC_SAVE_TO_DATABASE)
        {
            return ERROR_INVALID_PARAMETER.0 as i32;
        }

        if let Some(paths) = patharray
            && let Some(modes) = modeinfoarray
        {
            self.patharray = paths.to_vec();
            self.modeinfoarray = modes.to_vec();

            return ERROR_SUCCESS.0 as i32;
        }

        ERROR_INVALID_PARAMETER.0 as i32
    }
}

fn encode_utf16<const T: usize>(string: &str) -> [u16; T] {
    let mut bytes = [0; T];

    string
        .encode_utf16()
        .enumerate()
        .take(T)
        .for_each(|(index, byte)| bytes[index] = byte);

    bytes
}
