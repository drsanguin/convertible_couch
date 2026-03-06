use std::cmp::Ordering;

use crate::application_error::ApplicationError;

#[derive(Debug, PartialEq, Eq)]
pub struct DisplaysSettingsResult {
    pub reboot_required: bool,
    pub new_primary_display: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DisplayInfo {
    pub is_primary: bool,
    pub name: String,
}

impl Ord for DisplayInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .is_primary
            .cmp(&self.is_primary)
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for DisplayInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait DisplaysSettings {
    fn new(displays_settings_api: Box<dyn CurrentDisplaysSettingsApiTrait>) -> Self;

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaysSettingsResult, ApplicationError>;

    fn get_displays_infos(&self) -> Result<Vec<DisplayInfo>, ApplicationError>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_displays_settings::WindowsDisplaySettings as CurrentDisplaysSettings;

#[cfg(target_os = "windows")]
pub use windows::win_32::windows_api_based_win_32::WindowsApiBasedWin32 as CurrentDisplaysSettingsApi;

#[cfg(target_os = "windows")]
pub use windows::win_32::Win32 as CurrentDisplaysSettingsApiTrait;

#[cfg(target_os = "windows")]
pub const INTERNAL_DISPLAY_NAME: &str = "Internal Display";
