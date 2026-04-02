use crate::{
    application_error::ApplicationError,
    displays_settings::{
        DisplayInfo, DisplaysSettings, DisplaysSettingsResult, INTERNAL_DISPLAY_NAME,
    },
    trace_fn,
};
use log::{debug, info, warn};
use std::{collections::HashMap, fmt::Debug, mem};
use win_32::Win32;
use windows::{
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS, SDC_ALLOW_CHANGES, SDC_APPLY,
            SDC_SAVE_TO_DATABASE, SDC_USE_SUPPLIED_DISPLAY_CONFIG,
        },
        Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, POINTL},
        Graphics::Gdi::{DEVMODEW, DISPLAY_DEVICEW, ENUM_CURRENT_SETTINGS},
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
    core::PCWSTR,
};

pub mod win_32;

pub struct WindowsDisplaySettings {
    win32: Box<dyn Win32>,
}

impl DisplaysSettings for WindowsDisplaySettings {
    fn new(displays_settings_api: Box<dyn Win32>) -> Self {
        trace_fn!();

        Self {
            win32: displays_settings_api,
        }
    }

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaysSettingsResult, ApplicationError> {
        let mut patharray = Vec::new();
        let mut modeinfoarray = Vec::new();
        let mut query_display_config_result;

        loop {
            let mut numpatharrayelements = u32::default();
            let mut nummodeinfoarrayelements = u32::default();

            (unsafe {
                self.win32
                    .get_display_config_buffer_sizes(
                        QDC_ONLY_ACTIVE_PATHS,
                        &mut numpatharrayelements,
                        &mut nummodeinfoarrayelements,
                    )
                    .ok()
            })?;

            patharray.resize(
                numpatharrayelements.try_into()?,
                DISPLAYCONFIG_PATH_INFO::default(),
            );
            modeinfoarray.resize(
                nummodeinfoarrayelements.try_into()?,
                DISPLAYCONFIG_MODE_INFO::default(),
            );

            query_display_config_result = unsafe {
                self.win32.query_display_config(
                    QDC_ONLY_ACTIVE_PATHS,
                    &mut numpatharrayelements,
                    patharray.as_mut_ptr(),
                    &mut nummodeinfoarrayelements,
                    modeinfoarray.as_mut_ptr(),
                    None,
                )
            };

            patharray.resize(
                numpatharrayelements.try_into()?,
                DISPLAYCONFIG_PATH_INFO::default(),
            );
            modeinfoarray.resize(
                nummodeinfoarrayelements.try_into()?,
                DISPLAYCONFIG_MODE_INFO::default(),
            );

            if query_display_config_result != ERROR_INSUFFICIENT_BUFFER {
                break;
            }
        }

        query_display_config_result.ok()?;

        let mut new_position = POINTL { x: 0, y: 0 };
        let mut new_primary_monitor_name = String::default();
        let mut desktop_display_name_is_valid = false;
        let mut couch_display_name_is_valid = false;
        let mut possible_names = Vec::new();

        let size_of_displayconfig_target_device_name =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME, u32>();

        for (monitor_index, path) in patharray.iter().enumerate() {
            let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME {
                header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: size_of_displayconfig_target_device_name,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                },
                ..Default::default()
            };

            let display_config_get_device_info_result = unsafe {
                self.win32
                    .display_config_get_device_info(&mut target_name.header)
            };

            if display_config_get_device_info_result != ERROR_SUCCESS.0 as i32 {
                let error_message = format!(
                    "Failed to retrieve display configuration information about the device {} because of error {}",
                    path.targetInfo.id, display_config_get_device_info_result
                );
                let error = ApplicationError::Custom(error_message);

                return Err(error);
            }

            let source_mode_info_idx = unsafe { path.sourceInfo.Anonymous.modeInfoIdx };
            let source_mode = &modeinfoarray[source_mode_info_idx as usize];
            let position = unsafe { source_mode.Anonymous.sourceMode.position };

            let raw_display_friendly_device_name =
                from_utf16_trimed(&target_name.monitorFriendlyDeviceName)?;
            let display_friendly_device_name =
                from_raw_display_name(&raw_display_friendly_device_name);

            debug!(
                "index = {}, display_friendly_device_name = \"{}\", position = ({}, {})",
                monitor_index + 1,
                display_friendly_device_name,
                position.x,
                position.y
            );

            possible_names.push(display_friendly_device_name.clone());

            if display_friendly_device_name != desktop_display_name
                && display_friendly_device_name != couch_display_name
            {
                continue;
            }

            if (display_friendly_device_name == desktop_display_name) {
                desktop_display_name_is_valid = true;
            }

            if (display_friendly_device_name == couch_display_name) {
                couch_display_name_is_valid = true;
            }

            if position.x != 0 || position.y != 0 {
                new_position = position;
                new_primary_monitor_name = display_friendly_device_name;
            }
        }

