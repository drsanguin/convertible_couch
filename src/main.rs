use std::{fmt, mem::size_of};

use windows::{
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::{ERROR_SUCCESS, HWND, WIN32_ERROR},
        Graphics::Gdi::{
            ChangeDisplaySettingsExW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_NORESET,
            CDS_SET_PRIMARY, CDS_TYPE, CDS_UPDATEREGISTRY, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE,
            DISP_CHANGE_BADDUALVIEW, DISP_CHANGE_BADFLAGS, DISP_CHANGE_BADMODE,
            DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_NOTUPDATED, DISP_CHANGE_RESTART,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

struct MonitorPosition {
    x: i32,
    y: i32,
}

impl fmt::Display for MonitorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let desktop_monitor_name = "LG ULTRAWIDE";
    let couch_monitor_name = "M227WD";

    unsafe {
        let primary_monitor_name = get_primary_monitor_name().unwrap();
        let new_primary_monitor_name = if primary_monitor_name == desktop_monitor_name {
            couch_monitor_name
        } else {
            desktop_monitor_name
        };
        let new_primary_monitor_current_position =
            get_monitor_position(new_primary_monitor_name).unwrap();
        let set_monitors_to_position_result =
            set_monitors_to_position(new_primary_monitor_current_position);

        match set_monitors_to_position_result {
            Ok(()) => (),
            Err(message) => eprint!("Failed because of {0}", message),
        }
    }
}

unsafe fn get_monitor_name(monitor_device_path: &str) -> Result<String, String> {
    let mut n_path_informations = u32::default();
    let mut n_mode_informations = u32::default();

    let get_display_config_buffer_sizes_result = GetDisplayConfigBufferSizes(
        QDC_ONLY_ACTIVE_PATHS,
        &mut n_path_informations,
        &mut n_mode_informations,
    );

    match get_display_config_buffer_sizes_result {
        Ok(_) => {
            let n_path_informations_as_usize = usize::try_from(n_path_informations).unwrap();
            let n_mode_informations_as_usize = usize::try_from(n_mode_informations).unwrap();

            let mut path_informations = vec![DISPLAYCONFIG_PATH_INFO::default(); n_path_informations_as_usize];
            let mut mode_informations = vec![DISPLAYCONFIG_MODE_INFO::default(); n_mode_informations_as_usize];

            let query_display_config_result = QueryDisplayConfig(
                QDC_ONLY_ACTIVE_PATHS,
                &mut n_path_informations,
                path_informations.as_mut_ptr(),
                &mut n_mode_informations,
                mode_informations.as_mut_ptr(),
                None
            );

            match query_display_config_result {
                Ok(_) => {
                    let size_of_displayconfig_target_device_name_as_usize = size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();
                    let size_of_displayconfig_target_device_name = u32::try_from(size_of_displayconfig_target_device_name_as_usize).unwrap();

                    for mode_information in mode_informations.into_iter() {
                        if mode_information.infoType != DISPLAYCONFIG_MODE_INFO_TYPE_TARGET {
                            continue;
                        }

                        let mut displayconfig_target_device_name = DISPLAYCONFIG_TARGET_DEVICE_NAME::default();
                        displayconfig_target_device_name.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                            r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                            size: size_of_displayconfig_target_device_name,
                            adapterId: mode_information.adapterId,
                            id: mode_information.id
                        };

                        let display_config_get_device_info_result_as_i32 = DisplayConfigGetDeviceInfo(&mut displayconfig_target_device_name.header);
                        let display_config_get_device_info_result_as_u32 = u32::try_from(display_config_get_device_info_result_as_i32).unwrap();
                        let display_config_get_device_info_result = WIN32_ERROR(display_config_get_device_info_result_as_u32);

                        match display_config_get_device_info_result {
                            ERROR_SUCCESS => {
                                let monitor_friendly_device_name = String::from_utf16(&displayconfig_target_device_name.monitorFriendlyDeviceName).unwrap();
                                let current_monitor_device_path = String::from_utf16(&displayconfig_target_device_name.monitorDevicePath).unwrap();
                                let current_monitor_device_path_trimed = current_monitor_device_path.trim_end_matches('\0');

                                if current_monitor_device_path_trimed == monitor_device_path {
                                    let monitor_friendly_device_name_trimed = monitor_friendly_device_name.trim_end_matches('\0');

                                    return Ok(String::from(monitor_friendly_device_name_trimed));
                                } else {
                                    continue;
                                }
                            },
                            error => return Err(format!("Failed to retrieve display configuration information about the device: {0}", error.0))
                        }
                    }
                },
                Err(error) => return Err(format!("Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {0}", error))
            }
        },
        Err(error) => return Err(format!("Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {0}", error))
    }

    return Err(format!(
        "Failed to retrieve the name of the monitor at the device path {0}",
        monitor_device_path
    ));
}

