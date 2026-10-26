//! The pet window: placement, show/hide, and first-paint latency reporting.

use std::time::Instant;

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Runtime, State, WebviewWindow};

use crate::latency::{SHOW_BUDGET, ShowLatency};
use crate::placement::{self, Rect};

/// Label of the pet window in tauri.conf.json.
pub const PET_WINDOW_LABEL: &str = "pet";

/// Emitted to the pet webview right after it is shown, so it can report its first paint.
pub const PET_SHOWN_EVENT: &str = "pet://shown";

pub fn pet_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
    app.get_webview_window(PET_WINDOW_LABEL)
        .ok_or_else(|| tauri::Error::WindowNotFound)
}

/// Moves the pet to the bottom-right corner of the monitor it is on (or the primary one).
pub fn place_in_corner<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    let Some(monitor) = window.current_monitor()?.or(window.primary_monitor()?) else {
        return Ok(());
    };
    let area = monitor.work_area();
    let work_area = Rect {
        x: area.position.x,
        y: area.position.y,
        width: area.size.width,
        height: area.size.height,
    };
    let size = window.outer_size()?;
    let (x, y) =
        placement::bottom_right(work_area, size.width, size.height, monitor.scale_factor());
    window.set_position(PhysicalPosition::new(x, y))
}

/// Shows the pet if hidden, hides it if visible.
pub fn toggle<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let window = pet_window(app)?;
    if window.is_visible()? {
        return window.hide();
    }
    app.state::<ShowLatency>().start(Instant::now());
    window.show()?;
    window.emit(PET_SHOWN_EVENT, ())
}

/// Called by the pet webview once it has painted after a show; logs the latency.
///
/// Returns the measured latency in milliseconds, or `None` if no show was in flight.
#[tauri::command]
pub fn report_pet_painted(latency: State<'_, ShowLatency>) -> Option<f64> {
    let elapsed = latency.finish(Instant::now())?;
    let millis = elapsed.as_secs_f64() * 1000.0;
    let verdict = if elapsed <= SHOW_BUDGET {
        "within"
    } else {
        "OVER"
    };
    eprintln!(
        "[capri] hotkey to first paint: {millis:.1} ms ({verdict} {} ms budget)",
        SHOW_BUDGET.as_millis()
    );
    Some(millis)
}
