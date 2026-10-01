use mtime_core::{EvidenceState,Provenance,QualityClass};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum AltitudeBasis{Topocentric,Geocentric,Unknown}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum ElongationBasis{GeocentricCenterToCenter,GeocentricApparent,TopocentricApparent,Unknown}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct GeometrySemantics{pub altitude_basis:AltitudeBasis,pub elongation_basis:ElongationBasis}
impl GeometrySemantics{#[must_use]pub const fn mabims_required()->Self{Self{altitude_basis:AltitudeBasis::Topocentric,elongation_basis:ElongationBasis::GeocentricCenterToCenter}}}
#[derive(Debug,Clone,PartialEq)]pub struct HijriAstronomicalState{pub conjunction_jd_tt:Option<f64>,pub sunset_jd_ut1:Option<f64>,pub moon_altitude_topocentric_deg:f64,pub elongation_geocentric_deg:f64,pub geometry_semantics:GeometrySemantics,pub moon_age_hours:Option<f64>,pub moon_lag_minutes:Option<f64>,pub site_id:String,pub ephemeris_source:String,pub quality:QualityClass}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Comparator{GreaterOrEqual,GreaterThan,LessOrEqual,LessThan}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Metric{MoonAltitudeTopocentricDeg,ElongationGeocentricDeg,MoonAgeHours,MoonLagMinutes}
#[derive(Debug,Clone,PartialEq)]pub struct ThresholdClause{pub id:&'static str,pub metric:Metric,pub comparator:Comparator,pub threshold:f64,pub unit:&'static str}
#[derive(Debug,Clone,PartialEq)]pub struct ClauseResult{pub clause_id:&'static str,pub actual:Option<f64>,pub threshold:f64,pub passed:Option<bool>,pub reason:String}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum RuleIfNotMet{CompleteCurrentMonthTo30Days,NoAutomaticCalendarOutcome}
#[derive(Debug,Clone,PartialEq)]pub struct CalendarProfile{pub id:&'static str,pub version:&'static str,pub effective_from:&'static str,pub effective_through:Option<&'static str>,pub clauses:Vec<ThresholdClause>,pub require_all:bool,pub if_not_met:RuleIfNotMet,pub observation_is_separate:bool,pub authority_is_separate:bool,pub provenance:Provenance}
impl CalendarProfile{
#[must_use]pub fn mabims_indonesia_2026()->Self{Self{id:"MABIMS_ID_2026",version:"PMA_1_2026",effective_from:"2026-01-01",effective_through:None,clauses:vec![ThresholdClause{id:"ALTITUDE_TOPOCENTRIC_GE_3",metric:Metric::MoonAltitudeTopocentricDeg,comparator:Comparator::GreaterOrEqual,threshold:3.0,unit:"deg"},ThresholdClause{id:"ELONGATION_GEOCENTRIC_GE_6_4",metric:Metric::ElongationGeocentricDeg,comparator:Comparator::GreaterOrEqual,threshold:6.4,unit:"deg"}],require_all:true,if_not_met:RuleIfNotMet::CompleteCurrentMonthTo30Days,observation_is_separate:true,authority_is_separate:true,provenance:Provenance{source:"Kementerian Agama RI, PMA No. 1 Tahun 2026".into(),source_version:Some("PMA-1-2026".into()),retrieved_at:None}}}
#[must_use]pub fn evaluate(&self,state:&HijriAstronomicalState)->CriterionResult{let clauses=self.clauses.iter().map(|c|evaluate_clause(c,state)).collect::<Vec<_>>();let known=clauses.iter().all(|x|x.passed.is_some());let met=known&&if self.require_all{clauses.iter().all(|x|x.passed==Some(true))}else{clauses.iter().any(|x|x.passed==Some(true))};CriterionResult{profile_id:self.id,profile_version:self.version,met:if known{Some(met)}else{None},clauses,if_not_met:self.if_not_met,evidence:EvidenceState::Calculated}}
}
fn metric_value(metric:Metric,state:&HijriAstronomicalState)->Option<f64>{match metric{Metric::MoonAltitudeTopocentricDeg=>Some(state.moon_altitude_topocentric_deg),Metric::ElongationGeocentricDeg=>Some(state.elongation_geocentric_deg),Metric::MoonAgeHours=>state.moon_age_hours,Metric::MoonLagMinutes=>state.moon_lag_minutes}}
fn evaluate_clause(c:&ThresholdClause,state:&HijriAstronomicalState)->ClauseResult{
let actual=metric_value(c.metric,state);
let semantics_ok=match c.metric{
Metric::MoonAltitudeTopocentricDeg=>state.geometry_semantics.altitude_basis==AltitudeBasis::Topocentric,
Metric::ElongationGeocentricDeg=>state.geometry_semantics.elongation_basis==ElongationBasis::GeocentricCenterToCenter,
Metric::MoonAgeHours|Metric::MoonLagMinutes=>true};
if !semantics_ok{return ClauseResult{clause_id:c.id,actual,threshold:c.threshold,passed:None,reason:"incompatible geometry semantics for this calendar profile".into()};}
let passed=actual.map(|v|match c.comparator{Comparator::GreaterOrEqual=>v>=c.threshold,Comparator::GreaterThan=>v>c.threshold,Comparator::LessOrEqual=>v<=c.threshold,Comparator::LessThan=>v<c.threshold});
ClauseResult{clause_id:c.id,actual,threshold:c.threshold,passed,reason:match(actual,passed){(None,_)=>"metric unavailable".into(),(Some(v),Some(true))=>format!("{v:.6} {} passes threshold {:.6}",c.unit,c.threshold),(Some(v),Some(false))=>format!("{v:.6} {} fails threshold {:.6}",c.unit,c.threshold),_=>"unknown".into()}}}
#[derive(Debug,Clone,PartialEq)]pub struct CriterionResult{pub profile_id:&'static str,pub profile_version:&'static str,pub met:Option<bool>,pub clauses:Vec<ClauseResult>,pub if_not_met:RuleIfNotMet,pub evidence:EvidenceState}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum ComputedMonthAction{BeginNewMonthNextLocalDay,CompleteCurrentMonthTo30Days,AwaitAdditionalRuleOrObservation,Unknown}
#[must_use]pub fn computed_action(result:&CriterionResult)->ComputedMonthAction{match result.met{Some(true)=>ComputedMonthAction::BeginNewMonthNextLocalDay,Some(false)if result.if_not_met==RuleIfNotMet::CompleteCurrentMonthTo30Days=>ComputedMonthAction::CompleteCurrentMonthTo30Days,Some(false)=>ComputedMonthAction::AwaitAdditionalRuleOrObservation,None=>ComputedMonthAction::Unknown}}
#[cfg(test)]mod tests{use super::*;fn state(a:f64,e:f64)->HijriAstronomicalState{HijriAstronomicalState{conjunction_jd_tt:None,sunset_jd_ut1:None,moon_altitude_topocentric_deg:a,elongation_geocentric_deg:e,geometry_semantics:GeometrySemantics::mabims_required(),moon_age_hours:None,moon_lag_minutes:None,site_id:"TEST".into(),ephemeris_source:"fixture".into(),quality:QualityClass::Reference}}#[test]fn mabims_requires_both_thresholds(){let p=CalendarProfile::mabims_indonesia_2026();assert_eq!(p.evaluate(&state(3.0,6.4)).met,Some(true));assert_eq!(p.evaluate(&state(2.99,7.0)).met,Some(false));assert_eq!(p.evaluate(&state(5.0,6.39)).met,Some(false));}#[test]fn failure_completes_month(){let r=CalendarProfile::mabims_indonesia_2026().evaluate(&state(1.0,5.0));assert_eq!(computed_action(&r),ComputedMonthAction::CompleteCurrentMonthTo30Days);}#[test]fn observation_and_authority_are_separate(){let p=CalendarProfile::mabims_indonesia_2026();assert!(p.observation_is_separate&&p.authority_is_separate);}
#[test]fn mabims_rejects_apparent_geocentric_elongation_as_semantically_incompatible(){let mut s=state(4.0,7.0);s.geometry_semantics.elongation_basis=ElongationBasis::GeocentricApparent;let r=CalendarProfile::mabims_indonesia_2026().evaluate(&s);assert_eq!(r.met,None);assert!(r.clauses.iter().any(|x|x.reason.contains("incompatible geometry semantics")));}}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Indonesia1447PilotCase {
    pub id: &'static str,
    pub observation_date: &'static str,
    pub altitude_min_deg: f64,
    pub altitude_max_deg: f64,
    pub elongation_min_deg: f64,
    pub elongation_max_deg: f64,
    pub observation_sites: u16,
    pub accepted_positive_sightings: u16,
    pub official_month_start: &'static str,
    pub source: &'static str,
}

