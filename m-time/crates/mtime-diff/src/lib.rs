use mtime_authority::AuthorityDecision;
use mtime_calendar::CriterionResult;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum DiffCategory { PhysicalInput, Ephemeris, Observer, Criterion, Observation, Jurisdiction, Authority, SourceVersion, Unknown }

#[derive(Debug,Clone,PartialEq)]
pub struct Difference {pub category:DiffCategory,pub message:String}

#[derive(Debug,Clone,PartialEq)]
pub struct ResolutionSummary {
 pub astronomical_fingerprint:String,
 pub observer_id:String,
 pub criterion:CriterionResult,
 pub observation_fingerprint:Option<String>,
 pub authority:Option<AuthorityDecision>,
 pub calendar_result:String,
}

pub fn plain_language(diffs:&[Difference])->String{
 if diffs.is_empty(){return "No represented cause differs.".into()}
 diffs.iter().map(|d|match d.category{
  DiffCategory::PhysicalInput=>format!("Astronomical/reference input differs: {}",d.message),
  DiffCategory::Ephemeris=>format!("Ephemeris/source differs: {}",d.message),
  DiffCategory::Observer=>format!("Observer/site differs: {}",d.message),
  DiffCategory::Criterion=>format!("Calendar criterion/profile differs: {}",d.message),
  DiffCategory::Observation=>format!("Observation evidence differs: {}",d.message),
  DiffCategory::Jurisdiction=>format!("Jurisdiction differs: {}",d.message),
  DiffCategory::Authority=>format!("Authority decision differs: {}",d.message),
  DiffCategory::SourceVersion=>format!("Source version differs: {}",d.message),
  DiffCategory::Unknown=>format!("Cause is not fully represented: {}",d.message),
 }).collect::<Vec<_>>().join(" ")
}
pub fn explanation_confidence(diffs:&[Difference])->&'static str{
 if diffs.iter().any(|x|x.category==DiffCategory::Unknown){"INCOMPLETE"}else{"REPRESENTED_CAUSES_ONLY"}
}

pub fn explain_difference(a:&ResolutionSummary,b:&ResolutionSummary)->Vec<Difference>{
 let mut out=Vec::new();
 if a.astronomical_fingerprint!=b.astronomical_fingerprint{out.push(Difference{category:DiffCategory::PhysicalInput,message:"astronomical state/reference input differs".into()});}
 if a.observer_id!=b.observer_id{out.push(Difference{category:DiffCategory::Observer,message:"observer/site differs".into()});}
 if a.criterion.profile_id!=b.criterion.profile_id || a.criterion.profile_version!=b.criterion.profile_version || a.criterion.pass!=b.criterion.pass {
   out.push(Difference{category:DiffCategory::Criterion,message:"calendar profile/version/outcome differs".into()});
 }
 if a.observation_fingerprint!=b.observation_fingerprint{out.push(Difference{category:DiffCategory::Observation,message:"observation evidence differs".into()});}
 match (&a.authority,&b.authority){
  (Some(x),Some(y))=>{
   if x.jurisdiction.id!=y.jurisdiction.id{out.push(Difference{category:DiffCategory::Jurisdiction,message:"jurisdiction differs".into()});}
   if x.authority!=y.authority || x.decision!=y.decision{out.push(Difference{category:DiffCategory::Authority,message:"authority/decision differs".into()});}
  }
  (None,None)=>{}
  _=>out.push(Difference{category:DiffCategory::Authority,message:"only one resolution has an authority decision".into()}),
 }
 if out.is_empty() && a.calendar_result!=b.calendar_result{out.push(Difference{category:DiffCategory::Unknown,message:"calendar results differ but represented causes are equal".into()});}
 out
}

#[cfg(test)]
mod tests{
 use super::*;use mtime_calendar::*;use mtime_authority::*;use mtime_core::Provenance;
 fn criterion(id:&str,pass:bool)->CriterionResult{CriterionResult{profile_id:id.into(),profile_version:"1".into(),clauses:vec![],pass}}
 fn base(id:&str)->ResolutionSummary{ResolutionSummary{astronomical_fingerprint:"same-sky".into(),observer_id:"jakarta".into(),criterion:criterion(id,true),observation_fingerprint:None,authority:None,calendar_result:"date-a".into()}}
 #[test] fn explains_profile_difference(){let a=base("A");let b=base("B");let d=explain_difference(&a,&b);assert!(d.iter().any(|x|x.category==DiffCategory::Criterion));}
 #[test] fn unknown_is_explicit(){let a=base("A");let mut b=base("A");b.calendar_result="date-b".into();let d=explain_difference(&a,&b);assert_eq!(d[0].category,DiffCategory::Unknown);assert_eq!(explanation_confidence(&d),"INCOMPLETE");assert!(plain_language(&d).contains("not fully represented"));}
}


