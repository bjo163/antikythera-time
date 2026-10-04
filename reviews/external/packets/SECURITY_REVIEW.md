# M20-C — External Security Review Packet

Category: `security`  
Issue: #40  
Candidate: `m20-review-candidate-2`  
Candidate Git SHA: `799e70d87bd3b4a918986a3f8877b9ca44a13696`

## Required topics

- `protocol_decoders`
- `ffi_boundary`
- `signed_bundle_and_key_lifecycle`
- `replay_and_identity_controls`
- `device_update_threats`

## Review scope

### protocol_decoders

Inspect MTS-2 and other externally supplied packet/data decoding for:

- truncation and length confusion;
- invalid UTF-8/profile identifiers;
- non-finite floating-point inputs;
- integer boundary/overflow behavior;
- unknown-version handling;
- malformed JSON and binary input;
- fail-open parsing or guessed layouts.

### ffi_boundary

Review the C/ABI boundary for:

- pointer and length contracts;
- ownership/lifetime;
- null handling;
- panic/error containment;
- numeric width/sign conversion;
- cross-language i128 representation.

### signed_bundle_and_key_lifecycle

Review:

- bundle/source-integrity verification;
- key identity and trust roots;
- expiry/revocation handling;
- hash/signature substitution;
- rollback/stale-bundle behavior;
- operational key rotation.

### replay_and_identity_controls

Review observation/authority paths for:

- replay;
- duplicate identifiers with conflicting content;
- cross-profile confusion;
- jurisdiction/authority identity confusion;
- stale reference data.

### device_update_threats

Review M-Clock/device assumptions including:

- update authenticity;
- rollback;
- compromised upstream time/reference source;
- GNSS spoofing/jamming boundary;
- lock/holdover state integrity;
- boot/update recovery expectations.

## Baseline documents

- `docs/M19_THREAT_REVIEW.md`
- `spec/MTIME-2.0.md`
- `spec/MTBUNDLE-1.md`
- `docs/FFI_SAFETY.md`
- `GOVERNANCE.md`

The internal threat review is input material, not external assurance.

## Verdict rule

A qualifying PASS must state what was tested versus document-reviewed and must identify unresolved findings. `PASS_WITH_FINDINGS` is useful evidence but does not satisfy the v1 gate under the current fail-closed policy.
