use crate::ApplicationError;

#[derive(Debug, PartialEq, Eq)]
pub struct DisplaysSettingsResult {
    pub reboot_required: bool,
    pub new_primary_display: String,
}

pub trait DisplaysSettings<TDisplaysSettingsApi> {
    fn new(displays_settings_api: TDisplaysSettingsApi) -> Self;

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaysSettingsResult, ApplicationError>;
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
