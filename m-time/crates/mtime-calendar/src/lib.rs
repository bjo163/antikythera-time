use serde::{Deserialize, Serialize};
use mtime_hijri::HijriAstronomicalState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Criterion { AltitudeAtLeast(f64), ElongationAtLeast(f64), All(Vec<Criterion>), Any(Vec<Criterion>) }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CriterionOutcome { pub pass: bool, pub clauses: Vec<String> }
impl Criterion {
    pub fn evaluate(&self, s: &HijriAstronomicalState) -> CriterionOutcome {
        match self {
            Criterion::AltitudeAtLeast(x) => CriterionOutcome { pass: s.moon_altitude_deg >= *x, clauses: vec![format!("moon_altitude_deg {} >= {}", s.moon_altitude_deg, x)] },
            Criterion::ElongationAtLeast(x) => CriterionOutcome { pass: s.elongation_deg >= *x, clauses: vec![format!("elongation_deg {} >= {}", s.elongation_deg, x)] },
            Criterion::All(v) => { let mut clauses = vec![]; let mut pass = true; for c in v { let o = c.evaluate(s); pass &= o.pass; clauses.extend(o.clauses); } CriterionOutcome { pass, clauses } },
            Criterion::Any(v) => { let mut clauses = vec![]; let mut pass = false; for c in v { let o = c.evaluate(s); pass |= o.pass; clauses.extend(o.clauses); } CriterionOutcome { pass, clauses } },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarProfile { pub id: String, pub version: String, pub criterion: Criterion, pub complete_to_30_if_fail: bool, pub provenance: String }
impl CalendarProfile {
    pub fn mabims_2026() -> Self { Self { id: "MABIMS".into(), version: "2026-PMA-1".into(), criterion: Criterion::All(vec![Criterion::AltitudeAtLeast(3.0), Criterion::ElongationAtLeast(6.4)]), complete_to_30_if_fail: true, provenance: "Indonesia PMA No. 1/2026 / MABIMS criterion".into() } }
    pub fn evaluate(&self, s: &HijriAstronomicalState) -> CriterionOutcome { self.criterion.evaluate(s) }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn state(a: f64, e: f64) -> HijriAstronomicalState { HijriAstronomicalState::synthetic(a, e) }
    #[test] fn mabims_pass_fail() { let p = CalendarProfile::mabims_2026(); assert!(p.evaluate(&state(3.1, 6.5)).pass); assert!(!p.evaluate(&state(2.9, 6.5)).pass); assert!(!p.evaluate(&state(3.1, 6.3)).pass); }
}
