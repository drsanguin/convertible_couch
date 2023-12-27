use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, HWND},
        Graphics::Gdi::{
            ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_TYPE,
            DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, ENUM_DISPLAY_SETTINGS_MODE,
        },
    },
};

pub trait Win32GraphicsGdi {
    unsafe fn change_display_settings_ex_w(
        &self,
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

pub struct Win32GraphicsGdiImpl;

impl Win32GraphicsGdi for Win32GraphicsGdiImpl {
    unsafe fn change_display_settings_ex_w(
        &self,
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