#[derive(Debug,Clone,PartialEq)]
pub struct CalendarTrace {
 pub sky_event_id:String,
 pub observer_scope:String,
 pub rule_id:String,
 pub rule_version:String,
 pub rule_outcome:String,
 pub observation_policy:String,
 pub observation_result:Option<String>,
 pub jurisdiction:String,
 pub authority:String,
 pub official_result:String,
}

pub fn explain_calendar_trace_difference(a:&CalendarTrace,b:&CalendarTrace)->Vec<Difference>{
 let mut out=Vec::new();
 if a.sky_event_id!=b.sky_event_id{
  out.push(Difference{category:DiffCategory::PhysicalInput,message:"compared traces do not reference the same celestial event".into()});
 }
 if a.observer_scope!=b.observer_scope{
  out.push(Difference{category:DiffCategory::Observer,message:format!("scope differs: {} vs {}",a.observer_scope,b.observer_scope)});
 }
 if a.rule_id!=b.rule_id || a.rule_version!=b.rule_version || a.rule_outcome!=b.rule_outcome{
  out.push(Difference{category:DiffCategory::Criterion,message:format!("rule differs: {}:{} ({}) vs {}:{} ({})",a.rule_id,a.rule_version,a.rule_outcome,b.rule_id,b.rule_version,b.rule_outcome)});
 }
 if a.observation_policy!=b.observation_policy || a.observation_result!=b.observation_result{
  out.push(Difference{category:DiffCategory::Observation,message:format!("observation path differs: {} {:?} vs {} {:?}",a.observation_policy,a.observation_result,b.observation_policy,b.observation_result)});
 }
 if a.jurisdiction!=b.jurisdiction{
  out.push(Difference{category:DiffCategory::Jurisdiction,message:format!("jurisdiction differs: {} vs {}",a.jurisdiction,b.jurisdiction)});
 }
 if a.authority!=b.authority || a.official_result!=b.official_result{
  out.push(Difference{category:DiffCategory::Authority,message:format!("official result differs: {} -> {} vs {} -> {}",a.authority,a.official_result,b.authority,b.official_result)});
 }
 out
}

#[cfg(test)]
mod flagship_trace_tests{
 use super::*;
 #[test]
 fn syawal_1447_indonesia_vs_khgt_explains_a_one_day_difference(){
  let gov=CalendarTrace{
   sky_event_id:"conjunction-2026-03-19".into(),
   observer_scope:"Indonesia national observation network".into(),
   rule_id:"MABIMS-ID".into(),rule_version:"PMA-1-2026".into(),rule_outcome:"not satisfied nationally".into(),
   observation_policy:"hisab + rukyatulhilal + Sidang Isbat".into(),observation_result:Some("no hilal sighted at 117 sites".into()),
   jurisdiction:"Indonesia".into(),authority:"Kementerian Agama RI / Sidang Isbat".into(),official_result:"1 Syawal 1447 = 2026-03-21".into()
  };
  let khgt=CalendarTrace{
   sky_event_id:"conjunction-2026-03-19".into(),
   observer_scope:"single global matlak / anywhere before 24:00 UTC".into(),
   rule_id:"KHGT-MUHAMMADIYAH".into(),rule_version:"MUNAS-XXXII/KEP-86-2025".into(),rule_outcome:"PKG1 satisfied globally".into(),
   observation_policy:"hisab hakiki kontemporer / global criterion".into(),observation_result:None,
   jurisdiction:"global Muhammadiyah calendar".into(),authority:"PP Muhammadiyah".into(),official_result:"1 Syawal 1447 = 2026-03-20".into()
  };
  let d=explain_calendar_trace_difference(&gov,&khgt);
  assert!(!d.iter().any(|x|x.category==DiffCategory::PhysicalInput));
  assert!(d.iter().any(|x|x.category==DiffCategory::Observer));
  assert!(d.iter().any(|x|x.category==DiffCategory::Criterion));
  assert!(d.iter().any(|x|x.category==DiffCategory::Observation));
  assert!(d.iter().any(|x|x.category==DiffCategory::Authority));
 }
}
