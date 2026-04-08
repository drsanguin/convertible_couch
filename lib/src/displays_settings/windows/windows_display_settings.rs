use crate::application_result::ApplicationResult;
use crate::displays_settings::windows::windows_api::WindowsApi;
use crate::{
    application_error::ApplicationError,
    displays_settings::{
        DisplayInfo, DisplaysSettings, DisplaysSettingsResult, INTERNAL_DISPLAY_NAME,
    },
    trace_fn,
};
use log::info;
use std::{fmt::Debug, mem};
use windows::Win32::{
    Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
        DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TARGET_DEVICE_NAME,
        QDC_ONLY_ACTIVE_PATHS, SDC_ALLOW_CHANGES, SDC_APPLY, SDC_SAVE_TO_DATABASE,
        SDC_USE_SUPPLIED_DISPLAY_CONFIG,
    },
    Foundation::{ERROR_INSUFFICIENT_BUFFER, POINTL, WIN32_ERROR},
};

pub struct WindowsDisplaySettings {
    windows_api: Box<dyn WindowsApi>,
}

impl DisplaysSettings for WindowsDisplaySettings {
    fn new(displays_settings_api: Box<dyn WindowsApi>) -> Self {
        trace_fn!();

        Self {
            windows_api: displays_settings_api,
        }
    }

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> ApplicationResult<DisplaysSettingsResult> {
        trace_fn!();
        info!("Changing primary display");

        let (patharray, mut modeinfoarray) = self.query_display_config()?;

        let mut new_position = POINTL { x: 0, y: 0 };
        let mut new_primary_monitor_name = String::default();
        let mut desktop_display_name_is_valid = false;
        let mut couch_display_name_is_valid = false;
        let mut possible_names = Vec::new();

        let size_of_displayconfig_target_device_name =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME, u32>();

        for path in &patharray {
            let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME {
                header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: size_of_displayconfig_target_device_name,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                },
                ..Default::default()
            };

            let display_config_get_device_info_result_code = unsafe {
                self.windows_api
                    .display_config_get_device_info(&mut target_name.header)
            };
            let display_config_get_device_info_result =
                WIN32_ERROR(display_config_get_device_info_result_code.try_into()?);

            display_config_get_device_info_result.ok()?;

            let source_mode_info_idx = unsafe { path.sourceInfo.Anonymous.modeInfoIdx };
            let source_mode = &modeinfoarray[source_mode_info_idx as usize];
            let position = unsafe { source_mode.Anonymous.sourceMode.position };

            let raw_display_friendly_device_name =
                from_utf16_trimed(&target_name.monitorFriendlyDeviceName)?;
            let display_friendly_device_name =
                from_raw_display_name(&raw_display_friendly_device_name);

            possible_names.push(display_friendly_device_name.clone());

            if display_friendly_device_name != desktop_display_name
                && display_friendly_device_name != couch_display_name
            {
                continue;
            }

            if display_friendly_device_name == desktop_display_name {
                desktop_display_name_is_valid = true;
            }

            if display_friendly_device_name == couch_display_name {
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

        let set_display_config_result_code = unsafe {
            self.windows_api.set_display_config(
                Some(&patharray),
                Some(&modeinfoarray),
                SDC_APPLY
                    | SDC_USE_SUPPLIED_DISPLAY_CONFIG
                    | SDC_ALLOW_CHANGES
                    | SDC_SAVE_TO_DATABASE,
            )
        };
        let set_display_config_result = WIN32_ERROR(set_display_config_result_code.try_into()?);

        set_display_config_result.ok()?;

        Ok(DisplaysSettingsResult {
            new_primary_display: new_primary_monitor_name,
        })
    }

    fn get_displays_infos(&mut self) -> ApplicationResult<Vec<DisplayInfo>> {
        trace_fn!();
        info!("Getting displays informations");

        let (patharray, modeinfoarray) = self.query_display_config()?;
        let mut displays_info = Vec::new();
        let size_of_displayconfig_target_device_name =
            size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME, u32>();

        for path in patharray {
            let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME {
                header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: size_of_displayconfig_target_device_name,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                },
                ..Default::default()
            };

            let display_config_get_device_info_result_code = unsafe {
                self.windows_api
                    .display_config_get_device_info(&mut target_name.header)
            };
            let display_config_get_device_info_result =
                WIN32_ERROR(display_config_get_device_info_result_code.try_into()?);

            display_config_get_device_info_result.ok()?;

            let source_mode_info_idx = unsafe { path.sourceInfo.Anonymous.modeInfoIdx };
            let source_mode = &modeinfoarray[source_mode_info_idx as usize];
            let position = unsafe { source_mode.Anonymous.sourceMode.position };

            let raw_display_friendly_device_name =
                from_utf16_trimed(&target_name.monitorFriendlyDeviceName)?;
            let display_friendly_device_name =
                from_raw_display_name(&raw_display_friendly_device_name);

            displays_info.push(DisplayInfo {
                name: display_friendly_device_name.clone(),
                is_primary: position.x == 0 && position.y == 0,
            });
        }

        displays_info.sort();

        Ok(displays_info)
    }
}

impl WindowsDisplaySettings {
    fn query_display_config(
        &mut self,
    ) -> ApplicationResult<(Vec<DISPLAYCONFIG_PATH_INFO>, Vec<DISPLAYCONFIG_MODE_INFO>)> {
        trace_fn!();

        let mut patharray = Vec::new();
        let mut modeinfoarray = Vec::new();
        let mut query_display_config_result;

        loop {
            let mut numpatharrayelements = u32::default();
            let mut nummodeinfoarrayelements = u32::default();

            unsafe {
                self.windows_api.get_display_config_buffer_sizes(
                    QDC_ONLY_ACTIVE_PATHS,
                    &mut numpatharrayelements,
                    &mut nummodeinfoarrayelements,
                )
            }
            .ok()?;

            patharray.resize(
                numpatharrayelements.try_into()?,
                DISPLAYCONFIG_PATH_INFO::default(),
            );
            modeinfoarray.resize(
                nummodeinfoarrayelements.try_into()?,
                DISPLAYCONFIG_MODE_INFO::default(),
            );

            query_display_config_result = unsafe {
                self.windows_api.query_display_config(
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

            if self.is_not_an_insufficient_buffer_error(&query_display_config_result) {
                break;
            }
        }

        query_display_config_result.ok()?;

        Ok((patharray, modeinfoarray))
    }

    fn is_not_an_insufficient_buffer_error(&self, error: &WIN32_ERROR) -> bool {
        trace_fn!();

        *error != ERROR_INSUFFICIENT_BUFFER
    }
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

fn from_utf16_trimed(bytes: &[u16]) -> ApplicationResult<String> {
    trace_fn!();

    let str = String::from_utf16(bytes)?;

    Ok(str.trim_end_matches('\0').to_string())
}