pub const INDONESIA_1447_PILOTS: &[Indonesia1447PilotCase] = &[
    Indonesia1447PilotCase {
        id: "RAMADAN_1447",
        observation_date: "2026-02-17",
        altitude_min_deg: -2.411_666_7,
        altitude_max_deg: -0.979_722_2,
        elongation_min_deg: 0.939_722_2,
        elongation_max_deg: 1.893_333_3,
        observation_sites: 96,
        accepted_positive_sightings: 0,
        official_month_start: "2026-02-19",
        source: "Kementerian Agama RI, Ramadan 1447 H Sidang Isbat / rukyat releases",
    },
    Indonesia1447PilotCase {
        id: "SHAWWAL_1447",
        observation_date: "2026-03-19",
        altitude_min_deg: 0.907_5,
        altitude_max_deg: 3.131_111_1,
        elongation_min_deg: 4.544_444_4,
        elongation_max_deg: 6.103_055_6,
        observation_sites: 117,
        accepted_positive_sightings: 0,
        official_month_start: "2026-03-21",
        source: "Kementerian Agama RI, Syawal 1447 H Sidang Isbat release",
    },
    Indonesia1447PilotCase {
        id: "DHULHIJJAH_1447",
        observation_date: "2026-05-17",
        altitude_min_deg: 3.292_5,
        altitude_max_deg: 6.949_444_4,
        elongation_min_deg: 8.913_611_1,
        elongation_max_deg: 10.618_611_1,
        observation_sites: 88,
        accepted_positive_sightings: 2,
        official_month_start: "2026-05-18",
        source: "Kementerian Agama RI, Zulhijjah 1447 H Sidang Isbat release",
    },
];

