// https://blog.lohr.dev/primary-display-windows
// https://github.com/michidk/displayz

// use std::mem::size_of;

// use std::mem::size_of;

// use windows::{Win32::{Graphics::Gdi::{DISPLAY_DEVICEW, EnumDisplayDevicesW}, UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME}, core::PCWSTR};

/* 
\\?\DISPLAY#GSM59F2#5&de985f&0&UID28929#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - LG ULTRAWIDE
\\?\DISPLAY#GSM56D5#5&de985f&0&UID28932#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - M227WD
\\?\DISPLAY#GSM81CD#5&de985f&0&UID28928#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - LG TV SSCR2
\\?\DISPLAY#GSM59F2#5&de985f&0&UID28929#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - \\.\DISPLAY1\Monitor0 - Generic PnP Monitor - \Registry\Machine\System\CurrentControlSet\Control\Class\{4d36e96e-e325-11ce-bfc1-08002be10318}\0001 - 3
2560 x 1080 at 0,0
\\?\DISPLAY#GSM56D5#5&de985f&0&UID28932#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - \\.\DISPLAY2\Monitor0 - Generic PnP Monitor - \Registry\Machine\System\CurrentControlSet\Control\Class\{4d36e96e-e325-11ce-bfc1-08002be10318}\0011 - 3
1920 x 1080 at -1920,0
\\?\DISPLAY#GSM81CD#5&de985f&0&UID28928#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7} - \\.\DISPLAY3\Monitor0 - Generic PnP Monitor - \Registry\Machine\System\CurrentControlSet\Control\Class\{4d36e96e-e325-11ce-bfc1-08002be10318}\0003 - 3
4096 x 2160 at 2560,0
Failed to retrieve display device informations from the display adapter \\.\DISPLAY4
 */

use std::{collections::HashMap, mem::size_of, fmt};