        let invalid_params_error_message =
            match (desktop_display_name_is_valid, couch_display_name_is_valid) {
                (false, false) => Some("Desktop and couch displays are invalid"),
                (false, _) => Some("Desktop display is invalid"),
                (_, false) => Some("Couch display is invalid"),
                _ => None,
            };

        if let Some(invalid_params_error_message_fragment) = invalid_params_error_message {
            possible_names.sort();
            let possible_values_fragment = possible_names.join(", ");

            let error_message = format!(
                "{invalid_params_error_message_fragment}, possible values are [{possible_values_fragment}]"
            );
            let error = ApplicationError::Custom(error_message);

            return Err(error);
        }

        for path in &patharray {
            let mode_info_idx = unsafe { path.sourceInfo.Anonymous.modeInfoIdx };
            let mode_info = &mut modeinfoarray[mode_info_idx as usize];

            unsafe { mode_info.Anonymous.sourceMode.position.x -= new_position.x };
            unsafe { mode_info.Anonymous.sourceMode.position.y -= new_position.y };
        }

        unsafe {
            self.win32.set_display_config(
                Some(&patharray),
                Some(&modeinfoarray),
                SDC_APPLY
                    | SDC_USE_SUPPLIED_DISPLAY_CONFIG
                    | SDC_ALLOW_CHANGES
                    | SDC_SAVE_TO_DATABASE,
            );
        };

        Ok(DisplaysSettingsResult {
            reboot_required: false,
            new_primary_display: new_primary_monitor_name,
        })
    }

    fn get_displays_infos(&self) -> Result<Vec<DisplayInfo>, ApplicationError> {
        trace_fn!();
        info!("Getting displays informations");

        let names_by_device_ids = self.get_all_displays_names()?;
        let positions_by_device_ids = self.get_all_displays_positions()?;

        debug!("names_by_device_ids = {:?}", names_by_device_ids);
        debug!("positions_by_device_ids = {:?}", positions_by_device_ids);

        let mut displays_info = positions_by_device_ids
            .iter()
            .map(|(device_id, position)| DisplayInfo {
                is_primary: position.x == 0 && position.y == 0,
                name: names_by_device_ids.get(device_id).unwrap().to_string(),
            })
            .collect::<Vec<_>>();

        displays_info.sort();

        Ok(displays_info)
    }
}

impl WindowsDisplaySettings {
    fn get_all_displays_names(&self) -> Result<HashMap<String, String>, ApplicationError> {
        trace_fn!();

        let mut path_informations_length = u32::default();
        let mut mode_informations_length = u32::default();

        let get_display_config_buffer_sizes_return_code = unsafe {
            self.win32.get_display_config_buffer_sizes(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_informations_length,
                &mut mode_informations_length,
            )
        };

        if get_display_config_buffer_sizes_return_code.is_err() {
            let error_message = format!(
                "Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {}",
                get_display_config_buffer_sizes_return_code.0
            );
            let error = ApplicationError::Custom(error_message);

            return Err(error);
        }

        let mut path_informations =
            vec![DISPLAYCONFIG_PATH_INFO::default(); path_informations_length.try_into()?];
        let mut mode_informations =
            vec![DISPLAYCONFIG_MODE_INFO::default(); mode_informations_length.try_into()?];

        let query_display_config_return_code = unsafe {
            self.win32.query_display_config(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_informations_length,
                path_informations.as_mut_ptr(),
                &mut mode_informations_length,
                mode_informations.as_mut_ptr(),
                None,
            )
        };

        if query_display_config_return_code.is_err() {
            let error_message = format!(
                "Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {}",
                query_display_config_return_code.0
            );
            let error = ApplicationError::Custom(error_message);

            return Err(error);
        }

        let mut names_by_device_ids = HashMap::new();
        let size_of_displayconfig_target_device_name =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME, u32>();

        for mode_information in mode_informations {
            if mode_information.infoType != DISPLAYCONFIG_MODE_INFO_TYPE_TARGET {
                continue;
            }

            let mut displayconfig_target_device_name = DISPLAYCONFIG_TARGET_DEVICE_NAME {
                header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: size_of_displayconfig_target_device_name,
                    adapterId: mode_information.adapterId,
                    id: mode_information.id,
                },
                ..Default::default()
            };

            let display_config_get_device_info_result = unsafe {
                self.win32
                    .display_config_get_device_info(&mut displayconfig_target_device_name.header)
            };

            if display_config_get_device_info_result != 0 {
                let error_message = format!(
                    "Failed to retrieve display configuration information about the device {} because of error {}",
                    mode_information.id, display_config_get_device_info_result
                );
                let error = ApplicationError::Custom(error_message);

                return Err(error);
            }

            let current_display_device_path =
                from_utf16_trimed(&displayconfig_target_device_name.monitorDevicePath)?;
            let raw_display_friendly_device_name =
                from_utf16_trimed(&displayconfig_target_device_name.monitorFriendlyDeviceName)?;
            let display_friendly_device_name =
                from_raw_display_name(&raw_display_friendly_device_name);

            names_by_device_ids.insert(current_display_device_path, display_friendly_device_name);
        }

