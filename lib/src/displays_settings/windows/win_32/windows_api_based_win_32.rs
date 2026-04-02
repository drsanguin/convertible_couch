use windows::Win32::{
    Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
        DISPLAYCONFIG_TOPOLOGY_ID, DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes,
        QUERY_DISPLAY_CONFIG_FLAGS, QueryDisplayConfig, SET_DISPLAY_CONFIG_FLAGS, SetDisplayConfig,
    },
    Foundation::WIN32_ERROR,
};

use crate::{displays_settings::windows::win_32::Win32, trace_fn};

pub struct WindowsApiBasedWin32;

impl Win32 for WindowsApiBasedWin32 {
    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR {
        trace_fn!();

        unsafe {
            GetDisplayConfigBufferSizes(flags, numpatharrayelements, nummodeinfoarrayelements)
        }
    }

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR {
        trace_fn!();

        unsafe {
            QueryDisplayConfig(
                flags,
                numpatharrayelements,
                patharray,
                nummodeinfoarrayelements,
                modeinfoarray,
                currenttopologyid,
            )
        }
    }

    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        trace_fn!();

        unsafe { DisplayConfigGetDeviceInfo(requestpacket) }
    }

    unsafe fn set_display_config(
        &mut self,
        patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>,
        modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32 {
        unsafe { SetDisplayConfig(patharray, modeinfoarray, flags) }
    }
}
