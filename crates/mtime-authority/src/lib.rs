use mtime_core::{EvidenceState,Provenance};
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Jurisdiction{pub id:String,pub label:String,pub scope:String}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum DecisionKind{BeginNewMonth,CompleteCurrentMonthTo30Days,NoDecision,Other}
#[derive(Debug,Clone,PartialEq)]pub struct AuthorityDecision{pub id:String,pub authority:String,pub jurisdiction:Jurisdiction,pub decided_at_utc:String,pub decision:DecisionKind,pub source_document:String,pub cited_profile_id:Option<String>,pub cited_observation_ids:Vec<String>,pub evidence:EvidenceState,pub provenance:Provenance}
impl AuthorityDecision{#[must_use]pub fn indonesia_sidang_isbat(decided_at_utc:impl Into<String>,decision:DecisionKind,source_document:impl Into<String>)->Self{Self{id:"ID-KEMENAG-ISBAT".into(),authority:"Kementerian Agama Republik Indonesia / Sidang Isbat".into(),jurisdiction:Jurisdiction{id:"ID".into(),label:"Indonesia".into(),scope:"national".into()},decided_at_utc:decided_at_utc.into(),decision,source_document:source_document.into(),cited_profile_id:Some("MABIMS_ID_2026".into()),cited_observation_ids:vec![],evidence:EvidenceState::Observed,provenance:Provenance::new("official authority decision")}}}
#[cfg(test)]mod tests{use super::*;#[test]fn decision_has_jurisdiction(){let d=AuthorityDecision::indonesia_sidang_isbat("2026-03-19T12:00:00Z",DecisionKind::CompleteCurrentMonthTo30Days,"Kemenag");assert_eq!(d.jurisdiction.id,"ID");}}


use mtime_integrity::{IngestedArtifact, SignatureStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedAuthorityArtifact {
    pub decision: AuthorityDecision,
    pub source_artifact: IngestedArtifact,
}

impl VerifiedAuthorityArtifact {
    #[must_use]
    pub fn source_signature_verified(&self) -> bool {
        matches!(
            self.source_artifact.signature_status,
            SignatureStatus::VerifiedEd25519 { .. }
        )
    }
}

#[must_use]
pub fn bind_authority_artifact(
    decision: AuthorityDecision,
    source_artifact: IngestedArtifact,
) -> VerifiedAuthorityArtifact {
    VerifiedAuthorityArtifact {
        decision,
        source_artifact,
    }
}

#[cfg(test)]
mod integrity_binding_tests {
    use super::*;
    use mtime_integrity::ingest_unsigned;

    #[test]
    fn authority_source_hash_is_preserved_even_when_unsigned() {
        let decision = AuthorityDecision::indonesia_sidang_isbat(
            "2026-03-19T12:00:00Z",
            DecisionKind::CompleteCurrentMonthTo30Days,
            "official source fixture",
        );
        let artifact = ingest_unsigned("authority-fixture", "application/json", b"{\"decision\":1}");
        let bound = bind_authority_artifact(decision, artifact);
        assert!(!bound.source_signature_verified());
        assert_eq!(bound.source_artifact.sha256_hex.len(), 64);
    }
}


use mtime_integrity::TrustedSourceArtifact;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceBackedAuthorityDecision {
    pub decision: AuthorityDecision,
    pub source: TrustedSourceArtifact,
}

impl SourceBackedAuthorityDecision {
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
pub fn bind_trusted_authority_source(
    decision: AuthorityDecision,
    source: TrustedSourceArtifact,
) -> SourceBackedAuthorityDecision {
    SourceBackedAuthorityDecision { decision, source }
}

#[cfg(test)]
mod trusted_source_binding_tests {
    use super::*;
    use mtime_integrity::{SourceTrust, TrustedSourceArtifact};

    #[test]
    fn authority_decision_preserves_source_identity_and_hash() {
        let decision = AuthorityDecision::indonesia_sidang_isbat(
            "2026-03-19T12:00:00Z",
            DecisionKind::CompleteCurrentMonthTo30Days,
            "official source fixture",
        );
        let source = TrustedSourceArtifact {
            source_id: "official-authority-source".into(),
            institution_id: "TEST-INSTITUTION".into(),
            canonical_url: "https://example.invalid/authority".into(),
            media_type: "text/html".into(),
            sha256_hex: "cd".repeat(32),
            retrieved_at_unix_seconds: 1_800_000_000,
            trust: SourceTrust::HashRecordedUnsigned,
        };
        let bound = bind_trusted_authority_source(decision, source);
        assert_eq!(bound.source_hash(), "cd".repeat(32));
        assert!(!bound.source_signature_verified());
    }
}
