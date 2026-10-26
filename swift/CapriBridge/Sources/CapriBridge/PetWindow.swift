import AppKit

/// How the pet window behaves in macOS window management.
///
/// Tauri's `visibleOnAllWorkspaces` only sets `.canJoinAllSpaces`; staying visible over
/// full-screen apps additionally needs `.fullScreenAuxiliary` (plus the app running with the
/// accessory activation policy, which the Rust side sets).
public enum PetWindowBehavior {
    public static let collectionBehavior: NSWindow.CollectionBehavior = [
        .canJoinAllSpaces,
        .fullScreenAuxiliary,
        .stationary,
        .ignoresCycle,
    ]

    /// Above normal and floating windows so full-screen apps don't cover the pet.
    public static let level: NSWindow.Level = .statusBar

    @MainActor
    public static func apply(to window: NSWindow) {
        window.collectionBehavior = collectionBehavior
        window.level = level
        window.hasShadow = false
        window.isOpaque = false
        window.backgroundColor = .clear
    }
}

/// Applies `PetWindowBehavior` to the NSWindow behind `windowPointer`.
///
/// - Parameter windowPointer: An unretained `NSWindow *`, as returned by Tauri's `ns_window()`.
/// - Returns: `false` if the pointer is null or the call is not on the main thread.
@_cdecl("capri_configure_pet_window")
public func capriConfigurePetWindow(_ windowPointer: UnsafeMutableRawPointer?) -> Bool {
    guard let windowPointer, Thread.isMainThread else { return false }
    let window = Unmanaged<NSWindow>.fromOpaque(windowPointer).takeUnretainedValue()
    MainActor.assumeIsolated {
        PetWindowBehavior.apply(to: window)
    }
    return true
}
