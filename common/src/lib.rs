pub mod win32;

#[derive(Debug, PartialEq)]
pub struct SwapPrimaryMonitorsResponse {
    pub reboot_required: bool,
    pub new_primary: Option<String>,
}
