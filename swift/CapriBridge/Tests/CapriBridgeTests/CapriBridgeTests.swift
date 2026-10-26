import AppKit
import Testing

@testable import CapriBridge

@Suite("Pet window behavior")
struct PetWindowBehaviorTests {
    @Test("joins every Space, including full-screen ones")
    func joinsAllSpacesIncludingFullScreen() {
        #expect(PetWindowBehavior.collectionBehavior.contains(.canJoinAllSpaces))
        #expect(PetWindowBehavior.collectionBehavior.contains(.fullScreenAuxiliary))
    }

    @Test("is skipped by Cmd-` window cycling")
    func ignoresWindowCycling() {
        #expect(PetWindowBehavior.collectionBehavior.contains(.ignoresCycle))
    }

    @Test("floats above normal and floating windows")
    func floatsAboveFloatingWindows() {
        #expect(PetWindowBehavior.level > .floating)
    }

    @Test("configuring a null window pointer fails safely")
    @MainActor
    func nullWindowPointerIsRejected() {
        #expect(capriConfigurePetWindow(nil) == false)
    }

    @Test("configures a real window on the main thread")
    @MainActor
    func configuresRealWindow() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 10, height: 10),
            styleMask: .borderless, backing: .buffered, defer: true)
        let pointer = Unmanaged.passUnretained(window).toOpaque()

        #expect(capriConfigurePetWindow(pointer))
        #expect(window.level == PetWindowBehavior.level)
        #expect(window.collectionBehavior == PetWindowBehavior.collectionBehavior)
        #expect(window.isOpaque == false)
    }
}

@Suite("C ABI helpers")
struct BridgeTests {
    @Test("reports the ABI version Rust expects")
    func abiVersion() {
        #expect(capriBridgeAbiVersion() == capriBridgeABIVersion)
    }

    @Test("frontmost app name round-trips through a C string")
    func frontmostAppNameIsFreeable() {
        // A headless CI session may have no frontmost app; both outcomes are valid.
        let pointer = capriFrontmostAppName()
        if let pointer {
            #expect(!String(cString: pointer).isEmpty)
        }
        capriFreeString(pointer)
    }
}
