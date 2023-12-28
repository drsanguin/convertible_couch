use windows::{
    core::Error,
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TOPOLOGY_ID, QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{BOOL, HWND},
        Graphics::Gdi::{
            ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_TYPE,
            DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, ENUM_DISPLAY_SETTINGS_MODE,
        },
    },
};

pub trait Win32 {
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32;

    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> Result<(), Error>;

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> Result<(), Error>;

    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: ::core::option::Option<*const DEVMODEW>,
        hwnd: HWND,
        dwflags: CDS_TYPE,
        lparam: ::core::option::Option<*const ::core::ffi::c_void>,
    ) -> DISP_CHANGE;

    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL;

    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL;
}

pub struct Win32Impl;

impl Win32 for Win32Impl {
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32 {
        DisplayConfigGetDeviceInfo(requestpacket)
    }

    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> Result<(), Error> {
        GetDisplayConfigBufferSizes(flags, numpatharrayelements, nummodeinfoarrayelements)
    }

    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> Result<(), Error> {
        QueryDisplayConfig(
            flags,
            numpatharrayelements,
            patharray,
            nummodeinfoarrayelements,
            modeinfoarray,
            currenttopologyid,
        )
    }

    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: core::option::Option<*const DEVMODEW>,
        hwnd: HWND,
        dwflags: CDS_TYPE,
        lparam: core::option::Option<*const core::ffi::c_void>,
    ) -> DISP_CHANGE {
        ChangeDisplaySettingsExW(lpszdevicename, lpdevmode, hwnd, dwflags, lparam)
    }

    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL {
        EnumDisplayDevicesW(lpdevice, idevnum, lpdisplaydevice, dwflags)
    }

    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL {
        EnumDisplaySettingsW(lpszdevicename, imodenum, lpdevmode)
    }
}
