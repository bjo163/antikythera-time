use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityError {
    InvalidHex,
    InvalidPublicKey,
    InvalidSignatureEncoding,
    SignatureRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureStatus {
    Unsigned,
    VerifiedEd25519 { key_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestedArtifact {
    pub source_id: String,
    pub media_type: String,
    pub sha256_hex: String,
    pub signature_status: SignatureStatus,
}

#[must_use]
pub fn sha256_hex(payload: &[u8]) -> String {
    let digest = Sha256::digest(payload);
    let mut out = String::with_capacity(64);
    for b in digest {
        use core::fmt::Write as _;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}

#[must_use]
pub fn ingest_unsigned(
    source_id: impl Into<String>,
    media_type: impl Into<String>,
    payload: &[u8],
) -> IngestedArtifact {
    IngestedArtifact {
        source_id: source_id.into(),
        media_type: media_type.into(),
        sha256_hex: sha256_hex(payload),
        signature_status: SignatureStatus::Unsigned,
    }
}

pub fn ingest_ed25519(
    source_id: impl Into<String>,
    media_type: impl Into<String>,
    payload: &[u8],
    key_id: impl Into<String>,
    public_key_hex: &str,
    signature_hex: &str,
) -> Result<IngestedArtifact, IntegrityError> {
    let public = decode_hex::<32>(public_key_hex)?;
    let sig_bytes = decode_hex_vec(signature_hex)?;
    let key = VerifyingKey::from_bytes(&public).map_err(|_| IntegrityError::InvalidPublicKey)?;
    let signature =
        Signature::from_slice(&sig_bytes).map_err(|_| IntegrityError::InvalidSignatureEncoding)?;
    key.verify(payload, &signature)
        .map_err(|_| IntegrityError::SignatureRejected)?;
    Ok(IngestedArtifact {
        source_id: source_id.into(),
        media_type: media_type.into(),
        sha256_hex: sha256_hex(payload),
        signature_status: SignatureStatus::VerifiedEd25519 {
            key_id: key_id.into(),
        },
    })
}

fn decode_hex<const N: usize>(s: &str) -> Result<[u8; N], IntegrityError> {
    let v = decode_hex_vec(s)?;
    v.try_into().map_err(|_| IntegrityError::InvalidHex)
}

fn decode_hex_vec(s: &str) -> Result<Vec<u8>, IntegrityError> {
    if s.len() % 2 != 0 {
        return Err(IntegrityError::InvalidHex);
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in (0..bytes.len()).step_by(2) {
        let hi = nibble(bytes[i]).ok_or(IntegrityError::InvalidHex)?;
        let lo = nibble(bytes[i + 1]).ok_or(IntegrityError::InvalidHex)?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn unsigned_ingestion_still_records_content_hash() {
        let a = ingest_unsigned("fixture", "application/json", b"{}");
        assert_eq!(a.sha256_hex.len(), 64);
        assert_eq!(a.signature_status, SignatureStatus::Unsigned);
    }

    #[test]
    fn verifies_rfc8032_empty_message_vector() {
        let public = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
        let signature = concat!(
            "e5564300c360ac729086e2cc806e828a",
            "84877f1eb8e5d974d873e06522490155",
            "5fb8821590a33bacc61e39701cf9b46b",
            "d25bf5f0595bbe24655141438e7a100b"
        );
        let a = ingest_ed25519(
            "rfc8032-test-1",
            "application/octet-stream",
            b"",
            "test-key",
            public,
            signature,
        )
        .unwrap();
        assert_eq!(
            a.signature_status,
            SignatureStatus::VerifiedEd25519 {
                key_id: "test-key".into()
            }
        );
    }

    #[test]
    fn modified_payload_rejects_signature() {
        let public = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
        let signature = concat!(
            "e5564300c360ac729086e2cc806e828a",
            "84877f1eb8e5d974d873e06522490155",
            "5fb8821590a33bacc61e39701cf9b46b",
            "d25bf5f0595bbe24655141438e7a100b"
        );
        assert_eq!(
            ingest_ed25519(
                "rfc8032-test-1",
                "application/octet-stream",
                b"x",
                "test-key",
                public,
                signature,
            ),
            Err(IntegrityError::SignatureRejected)
        );
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyStatus {
    Active,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedEd25519Key {
    pub key_id: String,
    pub institution_id: String,
    pub public_key_hex: String,
    /// Inclusive Unix-second validity start.
    pub valid_from_unix_seconds: i64,
    /// Exclusive Unix-second validity end; None means open-ended.
    pub valid_through_unix_seconds: Option<i64>,
    pub status: KeyStatus,
    pub provenance: String,
}

impl TrustedEd25519Key {
    #[must_use]
    pub fn valid_at(&self, unix_seconds: i64) -> bool {
        if self.status != KeyStatus::Active || unix_seconds < self.valid_from_unix_seconds {
            return false;
        }
        self.valid_through_unix_seconds
            .map(|end| unix_seconds < end)
            .unwrap_or(true)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustedKeyRegistry {
    keys: Vec<TrustedEd25519Key>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    DuplicateKeyId,
    UnknownKey,
    KeyNotValidAtInstant,
    Integrity(IntegrityError),
}

impl TrustedKeyRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn add(&mut self, key: TrustedEd25519Key) -> Result<(), RegistryError> {
        if self.keys.iter().any(|existing| existing.key_id == key.key_id) {
            return Err(RegistryError::DuplicateKeyId);
        }
        self.keys.push(key);
        Ok(())
    }

    #[must_use]
    pub fn key(&self, key_id: &str) -> Option<&TrustedEd25519Key> {
        self.keys.iter().find(|key| key.key_id == key_id)
    }

    pub fn verify_ed25519(
        &self,
        source_id: impl Into<String>,
        media_type: impl Into<String>,
        payload: &[u8],
        key_id: &str,
        signature_hex: &str,
        signed_at_unix_seconds: i64,
    ) -> Result<IngestedArtifact, RegistryError> {
        let key = self.key(key_id).ok_or(RegistryError::UnknownKey)?;
        if !key.valid_at(signed_at_unix_seconds) {
            return Err(RegistryError::KeyNotValidAtInstant);
        }
        ingest_ed25519(
            source_id,
            media_type,
            payload,
            key_id,
            &key.public_key_hex,
            signature_hex,
        )
        .map_err(RegistryError::Integrity)
    }
}

#[cfg(test)]
mod trusted_registry_tests {
    use super::*;

    fn test_key(status: KeyStatus) -> TrustedEd25519Key {
        TrustedEd25519Key {
            key_id: "institution-test-key-1".into(),
            institution_id: "TEST-INSTITUTION".into(),
            public_key_hex:
                "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a".into(),
            valid_from_unix_seconds: 1_700_000_000,
            valid_through_unix_seconds: Some(1_900_000_000),
            status,
            provenance: "RFC8032 test vector only; not a real institutional key".into(),
        }
    }

    fn signature() -> &'static str {
        concat!(
            "e5564300c360ac729086e2cc806e828a",
            "84877f1eb8e5d974d873e06522490155",
            "5fb8821590a33bacc61e39701cf9b46b",
            "d25bf5f0595bbe24655141438e7a100b"
        )
    }

    #[test]
    fn registry_verifies_active_key_inside_validity_window() {
        let mut registry = TrustedKeyRegistry::new();
        registry.add(test_key(KeyStatus::Active)).unwrap();
        let artifact = registry
            .verify_ed25519(
                "fixture",
                "application/octet-stream",
                b"",
                "institution-test-key-1",
                signature(),
                1_800_000_000,
            )
            .unwrap();
        assert!(matches!(
            artifact.signature_status,
            SignatureStatus::VerifiedEd25519 { .. }
        ));
    }

    #[test]
    fn revoked_key_is_rejected_before_signature_verification() {
        let mut registry = TrustedKeyRegistry::new();
        registry.add(test_key(KeyStatus::Revoked)).unwrap();
        assert_eq!(
            registry.verify_ed25519(
                "fixture",
                "application/octet-stream",
                b"",
                "institution-test-key-1",
                signature(),
                1_800_000_000,
            ),
            Err(RegistryError::KeyNotValidAtInstant)
        );
    }

    #[test]
    fn expired_key_is_rejected() {
        let mut registry = TrustedKeyRegistry::new();
        registry.add(test_key(KeyStatus::Active)).unwrap();
        assert_eq!(
            registry.verify_ed25519(
                "fixture",
                "application/octet-stream",
                b"",
                "institution-test-key-1",
                signature(),
                1_900_000_000,
            ),
            Err(RegistryError::KeyNotValidAtInstant)
        );
    }

    #[test]
    fn duplicate_key_ids_are_rejected() {
        let mut registry = TrustedKeyRegistry::new();
        registry.add(test_key(KeyStatus::Active)).unwrap();
        assert_eq!(
            registry.add(test_key(KeyStatus::Active)),
            Err(RegistryError::DuplicateKeyId)
        );
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceSignaturePolicy {
    AllowUnsigned,
    RequireTrustedEd25519,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceArtifactSpec {
    pub source_id: String,
    pub institution_id: String,
    pub canonical_url: String,
    pub media_type: String,
    pub expected_sha256_hex: Option<String>,
    pub signature_policy: SourceSignaturePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetachedEd25519Signature {
    pub key_id: String,
    pub signature_hex: String,
    pub signed_at_unix_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceTrust {
    HashRecordedUnsigned,
    TrustedEd25519 {
        key_id: String,
        institution_id: String,
        signed_at_unix_seconds: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedSourceArtifact {
    pub source_id: String,
    pub institution_id: String,
    pub canonical_url: String,
    pub media_type: String,
    pub sha256_hex: String,
    pub retrieved_at_unix_seconds: i64,
    pub trust: SourceTrust,
}

impl TrustedSourceArtifact {
    #[must_use]
    pub fn signature_verified(&self) -> bool {
        matches!(self.trust, SourceTrust::TrustedEd25519 { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceIngestionError {
    InvalidSourceMetadata,
    HashMismatch { expected: String, actual: String },
    SignatureRequired,
    SignerInstitutionMismatch {
        expected_institution_id: String,
        signer_institution_id: String,
    },
    Registry(RegistryError),
}

pub fn ingest_source_artifact(
    spec: &SourceArtifactSpec,
    payload: &[u8],
    retrieved_at_unix_seconds: i64,
    detached_signature: Option<&DetachedEd25519Signature>,
    registry: &TrustedKeyRegistry,
) -> Result<TrustedSourceArtifact, SourceIngestionError> {
    if spec.source_id.trim().is_empty()
        || spec.institution_id.trim().is_empty()
        || spec.media_type.trim().is_empty()
        || !(spec.canonical_url.starts_with("https://") || spec.canonical_url.starts_with("http://"))
    {
        return Err(SourceIngestionError::InvalidSourceMetadata);
    }

    let actual_hash = sha256_hex(payload);
    if let Some(expected) = &spec.expected_sha256_hex {
        if !expected.eq_ignore_ascii_case(&actual_hash) {
            return Err(SourceIngestionError::HashMismatch {
                expected: expected.clone(),
                actual: actual_hash,
            });
        }
    }

    let trust = match detached_signature {
        Some(signature) => {
            let key = registry
                .key(&signature.key_id)
                .ok_or(SourceIngestionError::Registry(RegistryError::UnknownKey))?;
            if key.institution_id != spec.institution_id {
                return Err(SourceIngestionError::SignerInstitutionMismatch {
                    expected_institution_id: spec.institution_id.clone(),
                    signer_institution_id: key.institution_id.clone(),
                });
            }
            registry
                .verify_ed25519(
                    spec.source_id.clone(),
                    spec.media_type.clone(),
                    payload,
                    &signature.key_id,
                    &signature.signature_hex,
                    signature.signed_at_unix_seconds,
                )
                .map_err(SourceIngestionError::Registry)?;
            SourceTrust::TrustedEd25519 {
                key_id: signature.key_id.clone(),
                institution_id: key.institution_id.clone(),
                signed_at_unix_seconds: signature.signed_at_unix_seconds,
            }
        }
        None => {
            if spec.signature_policy == SourceSignaturePolicy::RequireTrustedEd25519 {
                return Err(SourceIngestionError::SignatureRequired);
            }
            SourceTrust::HashRecordedUnsigned
        }
    };

    Ok(TrustedSourceArtifact {
        source_id: spec.source_id.clone(),
        institution_id: spec.institution_id.clone(),
        canonical_url: spec.canonical_url.clone(),
        media_type: spec.media_type.clone(),
        sha256_hex: actual_hash,
        retrieved_at_unix_seconds,
        trust,
    })
}

#[cfg(test)]
mod source_ingestion_tests {
    use super::*;

    fn spec(policy: SourceSignaturePolicy) -> SourceArtifactSpec {
        SourceArtifactSpec {
            source_id: "official-fixture".into(),
            institution_id: "TEST-INSTITUTION".into(),
            canonical_url: "https://example.invalid/source".into(),
            media_type: "application/octet-stream".into(),
            expected_sha256_hex: None,
            signature_policy: policy,
        }
    }

    fn trusted_key(institution_id: &str, status: KeyStatus) -> TrustedEd25519Key {
        TrustedEd25519Key {
            key_id: "test-key".into(),
            institution_id: institution_id.into(),
            public_key_hex:
                "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a".into(),
            valid_from_unix_seconds: 1_700_000_000,
            valid_through_unix_seconds: Some(1_900_000_000),
            status,
            provenance: "RFC8032 fixture only".into(),
        }
    }

    fn detached_signature() -> DetachedEd25519Signature {
        DetachedEd25519Signature {
            key_id: "test-key".into(),
            signature_hex: concat!(
                "e5564300c360ac729086e2cc806e828a",
                "84877f1eb8e5d974d873e06522490155",
                "5fb8821590a33bacc61e39701cf9b46b",
                "d25bf5f0595bbe24655141438e7a100b"
            )
            .into(),
            signed_at_unix_seconds: 1_800_000_000,
        }
    }

    #[test]
    fn real_source_contract_records_unsigned_hash_when_allowed() {
        let registry = TrustedKeyRegistry::new();
        let artifact = ingest_source_artifact(
            &spec(SourceSignaturePolicy::AllowUnsigned),
            b"official bytes",
            1_800_000_100,
            None,
            &registry,
        )
        .unwrap();
        assert_eq!(artifact.sha256_hex, sha256_hex(b"official bytes"));
        assert!(!artifact.signature_verified());
    }

    #[test]
    fn signature_required_is_fail_closed() {
        let registry = TrustedKeyRegistry::new();
        assert_eq!(
            ingest_source_artifact(
                &spec(SourceSignaturePolicy::RequireTrustedEd25519),
                b"",
                1_800_000_100,
                None,
                &registry,
            ),
            Err(SourceIngestionError::SignatureRequired)
        );
    }

    #[test]
    fn expected_hash_mismatch_is_rejected() {
        let mut s = spec(SourceSignaturePolicy::AllowUnsigned);
        s.expected_sha256_hex = Some("00".repeat(32));
        assert!(matches!(
            ingest_source_artifact(
                &s,
                b"changed",
                1_800_000_100,
                None,
                &TrustedKeyRegistry::new(),
            ),
            Err(SourceIngestionError::HashMismatch { .. })
        ));
    }

    #[test]
    fn trusted_signature_requires_matching_institution() {
        let mut registry = TrustedKeyRegistry::new();
        registry
            .add(trusted_key("OTHER-INSTITUTION", KeyStatus::Active))
            .unwrap();
        assert!(matches!(
            ingest_source_artifact(
                &spec(SourceSignaturePolicy::RequireTrustedEd25519),
                b"",
                1_800_000_100,
                Some(&detached_signature()),
                &registry,
            ),
            Err(SourceIngestionError::SignerInstitutionMismatch { .. })
        ));
    }

    #[test]
    fn trusted_signature_produces_trusted_source_record() {
        let mut registry = TrustedKeyRegistry::new();
        registry
            .add(trusted_key("TEST-INSTITUTION", KeyStatus::Active))
            .unwrap();
        let artifact = ingest_source_artifact(
            &spec(SourceSignaturePolicy::RequireTrustedEd25519),
            b"",
            1_800_000_100,
            Some(&detached_signature()),
            &registry,
        )
        .unwrap();
        assert!(artifact.signature_verified());
        assert!(matches!(
            artifact.trust,
            SourceTrust::TrustedEd25519 {
                ref key_id,
                ref institution_id,
                ..
            } if key_id == "test-key" && institution_id == "TEST-INSTITUTION"
        ));
    }

    #[test]
    fn revoked_key_remains_rejected_in_source_pipeline() {
        let mut registry = TrustedKeyRegistry::new();
        registry
            .add(trusted_key("TEST-INSTITUTION", KeyStatus::Revoked))
            .unwrap();
        assert_eq!(
            ingest_source_artifact(
                &spec(SourceSignaturePolicy::RequireTrustedEd25519),
                b"",
                1_800_000_100,
                Some(&detached_signature()),
                &registry,
            ),
            Err(SourceIngestionError::Registry(
                RegistryError::KeyNotValidAtInstant
            ))
        );
    }
}
