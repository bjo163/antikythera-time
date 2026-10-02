use mtime_core::Provenance;
use mtime_hijri::{
    CalendarProfile, Comparator, Metric, RuleIfNotMet, ThresholdClause,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileFile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub effective_from: String,
    pub effective_through: Option<String>,
    pub require_all: bool,
    pub fallback_if_not_met: String,
    pub observation_layer: String,
    pub authority_layer: String,
    pub source: String,
    pub source_url: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    pub clauses: Vec<ClauseFile>,
}

#[derive(Debug, Deserialize)]
pub struct ClauseFile {
    pub id: String,
    pub metric: String,
    pub reference: String,
    pub comparator: String,
    pub threshold: f64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileLoadError {
    Toml(String),
    UnsupportedMetric(String),
    UnsupportedReference(String),
    UnsupportedComparator(String),
    UnsupportedFallback(String),
    InvalidLayerSeparation,
    EmptyClauses,
}

pub fn parse_profile(text: &str) -> Result<ProfileFile, ProfileLoadError> {
    toml::from_str(text).map_err(|e| ProfileLoadError::Toml(e.to_string()))
}

fn metric(s: &str) -> Result<Metric, ProfileLoadError> {
    match s {
        "MOON_ALTITUDE_TOPOCENTRIC_DEG" => Ok(Metric::MoonAltitudeTopocentricDeg),
        "ELONGATION_GEOCENTRIC_DEG" => Ok(Metric::ElongationGeocentricDeg),
        "MOON_AGE_HOURS" => Ok(Metric::MoonAgeHours),
        "MOON_LAG_MINUTES" => Ok(Metric::MoonLagMinutes),
        x => Err(ProfileLoadError::UnsupportedMetric(x.into())),
    }
}

fn comparator(s: &str) -> Result<Comparator, ProfileLoadError> {
    match s {
        "GE" => Ok(Comparator::GreaterOrEqual),
        "GT" => Ok(Comparator::GreaterThan),
        "LE" => Ok(Comparator::LessOrEqual),
        "LT" => Ok(Comparator::LessThan),
        x => Err(ProfileLoadError::UnsupportedComparator(x.into())),
    }
}

fn validate_reference(metric: Metric, reference: &str) -> Result<(), ProfileLoadError> {
    match (metric, reference) {
        (Metric::MoonAltitudeTopocentricDeg, "TOPOCENTRIC")
        | (Metric::ElongationGeocentricDeg, "GEOCENTRIC_CENTER_TO_CENTER")
        | (Metric::MoonAgeHours, "ELAPSED_SI_TIME")
        | (Metric::MoonLagMinutes, "LOCAL_EVENT_INTERVAL") => Ok(()),
        (_, x) => Err(ProfileLoadError::UnsupportedReference(x.into())),
    }
}

pub fn compile_calendar_profile(file: &ProfileFile) -> Result<CalendarProfile, ProfileLoadError> {
    if file.clauses.is_empty() {
        return Err(ProfileLoadError::EmptyClauses);
    }
    if file.observation_layer != "SEPARATE" || file.authority_layer != "SEPARATE" {
        return Err(ProfileLoadError::InvalidLayerSeparation);
    }
    let fallback = match file.fallback_if_not_met.as_str() {
        "COMPLETE_CURRENT_MONTH_TO_30_DAYS" => RuleIfNotMet::CompleteCurrentMonthTo30Days,
        "NO_AUTOMATIC_CALENDAR_OUTCOME" => RuleIfNotMet::NoAutomaticCalendarOutcome,
        x => return Err(ProfileLoadError::UnsupportedFallback(x.into())),
    };
    let mut clauses = Vec::with_capacity(file.clauses.len());
    for c in &file.clauses {
        let m = metric(&c.metric)?;
        validate_reference(m, &c.reference)?;
        let id: &'static str = Box::leak(c.id.clone().into_boxed_str());
        let unit: &'static str = Box::leak(c.unit.clone().into_boxed_str());
        clauses.push(ThresholdClause {
            id,
            metric: m,
            comparator: comparator(&c.comparator)?,
            threshold: c.threshold,
            unit,
        });
    }
    let id: &'static str = Box::leak(file.id.clone().into_boxed_str());
    let version: &'static str = Box::leak(file.version.clone().into_boxed_str());
    let effective_from: &'static str = Box::leak(file.effective_from.clone().into_boxed_str());
    let effective_through: Option<&'static str> = file
        .effective_through
        .clone()
        .map(|x| Box::leak(x.into_boxed_str()) as &'static str);
    Ok(CalendarProfile {
        id,
        version,
        effective_from,
        effective_through,
        clauses,
        require_all: file.require_all,
        if_not_met: fallback,
        observation_is_separate: true,
        authority_is_separate: true,
        provenance: Provenance {
            source: file.source.clone(),
            source_version: file.source_url.clone(),
            retrieved_at: None,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MABIMS: &str = include_str!("../../../profiles/mabims-id-2026.toml");

    #[test]
    fn official_profile_file_compiles_to_expected_semantics() {
        let file = parse_profile(MABIMS).unwrap();
        let p = compile_calendar_profile(&file).unwrap();
        assert_eq!(p.id, "MABIMS_ID_2026");
        assert_eq!(p.version, "PMA_1_2026");
        assert_eq!(p.clauses.len(), 2);
        assert_eq!(p.clauses[0].threshold, 3.0);
        assert_eq!(p.clauses[1].threshold, 6.4);
        assert!(p.observation_is_separate && p.authority_is_separate);
    }

    #[test]
    fn profile_file_cannot_merge_authority_into_criterion() {
        let bad = MABIMS.replace("authority_layer = \"SEPARATE\"", "authority_layer = \"MERGED\"");
        let file = parse_profile(&bad).unwrap();
        assert_eq!(
            compile_calendar_profile(&file),
            Err(ProfileLoadError::InvalidLayerSeparation)
        );
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileScope {
    SingleState,
    GlobalAnySite,
}

#[derive(Debug)]
pub struct CompiledProfile {
    pub calendar: CalendarProfile,
    pub scope: ProfileScope,
}

pub fn compile_profile(file: &ProfileFile) -> Result<CompiledProfile, ProfileLoadError> {
    let scope = match file.scope.as_deref() {
        None | Some("SINGLE_STATE") => ProfileScope::SingleState,
        Some("GLOBAL_ANY_SITE") => ProfileScope::GlobalAnySite,
        Some(x) => return Err(ProfileLoadError::UnsupportedReference(x.into())),
    };
    Ok(CompiledProfile {
        calendar: compile_calendar_profile(file)?,
        scope,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalCriterionResult {
    pub met: Option<bool>,
    pub evaluated_sites: usize,
    pub passing_site_ids: Vec<String>,
}

#[must_use]
pub fn evaluate_global_any_site(
    profile: &CalendarProfile,
    states: &[mtime_hijri::HijriAstronomicalState],
) -> GlobalCriterionResult {
    let mut passing = Vec::new();
    let mut any_unknown = false;
    for state in states {
        match profile.evaluate(state).met {
            Some(true) => passing.push(state.site_id.clone()),
            Some(false) => {}
            None => any_unknown = true,
        }
    }
    let met = if !passing.is_empty() {
        Some(true)
    } else if any_unknown {
        None
    } else {
        Some(false)
    };
    GlobalCriterionResult {
        met,
        evaluated_sites: states.len(),
        passing_site_ids: passing,
    }
}

#[cfg(test)]
mod global_tests {
    use super::*;
    use mtime_core::QualityClass;
    use mtime_hijri::{GeometrySemantics, HijriAstronomicalState};

    const DIYANET: &str = include_str!("../../../profiles/diyanet-1978-global.toml");

    fn state(id: &str, altitude: f64, elongation: f64) -> HijriAstronomicalState {
        HijriAstronomicalState {
            conjunction_jd_tt: None,
            sunset_jd_ut1: None,
            moon_altitude_topocentric_deg: altitude,
            elongation_geocentric_deg: elongation,
            geometry_semantics: GeometrySemantics::mabims_required(),
            moon_age_hours: None,
            moon_lag_minutes: None,
            site_id: id.into(),
            ephemeris_source: "fixture".into(),
            quality: QualityClass::Reference,
        }
    }

    #[test]
    fn diyanet_profile_is_global_any_site_and_uses_5_8_thresholds() {
        let file = parse_profile(DIYANET).unwrap();
        let p = compile_profile(&file).unwrap();
        assert_eq!(p.scope, ProfileScope::GlobalAnySite);
        assert_eq!(p.calendar.clauses[0].threshold, 5.0);
        assert_eq!(p.calendar.clauses[1].threshold, 8.0);
    }

    #[test]
    fn one_passing_site_satisfies_global_any_site_profile() {
        let file = parse_profile(DIYANET).unwrap();
        let p = compile_profile(&file).unwrap();
        let r = evaluate_global_any_site(
            &p.calendar,
            &[state("A", 4.9, 9.0), state("B", 5.2, 8.1)],
        );
        assert_eq!(r.met, Some(true));
        assert_eq!(r.passing_site_ids, vec!["B"]);
    }
}
