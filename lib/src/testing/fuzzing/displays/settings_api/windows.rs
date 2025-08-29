use crate::{
    displays_settings::windows::win_32::Win32,
    testing::fuzzing::displays::{
        position::FuzzedDisplayPosition,
        settings_api::{
            behaviour::windows::FuzzedWindowsDisplaysSettingsApiBehaviour,
            FuzzedDisplaysSettingsApi,
        },
        video_output::FuzzedVideoOutput,
    },
};
use std::{collections::HashMap, ffi::c_void, mem::size_of};
use windows::{
    core::{BOOL, PCWSTR},
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID, QDC_ONLY_ACTIVE_PATHS,
            QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{ERROR_INVALID_PARAMETER, ERROR_SUCCESS, HWND, WIN32_ERROR},
        Graphics::Gdi::{
            CDS_NORESET, CDS_SET_PRIMARY, CDS_TYPE, CDS_UPDATEREGISTRY, DEVMODEW, DISPLAY_DEVICEW,
            DISP_CHANGE, DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_RESTART,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

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
    fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        let request_packet = requestpacket.cast::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();

        let size_of_displayconfig_target_device_name_as_usize =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();
        let size_of_displayconfig_target_device_name =
            u32::try_from(size_of_displayconfig_target_device_name_as_usize).unwrap();

        unsafe {
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
                .and_then(|video_output| {
                    let display = video_output.display.as_ref().unwrap();

                    if self.behaviour.getting_primary_display_name_fails
                        && display.position.is_positioned_at_origin()
                    {
                        return Some(1);
                    }

                    (*request_packet).monitorDevicePath = encode_utf16::<128>(&display.device_id);
                    (*request_packet).monitorFriendlyDeviceName = encode_utf16::<64>(&display.name);

                    Some(0)
                })
                .unwrap_or(1)
        }
    }

    fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR {
        if self
            .behaviour
            .get_display_config_buffer_sizes_error
            .is_some()
        {
            return self
                .behaviour
                .get_display_config_buffer_sizes_error
                .unwrap();
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

        unsafe {
            *numpatharrayelements = n_displays_as_u32;
            *nummodeinfoarrayelements = n_displays_as_u32 * 2;

            ERROR_SUCCESS
        }
    }

    fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        _numpatharrayelements: *mut u32,
        _patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR {
        if self.behaviour.query_display_config_error.is_some() {
            return self.behaviour.query_display_config_error.unwrap();
        }

        if flags != QDC_ONLY_ACTIVE_PATHS || currenttopologyid.is_some() {
            return ERROR_INVALID_PARAMETER;
        }

        unsafe {
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

    fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: Option<*const DEVMODEW>,
        hwnd: Option<HWND>,
        dwflags: CDS_TYPE,
        lparam: Option<*const c_void>,
    ) -> DISP_CHANGE {
        if lpszdevicename == PCWSTR::null()
            && lpdevmode.is_none()
            && hwnd == None
            && dwflags == CDS_TYPE::default()
            && lparam.is_none()
        {
            let displays_count = self
                .video_outputs
                .iter()
                .filter(|video_output| video_output.display.is_some())
                .count();

            let changes_to_commit_count = self.display_changes_to_commit.len();

            let displays_at_origin_count = self
                .display_changes_to_commit
                .values()
                .filter(|position| position.is_positioned_at_origin())
                .count();

            if changes_to_commit_count != displays_count || displays_at_origin_count != 1 {
                return DISP_CHANGE_FAILED;
            }

            return match self.behaviour.commit_display_settings_changes_error {
                Some(change_display_settings_error) => change_display_settings_error,
                _ => {
                    for (device_name, position) in self.display_changes_to_commit.iter() {
                        let video_output = self
                            .video_outputs
                            .iter_mut()
                            .find(|video_output| video_output.device_name == *device_name)
                            .unwrap();

                        let display = video_output.display.as_mut().unwrap();

                        display.position = *position;
                        display.primary = position.is_positioned_at_origin();
                    }

                    self.display_changes_to_commit.clear();

                    DISP_CHANGE_SUCCESSFUL
                }
            };
        }

        unsafe {
            let device_name = String::from_utf16(&lpszdevicename.as_wide()).unwrap();

            if self
                .behaviour
                .change_display_settings_error
                .is_some_and(|disp_change| disp_change != DISP_CHANGE_RESTART)
            {
                return self.behaviour.change_display_settings_error.unwrap();
            }

            return self
                .video_outputs
                .iter()
                .find(|video_output| video_output.device_name == device_name)
                .and_then(|video_output| video_output.display.clone())
                .and_then(|display| {
                    lpdevmode.and_then(|graphic_mode| {
                        Some((
                            display,
                            FuzzedDisplayPosition {
                                x: (*graphic_mode).Anonymous1.Anonymous2.dmPosition.x,
                                y: (*graphic_mode).Anonymous1.Anonymous2.dmPosition.y,
                            },
                        ))
                    })
                })
                .and_then(|(display, position)| {
                    if hwnd != None
                        || lparam.is_some()
                        || (dwflags & CDS_UPDATEREGISTRY == CDS_TYPE::default())
                        || (dwflags & CDS_NORESET == CDS_TYPE::default())
                        || (position.is_positioned_at_origin()
                            && (dwflags & CDS_SET_PRIMARY == CDS_TYPE::default()
                                || display.primary))
                        || (!position.is_positioned_at_origin()
                            && (dwflags & CDS_SET_PRIMARY == CDS_SET_PRIMARY))
                    {
                        return Some(DISP_CHANGE_BADPARAM);
                    }

                    self.display_changes_to_commit.insert(device_name, position);

                    let disp_change = if self.behaviour.change_display_settings_error.is_some() {
                        DISP_CHANGE_RESTART
                    } else {
                        DISP_CHANGE_SUCCESSFUL
                    };

                    Some(disp_change)
                })
                .unwrap_or(DISP_CHANGE_BADPARAM);
        }
    }

    fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL {
        // Iterating though video outputs
        if lpdevice == PCWSTR::null() {
            let video_output_index = usize::try_from(idevnum).unwrap();

            if self.video_outputs.is_empty()
                || dwflags != EDD_GET_DEVICE_INTERFACE_NAME
                || video_output_index > self.video_outputs.len() - 1
                || (unsafe { *lpdisplaydevice }).cb == 0
            {
                return BOOL(0);
            }

            let video_output = &self.video_outputs[video_output_index];
            let device_name = encode_utf16::<32>(&video_output.device_name);

            unsafe {
                (*lpdisplaydevice).DeviceName = device_name;

                BOOL(1)
            }
        }
        // Iterating though displays
        else {
            if dwflags != EDD_GET_DEVICE_INTERFACE_NAME || idevnum != 0 {
                return BOOL(0);
            }

            unsafe {
                let device_name = String::from_utf16(&lpdevice.as_wide()).unwrap();

                self.video_outputs
                    .iter()
                    .find(|video_output| video_output.device_name == device_name)
                    .and_then(|video_output| video_output.display.clone())
                    .and_then(|display| {
                        let device_id = encode_utf16::<128>(&display.device_id);

                        (*lpdisplaydevice).DeviceID = device_id;

                        Some(BOOL(1))
                    })
                    .unwrap_or(BOOL(0))
            }
        }
    }

    fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL {
        if imodenum != ENUM_CURRENT_SETTINGS || (unsafe { *lpdevmode }).dmSize == 0 {
            return BOOL(0);
        }

        unsafe {
            let device_name = String::from_utf16(&lpszdevicename.as_wide()).unwrap();

            self.video_outputs
                .iter()
                .find(|video_output| video_output.device_name == device_name)
                .and_then(|video_output| video_output.display.clone())
                .and_then(|display| {
                    if self
                        .behaviour
                        .display_not_possible_to_enum_display_settings_on
                        .as_ref()
                        .is_some_and(|display_not_possible_to_enum_display_settings_on| {
                            display.name == *display_not_possible_to_enum_display_settings_on
                        })
                    {
                        return Some(BOOL(0));
                    }

                    (*lpdevmode).Anonymous1.Anonymous2.dmPosition.x = display.position.x;
                    (*lpdevmode).Anonymous1.Anonymous2.dmPosition.y = display.position.y;

                    Some(BOOL(1))
                })
                .unwrap_or(BOOL(0))
        }
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
