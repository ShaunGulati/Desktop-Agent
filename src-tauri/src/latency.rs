//! Measures hotkey-to-first-paint latency for the M0 exit gate (pet visible in < 100 ms).

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The M0 gate: the pet must be painted within this long after the hotkey.
pub const SHOW_BUDGET: Duration = Duration::from_millis(100);

/// Tracks one in-flight "show the pet" request at a time.
#[derive(Default)]
pub struct ShowLatency {
    started_at: Mutex<Option<Instant>>,
}

impl ShowLatency {
    /// Records the moment the hotkey fired, replacing any unfinished measurement.
    pub fn start(&self, now: Instant) {
        *self.started_at.lock().expect("latency lock poisoned") = Some(now);
    }

    /// Ends the measurement when the frontend reports its first paint.
    ///
    /// Returns `None` if no show was in flight (e.g. a paint report after startup).
    pub fn finish(&self, now: Instant) -> Option<Duration> {
        let started_at = self
            .started_at
            .lock()
            .expect("latency lock poisoned")
            .take()?;
        Some(now.saturating_duration_since(started_at))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_time_between_start_and_finish() {
        let latency = ShowLatency::default();
        let t0 = Instant::now();
        latency.start(t0);
        assert_eq!(
            latency.finish(t0 + Duration::from_millis(42)),
            Some(Duration::from_millis(42))
        );
    }

    #[test]
    fn finish_without_start_reports_nothing() {
        assert_eq!(ShowLatency::default().finish(Instant::now()), None);
    }

    #[test]
    fn each_start_is_finished_only_once() {
        let latency = ShowLatency::default();
        let t0 = Instant::now();
        latency.start(t0);
        latency.finish(t0);
        assert_eq!(latency.finish(t0), None);
    }

    #[test]
    fn a_new_start_replaces_an_unfinished_one() {
        let latency = ShowLatency::default();
        let t0 = Instant::now();
        latency.start(t0);
        latency.start(t0 + Duration::from_millis(500));
        assert_eq!(
            latency.finish(t0 + Duration::from_millis(510)),
            Some(Duration::from_millis(10))
        );
    }
}
