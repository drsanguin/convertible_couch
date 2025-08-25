use windows::Win32::Graphics::Gdi::DISP_CHANGE;

#[derive(Clone)]
pub struct FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub change_display_settings_error_on_commit: Option<DISP_CHANGE>,
    pub change_display_settings_error_by_display: Option<DISP_CHANGE>,
    pub getting_primary_display_name_fails: bool,
}

impl FuzzedWindowsDisplaysSettingsApiBehaviour {
    pub fn default() -> Self {
        Self {
            change_display_settings_error_on_commit: None,
            change_display_settings_error_by_display: None,
            getting_primary_display_name_fails: false,
        }
    }
}
