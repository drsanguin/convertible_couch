use super::win_32::Win32;
use crate::display_settings::{DisplaySettings, DisplaySettingsResult, INTERNAL_DISPLAY_NAME};
use log::warn;
use std::{collections::BTreeSet, mem::size_of};
use windows::{
    core::PCWSTR,
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
            DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
        },
        Foundation::{ERROR_SUCCESS, WIN32_ERROR},
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

impl<TWin32: Win32> DisplaySettings<TWin32> for WindowsDisplaySettings<TWin32> {
    fn new(display_settings_api: TWin32) -> Self {
        Self {
            win32: display_settings_api,
        }
    }

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaySettingsResult, String> {
        self.validate_displays(desktop_display_name, couch_display_name)
            .and_then(|_| self.get_current_primary_display_name())
            .and_then(|current_primary_display_name| {
                Self::get_new_primary_display(
                    current_primary_display_name,
                    desktop_display_name,
                    couch_display_name,
                )
            })
            .and_then(|new_primary_display_name| {
                self.get_display_position(&new_primary_display_name)
            })
            .and_then(|new_primary_display_current_position| {
                self.set_displays_to_position(&new_primary_display_current_position)
            })
    }
}

#[derive(Debug, PartialEq)]
struct DisplayPosition {
    x: i32,
    y: i32,
}

