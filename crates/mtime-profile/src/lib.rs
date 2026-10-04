use mtime_core::Provenance;
use mtime_hijri::{
    CalendarProfile, Comparator, Metric, RuleIfNotMet, ThresholdClause,
};
use mtime_worship::{conjunction_before_fajr_ut1, SolarEvent};
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
    #[serde(default)]
    pub additional_calendar_conditions: Vec<String>,
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
    UnsupportedAdditionalCondition(String),
    InvalidPolicyContext(String),
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
    pub additional_calendar_conditions: Vec<String>,
}

impl CompiledProfile {
    #[must_use]
    pub fn threshold_result_is_complete_calendar_rule(&self) -> bool {
        self.additional_calendar_conditions.is_empty()
    }
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
        additional_calendar_conditions: file.additional_calendar_conditions.clone(),
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

    struct FixtureGeography;

    impl AmericasMainlandProvider for FixtureGeography {
        fn is_americas_mainland(&self, site_id: &str) -> Option<bool> {
            match site_id {
                "AMERICAS-CANDIDATE" => Some(true),
                "FIJI-CANDIDATE" => Some(false),
                "UNKNOWN-CANDIDATE" => None,
                _ => Some(false),
            }
        }
    }

    struct FixtureWellington {
        fajr: Option<SolarEvent>,
    }

    impl WellingtonFajrProvider for FixtureWellington {
        fn fajr_event(&self) -> Option<SolarEvent> {
            self.fajr
        }
    }

