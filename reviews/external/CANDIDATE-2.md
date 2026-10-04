# M20 External Review Candidate 2 — Superseded

Status: **SUPERSEDED BEFORE EXTERNAL REVIEW SUBMISSION**

Candidate ID: `m20-review-candidate-2`  
Frozen Git SHA: `799e70d87bd3b4a918986a3f8877b9ca44a13696`  
Release: `v0.15.0`

Candidate 2 remains immutable historical evidence.

It was superseded by `m20-review-candidate-3` after the project removed the physical-device/metrology scope and released the software-only v0.16.0 baseline.

No external review submission existed at supersession time.

A report targeting candidate 2 must never be relabeled as candidate 3 evidence.

---

## Original candidate-2 record

Candidate ID: `m20-review-candidate-2`

Frozen software/specification Git SHA:

`799e70d87bd3b4a918986a3f8877b9ca44a13696`

Release: `v0.15.0`

Freeze date: 2026-10-05

## Why candidate 2 exists

Candidate 1 was frozen before M12 gained a reproducible physical-metrology suite, analyzer Git provenance, raw-dataset/report cross-checking, and stricter physical evidence admission.

No external-review submission existed when candidate 2 was created. Therefore the project superseded candidate 1 before reviewer solicitation rather than asking reviewers to assess an outdated snapshot.

Candidate 1 remains preserved in `CANDIDATE-1.md`; its identity is never rewritten.

## Exact candidate validation

The v0.15.0 automated release pipeline completed successfully:

- release pipeline run `37230904736` — SUCCESS;
- dev → main promotion — SUCCESS;
- tag + GitHub Release — SUCCESS;
- post-release validation dispatch — SUCCESS.

The exact candidate commit passed:

- main history guard `37230960498`;
- compatibility `37230962001`;
- Antikythera calibration `37230963518`;
- M-Clock packet `37230964844`;
- WASM `37230966621`;
- IERS EOP `37230968094`;
- JPL reference `37230969569`;
- SPK reference `37230971193`;
- topocentric reference `37230972780`;
- topocentric matrix `37230974485`;
- topocentric multiyear `37230975935`;
- source ingestion `37230977426`;
- historical falsification `37230978921`;
- geospatial mainland `37230980233`;
- Wellington Fajr `37230981733`;
- Wellington seasonal oracle `37230983258`;
- prestandard `37230984576`;
- hardware-readiness software gate `37230986129`;
- GitHub Pages `37230987387`.

All listed runs completed SUCCESS.

## M12 state included in this candidate

Candidate 2 includes the repository-side M12 metrology pipeline:

- strict PPS/holdover CSV identity and duration checks;
- separate raw offset spread and residual jitter;
- fitted oscillator drift in ppm;
- five required PPS/1h/6h/24h/72h datasets;
- exact analysis-code Git SHA in generated reports;
- machine-readable metrology report schema;
- SHA-pinned raw evidence/report linkage;
- explicit prohibition on auto-assigning a physical accuracy class.

These are software/evidence controls. Candidate 2 still contains **no real physical M-Clock measurements**.

## Candidate boundaries

Candidate 2 does not claim:

- physical M-Clock existence;
- measured PPS/holdover accuracy;
- a physical accuracy class;
- unaffiliated independent reproduction;
- external security approval;
- external scientific approval;
- external historical reconstruction approval;
- external revelation/textual-boundary approval;
- v1.0 readiness.

M11, M12 and M20 remain evidence-gated.

## Immutability rule

External review must target both:

- `m20-review-candidate-2`
- `799e70d87bd3b4a918986a3f8877b9ca44a13696`

A later commit is a different review target.

If a review finding requires a semantic/model/specification/security-relevant change, create a new numbered candidate and re-evaluate affected reviews. Do not silently carry a PASS forward.

## Review intake

Use:

- `reviews/external/README.md`;
- `reviews/external/requirements.json`;
- `reviews/external/review-template.json`;
- `reviews/external/review.schema.json`;
- `reviews/external/packets/`.

Machine checks:

```bash
python scripts/review_packet_readiness.py
python scripts/external_review_readiness.py
python scripts/v1_readiness.py
```

A negative, inconclusive, or PASS_WITH_FINDINGS report remains evidence. It does not satisfy a v1 gate under the current fail-closed policy.

