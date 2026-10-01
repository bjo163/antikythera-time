use serde::{Deserialize, Serialize};
use mtime_core::EarthObserver;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SightingResult { Seen, NotSeen, Inconclusive, Invalidated }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObservationReport {
    pub id: String,
    pub observer: EarthObserver,
    pub organization: String,
    pub result: SightingResult,
    pub instrument: Option<String>,
    pub weather: Option<String>,
    pub start_utc: String,
    pub end_utc: String,
    pub accepted_by_authority: Option<bool>,
    pub attachment_sha256: Vec<String>,
    pub provenance: String,
}
