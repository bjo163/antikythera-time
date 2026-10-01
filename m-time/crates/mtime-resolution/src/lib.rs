use mtime_authority::{AuthorityDecision,DecisionKind,ObservationReport,ObservationOutcome};
use mtime_calendar::CriterionResult;
use mtime_diff::ResolutionSummary;
use mtime_hijri::{evaluate_profile,CalendarProfile,HijriAstronomicalState};

#[derive(Debug,Clone,PartialEq)]
pub enum ComputedMonthOutcome { CriterionSatisfied, CriterionNotSatisfied }

#[derive(Debug,Clone,PartialEq)]
pub struct TemporalResolution {
 pub id:String,
 pub astronomy:HijriAstronomicalState,
 pub criterion:CriterionResult,
 pub computed_outcome:ComputedMonthOutcome,
 pub observations:Vec<ObservationReport>,
 pub authority_decision:Option<AuthorityDecision>,
 pub official_calendar_result:Option<String>,
}

impl TemporalResolution {
 pub fn summary(&self)->ResolutionSummary{
  ResolutionSummary{
   astronomical_fingerprint:format!("{:.6}|{:.6}|{:.6}|{:.6}",self.astronomy.conjunction_jd_tt,self.astronomy.sunset_jd_utc,self.astronomy.moon_topocentric_altitude_deg,self.astronomy.moon_sun_geocentric_elongation_deg),
   observer_id:self.astronomy.site.id.clone(),
   criterion:self.criterion.clone(),
   observation_fingerprint:Some(self.observations.iter().map(|x|format!("{}:{:?}",x.id,x.outcome)).collect::<Vec<_>>().join("|")),
   authority:self.authority_decision.clone(),
   calendar_result:self.official_calendar_result.clone().unwrap_or_else(||"UNDECIDED".into())
  }
 }
}

pub fn resolve(profile:&CalendarProfile,state:HijriAstronomicalState,observations:Vec<ObservationReport>,authority:Option<AuthorityDecision>,official:Option<String>)->TemporalResolution{
 let criterion=evaluate_profile(profile,&state);
 let computed_outcome=if criterion.pass{ComputedMonthOutcome::CriterionSatisfied}else{ComputedMonthOutcome::CriterionNotSatisfied};
 TemporalResolution{id:"resolution".into(),astronomy:state,criterion,computed_outcome,observations,authority_decision:authority,official_calendar_result:official}
}

pub fn authority_consistency_notes(r:&TemporalResolution)->Vec<String>{
 let mut notes=Vec::new();
 if let Some(a)=&r.authority_decision{
  if a.decision==DecisionKind::StartNewMonth && !r.criterion.pass{notes.push("official start-new-month differs from computed criterion; inspect observation/authority rule".into());}
  if a.decision==DecisionKind::CompleteThirtyDays && r.observations.iter().any(|x|x.outcome==ObservationOutcome::Positive && x.accepted_by_authority==Some(true)){
    notes.push("accepted positive sighting exists although authority completed thirty days; inspect provenance".into());
  }
 }
 notes
}

#[cfg(test)]
mod tests{
 use super::*;use mtime_authority::*;use mtime_core::Provenance;use mtime_hijri::*;
 #[test] fn indonesia_syawal_1447_case_keeps_layers_separate(){
  let state=HijriAstronomicalState{
   conjunction_jd_tt:2461118.0,sunset_jd_utc:2461119.0,
   moon_topocentric_altitude_deg:3.13,moon_sun_geocentric_elongation_deg:6.10,
   moon_illumination_fraction:None,moon_age_hours:None,moon_lag_minutes:None,
   site:ObserverSite{id:"INDONESIA-RANGE-MAX".into(),latitude_deg:0.0,longitude_deg:0.0,height_m:0.0,timezone:"Asia/Jakarta".into(),datum:"range-summary".into()},
   provenance:vec![Provenance{source:"Kemenag Sidang Isbat Syawal 1447 summary".into(),version:Some("2026-03-19".into()),retrieved_at:None}]
  };
  let obs=ObservationReport{id:"117-sites-summary".into(),site_id:"INDONESIA".into(),organization:Some("Kemenag".into()),start_utc:"2026-03-19".into(),end_utc:"2026-03-19".into(),instrument:None,weather:None,horizon_condition:None,outcome:ObservationOutcome::Negative,accepted_by_authority:Some(true),attachment_hashes:vec![],provenance:Provenance{source:"Kemenag: no hilal observed at 117 sites".into(),version:Some("2026-03-19".into()),retrieved_at:None}};
  let auth=AuthorityDecision{id:"isbat-syawal-1447".into(),jurisdiction:Jurisdiction{id:"ID".into(),label:"Indonesia".into(),scope:"national".into()},authority:"Menteri Agama RI / Sidang Isbat".into(),decision_time_utc:"2026-03-19".into(),decision:DecisionKind::CompleteThirtyDays,cited_profile:Some("MABIMS-ID:PMA-1-2026".into()),cited_observation_ids:vec![obs.id.clone()],source_document:Provenance{source:"Kemenag official decision: 1 Syawal 1447 H = 21 March 2026".into(),version:Some("2026-03-19".into()),retrieved_at:None}};
  let r=resolve(&mabims_indonesia_2026(),state,vec![obs],Some(auth),Some("1 Syawal 1447 H = 2026-03-21".into()));
  assert!(!r.criterion.pass);
  assert_eq!(r.observations[0].outcome,ObservationOutcome::Negative);
  assert_eq!(r.authority_decision.as_ref().unwrap().decision,DecisionKind::CompleteThirtyDays);
  assert_eq!(r.official_calendar_result.as_deref(),Some("1 Syawal 1447 H = 2026-03-21"));
  assert!(authority_consistency_notes(&r).is_empty());
 }
}
