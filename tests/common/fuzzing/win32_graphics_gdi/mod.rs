use std::ffi::c_void;

use convertible_couch::display_settings::Win32GraphicsGdi;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, HWND},
        Graphics::Gdi::{
            CDS_SET_PRIMARY, CDS_TYPE, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, DISP_CHANGE_RESTART,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

use crate::common::utils::encode_utf16;

use super::video_output::FuzzedVideoOutput;

pub struct FuzzedWin32GraphicsGdi {
    pub video_outputs: Vec<FuzzedVideoOutput>,
    pub reboot_required: bool,
}

impl Win32GraphicsGdi for FuzzedWin32GraphicsGdi {
    unsafe fn change_display_settings_ex_w(
        &self,
        _lpszdevicename: PCWSTR,
        _lpdevmode: Option<*const DEVMODEW>,
        _hwnd: HWND,
        _dwflags: CDS_TYPE,
        _lparam: Option<*const c_void>,
    ) -> DISP_CHANGE {
        if _dwflags & CDS_SET_PRIMARY == CDS_TYPE::default() && self.reboot_required {
            return DISP_CHANGE_RESTART;
        }

        DISP_CHANGE_SUCCESSFUL
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
            let device_name = encode_utf16::<32>(&video_output.id);

            (*lpdisplaydevice).DeviceName = device_name;

            BOOL(1)
        }
        // Iterating though monitors
        else {
            if dwflags != EDD_GET_DEVICE_INTERFACE_NAME || idevnum != 0 {
                return BOOL(0);
            }

            let video_output_id = String::from_utf16(&lpdevice.as_wide()).unwrap();

            self.video_outputs
                .iter()
                .find(|x| x.id == video_output_id)
                .and_then(|video_output| video_output.monitor.clone())
                .and_then(|monitor| {
                    let device_id = encode_utf16::<128>(&monitor.id);

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

        let video_output_id = String::from_utf16(&lpszdevicename.as_wide()).unwrap();

        self.video_outputs
            .iter()
            .find(|x| x.id == video_output_id)
            .and_then(|video_output| video_output.monitor.clone())
            .and_then(|monitor| {
                (*lpdevmode).Anonymous1.Anonymous2.dmPosition.x = monitor.position.x;
                (*lpdevmode).Anonymous1.Anonymous2.dmPosition.y = monitor.position.y;

                Some(BOOL(1))
            })
            .unwrap_or(BOOL(0))
    }
}
