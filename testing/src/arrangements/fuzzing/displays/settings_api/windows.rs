use crate::arrangements::fuzzing::displays::{
    display_name,
    position::FuzzedDisplayPosition,
    settings_api::{
        FuzzedDisplaysSettingsApi, behaviour::windows::FuzzedWindowsDisplaysSettingsApiBehaviour,
    },
    video_output::FuzzedVideoOutput,
};
use convertible_couch_lib::displays_settings::windows::win_32::Win32;
use std::{collections::HashMap, mem::size_of};
use windows::Win32::{
    Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
        DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_0, DISPLAYCONFIG_MODE_INFO_TYPE,
        DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET,
        DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_PATH_SOURCE_INFO, DISPLAYCONFIG_PATH_SOURCE_INFO_0,
        DISPLAYCONFIG_PATH_TARGET_INFO, DISPLAYCONFIG_PATH_TARGET_INFO_0,
        DISPLAYCONFIG_SOURCE_MODE, DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID,
        QDC_ONLY_ACTIVE_PATHS, QUERY_DISPLAY_CONFIG_FLAGS, SET_DISPLAY_CONFIG_FLAGS,
    },
    Foundation::{ERROR_INVALID_PARAMETER, ERROR_SUCCESS, LUID, POINTL, WIN32_ERROR},
};

#[derive(Clone, Default)]
pub struct FuzzedWin32_2 {
    patharray: Vec<DISPLAYCONFIG_PATH_INFO>,
    modeinfoarray: Vec<DISPLAYCONFIG_MODE_INFO>,
    display_names: HashMap<u32, String>,
    behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
}

impl FuzzedDisplaysSettingsApi for FuzzedWin32_2 {
    fn new(
        video_outputs: Vec<FuzzedVideoOutput>,
        behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
    ) -> Self {
        let mut patharray: Vec<DISPLAYCONFIG_PATH_INFO> = Vec::new();
        let mut modeinfoarray: Vec<DISPLAYCONFIG_MODE_INFO> = Vec::new();
        let mut display_names = HashMap::new();

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

            display_names.insert(display.config_mode_info_id, display.name.clone());
        }

        Self {
            patharray,
            modeinfoarray,
            display_names,
            behaviour,
        }
    }
}

impl Win32 for FuzzedWin32_2 {
    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR {
        todo!()
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
        todo!()
    }

    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        todo!()
    }

    unsafe fn set_display_config(
        &mut self,
        patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>,
        modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32 {
        todo!()
    }
}

#[derive(Clone, Default)]
pub struct FuzzedWin32 {
    video_outputs: Vec<FuzzedVideoOutput>,
    display_changes_to_commit: HashMap<String, FuzzedDisplayPosition>,
    behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
}

impl FuzzedDisplaysSettingsApi for FuzzedWin32 {
    fn new(
        video_outputs: Vec<FuzzedVideoOutput>,
        behaviour: FuzzedWindowsDisplaysSettingsApiBehaviour,
    ) -> Self {
        let n_display = video_outputs
            .iter()
            .filter(|video_output| video_output.display.is_some())
            .count();

        Self {
            video_outputs,
            display_changes_to_commit: HashMap::with_capacity(n_display),
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
        unsafe {
            if let Some(get_display_config_buffer_sizes_error) =
                self.behaviour.get_display_config_buffer_sizes_error
            {
                return get_display_config_buffer_sizes_error;
            }

            if flags != QDC_ONLY_ACTIVE_PATHS {
                return ERROR_INVALID_PARAMETER;
            }

            let n_displays = self
                .video_outputs
                .iter()
                .filter(|video_output| video_output.display.is_some())
                .count();

            let n_displays_as_u32 = u32::try_from(n_displays).unwrap();

            *numpatharrayelements = n_displays_as_u32;
            *nummodeinfoarrayelements = n_displays_as_u32 * 2;

            ERROR_SUCCESS
        }
    }

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        _numpatharrayelements: *mut u32,
        _patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR {
        unsafe {
            if let Some(query_display_config_error) = self.behaviour.query_display_config_error {
                return query_display_config_error;
            }

            if flags != QDC_ONLY_ACTIVE_PATHS || currenttopologyid.is_some() {
                return ERROR_INVALID_PARAMETER;
            }

            let mode_informations_size = usize::try_from(*nummodeinfoarrayelements).unwrap();

            for i in 0..mode_informations_size {
                let mode_information = modeinfoarray.add(i);

                if i % 2 != 0 {
                    continue;
                }

                match self
                    .video_outputs
                    .iter()
                    .filter_map(|video_output| match &video_output.display {
                        Some(display) => Some(display),
                        None => None,
                    })
                    .nth(i / 2)
                {
                    Some(display) => {
                        (*mode_information).infoType = DISPLAYCONFIG_MODE_INFO_TYPE_TARGET;
                        (*mode_information).id = display.config_mode_info_id;
                    }
                    None => return ERROR_INVALID_PARAMETER,
                }
            }

            ERROR_SUCCESS
        }
    }

    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        unsafe {
            let request_packet = requestpacket.cast::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();

            let size_of_displayconfig_target_device_name_as_usize =
                size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();
            let size_of_displayconfig_target_device_name =
                u32::try_from(size_of_displayconfig_target_device_name_as_usize).unwrap();

            if (*request_packet).header.r#type != DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME
                || (*request_packet).header.size != size_of_displayconfig_target_device_name
            {
                return 1;
            }

            let config_mode_info_id = (*request_packet).header.id;

            self.video_outputs
                .iter()
                .find(|video_output| {
                    if video_output.display.is_none() {
                        return false;
                    }

                    match &video_output.display {
                        Some(display) => display.config_mode_info_id == config_mode_info_id,
                        None => false,
                    }
                })
                .map(|video_output| {
                    let display = video_output.display.as_ref().unwrap();

                    if self.behaviour.getting_primary_display_name_fails
                        && display.position.is_positioned_at_origin()
                    {
                        return 1;
                    }

                    (*request_packet).monitorDevicePath = encode_utf16::<128>(&display.device_id);
                    (*request_packet).monitorFriendlyDeviceName = encode_utf16::<64>(&display.name);

                    0
                })
                .unwrap_or(1)
        }
    }

    unsafe fn set_display_config(
        &mut self,
        patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>,
        modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32 {
        todo!()
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
