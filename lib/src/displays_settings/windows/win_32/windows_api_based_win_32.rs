use windows::{
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TOPOLOGY_ID, DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes,
            QUERY_DISPLAY_CONFIG_FLAGS, QueryDisplayConfig,
        },
        Foundation::{HWND, WIN32_ERROR},
        Graphics::Gdi::{
            CDS_TYPE, ChangeDisplaySettingsExW, DEVMODEW, DISP_CHANGE, DISPLAY_DEVICEW,
            ENUM_DISPLAY_SETTINGS_MODE, EnumDisplayDevicesW, EnumDisplaySettingsW,
        },
    },
    core::{BOOL, PCWSTR},
};

use crate::{displays_settings::windows::win_32::Win32, trace_fn};

pub struct WindowsApiBasedWin32;

impl Win32 for WindowsApiBasedWin32 {
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        trace_fn!();
        unsafe { DisplayConfigGetDeviceInfo(requestpacket) }
    }

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

    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: core::option::Option<*const DEVMODEW>,
        hwnd: Option<HWND>,
        dwflags: CDS_TYPE,
        lparam: core::option::Option<*const core::ffi::c_void>,
    ) -> DISP_CHANGE {
        trace_fn!();
        unsafe { ChangeDisplaySettingsExW(lpszdevicename, lpdevmode, hwnd, dwflags, lparam) }
    }

    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL {
        trace_fn!();
        unsafe { EnumDisplayDevicesW(lpdevice, idevnum, lpdisplaydevice, dwflags) }
    }

    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL {
        trace_fn!();
        unsafe { EnumDisplaySettingsW(lpszdevicename, imodenum, lpdevmode) }
    }
}
