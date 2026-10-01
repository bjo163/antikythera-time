use mtime_authority::{AuthorityDecision, DecisionKind};
use mtime_core::QualityClass;
use mtime_explain::{explain_difference, official_vs_computed_conflict, DiffCategory, TemporalResolution};
use mtime_hijri::{
    computed_action, CalendarProfile, HijriAstronomicalState, INDONESIA_1447_PILOTS,
};
use mtime_observation::ObservationSummary;

fn state_from_case(index: usize, use_minima: bool) -> HijriAstronomicalState {
    let c = &INDONESIA_1447_PILOTS[index];
    HijriAstronomicalState {
        conjunction_jd_tt: None,
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: if use_minima { c.altitude_min_deg } else { c.altitude_max_deg },
        elongation_geocentric_deg: if use_minima { c.elongation_min_deg } else { c.elongation_max_deg },
        moon_age_hours: None,
        moon_lag_minutes: None,
        site_id: "INDONESIA_NATIONAL_RANGE".into(),
        ephemeris_source: c.source.into(),
        quality: QualityClass::Reference,
    }
}

fn resolution(index: usize, use_minima: bool, official: DecisionKind) -> TemporalResolution {
    let case = &INDONESIA_1447_PILOTS[index];
    let astronomy = state_from_case(index, use_minima);
    let criterion = CalendarProfile::mabims_indonesia_2026().evaluate(&astronomy);
    let computed_action = computed_action(&criterion);
    let summary = ObservationSummary {
        id: format!("{}-RUKYAT", case.id),
        date: case.observation_date.into(),
        jurisdiction_id: "ID".into(),
        reported_sites: case.observation_sites,
        accepted_positive_sightings: case.accepted_positive_sightings,
        source: case.source.into(),
        notes: Some(format!("Official month start: {}", case.official_month_start)),
    };
    TemporalResolution {
        id: case.id.into(),
        astronomy,
        criterion,
        computed_action,
        observations: vec![],
        observation_summary: Some(summary),
        authority: Some(AuthorityDecision::indonesia_sidang_isbat(
            format!("{}T12:00:00Z", case.observation_date),
            official,
            case.source,
        )),
    }
}

#[test]
fn ramadan_1447_replays_as_istikmal_without_semantic_conflict() {
    let r = resolution(0, false, DecisionKind::CompleteCurrentMonthTo30Days);
    assert_eq!(r.criterion.met, Some(false));
    assert!(!r.observation_summary.as_ref().unwrap().any_accepted_positive());
    assert!(!official_vs_computed_conflict(&r));
}

#[test]
fn shawwal_1447_replays_as_istikmal_without_semantic_conflict() {
    let r = resolution(1, false, DecisionKind::CompleteCurrentMonthTo30Days);
    assert_eq!(r.criterion.met, Some(false));
    assert_eq!(r.observation_summary.as_ref().unwrap().reported_sites, 117);
    assert!(!official_vs_computed_conflict(&r));
}

#[test]
fn dhulhijjah_1447_replays_as_new_month_with_positive_rukyat() {
    let r = resolution(2, true, DecisionKind::BeginNewMonth);
    assert_eq!(r.criterion.met, Some(true));
    assert!(r.observation_summary.as_ref().unwrap().any_accepted_positive());
    assert!(!official_vs_computed_conflict(&r));
}

#[test]
fn same_astronomy_but_different_official_decision_is_explained_as_authority() {
    let a = resolution(2, true, DecisionKind::BeginNewMonth);
    let mut b = a.clone();
    b.id = "COUNTERFACTUAL_AUTHORITY_ONLY".into();
    b.authority = Some(AuthorityDecision::indonesia_sidang_isbat(
        "2026-05-17T12:00:00Z",
        DecisionKind::CompleteCurrentMonthTo30Days,
        "counterfactual test authority decision",
    ));
    let d = explain_difference(&a, &b);
    assert!(d.shared_astronomy);
    assert!(d.categories.contains(&DiffCategory::Authority));
    assert!(!d.categories.contains(&DiffCategory::Criterion));
}
