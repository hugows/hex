//! A passive, bare middle-button hold. Never consumes or replays mouse events.
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use crate::audio::CaptureInstant;

const HOLD: Duration = Duration::from_millis(500);
static GESTURE: LazyLock<Mutex<Gesture>> = LazyLock::new(|| Mutex::new(Gesture::default()));

#[derive(Default)]
struct Gesture {
    down: bool,
    since: Option<CaptureInstant>,
    ready: Option<CaptureInstant>,
    origin: (f64, f64),
}

impl Gesture {
    fn press(&mut self, at: CaptureInstant, eligible: bool, point: (f64, f64)) {
        if !self.down {
            self.down = true;
            self.origin = point;
            self.since = eligible.then_some(at);
        }
    }

    fn cancel(&mut self) {
        self.since = None;
    }

    fn release(&mut self, at: CaptureInstant) {
        if self.since.is_some_and(|since| {
            at.checked_duration_since(since)
                .is_some_and(|age| age >= HOLD)
        }) {
            self.ready = Some(at);
        }
        self.down = false;
        self.cancel();
    }

    fn drag(&mut self, point: (f64, f64)) {
        let dx = point.0 - self.origin.0;
        let dy = point.1 - self.origin.1;
        if dx * dx + dy * dy > 36.0 {
            self.cancel();
        }
    }

    fn poll(&mut self, now: CaptureInstant, enabled: bool, held: bool) -> Option<CaptureInstant> {
        if !enabled {
            self.ready = None;
            self.cancel();
            return None;
        }
        if let Some(at) = self.ready.take() {
            return Some(at);
        }
        if !held {
            // The native state query can lead the queued release callback,
            // or briefly lag a down callback. Keep the observed timestamp so
            // the release can still qualify the hold; never fire while up.
            self.down = false;
            return None;
        }
        let since = self.since?;
        self.down = true;
        if now.checked_duration_since(since)? < HOLD {
            return None;
        }
        self.cancel();
        Some(now)
    }
}

/// Called once per native session event, whichever existing tap receives it.
pub(crate) fn observe(kind: u32, button: i64, flags: u64, at: CaptureInstant, point: (f64, f64)) {
    let mut gesture = GESTURE.lock().unwrap_or_else(|e| e.into_inner());
    match kind {
        25 if button == 2 => gesture.press(
            at,
            crate::labs::middle_mouse_dictation()
                && !crate::app_settings::shortcuts_suspended()
                && !crate::app_settings::runtime_hotkeys()
                    .all()
                    .iter()
                    .flatten()
                    .any(|binding| {
                        binding.key_code == crate::app_settings::HotkeyKey::mouse_code(2)
                    })
                && flags & ((1 << 17) | (1 << 18) | (1 << 19) | (1 << 20) | (1 << 23)) == 0,
            point,
        ),
        26 if button == 2 => gesture.release(at),
        27 if button == 2 => gesture.drag(point),
        // Keys, modifiers, another button, drag, scroll, or tap loss cancel
        // this attempt until the middle button is released and pressed again.
        1 | 3 | 10 | 12 | 22 | 25 | 27 | 0xffff_fffe | 0xffff_ffff => gesture.cancel(),
        _ => {}
    }
}

pub(crate) fn reset() {
    *GESTURE.lock().unwrap_or_else(|e| e.into_inner()) = Gesture::default();
}

pub(crate) fn poll(available: bool) -> Option<CaptureInstant> {
    GESTURE.lock().unwrap_or_else(|e| e.into_inner()).poll(
        CaptureInstant::now(),
        available && crate::labs::middle_mouse_dictation(),
        crate::suppression::middle_mouse_held(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(ms: u64) -> CaptureInstant {
        CaptureInstant::from_nanos(ms * 1_000_000)
    }

    #[test]
    fn short_click_does_not_toggle_and_hold_fires_only_once() {
        let mut gesture = Gesture::default();
        gesture.press(at(0), true, (0.0, 0.0));
        assert_eq!(gesture.poll(at(499), true, true), None);
        assert_eq!(gesture.poll(at(500), true, true), Some(at(500)));
        assert_eq!(gesture.poll(at(2000), true, true), None);
        gesture.press(at(2100), true, (0.0, 0.0));
        assert_eq!(
            gesture.poll(at(3000), true, true),
            None,
            "repeat down cannot rearm"
        );
        gesture = Gesture::default();
        gesture.press(at(3100), true, (0.0, 0.0));
        assert_eq!(gesture.poll(at(3600), true, true), Some(at(3600)));
    }

    #[test]
    fn releasing_a_short_click_leaves_nothing_to_fire() {
        let mut gesture = Gesture::default();
        gesture.press(at(0), true, (0.0, 0.0));
        assert_eq!(gesture.poll(at(200), true, true), None);
        gesture.release(at(200));
        assert_eq!(gesture.poll(at(900), true, true), None);
    }

    #[test]
    fn two_holds_finish_even_when_the_second_threshold_is_polled_after_release() {
        let mut gesture = Gesture::default();
        gesture.press(at(0), true, (100.0, 100.0));
        assert_eq!(gesture.poll(at(500), true, true), Some(at(500)));
        gesture.release(at(700));
        assert_eq!(gesture.poll(at(710), true, false), None);
        gesture.press(at(2000), true, (100.0, 100.0));
        gesture.drag((102.0, 103.0));
        gesture.release(at(2700));
        assert_eq!(gesture.poll(at(2800), true, false), Some(at(2700)));
        assert_eq!(gesture.poll(at(2900), true, false), None);
    }

    #[test]
    fn physical_state_queries_do_not_erase_queued_button_edges() {
        let mut gesture = Gesture::default();
        gesture.press(at(0), true, (0.0, 0.0));
        assert_eq!(gesture.poll(at(1), true, false), None);
        assert_eq!(gesture.poll(at(500), true, true), Some(at(500)));
        gesture.release(at(700));
        gesture.press(at(1000), true, (0.0, 0.0));
        // A released physical button can be read before its callback arrives.
        assert_eq!(gesture.poll(at(1700), true, false), None);
        gesture.release(at(1600));
        assert_eq!(gesture.poll(at(1800), true, false), Some(at(1600)));
    }

    #[test]
    fn a_real_drag_and_a_disabled_lab_discard_the_pending_hold() {
        let mut gesture = Gesture::default();
        gesture.press(at(0), true, (0.0, 0.0));
        gesture.drag((10.0, 0.0));
        gesture.release(at(700));
        assert_eq!(gesture.poll(at(800), true, false), None);
        gesture.press(at(1000), true, (0.0, 0.0));
        gesture.release(at(1700));
        assert_eq!(gesture.poll(at(1800), false, false), None);
        assert_eq!(gesture.poll(at(1900), true, false), None);
    }

    #[test]
    fn cancelled_disabled_and_modified_holds_wait_for_a_fresh_press() {
        for enabled in [false, true] {
            let mut gesture = Gesture::default();
            gesture.press(at(0), true, (0.0, 0.0));
            if enabled {
                gesture.cancel();
            }
            assert_eq!(gesture.poll(at(600), enabled, true), None);
            assert_eq!(gesture.poll(at(900), true, true), None);
        }
        let mut gesture = Gesture::default();
        gesture.press(at(0), false, (0.0, 0.0));
        assert_eq!(gesture.poll(at(900), true, true), None);
    }
}