unsafe fn get_primary_monitor_name() -> Result<String, String> {
    let mut display_adapter_index = 0;
    let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
    let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

    loop {
        let mut display_adapter = DISPLAY_DEVICEW::default();
        display_adapter.cb = size_of_display_devicew;

        let is_success_display_adapter_as_win32_bool = EnumDisplayDevicesW(
            PCWSTR::null(),
            display_adapter_index,
            &mut display_adapter,
            EDD_GET_DEVICE_INTERFACE_NAME,
        );
        let is_success_display_adapter = is_success_display_adapter_as_win32_bool.as_bool();

        if is_success_display_adapter {
            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

            let is_success_display_device_as_win32_bool = EnumDisplayDevicesW(
                display_adapter_device_name,
                0,
                &mut display_device,
                EDD_GET_DEVICE_INTERFACE_NAME,
            );
            let is_success_display_device = is_success_display_device_as_win32_bool.as_bool();

            if is_success_display_device {
                let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
                let display_adapter_device_name =
                    PCWSTR::from_raw(display_adapter_device_name_as_ptr);

                let size_of_devmode_as_usize = size_of::<DEVMODEW>();
                let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

                let mut display_adapter_graphics_mode = DEVMODEW::default();
                display_adapter_graphics_mode.dmSize = size_of_devmode;

                let has_enum_display_settings_succeded_as_win35_bool = EnumDisplaySettingsW(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                );
                let has_enum_display_settings_succeded =
                    has_enum_display_settings_succeded_as_win35_bool.as_bool();

                if has_enum_display_settings_succeded {
                    if display_adapter_graphics_mode
                        .Anonymous1
                        .Anonymous2
                        .dmPosition
                        .x
                        == 0
                        && display_adapter_graphics_mode
                            .Anonymous1
                            .Anonymous2
                            .dmPosition
                            .y
                            == 0
                    {
                        let display_device_device_id =
                            String::from_utf16(&display_device.DeviceID).unwrap();
                        let display_device_device_id_trimed =
                            display_device_device_id.trim_end_matches('\0');
                        let current_monitor_name =
                            get_monitor_name(display_device_device_id_trimed).unwrap();

                        return Ok(current_monitor_name);
                    }
                } else {
                    eprintln!(
                        "Failed to enum display settings for display device {0}",
                        display_adapter_device_name.to_string().unwrap()
                    );
                }
            } else {
                eprintln!(
                    "Failed to retrieve display device informations from the display adapter {0}",
                    display_adapter_device_name.to_string().unwrap()
                );
            }
        } else {
            break;
        }

        display_adapter_index += 1;
    }

    return Err(String::from("Failed to retrieve the primary monitor"));
}

unsafe fn get_monitor_position(monitor_name: &str) -> Result<MonitorPosition, String> {
    let mut display_adapter_index = 0;
    let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
    let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

    loop {
        let mut display_adapter = DISPLAY_DEVICEW::default();
        display_adapter.cb = size_of_display_devicew;

        let is_success_display_adapter_as_win32_bool = EnumDisplayDevicesW(
            PCWSTR::null(),
            display_adapter_index,
            &mut display_adapter,
            EDD_GET_DEVICE_INTERFACE_NAME,
        );
        let is_success_display_adapter = is_success_display_adapter_as_win32_bool.as_bool();

        if is_success_display_adapter {
            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

            let is_success_display_device_as_win32_bool = EnumDisplayDevicesW(
                display_adapter_device_name,
                0,
                &mut display_device,
                EDD_GET_DEVICE_INTERFACE_NAME,
            );
            let is_success_display_device = is_success_display_device_as_win32_bool.as_bool();

            if is_success_display_device {
                let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
                let display_adapter_device_name =
                    PCWSTR::from_raw(display_adapter_device_name_as_ptr);

                let size_of_devmode_as_usize = size_of::<DEVMODEW>();
                let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

                let mut display_adapter_graphics_mode = DEVMODEW::default();
                display_adapter_graphics_mode.dmSize = size_of_devmode;

                let has_enum_display_settings_succeded_as_win35_bool = EnumDisplaySettingsW(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                );
                let has_enum_display_settings_succeded =
                    has_enum_display_settings_succeded_as_win35_bool.as_bool();

                if has_enum_display_settings_succeded {
                    let display_device_device_id =
                        String::from_utf16(&display_device.DeviceID).unwrap();
                    let display_device_device_id_trimed =
                        display_device_device_id.trim_end_matches('\0');
                    let current_monitor_name =
                        get_monitor_name(display_device_device_id_trimed).unwrap();

                    if current_monitor_name == monitor_name {
                        let monitor_position = MonitorPosition {
                            x: display_adapter_graphics_mode
                                .Anonymous1
                                .Anonymous2
                                .dmPosition
                                .x,
                            y: display_adapter_graphics_mode
                                .Anonymous1
                                .Anonymous2
                                .dmPosition
                                .y,
                        };

                        return Ok(monitor_position);
                    }
                } else {
                    eprintln!(
                        "Failed to enum display settings for display device {0}",
                        display_adapter_device_name.to_string().unwrap()
                    );
                }
            } else {
                eprintln!(
                    "Failed to retrieve display device informations from the display adapter {0}",
                    display_adapter_device_name.to_string().unwrap()
                );
            }
        } else {
            break;
        }

        display_adapter_index += 1;
    }

    return Err(format!(
        "Failed to retrieve the position of monitor {0}",
        monitor_name
    ));
}

