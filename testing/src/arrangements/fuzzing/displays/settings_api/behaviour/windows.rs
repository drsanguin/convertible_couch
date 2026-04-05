use windows::Win32::Foundation::WIN32_ERROR;

use crate::arrangements::fuzzing::displays::settings_api::behaviour::FuzzedDisplaysSettingsApiBehaviour;

#[derive(Clone, Default)]
pub struct FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub get_display_config_buffer_sizes_error: Option<WIN32_ERROR>,
    pub query_display_config_error: Option<WIN32_ERROR>,
    pub display_config_get_device_info_error: Option<WIN32_ERROR>,
    pub set_display_config_error: Option<WIN32_ERROR>,
}

impl FuzzedDisplaysSettingsApiBehaviour for FuzzedWindowsDisplaysSettingsApiBehaviour {}
