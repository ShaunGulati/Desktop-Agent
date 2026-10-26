// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "CapriBridge",
    // Keep in sync with SwiftLinker::new(...) in src-tauri/build.rs and
    // bundle.macOS.minimumSystemVersion in src-tauri/tauri.conf.json.
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "CapriBridge", type: .static, targets: ["CapriBridge"])
    ],
    targets: [
        .target(name: "CapriBridge"),
        .testTarget(name: "CapriBridgeTests", dependencies: ["CapriBridge"]),
    ]
)
