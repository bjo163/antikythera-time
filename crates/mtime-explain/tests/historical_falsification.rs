use mtime_authority::{AuthorityDecision, DecisionKind, Jurisdiction};
use mtime_core::{EvidenceState, Provenance, QualityClass};
use mtime_explain::{
    audit_cross_jurisdiction_replay, audit_historical_resolution, ReplayVerdict,
    TemporalResolution,
};
use mtime_hijri::{
    computed_action, CalendarProfile, CalendarResult, CalendarResultKind, ComputedMonthAction,
    CriterionResult, GeometrySemantics, HijriAstronomicalState, HijriDate, HijriMonth,
    RuleIfNotMet, INDONESIA_1447_PILOTS,
};
use mtime_observation::ObservationSummary;

fn indonesia_resolution(index: usize, use_minima: bool, decision: DecisionKind) -> TemporalResolution {
    let case = &INDONESIA_1447_PILOTS[index];
    let astronomy = HijriAstronomicalState {
        conjunction_jd_tt: None,
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: if use_minima {
            case.altitude_min_deg
        } else {
            case.altitude_max_deg
        },
        elongation_geocentric_deg: if use_minima {
            case.elongation_min_deg
        } else {
            case.elongation_max_deg
        },
        geometry_semantics: GeometrySemantics::mabims_required(),
        moon_age_hours: None,
        moon_lag_minutes: None,
        site_id: "INDONESIA_NATIONAL_RANGE".into(),
        ephemeris_source: case.source.into(),
        quality: QualityClass::Reference,
    };
    let criterion = CalendarProfile::mabims_indonesia_2026().evaluate(&astronomy);
    let computed = computed_action(&criterion);
    let month = match index {
        0 => HijriMonth::Ramadan,
        1 => HijriMonth::Shawwal,
        2 => HijriMonth::DhulHijjah,
        _ => panic!("unknown Indonesia pilot case"),
    };

    TemporalResolution {
        id: format!("ID-{}", case.id),
        astronomy,
        criterion,
        computed_action: computed,
        observations: vec![],
        observation_summary: Some(ObservationSummary {
            id: format!("{}-RUKYAT", case.id),
            date: case.observation_date.into(),
            jurisdiction_id: "ID".into(),
            reported_sites: case.observation_sites,
            accepted_positive_sightings: case.accepted_positive_sightings,
            source: case.source.into(),
            notes: None,
        }),
        authority: Some(AuthorityDecision::indonesia_sidang_isbat(
            format!("{}T12:00:00Z", case.observation_date),
            decision,
            case.source,
        )),
        calendar_result: Some(CalendarResult {
            hijri_date: HijriDate::new(1447, month, 1).unwrap(),
            civil_date: case.official_month_start.into(),
            kind: CalendarResultKind::OfficialAuthority,
            profile_id: Some("MABIMS_ID_2026".into()),
            authority_id: Some("ID-KEMENAG-ISBAT".into()),
            provenance: case.source.into(),
        }),
    }
}

