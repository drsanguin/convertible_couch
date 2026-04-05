use std::cmp::Ordering;

use crate::application_error::ApplicationError;
use crate::trace_fn;

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
        trace_fn!();

        other
            .is_primary
            .cmp(&self.is_primary)
            .then(self.name.cmp(&other.name))
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for DisplayInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        trace_fn!();

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

    fn get_displays_infos(&mut self) -> Result<Vec<DisplayInfo>, ApplicationError>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::windows_display_settings::WindowsDisplaySettings as CurrentDisplaysSettings;

#[cfg(target_os = "windows")]
pub use windows::win_32_based_windows_api::Win32BasedWindowsApi as CurrentDisplaysSettingsApi;

#[cfg(target_os = "windows")]
pub use windows::windows_api::WindowsApi as CurrentDisplaysSettingsApiTrait;

#[cfg(target_os = "windows")]
pub const INTERNAL_DISPLAY_NAME: &str = "Internal Display";
