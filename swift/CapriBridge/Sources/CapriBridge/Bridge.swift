import AppKit

/// Bumped whenever an exported function's signature changes, so the Rust side can refuse to
/// run against a stale build of this library.
public let capriBridgeABIVersion: Int32 = 1

@_cdecl("capri_bridge_abi_version")
public func capriBridgeAbiVersion() -> Int32 {
    capriBridgeABIVersion
}

/// Name of the frontmost application, or null when there is none (e.g. no GUI session).
///
/// The returned C string is heap-allocated; release it with `capri_free_string`.
@_cdecl("capri_frontmost_app_name")
public func capriFrontmostAppName() -> UnsafeMutablePointer<CChar>? {
    guard let name = NSWorkspace.shared.frontmostApplication?.localizedName else { return nil }
    return strdup(name)
}

@_cdecl("capri_free_string")
public func capriFreeString(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}