use windows::{
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::{ERROR_SUCCESS, WIN32_ERROR},
        Graphics::Gdi::{
            EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW, DISPLAY_DEVICEW,
            ENUM_CURRENT_SETTINGS, DISPLAY_DEVICE_PRIMARY_DEVICE, ChangeDisplaySettingsExW,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

struct MonitorPosition {
    x: i32,
    y: i32
}

impl fmt::Display for MonitorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let desktop_monitor_name = "LG ULTRAWIDE";
    let couch_monitor_name = "LG TV SSCR2";

    // unsafe {
    //     let desktop_monitor_path_result = get_display_path(desktop_monitor_name);

    //     match desktop_monitor_path_result {
    //         Ok(desktop_monitor_path) => println!("path of {0} is {1}", desktop_monitor_name, desktop_monitor_path),
    //         Err(error) => println!("failed to find path of {0} because of {1}", desktop_monitor_name, error)
    //     }
    // }

    // unsafe {
    //     let desktop_monitor_device_path = get_monitor_device_path(desktop_monitor_name).unwrap();
    //     println!("device path of {0} is {1}", desktop_monitor_name, desktop_monitor_device_path);
        
    //     let dektop_monitor_name_from_device_path = get_monitor_name(desktop_monitor_device_path.as_str()).unwrap();
    //     println!("name of {0} is {1}", desktop_monitor_device_path, dektop_monitor_name_from_device_path);

    //     let primary_monitor = get_primary_monitor_name().unwrap();
    //     println!("primay monitor is {0}", primary_monitor);

    //     let desktop_monitor_position = get_monitor_position(desktop_monitor_name).unwrap();
    //     println!("position of {0} is {1}", desktop_monitor_name, desktop_monitor_position);

    //     let couch_monitor_position = get_monitor_position(couch_monitor_name).unwrap();
    //     println!("position of {0} is {1}", couch_monitor_name, couch_monitor_position);
    // }

    unsafe {
        let primary_monitor_name = get_primary_monitor_name().unwrap();
        let new_primary_monitor_name =
            if primary_monitor_name == desktop_monitor_name { couch_monitor_name }
            else { desktop_monitor_name };
        let new_primary_monitor_current_position = get_monitor_position(new_primary_monitor_name).unwrap();
        let _ = set_monitors_to_position(new_primary_monitor_current_position);
    }

    // unsafe {
    //     tests();
    // }
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

    return Err(format!("Failed to retrieve the name of the monitor at the device path {0}", monitor_device_path));
}

unsafe fn get_monitor_device_path(monitor_name: &str) -> Result<String, String> {
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
                                let monitor_friendly_device_name_trimed = monitor_friendly_device_name.trim_end_matches('\0');
                                let monitor_device_path = String::from_utf16(&displayconfig_target_device_name.monitorDevicePath).unwrap();
                                
                                if monitor_friendly_device_name_trimed == monitor_name {
                                    let monitor_device_path_trimed = monitor_device_path.trim_end_matches('\0');
                                    return Ok(String::from(monitor_device_path_trimed));
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

    return Err(format!("Failed to retrieve the path of the monitor {0}", monitor_name));
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
            let display_adapter_device_name =
                PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                    if display_adapter_graphics_mode.Anonymous1
                        .Anonymous2
                        .dmPosition
                        .x == 0
                    && display_adapter_graphics_mode.Anonymous1
                        .Anonymous2
                        .dmPosition
                        .y == 0 {
                        let display_device_device_id = String::from_utf16(&display_device.DeviceID).unwrap();
                        let display_device_device_id_trimed = display_device_device_id.trim_end_matches('\0');
                        let current_monitor_name = get_monitor_name(display_device_device_id_trimed).unwrap();

                        return Ok(current_monitor_name);
                    }
                } else {
                    eprintln!(
                        "Failed to enum display settings for display device {0}",
                        display_adapter_device_name.to_string().unwrap()
                    );
                }
            } else {
                eprintln!("Failed to retrieve display device informations from the display adapter {0}", display_adapter_device_name.to_string().unwrap());
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
            let display_adapter_device_name =
                PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                    let display_device_device_id = String::from_utf16(&display_device.DeviceID).unwrap();
                    let display_device_device_id_trimed = display_device_device_id.trim_end_matches('\0');
                    let current_monitor_name = get_monitor_name(display_device_device_id_trimed).unwrap();

                    if current_monitor_name == monitor_name {
                        let monitor_position = MonitorPosition {
                            x: display_adapter_graphics_mode.Anonymous1
                            .Anonymous2
                            .dmPosition
                            .x,
                            y: display_adapter_graphics_mode.Anonymous1
                            .Anonymous2
                            .dmPosition
                            .y
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
                eprintln!("Failed to retrieve display device informations from the display adapter {0}", display_adapter_device_name.to_string().unwrap());
            }
        } else {
            break;
        }

        display_adapter_index += 1;
    }

    return Err(format!("Failed to retrieve the position of monitor {0}", monitor_name));
}

unsafe fn set_monitors_to_position(position: MonitorPosition) -> Result<(), String> {
    todo!()
}

