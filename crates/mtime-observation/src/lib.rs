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


use mtime_integrity::{IngestedArtifact, SignatureStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedObservationArtifact {
    pub report: ObservationReport,
    pub source_artifact: IngestedArtifact,
}

impl VerifiedObservationArtifact {
    #[must_use]
    pub fn source_signature_verified(&self) -> bool {
        matches!(
            self.source_artifact.signature_status,
            SignatureStatus::VerifiedEd25519 { .. }
        )
    }
}

#[must_use]
pub fn bind_observation_artifact(
    report: ObservationReport,
    source_artifact: IngestedArtifact,
) -> VerifiedObservationArtifact {
    VerifiedObservationArtifact {
        report,
        source_artifact,
    }
}

#[cfg(test)]
mod integrity_binding_tests {
    use super::*;
    use mtime_integrity::ingest_unsigned;

    fn fixture_report() -> ObservationReport {
        ObservationReport {
            id: "obs-1".into(),
            site_id: "site-1".into(),
            organization: "fixture".into(),
            longitude_deg: None,
            latitude_deg: None,
            time_start_utc: None,
            time_end_utc: None,
            instrument: None,
            weather: None,
            horizon_condition: None,
            sighting: SightingStatus::Negative,
            verification: VerificationStatus::Reviewed,
            attachment_hashes: vec![],
            evidence: EvidenceState::Observed,
            quality: QualityClass::Reference,
            provenance: Provenance::new("fixture"),
        }
    }

    #[test]
    fn unsigned_source_remains_explicitly_unsigned() {
        let artifact = ingest_unsigned("official-json-fixture", "application/json", b"{}");
        let bound = bind_observation_artifact(fixture_report(), artifact);
        assert!(!bound.source_signature_verified());
        assert_eq!(bound.source_artifact.sha256_hex.len(), 64);
    }
}


use mtime_integrity::TrustedSourceArtifact;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceBackedObservation {
    pub report: ObservationReport,
    pub source: TrustedSourceArtifact,
}

impl SourceBackedObservation {
    #[must_use]
    pub fn source_signature_verified(&self) -> bool {
        self.source.signature_verified()
    }

    #[must_use]
    pub fn source_hash(&self) -> &str {
        &self.source.sha256_hex
    }
}

#[must_use]
pub fn bind_trusted_observation_source(
    report: ObservationReport,
    source: TrustedSourceArtifact,
) -> SourceBackedObservation {
    SourceBackedObservation { report, source }
}

#[cfg(test)]
mod trusted_source_binding_tests {
    use super::*;
    use mtime_integrity::{SourceTrust, TrustedSourceArtifact};

    #[test]
    fn observation_preserves_real_source_identity_and_hash() {
        let report = ObservationReport {
            id: "obs-source-backed".into(),
            site_id: "site-1".into(),
            organization: "TEST-INSTITUTION".into(),
            longitude_deg: None,
            latitude_deg: None,
            time_start_utc: None,
            time_end_utc: None,
            instrument: None,
            weather: None,
            horizon_condition: None,
            sighting: SightingStatus::Negative,
            verification: VerificationStatus::Reviewed,
            attachment_hashes: vec![],
            evidence: EvidenceState::Observed,
            quality: QualityClass::Reference,
            provenance: Provenance::new("source-backed fixture"),
        };
        let source = TrustedSourceArtifact {
            source_id: "official-source".into(),
            institution_id: "TEST-INSTITUTION".into(),
            canonical_url: "https://example.invalid/source".into(),
            media_type: "application/json".into(),
            sha256_hex: "ab".repeat(32),
            retrieved_at_unix_seconds: 1_800_000_000,
            trust: SourceTrust::HashRecordedUnsigned,
        };
        let bound = bind_trusted_observation_source(report, source);
        assert_eq!(bound.source_hash(), "ab".repeat(32));
        assert!(!bound.source_signature_verified());
    }
}


pub const OBSERVATION_PACKET_VERSION: &str = "MOBS-1";

#[derive(Debug, Clone, PartialEq)]
pub struct ObservationPacketV1 {
    pub packet_version: &'static str,
    pub report_id: String,
    pub site_id: String,
    pub institution: String,
    pub instrument_calibration_id: Option<String>,
    pub local_horizon_profile_id: Option<String>,
    pub weather_summary: Option<String>,
    pub attachment_hashes: Vec<String>,
    pub source_sha256: Option<String>,
    pub source_signature_verified: Option<bool>,
}

impl ObservationPacketV1 {
    #[must_use]
    pub fn from_report(report: &ObservationReport) -> Self {
        Self {
            packet_version: OBSERVATION_PACKET_VERSION,
            report_id: report.id.clone(),
            site_id: report.site_id.clone(),
            institution: report.organization.clone(),
            instrument_calibration_id: None,
            local_horizon_profile_id: None,
            weather_summary: report.weather.clone(),
            attachment_hashes: report.attachment_hashes.clone(),
            source_sha256: None,
            source_signature_verified: None,
        }
    }

    #[must_use]
    pub fn from_source_backed(source: &SourceBackedObservation) -> Self {
        let mut packet = Self::from_report(&source.report);
        packet.source_sha256 = Some(source.source.sha256_hex.clone());
        packet.source_signature_verified = Some(source.source_signature_verified());
        packet
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationConflict {
    pub report_id: String,
    pub reason: String,
}

#[must_use]
pub fn detect_observation_conflicts(reports: &[ObservationReport]) -> Vec<ObservationConflict> {
    let mut conflicts = Vec::new();
    for (i, a) in reports.iter().enumerate() {
        for b in reports.iter().skip(i + 1) {
            if a.id == b.id
                && (a.site_id != b.site_id
                    || a.sighting != b.sighting
                    || a.verification != b.verification
                    || a.attachment_hashes != b.attachment_hashes)
            {
                conflicts.push(ObservationConflict {
                    report_id: a.id.clone(),
                    reason: "duplicate report ID carries conflicting site/status/evidence".into(),
                });
            }
        }
    }
    conflicts
}

#[cfg(test)]
mod observation_packet_tests {
    use super::*;

    fn report(id: &str, sighting: SightingStatus) -> ObservationReport {
        ObservationReport {
            id: id.into(),
            site_id: "SITE".into(),
            organization: "ORG".into(),
            longitude_deg: Some(1.0),
            latitude_deg: Some(2.0),
            time_start_utc: None,
            time_end_utc: None,
            instrument: Some("scope".into()),
            weather: Some("clear".into()),
            horizon_condition: None,
            sighting,
            verification: VerificationStatus::Reviewed,
            attachment_hashes: vec!["aa".repeat(32)],
            evidence: EvidenceState::Observed,
            quality: QualityClass::Reference,
            provenance: Provenance::new("fixture"),
        }
    }

    #[test]
    fn conflicting_duplicate_ids_are_reported() {
        let conflicts = detect_observation_conflicts(&[
            report("same", SightingStatus::Positive),
            report("same", SightingStatus::Negative),
        ]);
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn packet_does_not_invent_calibration_metadata() {
        let p = ObservationPacketV1::from_report(&report("x", SightingStatus::Positive));
        assert_eq!(p.packet_version, "MOBS-1");
        assert_eq!(p.instrument_calibration_id, None);
        assert_eq!(p.source_signature_verified, None);
    }
}
