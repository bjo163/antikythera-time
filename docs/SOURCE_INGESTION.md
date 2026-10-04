# Auditable Source Ingestion — v0.6.0

M-Time distinguishes three separate claims:

1. **Acquisition** — bytes were retrieved from a stated source URL.
2. **Integrity** — the exact bytes have a recorded or pinned SHA-256 digest.
3. **Authentication** — a detached Ed25519 signature verifies under a valid trusted institutional key.

These claims are never collapsed into one boolean.

## Source contract

A source ingestion record contains:

```text
source_id
institution_id
canonical_url
media_type
retrieved_at_unix_seconds
sha256
signature policy
trust state
```

The signature policy is either:

```text
AllowUnsigned
RequireTrustedEd25519
```

When trusted signing is required, ingestion fails unless all of these hold:

- a detached signature is present;
- the key exists in the trusted registry;
- the key is active at the signing instant;
- the key is not revoked;
- the signer's institution matches the declared source institution;
- the Ed25519 signature verifies over the exact bytes.

If an expected SHA-256 is supplied, a mismatch is rejected before trust is granted.

## Real-source CI

Actions run **37193413659** downloaded and ingested:

### IERS finals.all IAU2000

```text
source_id=IERS_FINALS_ALL_IAU2000
institution_id=IERS
canonical_url=https://datacenter.iers.org/data/latestVersion/finals.all.iau2000.txt
sha256=9fbc14ae5e71de96cc1e6b43b54a547acc80c7a6ce910c0209ad6230ff3d30cd
signature_verified=false
```

### NAIF DE440 short kernel

```text
source_id=NAIF_DE440_SHORT
institution_id=NASA-JPL-NAIF
canonical_url=https://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/spk/de440s.bsp
sha256=c1c7feeab882263fc493a9d5a5b2ddd71b54826cdf65d8d17a76126b260a49f2
signature_verified=false
```

The explicit false signature state is intentional. HTTPS acquisition from an official domain is not represented as an Ed25519 signature.

## Binding to calendar evidence

`mtime-observation` can bind an `ObservationReport` to a `TrustedSourceArtifact`.

`mtime-authority` can bind an `AuthorityDecision` to a `TrustedSourceArtifact`.

This preserves exact source identity and hash across the evidence chain.

## Remaining production dependency

M-Time now has the executable signed-ingestion machinery, but production cryptographic authentication of institutional calendar/observation sources still requires institutions to publish or provide:

- detached signatures over source artifacts;
- stable key IDs;
- public keys;
- validity periods;
- revocation information.

Until those exist, such sources remain explicitly hash-recorded/unsigned rather than being mislabeled as cryptographically authenticated.