fn map_disp_change_to_string(disp_change: DISP_CHANGE) -> String {
    match disp_change {
        DISP_CHANGE_BADDUALVIEW => String::from("The settings change was unsuccessful because the system is DualView capable. "),
        DISP_CHANGE_BADFLAGS => String::from("An invalid set of flags was passed in. "),
        DISP_CHANGE_BADMODE => String::from("The graphics mode is not supported."),
        DISP_CHANGE_BADPARAM => String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags."),
        DISP_CHANGE_FAILED => String::from("The display driver failed the specified graphics mode."),
        DISP_CHANGE_NOTUPDATED => String::from("Unable to write settings to the registry. "),
        DISP_CHANGE_RESTART => String::from("The computer must be restarted for the graphics mode to work."),
        _ => String::from("The settings change was successful.")
    }
}

unsafe fn set_monitors_to_position(position: MonitorPosition) -> Result<(), String> {
    let mut display_adapter_index = 0;
    let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
    let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

    loop {
        let mut display_adapter = DISPLAY_DEVICEW::default();
        display_adapter.cb = size_of_display_devicew;

        let is_success_display_adapter_as_win32_bool = EnumDisplayDevicesW(
            PCWSTR::null(),
            display_adapter_index,
            &mut display_adapter,
            EDD_GET_DEVICE_INTERFACE_NAME,
        );
        let is_success_display_adapter = is_success_display_adapter_as_win32_bool.as_bool();

        if is_success_display_adapter {
            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

            let is_success_display_device_as_win32_bool = EnumDisplayDevicesW(
                display_adapter_device_name,
                0,
                &mut display_device,
                EDD_GET_DEVICE_INTERFACE_NAME,
            );
            let is_success_display_device = is_success_display_device_as_win32_bool.as_bool();

            if is_success_display_device {
                let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
                let display_adapter_device_name =
                    PCWSTR::from_raw(display_adapter_device_name_as_ptr);

                let size_of_devmode_as_usize = size_of::<DEVMODEW>();
                let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

                let mut display_adapter_graphics_mode = DEVMODEW::default();
                display_adapter_graphics_mode.dmSize = size_of_devmode;

                let has_enum_display_settings_succeded_as_win35_bool = EnumDisplaySettingsW(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                );
                let has_enum_display_settings_succeded =
                    has_enum_display_settings_succeded_as_win35_bool.as_bool();

                if has_enum_display_settings_succeded {
                    display_adapter_graphics_mode
                        .Anonymous1
                        .Anonymous2
                        .dmPosition
                        .x -= position.x;
                    display_adapter_graphics_mode
                        .Anonymous1
                        .Anonymous2
                        .dmPosition
                        .y -= position.y;

                    let mut dwflags = CDS_UPDATEREGISTRY | CDS_NORESET;

                    if display_adapter_graphics_mode
                        .Anonymous1
                        .Anonymous2
                        .dmPosition
                        .x
                        == 0
                        && display_adapter_graphics_mode
                            .Anonymous1
                            .Anonymous2
                            .dmPosition
                            .y
                            == 0
                    {
                        dwflags |= CDS_SET_PRIMARY;
                    }

                    let change_display_settings_ex_result = ChangeDisplaySettingsExW(
                        display_adapter_device_name,
                        Some(&display_adapter_graphics_mode),
                        HWND::default(),
                        dwflags,
                        None,
                    );

                    match change_display_settings_ex_result {
                        DISP_CHANGE_SUCCESSFUL => {}
                        DISP_CHANGE_RESTART => eprintln!("The computer must be restarted"),
                        _ => {
                            return Err(map_disp_change_to_string(
                                change_display_settings_ex_result,
                            ));
                        }
                    }
                } else {
                    eprintln!(
                        "Failed to enum display settings for display device {0}",
                        display_adapter_device_name.to_string().unwrap()
                    );
                }
            } else {
                eprintln!(
                    "Failed to retrieve display device informations from the display adapter {0}",
                    display_adapter_device_name.to_string().unwrap()
                );
                break;
            }
        } else {
            break;
        }

        display_adapter_index += 1;
    }

    let change_display_settings_ex_result = ChangeDisplaySettingsExW(
        PCWSTR::null(),
        None,
        HWND::default(),
        CDS_TYPE::default(),
        None,
    );

    match change_display_settings_ex_result {
        DISP_CHANGE_SUCCESSFUL => return Ok(()),
        DISP_CHANGE_RESTART => {
            eprintln!("The computer must be restarted");
            return Ok(());
        }
        _ => {
            return Err(map_disp_change_to_string(change_display_settings_ex_result));
        }
    }
}
