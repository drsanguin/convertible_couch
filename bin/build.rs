cfg_select! {
    target_os = "windows" => {
        use std::io::Result;
        use windows::Win32::System::SystemServices::LANG_ENGLISH;
        use winresource::WindowsResource;

        fn main() -> Result<()> {
            WindowsResource::new()
                .set_language(LANG_ENGLISH as u16)
                .set_icon("icon.ico")
                .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
                .compile()
        }
    }
}
