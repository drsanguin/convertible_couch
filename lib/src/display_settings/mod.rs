#[derive(Debug, PartialEq, Eq)]
pub struct DisplaySettingsResult {
    pub reboot_required: bool,
    pub new_primary: Option<String>,
}

pub trait DisplaySettings<TDisplaySettingsApi> {
    fn new(display_settings_api: TDisplaySettingsApi) -> Self;

    fn change_primary_display(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<DisplaySettingsResult, String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_display_settings::WindowsDisplaySettings as CurrentDisplaySettings;

#[cfg(target_os = "windows")]
pub use windows::win_32::WindowsApiBasedWin32 as CurrentDisplaySettingsApi;

#[cfg(target_os = "windows")]
pub const INTERNAL_DISPLAY_NAME: &'static str = "Internal Display";
