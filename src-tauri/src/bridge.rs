//! Safe wrappers around the CapriBridge Swift library (swift/CapriBridge).
//!
//! Every macOS-only capability goes through this module so the rest of the core stays
//! portable; on other platforms the functions degrade to no-ops.

use std::fmt;

/// ABI version this build of the core was written against; must match the Swift side.
pub const EXPECTED_ABI_VERSION: i32 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum BridgeError {
    AbiMismatch { expected: i32, found: i32 },
    WindowUnavailable(String),
    ConfigureRejected,
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbiMismatch { expected, found } => write!(
                f,
                "CapriBridge ABI mismatch: core expects {expected}, library reports {found}"
            ),
            Self::WindowUnavailable(reason) => write!(f, "native window unavailable: {reason}"),
            Self::ConfigureRejected => {
                write!(
                    f,
                    "CapriBridge rejected the window (null pointer or off main thread)"
                )
            }
        }
    }
}

impl std::error::Error for BridgeError {}

#[cfg(target_os = "macos")]
mod ffi {
    use std::ffi::{c_char, c_void};

    unsafe extern "C" {
        pub fn capri_bridge_abi_version() -> i32;
        pub fn capri_configure_pet_window(window: *mut c_void) -> bool;
        pub fn capri_frontmost_app_name() -> *mut c_char;
        pub fn capri_free_string(pointer: *mut c_char);
    }
}

/// Fails if the linked Swift library was built for a different ABI version.
pub fn check_abi() -> Result<(), BridgeError> {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: takes no arguments and returns a plain integer.
        let found = unsafe { ffi::capri_bridge_abi_version() };
        if found != EXPECTED_ABI_VERSION {
            return Err(BridgeError::AbiMismatch {
                expected: EXPECTED_ABI_VERSION,
                found,
            });
        }
    }
    Ok(())
}

/// Makes the pet window float over every Space, including full-screen apps.
///
/// Must be called on the main thread (Tauri's `setup` hook runs there).
pub fn configure_pet_window<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
) -> Result<(), BridgeError> {
    #[cfg(target_os = "macos")]
    {
        let ns_window = window
            .ns_window()
            .map_err(|e| BridgeError::WindowUnavailable(e.to_string()))?;
        // SAFETY: `ns_window` is a live NSWindow owned by Tauri for the window's lifetime;
        // the Swift side does not retain it and checks for null and the main thread.
        if !unsafe { ffi::capri_configure_pet_window(ns_window) } {
            return Err(BridgeError::ConfigureRejected);
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = window;
    Ok(())
}

/// Localized name of the frontmost application, if there is one.
pub fn frontmost_app_name() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: returns null or a strdup'd, NUL-terminated string that we own and free below.
        unsafe {
            let pointer = ffi::capri_frontmost_app_name();
            if pointer.is_null() {
                return None;
            }
            let name = std::ffi::CStr::from_ptr(pointer)
                .to_string_lossy()
                .into_owned();
            ffi::capri_free_string(pointer);
            Some(name)
        }
    }
    #[cfg(not(target_os = "macos"))]
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_swift_library_matches_expected_abi() {
        assert_eq!(check_abi(), Ok(()));
    }

    #[test]
    fn frontmost_app_name_is_none_or_non_empty() {
        // Headless CI has no frontmost app; an interactive session has a named one.
        if let Some(name) = frontmost_app_name() {
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn abi_mismatch_message_names_both_versions() {
        let message = BridgeError::AbiMismatch {
            expected: 1,
            found: 2,
        }
        .to_string();
        assert!(message.contains("expects 1") && message.contains("reports 2"));
    }
}