    fn fajr(jd_ut1: f64) -> SolarEvent {
        SolarEvent {
            jd_ut1,
            altitude_deg: -18.0,
            kind: mtime_worship::SolarEventKind::FajrThreshold,
        }
    }

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
        assert_eq!(p.additional_calendar_conditions.len(), 2);
        assert!(!p.threshold_result_is_complete_calendar_rule());
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


#[must_use]
pub fn bundled_mabims_id_2026() -> CompiledProfile {
    compile_profile(&parse_profile(include_str!("../../../profiles/mabims-id-2026.toml")).expect("bundled MABIMS profile must parse"))
        .expect("bundled MABIMS profile must compile")
}

#[must_use]
pub fn bundled_diyanet_1978_global() -> CompiledProfile {
    compile_profile(&parse_profile(include_str!("../../../profiles/diyanet-1978-global.toml")).expect("bundled Diyanet profile must parse"))
        .expect("bundled Diyanet profile must compile")
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditionalCalendarCondition {
    VisibilityOnAmericanMainland,
    ConjunctionBeforeWellingtonFajr,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AdditionalCalendarContext {
    /// Whether the 5°/8° visibility criterion is met at a qualifying site
    /// on the North or South American mainland.
    pub visibility_on_american_mainland: Option<bool>,
    /// Whether conjunction occurs before Wellington/New Zealand fajr,
    /// according to the explicitly selected worship-time profile.
    pub conjunction_before_wellington_fajr: Option<bool>,
}


pub trait AmericasMainlandProvider {
    /// Returns Some(true/false) when the provider can classify the site,
    /// or None when the available geospatial evidence is insufficient.
    fn is_americas_mainland(&self, site_id: &str) -> Option<bool>;
}

pub trait WellingtonFajrProvider {
    /// Return the computed Wellington imsak/fajr event using an explicitly
    /// versioned worship-time method. None means the event is unavailable.
    fn fajr_event(&self) -> Option<SolarEvent>;
}

pub fn derive_additional_calendar_context<G, W>(
    threshold_met: Option<bool>,
    passing_site_ids: &[String],
    conjunction_jd_ut1: Option<f64>,
    geography: &G,
    wellington: &W,
) -> Result<AdditionalCalendarContext, ProfileLoadError>
where
    G: AmericasMainlandProvider,
    W: WellingtonFajrProvider,
{
    let visibility_on_american_mainland = match threshold_met {
        Some(false) => Some(false),
        None => None,
        Some(true) => {
            let mut unknown = false;
            let mut matched = false;
            for site_id in passing_site_ids {
                match geography.is_americas_mainland(site_id) {
                    Some(true) => {
                        matched = true;
                        break;
                    }
                    Some(false) => {}
                    None => unknown = true,
                }
            }
            if matched {
                Some(true)
            } else if unknown {
                None
            } else {
                Some(false)
            }
        }
    };

    let conjunction_before_wellington_fajr =
        match (conjunction_jd_ut1, wellington.fajr_event()) {
            (Some(conjunction), Some(fajr)) => Some(
                conjunction_before_fajr_ut1(conjunction, fajr).map_err(|error| {
                    ProfileLoadError::InvalidPolicyContext(format!(
                        "invalid Wellington fajr context: {error:?}"
                    ))
                })?,
            ),
            _ => None,
        };

    Ok(AdditionalCalendarContext {
        visibility_on_american_mainland,
        conjunction_before_wellington_fajr,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalConditionResult {
    pub condition: AdditionalCalendarCondition,
    pub met: Option<bool>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledProfileEvaluation {
    pub threshold_met: Option<bool>,
    pub additional_conditions_met: Option<bool>,
    pub complete_rule_met: Option<bool>,
    pub evaluated_sites: usize,
    pub passing_site_ids: Vec<String>,
    pub additional_results: Vec<AdditionalConditionResult>,
}

fn additional_condition(id: &str) -> Result<AdditionalCalendarCondition, ProfileLoadError> {
    match id {
        "VISIBILITY_ON_NORTH_OR_SOUTH_AMERICA_MAINLAND" => {
            Ok(AdditionalCalendarCondition::VisibilityOnAmericanMainland)
        }
        "CONJUNCTION_BEFORE_WELLINGTON_FAJR" => {
            Ok(AdditionalCalendarCondition::ConjunctionBeforeWellingtonFajr)
        }
        other => Err(ProfileLoadError::UnsupportedAdditionalCondition(other.into())),
    }
}

pub fn evaluate_additional_conditions(
    ids: &[String],
    context: &AdditionalCalendarContext,
) -> Result<Vec<AdditionalConditionResult>, ProfileLoadError> {
    ids.iter()
        .map(|id| {
            let condition = additional_condition(id)?;
            let (met, reason) = match condition {
                AdditionalCalendarCondition::VisibilityOnAmericanMainland => (
                    context.visibility_on_american_mainland,
                    "Diyanet 2026 methodology: qualifying ru'yet/visibility must occur on North or South American mainland".to_string(),
                ),
                AdditionalCalendarCondition::ConjunctionBeforeWellingtonFajr => (
                    context.conjunction_before_wellington_fajr,
                    "Diyanet 2026 methodology: conjunction must occur before Wellington/New Zealand fajr".to_string(),
                ),
            };
            Ok(AdditionalConditionResult { condition, met, reason })
        })
        .collect()
}

fn combine_all(values: impl Iterator<Item = Option<bool>>) -> Option<bool> {
    let mut any_unknown = false;
    for value in values {
        match value {
            Some(false) => return Some(false),
            Some(true) => {}
            None => any_unknown = true,
        }
    }
    if any_unknown { None } else { Some(true) }
}

/// Execute both the numerical threshold component and any additional,
/// explicitly represented calendar-policy conditions.
///
/// The caller remains responsible for producing the policy context from
/// independently versioned astronomy/worship computations. M-Time does not
/// infer "American mainland" or Wellington fajr from an unlabeled scalar.
pub fn evaluate_compiled_profile(
    profile: &CompiledProfile,
    states: &[mtime_hijri::HijriAstronomicalState],
    context: &AdditionalCalendarContext,
) -> Result<CompiledProfileEvaluation, ProfileLoadError> {
    let (threshold_met, passing_site_ids) = match profile.scope {
        ProfileScope::SingleState => {
            let result = states.first().map(|state| profile.calendar.evaluate(state).met).unwrap_or(None);
            let passing = if result == Some(true) {
                states.first().map(|s| vec![s.site_id.clone()]).unwrap_or_default()
            } else {
                Vec::new()
            };
            (result, passing)
        }
        ProfileScope::GlobalAnySite => {
            let result = evaluate_global_any_site(&profile.calendar, states);
            (result.met, result.passing_site_ids)
        }
    };

    let additional_results =
        evaluate_additional_conditions(&profile.additional_calendar_conditions, context)?;
    let additional_conditions_met =
        combine_all(additional_results.iter().map(|result| result.met));
    let complete_rule_met = combine_all([threshold_met, additional_conditions_met].into_iter());

    Ok(CompiledProfileEvaluation {
        threshold_met,
        additional_conditions_met,
        complete_rule_met,
        evaluated_sites: states.len(),
        passing_site_ids,
        additional_results,
    })
}


pub fn evaluate_compiled_profile_with_providers<G, W>(
    profile: &CompiledProfile,
    states: &[mtime_hijri::HijriAstronomicalState],
    conjunction_jd_ut1: Option<f64>,
    geography: &G,
    wellington: &W,
) -> Result<CompiledProfileEvaluation, ProfileLoadError>
where
    G: AmericasMainlandProvider,
    W: WellingtonFajrProvider,
{
    let preliminary =
        evaluate_compiled_profile(profile, states, &AdditionalCalendarContext::default())?;
    let context = derive_additional_calendar_context(
        preliminary.threshold_met,
        &preliminary.passing_site_ids,
        conjunction_jd_ut1,
        geography,
        wellington,
    )?;
    evaluate_compiled_profile(profile, states, &context)
}

#[cfg(test)]
mod executable_policy_tests {
    use super::*;
    use mtime_core::QualityClass;
    use mtime_hijri::{GeometrySemantics, HijriAstronomicalState};

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
    fn diyanet_threshold_pass_is_not_complete_without_policy_context() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile(
            &p,
            &[state("AMERICAS-CANDIDATE", 5.5, 8.5)],
            &AdditionalCalendarContext::default(),
        ).unwrap();
        assert_eq!(r.threshold_met, Some(true));
        assert_eq!(r.additional_conditions_met, None);
        assert_eq!(r.complete_rule_met, None);
    }

    #[test]
    fn diyanet_complete_rule_passes_when_all_four_conditions_are_explicitly_met() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile(
            &p,
            &[state("AMERICAS-CANDIDATE", 5.5, 8.5)],
            &AdditionalCalendarContext {
                visibility_on_american_mainland: Some(true),
                conjunction_before_wellington_fajr: Some(true),
            },
        ).unwrap();
        assert_eq!(r.complete_rule_met, Some(true));
        assert_eq!(r.additional_results.len(), 2);
    }

    #[test]
    fn diyanet_complete_rule_fails_if_wellington_condition_fails() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile(
            &p,
            &[state("AMERICAS-CANDIDATE", 5.5, 8.5)],
            &AdditionalCalendarContext {
                visibility_on_american_mainland: Some(true),
                conjunction_before_wellington_fajr: Some(false),
            },
        ).unwrap();
        assert_eq!(r.threshold_met, Some(true));
        assert_eq!(r.complete_rule_met, Some(false));
    }

    #[test]
    fn provider_wiring_can_complete_diyanet_policy_without_manual_booleans() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile_with_providers(
            &p,
            &[state("AMERICAS-CANDIDATE", 5.5, 8.5)],
            Some(2_460_000.20),
            &FixtureGeography,
            &FixtureWellington {
                fajr: Some(fajr(2_460_000.25)),
            },
        )
        .unwrap();
        assert_eq!(r.threshold_met, Some(true));
        assert_eq!(r.additional_conditions_met, Some(true));
        assert_eq!(r.complete_rule_met, Some(true));
    }

    #[test]
    fn provider_wiring_rejects_non_mainland_visibility() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile_with_providers(
            &p,
            &[state("FIJI-CANDIDATE", 5.5, 8.5)],
            Some(2_460_000.20),
            &FixtureGeography,
            &FixtureWellington {
                fajr: Some(fajr(2_460_000.25)),
            },
        )
        .unwrap();
        assert_eq!(r.threshold_met, Some(true));
        assert_eq!(r.additional_conditions_met, Some(false));
        assert_eq!(r.complete_rule_met, Some(false));
    }

    #[test]
    fn provider_wiring_preserves_unknown_geospatial_evidence() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile_with_providers(
            &p,
            &[state("UNKNOWN-CANDIDATE", 5.5, 8.5)],
            Some(2_460_000.20),
            &FixtureGeography,
            &FixtureWellington {
                fajr: Some(fajr(2_460_000.25)),
            },
        )
        .unwrap();
        assert_eq!(r.additional_conditions_met, None);
        assert_eq!(r.complete_rule_met, None);
    }

    #[test]
    fn provider_wiring_rejects_conjunction_at_or_after_wellington_fajr() {
        let p = bundled_diyanet_1978_global();
        let r = evaluate_compiled_profile_with_providers(
            &p,
            &[state("AMERICAS-CANDIDATE", 5.5, 8.5)],
            Some(2_460_000.30),
            &FixtureGeography,
            &FixtureWellington {
                fajr: Some(fajr(2_460_000.25)),
            },
        )
        .unwrap();
        assert_eq!(r.additional_conditions_met, Some(false));
        assert_eq!(r.complete_rule_met, Some(false));
    }

    #[test]
    fn mabims_has_no_hidden_additional_policy_condition() {
        let p = bundled_mabims_id_2026();
        let r = evaluate_compiled_profile(
            &p,
            &[state("ID", 3.1, 6.5)],
            &AdditionalCalendarContext::default(),
        ).unwrap();
        assert_eq!(r.threshold_met, Some(true));
        assert_eq!(r.additional_conditions_met, Some(true));
        assert_eq!(r.complete_rule_met, Some(true));
    }
}
