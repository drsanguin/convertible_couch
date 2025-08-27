use super::win_32::Win32;
use crate::{
    displays_settings::{DisplaysSettings, DisplaysSettingsResult, INTERNAL_DISPLAY_NAME},
    ApplicationError,
};
use log::warn;
use std::{collections::HashMap, fmt::Debug, mem};
use windows::{
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
        },
        Graphics::Gdi::{
            CDS_NORESET, CDS_SET_PRIMARY, CDS_TYPE, CDS_UPDATEREGISTRY, DEVMODEW, DISPLAY_DEVICEW,
            DISP_CHANGE, DISP_CHANGE_BADDUALVIEW, DISP_CHANGE_BADFLAGS, DISP_CHANGE_BADMODE,
            DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_NOTUPDATED, DISP_CHANGE_RESTART,
            DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS,
        },
        UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME,
    },
};

pub struct WindowsDisplaySettings<TWin32: Win32> {
    win32: TWin32,
}

impl<TWin32: Win32> DisplaysSettings<TWin32> for WindowsDisplaySettings<TWin32> {
    fn new(displays_settings_api: TWin32) -> Self {
        Self {
            win32: displays_settings_api,
        }
    }

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaysSettingsResult, ApplicationError> {
        let names_by_device_ids = self.get_all_displays_names()?;
        let positions_by_device_ids = self.get_all_displays_positions()?;

        let mut desktop_display_device_id: Option<&String> = None;
        let mut couch_display_device_id: Option<&String> = None;
        let mut is_current_primary_display_the_desktop_one = false;

        for (device_id, position) in &positions_by_device_ids {
            let display_name = names_by_device_ids.get(device_id);

            if position.x == 0 && position.y == 0 {
                is_current_primary_display_the_desktop_one =
                    display_name.is_some_and(|x| x == desktop_display_name);
            }

            if display_name.is_some_and(|x| x == desktop_display_name) {
                desktop_display_device_id = Some(device_id);
            }

            if display_name.is_some_and(|x| x == couch_display_name) {
                couch_display_device_id = Some(device_id);
            }
        }

        if desktop_display_device_id.is_none() && couch_display_device_id.is_none() {
            let mut possible_values: Vec<String> = names_by_device_ids.into_values().collect();

            possible_values.sort();

            return Err(ApplicationError::Custom(format!(
                "Desktop and couch displays are invalid, possible values are [{}]",
                possible_values.join(", ")
            )));
        }

        if desktop_display_device_id.is_none() {
            let mut possible_values: Vec<String> = names_by_device_ids.into_values().collect();

            possible_values.sort();

            return Err(ApplicationError::Custom(format!(
                "Desktop display is invalid, possible values are [{}]",
                possible_values.join(", ")
            )));
        }

        if couch_display_device_id.is_none() {
            let mut possible_values: Vec<String> = names_by_device_ids.into_values().collect();

            possible_values.sort();

            return Err(ApplicationError::Custom(format!(
                "Couch display is invalid, possible values are [{}]",
                possible_values.join(", ")
            )));
        }

        let (new_primary_display_device_id, new_primary_display_name) =
            if is_current_primary_display_the_desktop_one {
                (couch_display_device_id.unwrap(), couch_display_name)
            } else {
                (desktop_display_device_id.unwrap(), desktop_display_name)
            };

        let new_primary_display_position = positions_by_device_ids
            .get(new_primary_display_device_id)
            .unwrap();

        let reboot_required = self.set_displays_to_position(new_primary_display_position)?;

        Ok(DisplaysSettingsResult {
            reboot_required,
            new_primary_display: new_primary_display_name.to_string(),
        })
    }
}

