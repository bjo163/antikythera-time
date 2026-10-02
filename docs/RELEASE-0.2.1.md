# M-Time v0.2.1 — Integrity & Reproducibility Hardening

Date: 2026-10-02

Status: **INTERNALLY REPRODUCIBLE RESEARCH PROTOTYPE**

## Increment over v0.2

- ObservationReport can be bound to an `IngestedArtifact`.
- AuthorityDecision can be bound to an `IngestedArtifact`.
- SHA-256 is always preserved for ingested source payloads.
- Ed25519 verification status remains explicit.
- Unsigned institutional/source material remains explicitly `Unsigned`.
- The DE440/SPK reference workflow now archives the downloaded kernel SHA-256 and matching Horizons oracle inputs.

## Gates

- Rust CI: **62 passed / 0 failed** — run `36960648548`.
- Rust/Python compatibility: PASS.
- DE440/SPK ↔ Horizons reference: PASS — run `36960602706`.
- DE440 reference artifact: `mtime-de440-reference`, artifact ID `11207338395`.
- Existing sub-arcsecond topocentric reference gate remains PASS.

## Security / epistemic boundary

A valid signature means that the payload verifies against a declared key. It does not establish:

- astronomical correctness;
- legal/religious authority;
- truth of a rukyat report;
- correctness of a calendar methodology.

Those remain separate evidence/authority layers.

## Current verdict

M-Time's flagship Hijri interoperability path is internally complete as a high-precision research prototype. The remaining work is broader validation, operational institutional ingestion/key registries, larger historical corpus, independent review, and external standardization/adoption.
