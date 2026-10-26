/// Minimum macOS version; keep in sync with swift/CapriBridge/Package.swift and
/// bundle.macOS.minimumSystemVersion in tauri.conf.json.
#[cfg(target_os = "macos")]
const MACOS_MIN_VERSION: &str = "13.0";

fn main() {
    #[cfg(target_os = "macos")]
    {
        swift_rs::SwiftLinker::new(MACOS_MIN_VERSION)
            .with_package("CapriBridge", "../swift/CapriBridge")
            .link();
        // The Swift runtime (e.g. libswift_Concurrency) ships with macOS in /usr/lib/swift;
        // without this rpath every binary target, not just the lib tests, fails to launch.
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }

    tauri_build::build()
}
