#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::{
    env,
    fs::{copy, create_dir_all},
    path::Path,
    process::Command,
};

fn main() {
    let audio_endpoint_library_solution_path = Path::new(".").join("AudioEndPointLibrary");
    let audio_endpoint_library_project_path = Path::new(".")
        .join("AudioEndPointLibrary")
        .join("AudioEndPointLibrary.vcxproj");

    let audio_endpoint_library_project_path_as_str =
        audio_endpoint_library_project_path.to_str().unwrap();

    let output = Command::new("msbuild")
        .current_dir(audio_endpoint_library_solution_path)
        .args([
            audio_endpoint_library_project_path_as_str,
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

    let dll_source = Path::new(".")
        .join("AudioEndPointLibrary")
        .join("AudioEndPointLibrary")
        .join("bin")
        .join("x64")
        .join("Release")
        .join("AudioEndPointLibrary.dll");

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target"));
    let profile = env::var("PROFILE").unwrap();
    let dll_dest = Path::new("..")
        .join(&target_dir)
        .join(&profile)
        .join("AudioEndPointLibrary.dll");

    if !dll_source.exists() {
        panic!(
            "Expected DLL at {:?}, but it does not exist. Contents of directory:\n{:?}",
            dll_source,
            dll_source
                .parent()
                .unwrap()
                .read_dir()
                .unwrap()
                .collect::<Vec<_>>()
        );
    }

    let dll_dest_dir = Path::new("..").join(target_dir).join(profile);

    create_dir_all(&dll_dest_dir).expect("Failed to create dll_dest_dir");

    if !dll_dest_dir.exists() {
        panic!(
            "Expected dir at {:?}, but it does not exist. Contents of directory:\n{:?}",
            dll_dest_dir,
            dll_dest_dir
                .parent()
                .unwrap()
                .read_dir()
                .unwrap()
                .collect::<Vec<_>>()
        );
    }

    copy(dll_source, dll_dest).expect("Failed to copy AudioEndPointLibrary.dll");
}
