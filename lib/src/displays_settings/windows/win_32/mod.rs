use windows::{
    core::{BOOL, PCWSTR},
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TOPOLOGY_ID, QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{HWND, WIN32_ERROR},
        Graphics::Gdi::{
            CDS_TYPE, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, ENUM_DISPLAY_SETTINGS_MODE,
        },
    },
};

pub mod windows_api_based_win_32;

pub trait Win32 {
    fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32;

    fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR;

    fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR;

    fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: ::core::option::Option<*const DEVMODEW>,
        hwnd: Option<HWND>,
        dwflags: CDS_TYPE,
        lparam: ::core::option::Option<*const ::core::ffi::c_void>,
    ) -> DISP_CHANGE;

    fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL;

    fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL;
}