        Ok(names_by_device_ids)
    }

    fn get_all_displays_positions(
        &self,
    ) -> Result<HashMap<String, DisplayPosition>, ApplicationError> {
        trace_fn!();

        let mut positions = HashMap::new();

        for idevnum in 0..=u32::MAX {
            let mut display_adapter = get_default_display_devicew();

            let is_success_display_adapter = unsafe {
                self.win32
                    .enum_display_devices_w(
                        PCWSTR::null(),
                        idevnum,
                        &mut display_adapter,
                        EDD_GET_DEVICE_INTERFACE_NAME,
                    )
                    .as_bool()
            };

            if !is_success_display_adapter {
                break;
            }

            let display_adapter_device_name = get_pcwstr_from_raw(&display_adapter.DeviceName);
            let mut display_device = get_default_display_devicew();

            let is_success_display_device = unsafe {
                self.win32
                    .enum_display_devices_w(
                        display_adapter_device_name,
                        0,
                        &mut display_device,
                        EDD_GET_DEVICE_INTERFACE_NAME,
                    )
                    .as_bool()
            };

            if !is_success_display_device {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!(
                    "Failed to retrieve display device informations from the display adapter {display_adapter_device_name}"
                );

                continue;
            }

            let mut display_adapter_graphics_mode = get_default_devmodew();

            let has_enum_display_settings_succeded = unsafe {
                self.win32
                    .enum_display_settings_w(
                        display_adapter_device_name,
                        ENUM_CURRENT_SETTINGS,
                        &mut display_adapter_graphics_mode,
                    )
                    .as_bool()
            };

            if !has_enum_display_settings_succeded {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!(
                    "Failed to enum display settings for display device {display_adapter_device_name}"
                );

                continue;
            }

            let display_device_device_id = from_utf16_trimed(&display_device.DeviceID)?;

            let display_position = unsafe {
                DisplayPosition {
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
                }
            };

            positions.insert(display_device_device_id, display_position);
        }

        Ok(positions)
    }
}

#[derive(Debug, PartialEq)]
struct DisplayPosition {
    x: i32,
    y: i32,
}

fn get_default_display_devicew() -> DISPLAY_DEVICEW {
    trace_fn!();

    let cb = size_of::<DISPLAY_DEVICEW, u32>();

    DISPLAY_DEVICEW {
        cb,
        ..DISPLAY_DEVICEW::default()
    }
}

fn get_default_devmodew() -> DEVMODEW {
    trace_fn!();

    let dm_size = size_of::<DEVMODEW, u16>();

    DEVMODEW {
        dmSize: dm_size,
        ..DEVMODEW::default()
    }
}

fn get_pcwstr_from_raw(raw: &[u16; 32]) -> PCWSTR {
    trace_fn!();

    PCWSTR::from_raw(raw.as_ptr())
}

fn from_raw_display_name(raw_display_name: &str) -> String {
    trace_fn!();

    let display_name = if raw_display_name.is_empty() {
        INTERNAL_DISPLAY_NAME
    } else {
        raw_display_name
    };

    String::from(display_name)
}

fn size_of<T1, T2: TryFrom<usize>>() -> T2
where
    <T2 as TryFrom<usize>>::Error: Debug,
{
    trace_fn!();

    let size = mem::size_of::<T1>();
    T2::try_from(size).unwrap()
}

fn from_utf16_trimed(bytes: &[u16]) -> Result<String, ApplicationError> {
    trace_fn!();

    let str = String::from_utf16(bytes)?;

    Ok(str.trim_end_matches('\0').to_string())
}
