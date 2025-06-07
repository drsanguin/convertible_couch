use std::{env, fs::copy, path::Path, process::Command};

fn main() {
    let output = Command::new("msbuild")
        .current_dir("..\\AudioEndPointLibrary")
        .args([
            ".\\AudioEndPointLibrary\\AudioEndPointLibrary.vcxproj",
            "/p:Configuration=Release",
            "/p:Platform=x64",
        ])
        .output()
        .expect("Failed to compile AudioEndPointLibrary");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));

        panic!("Failed to compile AudioEndPointLibrary");
    }

    let dll_source = Path::new("..")
        .join("AudioEndPointLibrary")
        .join("AudioEndPointLibrary")
        .join("bin")
        .join("x64")
        .join("Release")
        .join("AudioEndPointLibrary.dll");

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target"));
    let profile = env::var("PROFILE").unwrap();
    let dll_dest = Path::new("..")
        .join(target_dir)
        .join(profile)
        .join("AudioEndPointLibrary.dll");

    copy(dll_source, dll_dest).expect("Failed to copy AudioEndPointLibrary.dll");
}
