use mtime_authority::{AuthorityDecision, DecisionKind};
use mtime_hijri::{CalendarResult, ComputedMonthAction, CriterionResult, GeometrySemantics, HijriAstronomicalState};
use mtime_observation::{ObservationReport, ObservationSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiffCategory {
    PhysicalInput,
    Ephemeris,
    Observer,
    Criterion,
    Observation,
    Jurisdiction,
    Authority,
    SourceVersion,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemporalResolution {
    pub id: String,
    pub astronomy: HijriAstronomicalState,
    pub criterion: CriterionResult,
    pub computed_action: ComputedMonthAction,
    pub observations: Vec<ObservationReport>,
    pub observation_summary: Option<ObservationSummary>,
    pub authority: Option<AuthorityDecision>,
    pub calendar_result: Option<CalendarResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolutionDiff {
    pub categories: Vec<DiffCategory>,
    pub shared_astronomy: bool,
    pub outcome_differs: bool,
    pub explanation: String,
}

fn push_unique(v: &mut Vec<DiffCategory>, c: DiffCategory) {
    if !v.contains(&c) {
        v.push(c);
    }
}

#[must_use]
pub fn explain_difference(a: &TemporalResolution, b: &TemporalResolution) -> ResolutionDiff {
    let mut cats = Vec::new();
    let shared = (a.astronomy.moon_altitude_topocentric_deg - b.astronomy.moon_altitude_topocentric_deg).abs() < 1e-12
        && (a.astronomy.elongation_geocentric_deg - b.astronomy.elongation_geocentric_deg).abs() < 1e-12
        && a.astronomy.site_id == b.astronomy.site_id;

    if !shared {
        if a.astronomy.ephemeris_source != b.astronomy.ephemeris_source {
            push_unique(&mut cats, DiffCategory::Ephemeris);
        }
        if a.astronomy.site_id != b.astronomy.site_id {
            push_unique(&mut cats, DiffCategory::Observer);
        }
        if cats.is_empty() {
            push_unique(&mut cats, DiffCategory::PhysicalInput);
        }
    }

    if a.criterion.profile_id != b.criterion.profile_id
        || a.criterion.profile_version != b.criterion.profile_version
        || a.criterion.met != b.criterion.met
    {
        push_unique(&mut cats, DiffCategory::Criterion);
    }

    if a.observations != b.observations || a.observation_summary != b.observation_summary {
        push_unique(&mut cats, DiffCategory::Observation);
    }

    match (&a.authority, &b.authority) {
        (Some(x), Some(y)) => {
            if x.jurisdiction.id != y.jurisdiction.id {
                push_unique(&mut cats, DiffCategory::Jurisdiction);
            }
            if x.authority != y.authority || x.decision != y.decision {
                push_unique(&mut cats, DiffCategory::Authority);
            }
        }
        (None, None) => {}
        _ => push_unique(&mut cats, DiffCategory::Authority),
    }

    let outcome_differs = a.calendar_result != b.calendar_result;
    if (a.computed_action != b.computed_action || outcome_differs)
        && !cats.contains(&DiffCategory::Criterion)
        && !cats.contains(&DiffCategory::Authority)
        && !cats.contains(&DiffCategory::Observation)
        && !cats.contains(&DiffCategory::Jurisdiction)
    {
        push_unique(&mut cats, DiffCategory::Unknown);
    }

    cats.sort_unstable();
    let explanation = if cats.is_empty() {
        "No material difference detected in the represented layers.".into()
    } else {
        format!(
            "Different outcomes are attributable to: {}.",
            cats.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>().join(", ")
        )
    };

    ResolutionDiff {
        categories: cats,
        shared_astronomy: shared,
        outcome_differs,
        explanation,
    }
}

#[must_use]
pub fn official_vs_computed_conflict(r: &TemporalResolution) -> bool {
    let Some(a) = &r.authority else {
        return false;
    };
    !matches!(
        (r.computed_action, a.decision),
        (ComputedMonthAction::BeginNewMonthNextLocalDay, DecisionKind::BeginNewMonth)
            | (
                ComputedMonthAction::CompleteCurrentMonthTo30Days,
                DecisionKind::CompleteCurrentMonthTo30Days
            )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtime_core::QualityClass;
    use mtime_hijri::{computed_action, CalendarProfile, GeometrySemantics};

    fn state() -> HijriAstronomicalState {
        HijriAstronomicalState {
            conjunction_jd_tt: None,
            sunset_jd_ut1: None,
            moon_altitude_topocentric_deg: 3.1,
            elongation_geocentric_deg: 6.5,
        geometry_semantics: GeometrySemantics::mabims_required(),
            moon_age_hours: None,
            moon_lag_minutes: None,
            site_id: "JKT".into(),
            ephemeris_source: "same".into(),
            quality: QualityClass::Reference,
        }
    }

    #[test]
    fn profile_difference_is_explained_without_different_physics() {
        let sa = state();
        let sb = state();
        let pa = CalendarProfile::mabims_indonesia_2026();
        let ra = pa.evaluate(&sa);
        let mut pb = CalendarProfile::mabims_indonesia_2026();
        pb.id = "STRICT_EXAMPLE";
        pb.clauses[0].threshold = 5.0;
        let rb = pb.evaluate(&sb);
        let a = TemporalResolution {
            id: "A".into(),
            astronomy: sa,
            computed_action: computed_action(&ra),
            criterion: ra,
            observations: vec![],
            observation_summary: None,
            authority: None,
            calendar_result: None,
        };
        let b = TemporalResolution {
            id: "B".into(),
            astronomy: sb,
            computed_action: computed_action(&rb),
            criterion: rb,
            observations: vec![],
            observation_summary: None,
            authority: None,
            calendar_result: None,
        };
        let d = explain_difference(&a, &b);
        assert!(d.shared_astronomy);
        assert!(d.categories.contains(&DiffCategory::Criterion));
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayVerdict {
    Reproduced,
    Falsified,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayAudit {
    pub verdict: ReplayVerdict,
    pub reasons: Vec<String>,
}

#[must_use]
pub fn audit_historical_resolution(resolution: &TemporalResolution) -> HistoricalReplayAudit {
    let Some(authority) = &resolution.authority else {
        return HistoricalReplayAudit {
            verdict: ReplayVerdict::Incomplete,
            reasons: vec!["authority decision is missing".into()],
        };
    };

    match resolution.computed_action {
        ComputedMonthAction::Unknown => HistoricalReplayAudit {
            verdict: ReplayVerdict::Incomplete,
            reasons: vec!["computed month action is unknown".into()],
        },
        ComputedMonthAction::AwaitAdditionalRuleOrObservation => HistoricalReplayAudit {
            verdict: ReplayVerdict::Incomplete,
            reasons: vec![
                "represented criterion requires additional rule or observation context".into(),
            ],
        },
        _ if official_vs_computed_conflict(resolution) => HistoricalReplayAudit {
            verdict: ReplayVerdict::Falsified,
            reasons: vec![format!(
                "computed action {:?} conflicts with authority decision {:?}",
                resolution.computed_action, authority.decision
            )],
        },
        _ => HistoricalReplayAudit {
            verdict: ReplayVerdict::Reproduced,
            reasons: vec![
                "represented computed action is consistent with the recorded authority decision"
                    .into(),
            ],
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossJurisdictionReplayAudit {
    pub verdict: ReplayVerdict,
    pub diff: ResolutionDiff,
    pub reasons: Vec<String>,
}

#[must_use]
pub fn audit_cross_jurisdiction_replay(
    a: &TemporalResolution,
    b: &TemporalResolution,
) -> CrossJurisdictionReplayAudit {
    let diff = explain_difference(a, b);

    if !diff.outcome_differs {
        return CrossJurisdictionReplayAudit {
            verdict: ReplayVerdict::Reproduced,
            diff,
            reasons: vec!["represented jurisdictions produce the same recorded outcome".into()],
        };
    }

    if diff.categories.is_empty() || diff.categories == vec![DiffCategory::Unknown] {
        return CrossJurisdictionReplayAudit {
            verdict: ReplayVerdict::Falsified,
            diff,
            reasons: vec![
                "calendar outcome differs but the represented layers contain no explanatory difference"
                    .into(),
            ],
        };
    }

    if diff.categories.contains(&DiffCategory::Unknown) {
        return CrossJurisdictionReplayAudit {
            verdict: ReplayVerdict::Incomplete,
            diff,
            reasons: vec![
                "calendar outcome differs and at least one causal layer remains unknown".into(),
            ],
        };
    }

    let layer_list = diff
        .categories
        .iter()
        .map(|category| format!("{category:?}"))
        .collect::<Vec<_>>()
        .join(", ");

    CrossJurisdictionReplayAudit {
        verdict: ReplayVerdict::Reproduced,
        diff,
        reasons: vec![format!(
            "differing outcomes are explained by represented layer differences: {layer_list}"
        )],
    }
}
