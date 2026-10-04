# MTBUNDLE-1 Signed Reference/Profile Bundle

A bundle is an offline-deliverable set of versioned M-Time reference/profile artifacts.

Manifest fields include bundle identity/version, generation and validity intervals, per-artifact canonical path/media type/SHA-256/institution identity, optional Ed25519 signer metadata, and compatible M-Time/profile versions.

Rules:

1. Hash every artifact before use.
2. Required signatures must verify against the trusted-key registry and validity/revocation interval.
3. Bundle authenticity does not imply astronomical, legal, theological, or observational truth.
4. Stale/expired bundles must be visible to M-Clock.
5. Embedded devices may consume precomputed bundles but must preserve profile/version/source identity.