impl<TWin32: Win32> WindowsDisplaySettings<TWin32> {
    fn get_all_displays_names(&self) -> Result<HashMap<String, String>, ApplicationError> {
        let mut path_informations_length = u32::default();
        let mut mode_informations_length = u32::default();

        let get_display_config_buffer_sizes_return_code =
            self.win32.get_display_config_buffer_sizes(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_informations_length,
                &mut mode_informations_length,
            );

        if get_display_config_buffer_sizes_return_code.is_err() {
            let error_message = format!("Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {}", get_display_config_buffer_sizes_return_code.0);
            let error = ApplicationError::Custom(error_message);

            return Err(error);
        }

        let mut path_informations =
            vec![DISPLAYCONFIG_PATH_INFO::default(); path_informations_length.try_into()?];
        let mut mode_informations =
            vec![DISPLAYCONFIG_MODE_INFO::default(); mode_informations_length.try_into()?];

        let query_display_config_return_code = self.win32.query_display_config(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_informations_length,
            path_informations.as_mut_ptr(),
            &mut mode_informations_length,
            mode_informations.as_mut_ptr(),
            None,
        );

        if query_display_config_return_code.is_err() {
            let error_message = format!("Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {}", query_display_config_return_code.0);
            let error = ApplicationError::Custom(error_message);

            return Err(error);
        }

        let mut names_by_device_ids = HashMap::new();
        let size_of_displayconfig_target_device_name =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME, u32>();

        for mode_information in mode_informations.into_iter() {
            if mode_information.infoType != DISPLAYCONFIG_MODE_INFO_TYPE_TARGET {
                continue;
            }

            let mut displayconfig_target_device_name = DISPLAYCONFIG_TARGET_DEVICE_NAME::default();
            displayconfig_target_device_name.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                size: size_of_displayconfig_target_device_name,
                adapterId: mode_information.adapterId,
                id: mode_information.id,
            };

            let display_config_get_device_info_result = self
                .win32
                .display_config_get_device_info(&mut displayconfig_target_device_name.header);

            if display_config_get_device_info_result != 0 {
                let error_message = format!("Failed to retrieve display configuration information about the device {} because of error {}", mode_information.id, display_config_get_device_info_result);
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
        let mut positions = HashMap::new();

        for idevnum in 0..=u32::MAX {
            let mut display_adapter = get_default_display_devicew();

            let is_success_display_adapter = self
                .win32
                .enum_display_devices_w(
                    PCWSTR::null(),
                    idevnum,
                    &mut display_adapter,
                    EDD_GET_DEVICE_INTERFACE_NAME,
                )
                .as_bool();

            if !is_success_display_adapter {
                break;
            }

            let display_adapter_device_name = get_pcwstr_from_raw(&display_adapter.DeviceName);
            let mut display_device = get_default_display_devicew();

            let is_success_display_device = self
                .win32
                .enum_display_devices_w(
                    display_adapter_device_name,
                    0,
                    &mut display_device,
                    EDD_GET_DEVICE_INTERFACE_NAME,
                )
                .as_bool();

            if !is_success_display_device {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!("Failed to retrieve display device informations from the display adapter {display_adapter_device_name}");

                continue;
            }

            let mut display_adapter_graphics_mode = get_default_devmodew();

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!("Failed to enum display settings for display device {display_adapter_device_name}");

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

    fn set_displays_to_position(
        &mut self,
        position: &DisplayPosition,
    ) -> Result<bool, ApplicationError> {
        let mut reboot_required = false;

        for idevnum in 0..=u32::MAX {
            let mut display_adapter = get_default_display_devicew();

            let is_success_display_adapter = self
                .win32
                .enum_display_devices_w(
                    PCWSTR::null(),
                    idevnum,
                    &mut display_adapter,
                    EDD_GET_DEVICE_INTERFACE_NAME,
                )
                .as_bool();

            if !is_success_display_adapter {
                break;
            }

            let display_adapter_device_name = get_pcwstr_from_raw(&display_adapter.DeviceName);
            let mut display_device = get_default_display_devicew();

            let is_success_display_device = self
                .win32
                .enum_display_devices_w(
                    display_adapter_device_name,
                    0,
                    &mut display_device,
                    EDD_GET_DEVICE_INTERFACE_NAME,
                )
                .as_bool();

            if !is_success_display_device {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!("Failed to retrieve display device informations from the display adapter {display_adapter_device_name}");

                continue;
            }

            let mut display_adapter_graphics_mode = get_default_devmodew();

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                let display_adapter_device_name =
                    unsafe { display_adapter_device_name.to_string()? };

                warn!("Failed to enum display settings for display device {display_adapter_device_name}");

                continue;
            }

            unsafe {
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
            }

            let mut dwflags = CDS_UPDATEREGISTRY | CDS_NORESET;

            if is_positioned_at_origin(display_adapter_graphics_mode) {
                dwflags |= CDS_SET_PRIMARY;
            }

            let change_display_settings_ex_result = self.win32.change_display_settings_ex_w(
                display_adapter_device_name,
                Some(&display_adapter_graphics_mode),
                None,
                dwflags,
                None,
            );

            match change_display_settings_ex_result {
                DISP_CHANGE_SUCCESSFUL => continue,
                DISP_CHANGE_RESTART => {
                    reboot_required = true;
                    continue;
                }
                _ => {
                    let error_message =
                        map_disp_change_to_string(change_display_settings_ex_result);

                    return Err(ApplicationError::Custom(error_message));
                }
            }
        }

        let change_display_settings_ex_result = self.win32.change_display_settings_ex_w(
            PCWSTR::null(),
            None,
            None,
            CDS_TYPE::default(),
            None,
        );

        match change_display_settings_ex_result {
            DISP_CHANGE_SUCCESSFUL => Ok(reboot_required),
            DISP_CHANGE_RESTART => {
                reboot_required = true;

                Ok(reboot_required)
            }
            _ => {
                let error_message = map_disp_change_to_string(change_display_settings_ex_result);

                Err(ApplicationError::Custom(error_message))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct DisplayPosition {
    x: i32,
    y: i32,
}

fn is_positioned_at_origin(display_adapter_graphics_mode: DEVMODEW) -> bool {
    unsafe {
        display_adapter_graphics_mode
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
    }
}

fn get_default_display_devicew() -> DISPLAY_DEVICEW {
    let cb = size_of::<DISPLAY_DEVICEW, u32>();

    DISPLAY_DEVICEW {
        cb,
        ..DISPLAY_DEVICEW::default()
    }
}

fn get_default_devmodew() -> DEVMODEW {
    let dm_size = size_of::<DEVMODEW, u16>();

    DEVMODEW {
        dmSize: dm_size,
        ..DEVMODEW::default()
    }
}

fn get_pcwstr_from_raw(raw: &[u16; 32]) -> PCWSTR {
    PCWSTR::from_raw(raw.as_ptr())
}

fn map_disp_change_to_string(disp_change: DISP_CHANGE) -> String {
    match disp_change {
            DISP_CHANGE_BADDUALVIEW => String::from("The settings change was unsuccessful because the system is DualView capable."),
            DISP_CHANGE_BADFLAGS => String::from("An invalid set of flags was passed in."),
            DISP_CHANGE_BADMODE => String::from("The graphics mode is not supported."),
            DISP_CHANGE_BADPARAM => String::from("An invalid parameter was passed in. This can include an invalid flag or combination of flags."),
            DISP_CHANGE_FAILED => String::from("The display driver failed the specified graphics mode."),
            DISP_CHANGE_NOTUPDATED => String::from("Unable to write settings to the registry."),
            DISP_CHANGE_RESTART => String::from("The computer must be restarted for the graphics mode to work."),
            _ => String::from("The settings change was successful.")
        }
}

fn from_raw_display_name(raw_display_name: &str) -> String {
    let display_name = if raw_display_name == "" {
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
    let size = mem::size_of::<T1>();
    T2::try_from(size).unwrap()
}

fn from_utf16_trimed(bytes: &[u16]) -> Result<String, ApplicationError> {
    let str = String::from_utf16(bytes)?;

    Ok(str.trim_end_matches('\0').to_string())
}

#[cfg(test)]
mod tests {
    use crate::displays_settings::windows::windows_displays_settings::map_disp_change_to_string;
    use test_case::test_case;
    use windows::Win32::Graphics::Gdi::DISP_CHANGE;

    #[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_RESTART => String::from("The computer must be restarted for the graphics mode to work."); "when the error is DISP_CHANGE_RESTART")]
    #[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_SUCCESSFUL => String::from("The settings change was successful."); "when the error is DISP_CHANGE_SUCCESSFUL")]
    fn it_should_map_display_change_errors_to_a_human_friendly_message(
        disp_change: DISP_CHANGE,
    ) -> String {
        // Act
        map_disp_change_to_string(disp_change)
    }
}
