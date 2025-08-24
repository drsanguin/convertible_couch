#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::io::Result;
use winresource::WindowsResource;

const ENGLISH_LANGUAGE_ID: u16 = 0x0009;

fn main() -> Result<()> {
    WindowsResource::new()
        .set_language(ENGLISH_LANGUAGE_ID)
        .set_icon("icon.ico")
        .compile()
}