fn turkiye_shawwal_resolution() -> TemporalResolution {
    let astronomy = HijriAstronomicalState {
        conjunction_jd_tt: None,
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: 6.666_666_7,
        elongation_geocentric_deg: 8.0,
        geometry_semantics: GeometrySemantics::mabims_required(),
        moon_age_hours: None,
        moon_lag_minutes: None,
        site_id: "ANKARA_PUBLISHED".into(),
        ephemeris_source: "Diyanet Shawwal 1447 published astronomy".into(),
        quality: QualityClass::Reference,
    };
    let criterion = CriterionResult {
        profile_id: "DIYANET_1978_VISIBILITY",
        profile_version: "represented-1447-2026",
        met: Some(true),
        clauses: vec![],
        if_not_met: RuleIfNotMet::NoAutomaticCalendarOutcome,
        evidence: EvidenceState::Calculated,
    };
    TemporalResolution {
        id: "TR-SHAWWAL-1447".into(),
        astronomy,
        computed_action: ComputedMonthAction::BeginNewMonthNextLocalDay,
        criterion,
        observations: vec![],
        observation_summary: Some(ObservationSummary {
            id: "TR-SHAWWAL-1447-RUKYAT".into(),
            date: "2026-03-19".into(),
            jurisdiction_id: "TR".into(),
            reported_sites: 111,
            accepted_positive_sightings: 1,
            source: "Diyanet 1447 Shawwal official source set".into(),
            notes: Some("crescent reported observed and photographed in North America".into()),
        }),
        authority: Some(AuthorityDecision {
            id: "TR-DIYANET-SHAWWAL-1447".into(),
            authority: "Diyanet İşleri Başkanlığı".into(),
            jurisdiction: Jurisdiction {
                id: "TR".into(),
                label: "Türkiye".into(),
                scope: "national/global-methodology".into(),
            },
            decided_at_utc: "2026-03-19T15:24:00Z".into(),
            decision: DecisionKind::BeginNewMonth,
            source_document: "Diyanet Shawwal 1447 official releases".into(),
            cited_profile_id: Some("DIYANET_1978_VISIBILITY".into()),
            cited_observation_ids: vec!["TR-SHAWWAL-1447-RUKYAT".into()],
            evidence: EvidenceState::Observed,
            provenance: Provenance::new("Diyanet official Shawwal 1447 source set"),
        }),
        calendar_result: Some(CalendarResult {
            hijri_date: HijriDate::new(1447, HijriMonth::Shawwal, 1).unwrap(),
            civil_date: "2026-03-20".into(),
            kind: CalendarResultKind::OfficialAuthority,
            profile_id: Some("DIYANET_1978_VISIBILITY".into()),
            authority_id: Some("TR-DIYANET-SHAWWAL-1447".into()),
            provenance: "Diyanet official calendar".into(),
        }),
    }
}

#[test]
fn indonesia_ramadan_1447_is_reproduced() {
    let r = indonesia_resolution(0, false, DecisionKind::CompleteCurrentMonthTo30Days);
    assert_eq!(audit_historical_resolution(&r).verdict, ReplayVerdict::Reproduced);
}

#[test]
fn indonesia_shawwal_1447_is_reproduced() {
    let r = indonesia_resolution(1, false, DecisionKind::CompleteCurrentMonthTo30Days);
    assert_eq!(audit_historical_resolution(&r).verdict, ReplayVerdict::Reproduced);
}

#[test]
fn indonesia_dhulhijjah_1447_is_reproduced() {
    let r = indonesia_resolution(2, true, DecisionKind::BeginNewMonth);
    assert_eq!(audit_historical_resolution(&r).verdict, ReplayVerdict::Reproduced);
}

#[test]
fn counterfactual_indonesia_authority_flip_is_falsified() {
    let r = indonesia_resolution(2, true, DecisionKind::CompleteCurrentMonthTo30Days);
    let audit = audit_historical_resolution(&r);
    assert_eq!(audit.verdict, ReplayVerdict::Falsified);
    assert!(audit.reasons[0].contains("conflicts"));
}

#[test]
fn unknown_computation_is_incomplete_not_forced_to_pass_or_fail() {
    let mut r = indonesia_resolution(1, false, DecisionKind::CompleteCurrentMonthTo30Days);
    r.computed_action = ComputedMonthAction::Unknown;
    assert_eq!(audit_historical_resolution(&r).verdict, ReplayVerdict::Incomplete);
}

#[test]
fn indonesia_turkiye_shawwal_divergence_is_explained_by_layers() {
    let id = indonesia_resolution(1, false, DecisionKind::CompleteCurrentMonthTo30Days);
    let tr = turkiye_shawwal_resolution();
    let audit = audit_cross_jurisdiction_replay(&id, &tr);
    assert_eq!(audit.verdict, ReplayVerdict::Reproduced);
    assert!(audit.diff.outcome_differs);
    assert!(!audit.diff.categories.is_empty());
}

#[test]
fn unexplained_calendar_date_flip_is_falsified() {
    let a = indonesia_resolution(2, true, DecisionKind::BeginNewMonth);
    let mut b = a.clone();
    b.id = "UNEXPLAINED-FLIP".into();
    b.calendar_result.as_mut().unwrap().civil_date = "2026-05-19".into();
    let audit = audit_cross_jurisdiction_replay(&a, &b);
    assert_eq!(audit.verdict, ReplayVerdict::Falsified);
}
