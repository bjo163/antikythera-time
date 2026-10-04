#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockLockState {
    Unsynchronized,
    Locked,
    Holdover,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OscillatorCharacterization {
    pub frequency_error_ppm: f64,
    pub pps_capture_jitter_ns_p95: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceLock {
    pub source_id: String,
    pub lock_state: ClockLockState,
    pub last_sync_unix_seconds: i64,
    pub estimated_error_seconds: f64,
    pub oscillator: OscillatorCharacterization,
}

impl ReferenceLock {
    #[must_use]
    pub fn enter_holdover(&self, now_unix_seconds: i64) -> Self {
        let elapsed = (now_unix_seconds - self.last_sync_unix_seconds).max(0) as f64;
        let drift = elapsed * self.oscillator.frequency_error_ppm.abs() * 1e-6;
        Self {
            source_id: self.source_id.clone(),
            lock_state: ClockLockState::Holdover,
            last_sync_unix_seconds: self.last_sync_unix_seconds,
            estimated_error_seconds: self.estimated_error_seconds + drift,
            oscillator: self.oscillator,
        }
    }

    #[must_use]
    pub fn with_stale_threshold(self, max_error_seconds: f64) -> Self {
        if self.estimated_error_seconds > max_error_seconds {
            Self { lock_state: ClockLockState::Stale, ..self }
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSelfTest {
    pub clock_engine_ok: bool,
    pub reference_input_ok: bool,
    pub bundle_integrity_ok: bool,
    pub display_ok: bool,
}

impl DeviceSelfTest {
    #[must_use]
    pub const fn passed(self) -> bool {
        self.clock_engine_ok
            && self.reference_input_ok
            && self.bundle_integrity_ok
            && self.display_ok
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MClockDeviceStatus {
    pub firmware_id: String,
    pub reference: ReferenceLock,
    pub bundle_id: Option<String>,
    pub bundle_age_seconds: Option<u64>,
    pub self_test: DeviceSelfTest,
}

impl MClockDeviceStatus {
    #[must_use]
    pub fn safe_to_label_synchronized(&self) -> bool {
        self.self_test.passed()
            && self.reference.lock_state == ClockLockState::Locked
            && self.reference.estimated_error_seconds.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holdover_error_grows_from_characterized_oscillator() {
        let locked = ReferenceLock {
            source_id: "GNSS-PPS".into(),
            lock_state: ClockLockState::Locked,
            last_sync_unix_seconds: 1000,
            estimated_error_seconds: 1e-6,
            oscillator: OscillatorCharacterization {
                frequency_error_ppm: 1.0,
                pps_capture_jitter_ns_p95: 50.0,
            },
        };
        let h = locked.enter_holdover(4600);
        assert_eq!(h.lock_state, ClockLockState::Holdover);
        assert!((h.estimated_error_seconds - 0.003601).abs() < 1e-12);
    }

    #[test]
    fn stale_state_is_visible_not_silently_locked() {
        let h = ReferenceLock {
            source_id: "GNSS-PPS".into(),
            lock_state: ClockLockState::Holdover,
            last_sync_unix_seconds: 0,
            estimated_error_seconds: 2.0,
            oscillator: OscillatorCharacterization {
                frequency_error_ppm: 1.0,
                pps_capture_jitter_ns_p95: 50.0,
            },
        }
        .with_stale_threshold(1.0);
        assert_eq!(h.lock_state, ClockLockState::Stale);
    }
}