impl<TWin32: Win32> WindowsDisplaySettings<TWin32> {
    fn get_new_primary_display<'a>(
        current_primary_display_name: String,
        desktop_display_name: &'a str,
        couch_display_name: &'a str,
    ) -> Result<&'a str, String> {
        Ok(if current_primary_display_name == desktop_display_name {
            couch_display_name
        } else {
            desktop_display_name
        })
    }

    fn validate_displays(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<(), String> {
        self.get_all_displays().and_then(|displays| {
            match (
                displays.contains(desktop_display_name),
                displays.contains(couch_display_name),
            ) {
                (true, true) => Ok(()),
                (true, false) => Err(format!(
                    "Couch display is invalid, possible values are [{}]",
                    Self::stringify_displays_names(&displays)
                )),
                (false, true) => Err(format!(
                    "Desktop display is invalid, possible values are [{}]",
                    Self::stringify_displays_names(&displays)
                )),
                (false, false) => Err(format!(
                    "Desktop and couch displays are invalid, possible values are [{}]",
                    Self::stringify_displays_names(&displays)
                )),
            }
        })
    }

    fn stringify_displays_names(displays: &BTreeSet<String>) -> String {
        displays.iter().cloned().collect::<Vec<String>>().join(", ")
    }

    fn get_current_primary_display_name(&self) -> Result<String, String> {
        let mut display_adapter_index: i32 = -1;
        let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
        let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

        loop {
            display_adapter_index += 1;

            let mut display_adapter = DISPLAY_DEVICEW::default();
            display_adapter.cb = size_of_display_devicew;

            let idevnum = u32::try_from(display_adapter_index).unwrap();
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

            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                warn!(
                    "Failed to retrieve display device informations from the display adapter {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            let size_of_devmode_as_usize = size_of::<DEVMODEW>();
            let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

            let mut display_adapter_graphics_mode = DEVMODEW::default();
            display_adapter_graphics_mode.dmSize = size_of_devmode;

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                warn!(
                    "Failed to enum display settings for display device {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            if !self.is_positioned_at_origin(display_adapter_graphics_mode) {
                continue;
            }

            let display_device_device_id = String::from_utf16(&display_device.DeviceID).unwrap();
            let display_device_device_id_trimed = display_device_device_id.trim_end_matches('\0');
            let current_raw_display_name = self
                .get_display_name(display_device_device_id_trimed)
                .unwrap();
            let current_display_name = Self::from_raw_display_name(&current_raw_display_name);

            return Ok(current_display_name);
        }

        Err(String::from("Failed to retrieve the primary display"))
    }

    fn get_display_position(&self, display_name: &str) -> Result<DisplayPosition, String> {
        let mut display_adapter_index: i32 = -1;
        let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
        let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

        loop {
            display_adapter_index += 1;
            let mut display_adapter = DISPLAY_DEVICEW::default();
            display_adapter.cb = size_of_display_devicew;

            let idevnum = u32::try_from(display_adapter_index).unwrap();
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

            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                warn!(
                    "Failed to retrieve display device informations from the display adapter {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            let size_of_devmode_as_usize = size_of::<DEVMODEW>();
            let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

            let mut display_adapter_graphics_mode = DEVMODEW::default();
            display_adapter_graphics_mode.dmSize = size_of_devmode;

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                warn!(
                    "Failed to enum display settings for display device {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            let display_device_device_id = String::from_utf16(&display_device.DeviceID).unwrap();
            let display_device_device_id_trimed = display_device_device_id.trim_end_matches('\0');
            let current_display_name = self
                .get_display_name(display_device_device_id_trimed)
                .unwrap();

            if current_display_name != display_name {
                continue;
            }

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

            return Ok(display_position);
        }

        Err(format!(
            "Failed to retrieve the position of display {display_name}"
        ))
    }

    fn set_displays_to_position(
        &mut self,
        position: &DisplayPosition,
    ) -> Result<DisplaySettingsResult, String> {
        let mut display_adapter_index: i32 = -1;
        let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
        let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();
        let mut reboot_required = false;
        let mut new_primary = None;

        loop {
            display_adapter_index += 1;

            let mut display_adapter = DISPLAY_DEVICEW::default();
            display_adapter.cb = size_of_display_devicew;

            let idevnum = u32::try_from(display_adapter_index).unwrap();
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

            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                warn!(
                    "Failed to retrieve display device informations from the display adapter {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );
                continue;
            }

            let size_of_devmode_as_usize = size_of::<DEVMODEW>();
            let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

            let mut display_adapter_graphics_mode = DEVMODEW::default();
            display_adapter_graphics_mode.dmSize = size_of_devmode;

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                warn!(
                    "Failed to enum display settings for display device {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

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

            if self.is_positioned_at_origin(display_adapter_graphics_mode) {
                let display_device_device_id =
                    String::from_utf16(&display_device.DeviceID).unwrap();
                let display_device_device_id_trimed =
                    display_device_device_id.trim_end_matches('\0');

                dwflags |= CDS_SET_PRIMARY;
                new_primary = Some(
                    self.get_display_name(display_device_device_id_trimed)
                        .unwrap(),
                )
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
                    return Err(Self::map_disp_change_to_string(
                        change_display_settings_ex_result,
                    ));
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
            DISP_CHANGE_SUCCESSFUL => Ok(DisplaySettingsResult {
                reboot_required,
                new_primary: new_primary.unwrap(),
            }),
            DISP_CHANGE_RESTART => {
                reboot_required = true;

                Ok(DisplaySettingsResult {
                    reboot_required,
                    new_primary: new_primary.unwrap(),
                })
            }
            _ => Err(Self::map_disp_change_to_string(
                change_display_settings_ex_result,
            )),
        }
    }

    fn is_positioned_at_origin(&self, display_adapter_graphics_mode: DEVMODEW) -> bool {
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

    fn get_display_name(&self, display_device_path: &str) -> Result<String, String> {
        let mut n_path_informations = u32::default();
        let mut n_mode_informations = u32::default();

        let get_display_config_buffer_sizes_return_code =
            self.win32.get_display_config_buffer_sizes(
                QDC_ONLY_ACTIVE_PATHS,
                &mut n_path_informations,
                &mut n_mode_informations,
            );

        (match get_display_config_buffer_sizes_return_code {
            ERROR_SUCCESS => Ok(()),
            error_return_code => {
                let error_return_code_value = error_return_code.0;

                Err(format!("Failed to retrieve the size of the buffers that are required to call the QueryDisplayConfig function: {error_return_code_value}"))
            }
        }).and_then(|_| {
            let n_path_informations_as_usize = usize::try_from(n_path_informations).unwrap();
            let n_mode_informations_as_usize = usize::try_from(n_mode_informations).unwrap();

            let mut path_informations = vec![DISPLAYCONFIG_PATH_INFO::default(); n_path_informations_as_usize];
            let mut mode_informations = vec![DISPLAYCONFIG_MODE_INFO::default(); n_mode_informations_as_usize];

            let query_display_config_return_code = self.win32.query_display_config(
                QDC_ONLY_ACTIVE_PATHS,
                &mut n_path_informations,
                path_informations.as_mut_ptr(),
                &mut n_mode_informations,
                mode_informations.as_mut_ptr(),
                None
            );

            match query_display_config_return_code {
                ERROR_SUCCESS => Ok(mode_informations),
                error_return_code => {
                    let error_return_code_value = error_return_code.0;

                    Err(format!("Failed to retrieve information about all possible display paths for all display devices, or views, in the current setting: {error_return_code_value}"))
                }
            }
        })
        .and_then(|mode_informations| {
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

                let display_config_get_device_info_result_as_i32 = self.win32.display_config_get_device_info(&mut displayconfig_target_device_name.header);
                let display_config_get_device_info_result_as_u32 = u32::try_from(display_config_get_device_info_result_as_i32).unwrap();
                let display_config_get_device_info_result = WIN32_ERROR(display_config_get_device_info_result_as_u32);

                match display_config_get_device_info_result {
                    ERROR_SUCCESS => {
                        let display_friendly_device_name = String::from_utf16(&displayconfig_target_device_name.monitorFriendlyDeviceName).unwrap();
                        let current_display_device_path = String::from_utf16(&displayconfig_target_device_name.monitorDevicePath).unwrap();
                        let current_display_device_path_trimed = current_display_device_path.trim_end_matches('\0');

                        if current_display_device_path_trimed != display_device_path {
                            continue;
                        }

                        let raw_display_friendly_device_name_trimed = display_friendly_device_name.trim_end_matches('\0');

                        return Ok(Self::from_raw_display_name(raw_display_friendly_device_name_trimed));
                    },
                    error => return Err(format!("Failed to retrieve display configuration information about the device {} because of error {}", mode_information.id, error.0))
                }
            }

            Err(format!(
                "Failed to retrieve the name of the display at the device path {display_device_path}"
            ))
        })
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

    fn get_all_displays(&self) -> Result<BTreeSet<String>, String> {
        let mut displays_names = BTreeSet::new();
        let mut display_adapter_index: i32 = -1;
        let size_of_display_devicew_as_usize = size_of::<DISPLAY_DEVICEW>();
        let size_of_display_devicew = u32::try_from(size_of_display_devicew_as_usize).unwrap();

        loop {
            display_adapter_index += 1;

            let mut display_adapter = DISPLAY_DEVICEW::default();
            display_adapter.cb = size_of_display_devicew;

            let idevnum = u32::try_from(display_adapter_index).unwrap();
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

            let mut display_device = DISPLAY_DEVICEW::default();
            display_device.cb = size_of_display_devicew;

            let display_adapter_device_name_as_ptr = display_adapter.DeviceName.as_ptr();
            let display_adapter_device_name = PCWSTR::from_raw(display_adapter_device_name_as_ptr);

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
                warn!(
                    "Failed to retrieve display device informations from the display adapter {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            let size_of_devmode_as_usize = size_of::<DEVMODEW>();
            let size_of_devmode = u16::try_from(size_of_devmode_as_usize).unwrap();

            let mut display_adapter_graphics_mode = DEVMODEW::default();
            display_adapter_graphics_mode.dmSize = size_of_devmode;

            let has_enum_display_settings_succeded = self
                .win32
                .enum_display_settings_w(
                    display_adapter_device_name,
                    ENUM_CURRENT_SETTINGS,
                    &mut display_adapter_graphics_mode,
                )
                .as_bool();

            if !has_enum_display_settings_succeded {
                warn!(
                    "Failed to enum display settings for display device {}",
                    unsafe { display_adapter_device_name.to_string() }.unwrap()
                );

                continue;
            }

            let display_device_device_id = String::from_utf16(&display_device.DeviceID).unwrap();
            let display_device_device_id_trimed = display_device_device_id.trim_end_matches('\0');

            match self.get_display_name(display_device_device_id_trimed) {
                Ok(current_raw_display_name) => {
                    let current_display_name =
                        Self::from_raw_display_name(&current_raw_display_name);
                    displays_names.insert(current_display_name);
                    continue;
                }
                Err(reason) => return Err(reason),
            }
        }

        Ok(displays_names)
    }

    fn from_raw_display_name(raw_display_name: &str) -> String {
        let display_name = if raw_display_name == "" {
            INTERNAL_DISPLAY_NAME
        } else {
            raw_display_name
        };

        String::from(display_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        display_settings::DisplaySettings,
        func,
        testing::fuzzing::{display_settings::win_32::FuzzedWin32, Fuzzer},
    };
    use std::collections::HashSet;
    use test_case::test_case;
    use windows::Win32::Graphics::Gdi::DISP_CHANGE;

    use super::WindowsDisplaySettings;

    #[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_RESTART => String::from("The computer must be restarted for the graphics mode to work."); "when the error is DISP_CHANGE_RESTART")]
    #[test_case(windows::Win32::Graphics::Gdi::DISP_CHANGE_SUCCESSFUL => String::from("The settings change was successful."); "when the error is DISP_CHANGE_SUCCESSFUL")]
    fn it_should_map_display_change_errors_to_a_human_friendly_message(
        disp_change: DISP_CHANGE,
    ) -> String {
        // Act
        WindowsDisplaySettings::<FuzzedWin32>::map_disp_change_to_string(disp_change)
    }

    #[test]
    fn it_should_handle_the_case_of_getting_current_primary_display_name_when_the_computer_has_no_display(
    ) {
        // Arrange
        let mut fuzzer = Fuzzer::new(func!(), true);

        let computer = fuzzer.generate_computer().build_computer();

        let display_settings = WindowsDisplaySettings::new(computer.display_settings_api);

        // Act
        let result = display_settings.get_current_primary_display_name();

        //Assert
        assert_eq!(
            result,
            Err(String::from("Failed to retrieve the primary display"))
        );
    }

    #[test]
    fn it_should_handle_the_case_of_getting_the_position_of_a_non_existing_display() {
        // Arrange
        let mut fuzzer = Fuzzer::new(func!(), true);

        let forbidden_display_name = fuzzer.generate_display_name();
        let forbidden_display_names = HashSet::from([forbidden_display_name.as_str()]);

        let computer = fuzzer
            .generate_computer()
            .with_displays()
            .of_which_there_are_at_least(2)
            .whose_names_are_different_from(forbidden_display_names)
            .build_displays()
            .build_computer();

        let display_settings = WindowsDisplaySettings::new(computer.display_settings_api);

        // Act
        let result = display_settings.get_display_position(&forbidden_display_name);

        //Assert
        assert_eq!(
            result,
            Err(format!(
                "Failed to retrieve the position of display {forbidden_display_name}"
            ))
        );
    }

    #[test]
    fn it_should_handle_the_case_of_getting_the_name_of_a_display_at_a_non_existing_device_path() {
        // Arrange
        let mut fuzzer = Fuzzer::new(func!(), true);

        let forbidden_device_id = fuzzer.generate_device_id();
        let forbidden_device_ids = HashSet::from([&forbidden_device_id]);

        let computer = fuzzer
            .generate_computer()
            .with_displays()
            .of_which_there_are_at_least(2)
            .whose_device_ids_are_different_from(forbidden_device_ids)
            .build_displays()
            .build_computer();

        let display_settings = WindowsDisplaySettings::new(computer.display_settings_api);

        // Act
        let result = display_settings.get_display_name(&forbidden_device_id.full_id);

        //Assert
        assert_eq!(
            result,
            Err(format!(
                "Failed to retrieve the name of the display at the device path {forbidden_device_id}"
            ))
        );
    }
}
