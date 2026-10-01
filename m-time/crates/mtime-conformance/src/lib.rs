use mtime_antikythera::saros;
use mtime_calendar::CalendarProfile;
use mtime_hijri::HijriAstronomicalState;

#[derive(Debug, Clone, PartialEq)]
pub struct ConformanceReport { pub pass: bool, pub checks: Vec<(&'static str, bool)> }
pub fn run_core_conformance() -> ConformanceReport {
    let checks = vec![
        ("saros recurrence", (saros().recurrence(2460409.263, 1) - 2466994.5853).abs() < 1e-9),
        ("mabims pass", CalendarProfile::mabims_2026().evaluate(&HijriAstronomicalState::synthetic(3.0, 6.4)).pass),
        ("mabims altitude fail", !CalendarProfile::mabims_2026().evaluate(&HijriAstronomicalState::synthetic(2.99, 6.4)).pass),
    ];
    ConformanceReport { pass: checks.iter().all(|x| x.1), checks }
}
#[cfg(test)] mod tests { use super::*; #[test] fn core_vectors_pass() { assert!(run_core_conformance().pass) } }
