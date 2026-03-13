use std::io::Result;

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    use windows::Win32::System::SystemServices::LANG_ENGLISH;
    use winresource::WindowsResource;

    WindowsResource::new()
        .set_language(LANG_ENGLISH as u16)
        .set_icon("icon.ico")
        .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
        .compile()
}
