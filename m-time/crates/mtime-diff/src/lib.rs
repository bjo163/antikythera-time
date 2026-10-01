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
 #[test] fn unknown_is_explicit(){let a=base("A");let mut b=base("A");b.calendar_result="date-b".into();let d=explain_difference(&a,&b);assert_eq!(d[0].category,DiffCategory::Unknown);}
}