unsafe fn tests() {
        // let desktop_display_name = String::from("LG ULTRAWIDE");
    // let couch_display_name = String::from("LG TV SSCR2");

    unsafe {
        // Getting displays friendly name by path
        let mut n_path_informations = u32::default();
        let mut n_mode_informations = u32::default();
        let mut displays_names_by_path = HashMap::new();

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
                                    let monitor_device_path = String::from_utf16(&displayconfig_target_device_name.monitorDevicePath).unwrap();

                                    println!(
                                        "{0} - {1}",
                                        monitor_device_path,
                                        monitor_friendly_device_name
                                    );

                                    displays_names_by_path.insert(monitor_device_path, monitor_friendly_device_name);
                                },
                                error => eprintln!("Failed to retrieve display configuration information about the device: {0}", error.0)
                            }
                        }
                    },
                    Err(error) => eprintln!("Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {0}", error)
                }
            },
            Err(error) => eprintln!("Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {0}", error)
        }

        let mut displays_adapters_graphics_modes_by_path = HashMap::new();
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
                // let display_adapter_device_id =
                //     String::from_utf16(&display_adapter.DeviceID).unwrap();
                // let display_adapter_device_name =
                //     String::from_utf16(&display_adapter.DeviceName).unwrap();
                // let display_adapter_device_string =
                //     String::from_utf16(&display_adapter.DeviceString).unwrap();
                // let display_adapter_device_key =
                //     String::from_utf16(&display_adapter.DeviceKey).unwrap();

                // println!(
                //     "{0} - {1} - {2} - {3} - {4}",
                //     display_adapter_device_id,
                //     display_adapter_device_name,
                //     display_adapter_device_string,
                //     display_adapter_device_key,
                //     display_adapter.StateFlags
                // );

                // let mut display_device_index = 0;

                let mut display_device = DISPLAY_DEVICEW::default();
                display_device.cb = size_of_display_devicew;

                let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
                let display_adapter_device_name =
                    PCWSTR::from_raw(display_adapter_device_name_as_ptr);

                let is_success_display_device_as_win32_bool = EnumDisplayDevicesW(
                    display_adapter_device_name,
                    0,
                    &mut display_device,
                    EDD_GET_DEVICE_INTERFACE_NAME,
                );
                let is_success_display_device = is_success_display_device_as_win32_bool.as_bool();

                if is_success_display_device {
                    let display_device_device_id =
                        String::from_utf16(&display_device.DeviceID).unwrap();
                    let display_device_device_name =
                        String::from_utf16(&display_device.DeviceName).unwrap();
                    let display_device_device_string =
                        String::from_utf16(&display_device.DeviceString).unwrap();
                    let display_device_device_key =
                        String::from_utf16(&display_device.DeviceKey).unwrap();

                    // fn has(&self, other: Self) -> bool {
                    //     (self.0 & other.0) != 0
                    // }
                    // display_device.StateFlags;

                    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Devices/Display/fn.DisplayConfigGetDeviceInfo.html
                    // https://stackoverflow.com/questions/20060584/get-the-name-of-a-monitor
                    // https://stackoverflow.com/questions/4958683/how-do-i-get-the-actual-monitor-name-as-seen-in-the-resolution-dialog

                    println!(
                        "{0} - {1} - {2} - {3} - {4}",
                        display_device_device_id,
                        display_device_device_name,
                        display_device_device_string,
                        display_device_device_key,
                        display_device.StateFlags
                    );

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
                        displays_adapters_graphics_modes_by_path.insert(String::from_utf16(&display_adapter.DeviceName).unwrap(), display_adapter_graphics_mode);

                        println!(
                            "[{0}] {1} x {2} at {3},{4}",
                            display_adapter_device_name.to_string().unwrap(),
                            display_adapter_graphics_mode.dmPelsWidth,
                            display_adapter_graphics_mode.dmPelsHeight,
                            display_adapter_graphics_mode
                                .Anonymous1
                                .Anonymous2
                                .dmPosition
                                .x,
                                display_adapter_graphics_mode
                                .Anonymous1
                                .Anonymous2
                                .dmPosition
                                .y
                        );

                        if (display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE) != 0 {
                            println!("primary! {0}", display_device.StateFlags);
                        } else {
                            println!("not primary! {0}", display_device.StateFlags);
                        }
                    } else {
                        eprintln!(
                            "Failed to enum display settings for display device {0}",
                            display_adapter_device_name.to_string().unwrap()
                        );
                    }
                } else {
                    eprintln!("Failed to retrieve display device informations from the display adapter {0}", display_adapter_device_name.to_string().unwrap());
                }
            } else {
                break;
            }

            display_adapter_index += 1;
        }
    }
}