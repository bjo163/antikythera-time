# M-Time v0.6.0 — Auditable Source Ingestion

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE AUDITABLE SOURCE-INGESTION RESEARCH PROTOTYPE**

## Increment over v0.5.0

v0.6.0 turns source integrity from crypto primitives into an executable ingestion boundary.

### New fail-closed source contract

M-Time now records:

```text
source identity
institution identity
canonical URL
media type
retrieval instant
SHA-256
optional expected SHA-256
signature policy
detached signature metadata
trust result
```

### Failure conditions

The pipeline rejects:

- expected-hash mismatch;
- missing signature when a trusted signature is required;
- unknown key;
- expired key;
- revoked key;
- signer-institution mismatch;
- invalid Ed25519 signature.

### Live official-source ingestion

Actions run 37193413659 successfully ingested current official IERS and NAIF/JPL artifacts.

```text
IERS finals.all:
9fbc14ae5e71de96cc1e6b43b54a547acc80c7a6ce910c0209ad6230ff3d30cd

DE440 short:
c1c7feeab882263fc493a9d5a5b2ddd71b54826cdf65d8d17a76126b260a49f2
```

Both remain explicitly `signature_verified=false` because no upstream institutional detached Ed25519 signature/key was supplied to this run.

### Evidence binding

Observation and authority records can now preserve an auditable `TrustedSourceArtifact` alongside the semantic record.

### Gates

- Workspace Rust tests: **88 passed / 0 failed**.
- Signed-source contract tests: **6 / 6 PASS**.
- Rust/Python compatibility: **PASS**.
- Core CI / clippy / layering: **PASS**.
- Real IERS ingestion: **PASS**.
- Real DE440 ingestion: **PASS**.

## Boundary

v0.6.0 does not claim that official calendar institutions currently publish Ed25519-signed M-Time-ready artifacts. It provides the machinery to verify such sources without weakening trust semantics when institutional signatures become available.