#[cfg(test)]
mod indonesia_1447_pilot_tests {
    use super::*;

    fn state_from_max(case: &Indonesia1447PilotCase) -> HijriAstronomicalState {
        HijriAstronomicalState {
            conjunction_jd_tt: None,
            sunset_jd_ut1: None,
            moon_altitude_topocentric_deg: case.altitude_max_deg,
            elongation_geocentric_deg: case.elongation_max_deg,
            geometry_semantics: GeometrySemantics::mabims_required(),
            moon_age_hours: None,
            moon_lag_minutes: None,
            site_id: "INDONESIA_NATIONAL_RANGE_MAXIMA".into(),
            ephemeris_source: case.source.into(),
            quality: QualityClass::Reference,
        }
    }

    fn state_from_min(case: &Indonesia1447PilotCase) -> HijriAstronomicalState {
        HijriAstronomicalState {
            conjunction_jd_tt: None,
            sunset_jd_ut1: None,
            moon_altitude_topocentric_deg: case.altitude_min_deg,
            elongation_geocentric_deg: case.elongation_min_deg,
            geometry_semantics: GeometrySemantics::mabims_required(),
            moon_age_hours: None,
            moon_lag_minutes: None,
            site_id: "INDONESIA_NATIONAL_RANGE_MINIMA".into(),
            ephemeris_source: case.source.into(),
            quality: QualityClass::Reference,
        }
    }

    #[test]
    fn ramadan_1447_national_maxima_still_fail_mabims() {
        let profile = CalendarProfile::mabims_indonesia_2026();
        assert_eq!(profile.evaluate(&state_from_max(&INDONESIA_1447_PILOTS[0])).met, Some(false));
        assert_eq!(INDONESIA_1447_PILOTS[0].accepted_positive_sightings, 0);
    }

    #[test]
    fn shawwal_1447_national_maximum_elongation_still_fails() {
        let profile = CalendarProfile::mabims_indonesia_2026();
        assert_eq!(profile.evaluate(&state_from_max(&INDONESIA_1447_PILOTS[1])).met, Some(false));
        assert_eq!(INDONESIA_1447_PILOTS[1].accepted_positive_sightings, 0);
    }

    #[test]
    fn dhulhijjah_1447_national_minima_pass_mabims_and_sightings_exist() {
        let profile = CalendarProfile::mabims_indonesia_2026();
        assert_eq!(profile.evaluate(&state_from_min(&INDONESIA_1447_PILOTS[2])).met, Some(true));
        assert!(INDONESIA_1447_PILOTS[2].accepted_positive_sightings > 0);
    }
}
