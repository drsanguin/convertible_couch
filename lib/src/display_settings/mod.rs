use convertible_couch_common::SwapPrimaryDisplaysResponse;

pub trait DisplaySettings<TDisplaySettingsApi> {
    fn new(display_settings_api: TDisplaySettingsApi) -> Self;

    fn swap_primary_displays(
        &mut self,
        desktop_display_name: &str,
        couch_display_name: &str,
    ) -> Result<SwapPrimaryDisplaysResponse, String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsDisplaySettings as Current;

#[cfg(target_os = "windows")]
pub use convertible_couch_common::win32::WindowsApiBasedWin32 as CurrentDisplaySettingsApi;

#[cfg(target_os = "windows")]
pub const INTERNAL_DISPLAY_NAME: &'static str = "Internal Display";
