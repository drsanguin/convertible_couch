use windows::Win32::Graphics::Gdi::DISP_CHANGE;

#[derive(Clone)]
pub struct FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub change_display_settings_error: Option<DISP_CHANGE>,
    pub commit_display_settings_changes_error: Option<DISP_CHANGE>,
    pub getting_primary_display_name_fails: bool,
}

impl FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub fn default() -> Self {
        Self {
            change_display_settings_error: None,
            commit_display_settings_changes_error: None,
            getting_primary_display_name_fails: false,
        }
    }
}
