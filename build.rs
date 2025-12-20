use std::path::PathBuf;
use std::process::Command;

fn main() {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build_dir = project_dir.join("build");
    let lib_dir = project_dir.join("lib");

    let status = Command::new("/usr/bin/cmake")
        .current_dir(&project_dir)
        .args([
            "-DCMAKE_BUILD_TYPE:STRING=Debug",
            "-DCMAKE_EXPORT_COMPILE_COMMANDS:BOOL=TRUE",
            "-DCMAKE_C_COMPILER:FILEPATH=/usr/bin/gcc",
            "-DCMAKE_CXX_COMPILER:FILEPATH=/usr/bin/g++",
            "--no-warn-unused-cli",
            "-S",
            project_dir.to_str().unwrap(),
            "-B",
            build_dir.to_str().unwrap(),
            "-G",
            "Ninja",
        ])
        .status()
        .expect("Failed to run cmake");
    assert!(status.success(), "CMake configuration failed");

    let status = Command::new("/usr/bin/cmake")
        .args([
            "--build",
            build_dir.to_str().unwrap(),
            "--config",
            "Debug",
            "--target",
            "install",
            "--",
        ])
        .status()
        .expect("Failed to build library");
    assert!(status.success(), "CMake configuration failed");

    // Tell cargo where to find the library
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=akonadi_helper");

    // Set rpath so the library can be found at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());

    // Re-run if helper code changes
    println!("cargo:rerun-if-changed=helper/");
}
