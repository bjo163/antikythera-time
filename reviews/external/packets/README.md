# M20 External Reviewer Packets

These packets are handoff documents for the five external evidence lanes required before M-Time can claim v1 readiness.

Frozen target:

- candidate: `m20-review-candidate-3`
- Git SHA: `f5a14d8d00367117a6ae9f98ce6cac25f2f262e9`

The packets do not ask reviewers to endorse the project mission. They ask reviewers to test specific claims, boundaries, failure behavior, provenance, and reproducibility.

## Rules common to all reviewers

1. Review the frozen candidate, not a moving branch.
2. Report negative, inconclusive, and contradictory findings explicitly.
3. Do not weaken a threshold merely to obtain a PASS.
4. Distinguish what was directly tested from what was only inspected.
5. Record environment, methodology, limitations, and conflicts of interest.
6. Preserve the report as a file and pin it by SHA-256 in the external-review submission.
7. A reviewer verdict and repository evidence admission are separate decisions.
8. A project maintainer cannot manufacture the external verdict.

## Review lanes

- `INDEPENDENT_REPRODUCTION.md` — issue #39.
- `SECURITY_REVIEW.md` — issue #40.
- `SCIENTIFIC_REVIEW.md` — issue #41.
- `HISTORICAL_RECONSTRUCTION_REVIEW.md` — issue #42.
- `REVELATION_TEXTUAL_BOUNDARY_REVIEW.md` — issue #43.

The machine-readable mapping is `reviews/external/packets/index.json`.

## Reviewer kit artifact

The workflow `m-time-m20-review-kit` builds a downloadable artifact from the frozen candidate. The artifact intentionally contains public specifications, selected review documents, generated conformance-vector output, the review templates/packets, and checksums.

For the independent-reproduction lane, Rust implementation source is intentionally not copied into the handoff kit. The vector output is an oracle to test against, not source code to translate.

The kit does not prove correctness. It makes the exact material under review stable and reproducible.
