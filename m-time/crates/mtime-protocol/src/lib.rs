use serde::{Deserialize,Serialize};

pub const PROTOCOL_NAME:&str="M-Time";
pub const PROTOCOL_VERSION:&str="0.1.0-alpha";

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum EvidenceState{Observed,Measured,Calculated,Modeled,Inferred,Reconstructed,Speculative,TextualReference}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum QualityClass{Reference,HighPrecision,Approximate,Reconstruction,Conceptual}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct SourceRef{
 pub source:String,
 #[serde(skip_serializing_if="Option::is_none")] pub version:Option<String>,
 #[serde(skip_serializing_if="Option::is_none")] pub url:Option<String>,
 #[serde(skip_serializing_if="Option::is_none")] pub retrieved_at:Option<String>,
 #[serde(skip_serializing_if="Option::is_none")] pub sha256:Option<String>,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct AstronomicalStateRecord{
 pub conjunction_jd_tt:f64,
 pub sunset_jd_utc:f64,
 pub moon_topocentric_altitude_deg:f64,
 pub moon_geocentric_altitude_deg:f64,
 pub moon_sun_geocentric_elongation_deg:f64,
 pub moon_age_hours:Option<f64>,
 pub moon_lag_minutes:Option<f64>,
 pub observer_id:String,
 pub ephemeris_provider:String,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct CriterionRecord{
 pub profile_id:String,
 pub profile_version:String,
 pub pass:bool,
 pub clauses:Vec<String>,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct ObservationRecord{
 pub id:String,pub site_id:String,pub outcome:String,pub accepted_by_authority:Option<bool>,pub sources:Vec<SourceRef>
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct AuthorityRecord{
 pub jurisdiction:String,pub authority:String,pub decision:String,pub official_calendar_result:String,pub sources:Vec<SourceRef>
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct TemporalResolutionRecord{
 pub protocol:String,
 pub version:String,
 pub kind:String,
 pub evidence:EvidenceState,
 pub quality:QualityClass,
 pub astronomical_state:AstronomicalStateRecord,
 pub criterion:CriterionRecord,
 pub observations:Vec<ObservationRecord>,
 pub authority:Option<AuthorityRecord>,
 pub sources:Vec<SourceRef>,
}

impl TemporalResolutionRecord{
 pub fn validate(&self)->Result<(),Vec<String>>{
  let mut e=Vec::new();
  if self.protocol!=PROTOCOL_NAME{e.push("protocol must be M-Time".into())}
  if self.version!=PROTOCOL_VERSION{e.push("unsupported protocol version".into())}
  if self.kind!="temporal_resolution"{e.push("kind must be temporal_resolution".into())}
  if self.criterion.profile_id.is_empty()||self.criterion.profile_version.is_empty(){e.push("calendar profile id/version required".into())}
  if self.astronomical_state.observer_id.is_empty(){e.push("observer_id required".into())}
  if self.astronomical_state.ephemeris_provider.is_empty(){e.push("ephemeris_provider required".into())}
  if self.sources.is_empty(){e.push("at least one source/provenance reference required".into())}
  if [self.astronomical_state.conjunction_jd_tt,self.astronomical_state.sunset_jd_utc,self.astronomical_state.moon_topocentric_altitude_deg,self.astronomical_state.moon_geocentric_altitude_deg,self.astronomical_state.moon_sun_geocentric_elongation_deg].iter().any(|x|!x.is_finite()){e.push("finite astronomy values required".into())}
  if e.is_empty(){Ok(())}else{Err(e)}
 }
}

pub fn to_canonical_json(r:&TemporalResolutionRecord)->Result<String,serde_json::Error>{serde_json::to_string_pretty(r)}
pub fn from_json(s:&str)->Result<TemporalResolutionRecord,serde_json::Error>{serde_json::from_str(s)}

#[cfg(test)]
mod tests{
 use super::*;
 fn source()->SourceRef{SourceRef{source:"test".into(),version:Some("1".into()),url:None,retrieved_at:None,sha256:None}}
 fn record()->TemporalResolutionRecord{TemporalResolutionRecord{protocol:PROTOCOL_NAME.into(),version:PROTOCOL_VERSION.into(),kind:"temporal_resolution".into(),evidence:EvidenceState::Calculated,quality:QualityClass::Reference,astronomical_state:AstronomicalStateRecord{conjunction_jd_tt:2461118.5,sunset_jd_utc:2461118.9,moon_topocentric_altitude_deg:1.6,moon_geocentric_altitude_deg:2.5,moon_sun_geocentric_elongation_deg:5.7,moon_age_hours:Some(9.7),moon_lag_minutes:Some(10.0),observer_id:"Jakarta".into(),ephemeris_provider:"provider".into()},criterion:CriterionRecord{profile_id:"MABIMS-ID".into(),profile_version:"PMA-1-2026".into(),pass:false,clauses:vec!["alt>=3".into(),"elong>=6.4".into()]},observations:vec![],authority:None,sources:vec![source()]}}
 #[test]fn roundtrip_and_validate(){let r=record();assert!(r.validate().is_ok());let s=to_canonical_json(&r).unwrap();assert_eq!(from_json(&s).unwrap(),r);}
 #[test]fn profile_version_is_mandatory(){let mut r=record();r.criterion.profile_version.clear();assert!(r.validate().is_err());}
}
