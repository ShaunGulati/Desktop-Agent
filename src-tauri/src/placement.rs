//! Where the pet window appears on screen.

/// Gap between the pet and the edges of the usable screen area, in logical points.
pub const SCREEN_MARGIN: f64 = 24.0;

/// A rectangle in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Top-left position that puts a window in the bottom-right corner of `work_area`.
///
/// `work_area` excludes the menu bar and Dock. The margin is in logical points and is
/// scaled by `scale_factor`. A window larger than the work area is pinned to its top-left.
pub fn bottom_right(
    work_area: Rect,
    window_width: u32,
    window_height: u32,
    scale_factor: f64,
) -> (i32, i32) {
    let margin = (SCREEN_MARGIN * scale_factor).round() as i64;
    let right = i64::from(work_area.x) + i64::from(work_area.width);
    let bottom = i64::from(work_area.y) + i64::from(work_area.height);
    let x = (right - i64::from(window_width) - margin).max(i64::from(work_area.x));
    let y = (bottom - i64::from(window_height) - margin).max(i64::from(work_area.y));
    (x as i32, y as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAPTOP: Rect = Rect {
        x: 0,
        y: 50,
        width: 3024,
        height: 1800,
    };

    #[test]
    fn places_window_in_bottom_right_with_scaled_margin() {
        // Retina: 24 pt margin = 48 px.
        assert_eq!(
            bottom_right(LAPTOP, 320, 320, 2.0),
            (3024 - 320 - 48, 50 + 1800 - 320 - 48)
        );
    }

    #[test]
    fn respects_work_area_offset_on_secondary_monitor() {
        let left_monitor = Rect {
            x: -1920,
            y: 0,
            width: 1920,
            height: 1080,
        };
        assert_eq!(
            bottom_right(left_monitor, 160, 160, 1.0),
            (-1920 + 1920 - 160 - 24, 1080 - 160 - 24)
        );
    }

    #[test]
    fn pins_oversized_window_to_work_area_origin() {
        let tiny = Rect {
            x: 10,
            y: 20,
            width: 100,
            height: 100,
        };
        assert_eq!(bottom_right(tiny, 500, 500, 1.0), (10, 20));
    }
}
