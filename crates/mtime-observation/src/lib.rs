use mtime_core::{EvidenceState, Provenance, QualityClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SightingStatus {
    Positive,
    Negative,
    Inconclusive,
    Invalidated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    Raw,
    Reviewed,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObservationReport {
    pub id: String,
    pub site_id: String,
    pub organization: String,
    pub longitude_deg: Option<f64>,
    pub latitude_deg: Option<f64>,
    pub time_start_utc: Option<String>,
    pub time_end_utc: Option<String>,
    pub instrument: Option<String>,
    pub weather: Option<String>,
    pub horizon_condition: Option<String>,
    pub sighting: SightingStatus,
    pub verification: VerificationStatus,
    pub attachment_hashes: Vec<String>,
    pub evidence: EvidenceState,
    pub quality: QualityClass,
    pub provenance: Provenance,
}

impl ObservationReport {
    #[must_use]
    pub fn is_positive_accepted(&self) -> bool {
        self.sighting == SightingStatus::Positive && self.verification == VerificationStatus::Accepted
    }

    #[must_use]
    pub fn has_known_coordinates(&self) -> bool {
        self.longitude_deg.is_some() && self.latitude_deg.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSummary {
    pub id: String,
    pub date: String,
    pub jurisdiction_id: String,
    pub reported_sites: u16,
    pub accepted_positive_sightings: u16,
    pub source: String,
    pub notes: Option<String>,
}

impl ObservationSummary {
    #[must_use]
    pub fn any_accepted_positive(&self) -> bool {
        self.accepted_positive_sightings > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_positive_is_not_accepted_testimony() {
        let r = ObservationReport {
            id: "1".into(),
            site_id: "A".into(),
            organization: "O".into(),
            longitude_deg: None,
            latitude_deg: None,
            time_start_utc: None,
            time_end_utc: None,
            instrument: None,
            weather: None,
            horizon_condition: None,
            sighting: SightingStatus::Positive,
            verification: VerificationStatus::Raw,
            attachment_hashes: vec![],
            evidence: EvidenceState::Observed,
            quality: QualityClass::Reference,
            provenance: Provenance::new("fixture"),
        };
        assert!(!r.is_positive_accepted());
        assert!(!r.has_known_coordinates());
    }

    #[test]
    fn aggregate_summary_does_not_invent_site_coordinates() {
        let s = ObservationSummary {
            id: "national".into(),
            date: "2026-03-19".into(),
            jurisdiction_id: "ID".into(),
            reported_sites: 117,
            accepted_positive_sightings: 0,
            source: "official national summary".into(),
            notes: None,
        };
        assert!(!s.any_accepted_positive());
        assert_eq!(s.reported_sites, 117);
    }
}
