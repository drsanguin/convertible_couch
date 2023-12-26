use std::{ffi::c_void, mem::size_of};

use convertible_couch::display_settings::{Win32DevicesDisplay, Win32GraphicsGdi};
use windows::{
    core::{Error, PCWSTR},
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, DISPLAYCONFIG_TOPOLOGY_ID, QDC_ONLY_ACTIVE_PATHS,
            QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{BOOL, HWND},
        Graphics::Gdi::{
            CDS_SET_PRIMARY, CDS_TYPE, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, DISP_CHANGE_RESTART,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

pub struct FuzzedComputer {
    pub win32_devices_display: FuzzedWin32DevicesDisplay,
    pub win32_graphics_gdi: FuzzedWin32GraphicsGdi,
    pub primary_monitor: String,
    pub secondary_monitor: String,
}

pub struct Fuzzer {}

impl Fuzzer {
    pub fn generate_a_computer(&self) -> ComputerFuzzer {
        return ComputerFuzzer::new();
    }
}

#[derive(Clone)]
pub struct FuzzedVideoOutput {
    pub id: String,
    pub monitor: Option<FuzzedMonitor>,
}

impl FuzzedVideoOutput {
    pub fn new(index: u32, monitor: Option<FuzzedMonitor>) -> Self {
        let id = match monitor {
            Some(_) => format!(r"\\.\DISPLAY{0}\Monitor0", index),
            None => format!(r"\\.\DISPLAY{0}", index),
        };

        Self { id, monitor }
    }
}

#[derive(Clone)]
pub struct FuzzedMonitor {
    pub name: String,
    pub primary: bool,
    pub config_mode_info_id: u32,
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub position: (i32, i32),
}

impl FuzzedMonitor {
    pub fn new(index: u32, name: &str, primary: bool, width: u32, height: u32) -> Self {
        let config_mode_info_id = 28927 + index;
        let id = format!(
            r"\\?\DISPLAY#GSM59F2#5&de985f&0&UID{config_mode_info_id}#{{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7}}"
        );

        Self {
            name: name.to_string(),
            primary,
            config_mode_info_id,
            id,
            width,
            height,
            position: if primary {
                (0, 0)
            } else {
                (-i32::try_from(width).unwrap(), 0)
            },
        }
    }
}

pub struct ComputerFuzzer {
    video_outputs: Vec<FuzzedVideoOutput>,
    reboot_required: bool,
}

impl ComputerFuzzer {
    pub fn new() -> Self {
        Self {
            video_outputs: vec![],
            reboot_required: false,
        }
    }

    pub fn with_two_monitors_or_more(&mut self) -> &mut ComputerFuzzer {
        let monitor1 = FuzzedMonitor::new(1, "LG ULTRAWIDE", true, 2560, 1080);
        let monitor2 = FuzzedMonitor::new(2, "LG TV SSCR2", false, 4096, 2160);
        let monitor3 = FuzzedMonitor::new(3, "M227WD", false, 1920, 1080);

        let video_output_1 = FuzzedVideoOutput::new(1, Some(monitor1));
        let video_output_2 = FuzzedVideoOutput::new(2, Some(monitor2));
        let video_output_3 = FuzzedVideoOutput::new(3, Some(monitor3));
        let video_output_4 = FuzzedVideoOutput::new(4, None);

        self.video_outputs = vec![
            video_output_1,
            video_output_2,
            video_output_3,
            video_output_4,
        ];

        return self;
    }

    pub fn build_computer(&self) -> FuzzedComputer {
        let secondary_monitor = self
            .video_outputs
            .iter()
            .filter(|x| x.monitor.is_some())
            .map(|x| x.monitor.as_ref().unwrap())
            .find(|x| !x.primary)
            .unwrap()
            .name
            .clone();

        let primary_monitor = self
            .video_outputs
            .iter()
            .filter(|x| x.monitor.is_some())
            .map(|x| x.monitor.as_ref().unwrap())
            .find(|x| x.primary)
            .unwrap()
            .name
            .clone();

        let win32_devices_display = FuzzedWin32DevicesDisplay {
            video_outputs: self.video_outputs.clone(),
        };

        let win32_graphics_gdi = FuzzedWin32GraphicsGdi {
            video_outputs: self.video_outputs.clone(),
            reboot_required: self.reboot_required,
        };

        return FuzzedComputer {
            secondary_monitor,
            primary_monitor,
            win32_devices_display,
            win32_graphics_gdi,
        };
    }

    pub fn which_requires_reboot(&mut self) -> &mut ComputerFuzzer {
        self.reboot_required = true;

        return self;
    }
}

pub struct FuzzedWin32DevicesDisplay {
    video_outputs: Vec<FuzzedVideoOutput>,
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

            return match &x.monitor {
                Some(monitor) => monitor.config_mode_info_id == config_mode_info_id,
                None => false,
            };
        });

        return match video_output_result {
            Some(video_output) => {
                let monitor = video_output.monitor.as_ref().unwrap();

                (*request_packet).monitorDevicePath = encode_utf16::<128>(&monitor.id);
                (*request_packet).monitorFriendlyDeviceName = encode_utf16::<64>(&monitor.name);

                return 0;
            }
            None => 1,
        };
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

        return Ok(());
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

            (*mode_information).infoType = DISPLAYCONFIG_MODE_INFO_TYPE_TARGET;
            (*mode_information).id = self.video_outputs[i / 2]
                .monitor
                .as_ref()
                .unwrap()
                .config_mode_info_id;
        }

        return Ok(());
    }
}

pub struct FuzzedWin32GraphicsGdi {
    video_outputs: Vec<FuzzedVideoOutput>,
    reboot_required: bool,
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

        return DISP_CHANGE_SUCCESSFUL;
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

            return BOOL(1);
        }
        // Iterating though monitors
        else {
            if dwflags != EDD_GET_DEVICE_INTERFACE_NAME || idevnum != 0 {
                return BOOL(0);
            }

            let video_output_id = String::from_utf16(&lpdevice.as_wide()).unwrap();
            let video_output_option = &self.video_outputs.iter().find(|x| x.id == video_output_id);

            return match video_output_option {
                Some(video_output) => match &video_output.monitor {
                    Some(monitor) => {
                        let device_id = encode_utf16::<128>(&monitor.id);

                        (*lpdisplaydevice).DeviceID = device_id;

                        return BOOL(1);
                    }
                    None => BOOL(0),
                },
                None => BOOL(0),
            };
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
        let video_output_option = &self.video_outputs.iter().find(|x| x.id == video_output_id);

        return match video_output_option {
            Some(video_output) => match &video_output.monitor {
                Some(monitor) => {
                    (*lpdevmode).Anonymous1.Anonymous2.dmPosition.x = monitor.position.0;
                    (*lpdevmode).Anonymous1.Anonymous2.dmPosition.y = monitor.position.1;

                    return BOOL(1);
                }
                None => BOOL(0),
            },
            None => BOOL(0),
        };
    }
}

fn encode_utf16<const T: usize>(string: &str) -> [u16; T] {
    let mut bytes = [0; T];
    let string_as_utf16: Vec<u16> = string.encode_utf16().collect();

    for (pos, e) in string_as_utf16.iter().enumerate() {
        bytes[pos] = *e;
    }

    return bytes;
}
