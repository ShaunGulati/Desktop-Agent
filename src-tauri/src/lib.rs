//! CAPRI core: the Tauri app that hosts the pet and, in later milestones, the agent.

mod bridge;
mod latency;
mod pet;
mod placement;
mod tray;

use tauri::App;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use latency::ShowLatency;

/// Global hotkey that shows or hides the pet.
pub const TOGGLE_PET_SHORTCUT: &str = "Alt+Space";

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Accessory apps have no Dock icon and may float over other apps' full-screen Spaces.
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    bridge::check_abi()?;
    eprintln!(
        "[capri] Swift bridge ready (ABI {}); frontmost app: {}",
        bridge::EXPECTED_ABI_VERSION,
        bridge::frontmost_app_name().as_deref().unwrap_or("none"),
    );

    let window = pet::pet_window(app.handle())?;
    if let Err(error) = bridge::configure_pet_window(&window) {
        // The pet still works, it just won't cover full-screen apps.
        eprintln!("[capri] {error}");
    }
    pet::place_in_corner(&window)?;
    window.show()?;

    tray::create(app)?;
    app.global_shortcut()
        .on_shortcut(TOGGLE_PET_SHORTCUT, |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed
                && let Err(error) = pet::toggle(app)
            {
                eprintln!("[capri] could not toggle the pet: {error}");
            }
        })?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(ShowLatency::default())
        .invoke_handler(tauri::generate_handler![pet::report_pet_painted])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running CAPRI");
}
