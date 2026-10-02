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
