use std::mem::size_of;

use convertible_couch::display_settings::Win32DevicesDisplay;
use windows::{
    core::Error,
    Win32::Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
        DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
        DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID, QDC_ONLY_ACTIVE_PATHS,
        QUERY_DISPLAY_CONFIG_FLAGS,
    },
};

use crate::common::utils::encode_utf16;

use super::video_output::FuzzedVideoOutput;

pub struct FuzzedWin32DevicesDisplay {
    pub video_outputs: Vec<FuzzedVideoOutput>,
}

impl Win32DevicesDisplay for FuzzedWin32DevicesDisplay {
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

        let video_output_result = self.video_outputs.iter().find(|x| {
            if x.monitor.is_none() {
                return false;
            }

            match &x.monitor {
                Some(monitor) => monitor.config_mode_info_id == config_mode_info_id,
                None => false,
            }
        });

        match video_output_result {
            Some(video_output) => {
                let monitor = video_output.monitor.as_ref().unwrap();

                (*request_packet).monitorDevicePath = encode_utf16::<128>(&monitor.id);
                (*request_packet).monitorFriendlyDeviceName = encode_utf16::<64>(&monitor.name);

                0
            }
            None => 1,
        }
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
            .filter(|v| v.monitor.is_some())
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
}
