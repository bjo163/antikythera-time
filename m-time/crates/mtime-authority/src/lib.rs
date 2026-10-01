use mtime_core::Provenance;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ObservationOutcome { Positive, Negative, Inconclusive, Invalidated }

#[derive(Debug,Clone,PartialEq)]
pub struct ObservationReport {
 pub id:String,
 pub site_id:String,
 pub organization:Option<String>,
 pub start_utc:String,
 pub end_utc:String,
 pub instrument:Option<String>,
 pub weather:Option<String>,
 pub horizon_condition:Option<String>,
 pub outcome:ObservationOutcome,
 pub accepted_by_authority:Option<bool>,
 pub attachment_hashes:Vec<String>,
 pub provenance:Provenance,
}

#[derive(Debug,Clone,PartialEq)]
pub struct Jurisdiction {pub id:String,pub label:String,pub scope:String}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum DecisionKind { StartNewMonth, CompleteThirtyDays, Pending }

#[derive(Debug,Clone,PartialEq)]
pub struct AuthorityDecision {
 pub id:String,
 pub jurisdiction:Jurisdiction,
 pub authority:String,
 pub decision_time_utc:String,
 pub decision:DecisionKind,
 pub cited_profile:Option<String>,
 pub cited_observation_ids:Vec<String>,
 pub source_document:Provenance,
}
