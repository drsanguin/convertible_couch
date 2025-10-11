use windows::Win32::{Foundation::WIN32_ERROR, Graphics::Gdi::DISP_CHANGE};

use crate::testing::fuzzing::displays::settings_api::behaviour::FuzzedDisplaysSettingsApiBehaviour;

#[derive(Clone, Default)]
pub struct FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub change_display_settings_error: Option<DISP_CHANGE>,
    pub commit_display_settings_changes_error: Option<DISP_CHANGE>,
    pub getting_primary_display_name_fails: bool,
    pub get_display_config_buffer_sizes_error: Option<WIN32_ERROR>,
    pub query_display_config_error: Option<WIN32_ERROR>,
    pub display_not_possible_to_enum_display_settings_on: Option<String>,
}

impl FuzzedDisplaysSettingsApiBehaviour for FuzzedWindowsDisplaysSettingsApiBehaviour {}
