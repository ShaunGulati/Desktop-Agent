//! Menu-bar icon. CAPRI runs without a Dock icon, so this is how the user quits it.

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle};

use crate::pet;

const TOGGLE_ID: &str = "toggle-pet";
const QUIT_ID: &str = "quit";

pub fn create(app: &App) -> tauri::Result<()> {
    let toggle = MenuItem::with_id(
        app,
        TOGGLE_ID,
        "Show or Hide Pet",
        true,
        Some(crate::TOGGLE_PET_SHORTCUT),
    )?;
    let quit = MenuItem::with_id(app, QUIT_ID, "Quit CAPRI", true, Some("CmdOrCtrl+Q"))?;
    let menu = Menu::with_items(app, &[&toggle, &quit])?;

    let mut builder = TrayIconBuilder::with_id("capri")
        .tooltip("CAPRI")
        .menu(&menu)
        .on_menu_event(handle_menu_event);
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        TOGGLE_ID => {
            if let Err(error) = pet::toggle(app) {
                eprintln!("[capri] could not toggle the pet: {error}");
            }
        }
        QUIT_ID => app.exit(0),
        _ => {}
    }
}
