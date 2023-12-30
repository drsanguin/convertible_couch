use std::{ffi::c_void, mem::size_of};

use convertible_couch_lib::display_settings::Win32;
use windows::{
    core::Error,
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID, QDC_ONLY_ACTIVE_PATHS,
            QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{BOOL, HWND},
        Graphics::Gdi::{
            CDS_SET_PRIMARY, CDS_TYPE, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE,
            DISP_CHANGE_BADPARAM, DISP_CHANGE_RESTART, DISP_CHANGE_SUCCESSFUL,
            ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

use crate::utils::encode_utf16;

use super::{position::FuzzedMonitorPosition, video_output::FuzzedVideoOutput};

pub struct FuzzedWin32 {
    pub video_outputs: Vec<FuzzedVideoOutput>,
    pub reboot_required: bool,
}

impl FuzzedWin32 {
    pub fn new(video_outputs: Vec<FuzzedVideoOutput>, reboot_required: bool) -> Self {
        Self {
            video_outputs,
            reboot_required,
        }
    }
}

impl Win32 for FuzzedWin32 {
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
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
                if video_output.monitor.is_none() {
                    return false;
                }

                match &video_output.monitor {
                    Some(monitor) => monitor.config_mode_info_id == config_mode_info_id,
                    None => false,
                }
            })
            .and_then(|video_output| {
                let monitor = video_output.monitor.as_ref().unwrap();

                (*request_packet).monitorDevicePath = encode_utf16::<128>(&monitor.device_id);
                (*request_packet).monitorFriendlyDeviceName = encode_utf16::<64>(&monitor.name);

                Some(0)
            })
            .unwrap_or(1)
    }

    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> Result<(), Error> {
        if flags != QDC_ONLY_ACTIVE_PATHS {
            return Err(Error::from_win32());
        }

        let n_monitors = self
            .video_outputs
            .iter()
            .filter(|video_output| video_output.monitor.is_some())
            .count();

        let n_monitors_as_u32 = u32::try_from(n_monitors).unwrap();

        *numpatharrayelements = n_monitors_as_u32;
        *nummodeinfoarrayelements = n_monitors_as_u32 * 2;

        Ok(())
    }

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        _numpatharrayelements: *mut u32,
        _patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> Result<(), Error> {
        if flags != QDC_ONLY_ACTIVE_PATHS || currenttopologyid.is_some() {
            return Err(Error::from_win32());
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
                .filter_map(|video_output| match &video_output.monitor {
                    Some(monitor) => Some(monitor),
                    None => None,
                })
                .nth(i / 2)
            {
                Some(monitor) => {
                    (*mode_information).infoType = DISPLAYCONFIG_MODE_INFO_TYPE_TARGET;
                    (*mode_information).id = monitor.config_mode_info_id;
                }
                None => return Err(Error::from_win32()),
            }
        }

        Ok(())
    }

    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: Option<*const DEVMODEW>,
        hwnd: HWND,
        dwflags: CDS_TYPE,
        lparam: Option<*const c_void>,
    ) -> DISP_CHANGE {
        if lpszdevicename == PCWSTR::null()
            && lpdevmode.is_none()
            && hwnd == HWND::default()
            && dwflags == CDS_TYPE::default()
            && lparam.is_none()
        {
            return match self.reboot_required {
                true => DISP_CHANGE_RESTART,
                false => DISP_CHANGE_SUCCESSFUL,
            };
        }

        let device_name = String::from_utf16(&lpszdevicename.as_wide()).unwrap();

        self.video_outputs
            .iter()
            .find(|video_output| video_output.device_name == device_name)
            .and_then(|video_output| video_output.monitor.clone())
            .and_then(|monitor| {
                lpdevmode.and_then(|graphic_mode| {
                    Some((
                        monitor,
                        FuzzedMonitorPosition {
                            x: (*graphic_mode).Anonymous1.Anonymous2.dmPosition.x,
                            y: (*graphic_mode).Anonymous1.Anonymous2.dmPosition.y,
                        },
                    ))
                })
            })
            .and_then(|(monitor, position)| {
                if hwnd != HWND::default()
                    || lparam.is_some()
                    || (position.x == 0
                        && position.y == 0
                        && (dwflags & CDS_SET_PRIMARY == CDS_TYPE::default() || monitor.primary))
                {
                    return Some(DISP_CHANGE_BADPARAM);
                }

                Some(DISP_CHANGE_SUCCESSFUL)
            })
            .unwrap_or(DISP_CHANGE_BADPARAM)
    }

    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL {
        // Iterating though video outputs
        if lpdevice == PCWSTR::null() {
            let video_output_index = usize::try_from(idevnum).unwrap();

            if dwflags != EDD_GET_DEVICE_INTERFACE_NAME
                || video_output_index > self.video_outputs.len() - 1
            {
                return BOOL(0);
            }

            let video_output = &self.video_outputs[video_output_index];
            let device_name = encode_utf16::<32>(&video_output.device_name);

            (*lpdisplaydevice).DeviceName = device_name;

            BOOL(1)
        }
        // Iterating though monitors
        else {
            if dwflags != EDD_GET_DEVICE_INTERFACE_NAME || idevnum != 0 {
                return BOOL(0);
            }

            let device_name = String::from_utf16(&lpdevice.as_wide()).unwrap();

            self.video_outputs
                .iter()
                .find(|video_output| video_output.device_name == device_name)
                .and_then(|video_output| video_output.monitor.clone())
                .and_then(|monitor| {
                    let device_id = encode_utf16::<128>(&monitor.device_id);

                    (*lpdisplaydevice).DeviceID = device_id;

                    Some(BOOL(1))
                })
                .unwrap_or(BOOL(0))
        }
    }

    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL {
        if imodenum != ENUM_CURRENT_SETTINGS {
            return BOOL(0);
        }

        let device_name = String::from_utf16(&lpszdevicename.as_wide()).unwrap();

        self.video_outputs
            .iter()
            .find(|video_output| video_output.device_name == device_name)
            .and_then(|video_output| video_output.monitor.clone())
            .and_then(|monitor| {
                (*lpdevmode).Anonymous1.Anonymous2.dmPosition.x = monitor.position.x;
                (*lpdevmode).Anonymous1.Anonymous2.dmPosition.y = monitor.position.y;

                Some(BOOL(1))
            })
            .unwrap_or(BOOL(0))
    }
}