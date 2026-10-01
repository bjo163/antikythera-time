use serde::{Deserialize, Serialize};
use mtime_calendar::{CalendarProfile, CriterionOutcome};
use mtime_hijri::HijriAstronomicalState;
use mtime_observation::ObservationReport;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Jurisdiction { pub id: String, pub label: String, pub scope: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorityDecision { pub id: String, pub authority: String, pub jurisdiction: Jurisdiction, pub decided_at_utc: String, pub result_label: String, pub source_document: String, pub cited_observations: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalResolution { pub astronomical: HijriAstronomicalState, pub profile: CalendarProfile, pub criterion: CriterionOutcome, pub observations: Vec<ObservationReport>, pub authority: Option<AuthorityDecision> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiffCategory { PhysicalInput, Ephemeris, Observer, Criterion, Observation, Jurisdiction, Authority, SourceVersion }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolutionDiff { pub categories: Vec<DiffCategory>, pub shared_facts: Vec<String>, pub explanation: String }

pub fn explain_difference(a: &TemporalResolution, b: &TemporalResolution) -> ResolutionDiff {
    let mut categories = vec![];
    let mut shared = vec![];
    if (a.astronomical.conjunction_jd_tt - b.astronomical.conjunction_jd_tt).abs() < 1e-9 { shared.push("same conjunction instant".into()) } else { categories.push(DiffCategory::PhysicalInput); }
    if a.astronomical.ephemeris_provenance != b.astronomical.ephemeris_provenance { categories.push(DiffCategory::Ephemeris); }
    if a.astronomical.observer != b.astronomical.observer { categories.push(DiffCategory::Observer); }
    if a.profile.id != b.profile.id || a.profile.version != b.profile.version || a.criterion.pass != b.criterion.pass { categories.push(DiffCategory::Criterion); }
    if a.observations != b.observations { categories.push(DiffCategory::Observation); }
    match (&a.authority, &b.authority) {
        (Some(x), Some(y)) => { if x.jurisdiction != y.jurisdiction { categories.push(DiffCategory::Jurisdiction); } if x.authority != y.authority || x.result_label != y.result_label { categories.push(DiffCategory::Authority); } },
        (None, None) => {},
        _ => categories.push(DiffCategory::Authority),
    }
    categories.sort_by_key(|x| format!("{:?}", x)); categories.dedup();
    let explanation = if categories.is_empty() { "No material difference detected in represented layers.".into() } else { format!("Results diverge at: {}.", categories.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")) };
    ResolutionDiff { categories, shared_facts: shared, explanation }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtime_calendar::{CalendarProfile, Criterion};
    use mtime_core::EarthObserver;
    fn resolution(profile: CalendarProfile) -> TemporalResolution { let mut s = HijriAstronomicalState::synthetic(3.1, 6.5); s.observer = EarthObserver::wgs84("Jakarta", 106.8, -6.2, 10.0).unwrap(); let criterion = profile.evaluate(&s); TemporalResolution { astronomical: s, profile, criterion, observations: vec![], authority: None } }
    #[test] fn explains_criterion_difference() { let a = resolution(CalendarProfile::mabims_2026()); let mut p = CalendarProfile::mabims_2026(); p.id = "ALT_ONLY".into(); p.criterion = Criterion::AltitudeAtLeast(2.0); let b = resolution(p); let d = explain_difference(&a, &b); assert!(d.categories.contains(&DiffCategory::Criterion)); assert!(d.shared_facts.iter().any(|x| x.contains("conjunction"))); }
}
