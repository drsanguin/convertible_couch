pub mod audio_endpoint_library;
pub mod win32;

#[derive(Debug, PartialEq)]
pub struct SwapPrimaryDisplaysResponse {
    pub reboot_required: bool,
    pub new_primary: Option<String>,
}
