#[cfg(target_vendor = "apple")]
fn main() {
    use std::{env, process::Command};

    // Setup
    const LIBRARY_NAME: &str = "AppleBindings";

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let is_release_mode = env::var("PROFILE").unwrap() == "release";

    let library_path = project_root + "/swift/";
    let bridging_header_path =
        library_path.to_string() + "/Sources/" + LIBRARY_NAME + "/bridging-header.h";
    let swift_build_path =
        library_path.to_string() + "/.build/" + if is_release_mode { "release" } else { "debug" };

    let target_triple = env::var("TARGET")
        .unwrap()
        .replace("aarch64", "arm64") // Map the Rust target triple to a Swift target triple
        .replace("ios", "ios14")
        .replace("darwin", "macosx11");

    // Get path to SDK, for macOS or iOS depending on the target
    let platform = if target_triple.contains("ios") {
        "iphoneos"
    } else {
        "macosx"
    };

    let output = Command::new("xcrun")
        .arg("--sdk")
        .arg(platform)
        .arg("--show-sdk-path")
        .output()
        .unwrap();
    let sdk = String::from_utf8(output.stdout.as_slice().into())
        .unwrap()
        .trim()
        .to_string();

    // Compile the Swift package
    let mut swift_args = vec![
        "build",
        "-Xswiftc",
        "-target",
        "-Xswiftc",
        &target_triple,
        "--sdk",
        &sdk,
        "--package-path",
        &library_path,
        "-Xswiftc",
        "-static",
        "-Xswiftc",
        "-import-objc-header",
        "-Xswiftc",
        &bridging_header_path,
    ];

    if is_release_mode {
        swift_args.push("-c");
        swift_args.push("release");
    }

    let build_status = Command::new("swift")
        .args(swift_args)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if !build_status.status.success() {
        panic!(
            "Swift failed to compile the project! Stdout:\n{}\n\nStderr:\n{}",
            String::from_utf8(build_status.stderr).unwrap(),
            String::from_utf8(build_status.stdout).unwrap()
        );
    }

    // Find XCode
    let xcode_path = match Command::new("xcode-select").arg("--print-path").output() {
        Ok(output) => {
            String::from_utf8(output.stdout.as_slice().into())
                .unwrap()
                .trim()
                .to_string()
                + "/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/"
                + platform
        }
        Err(e) => {
            panic!("Failed to detect XCode installation: {}", e);
        }
    };

    // Tell Cargo to statically link the Swift package
    println!("cargo:rustc-link-lib=static={}", LIBRARY_NAME); // tells cargo to include the compiled swift lib
    println!("cargo:rustc-link-search={}", swift_build_path); // should find the lib here
    println!("cargo:rustc-link-search={}", xcode_path);
    println!("cargo:rustc-link-search=/usr/lib/swift");
}

#[cfg(not(target_vendor = "apple"))]
fn main() {}
