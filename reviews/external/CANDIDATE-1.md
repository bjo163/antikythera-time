# M20 External Review Candidate 1 — Superseded

Status: **SUPERSEDED BEFORE EXTERNAL REVIEW SUBMISSION**

Candidate ID: `m20-review-candidate-1`

Frozen Git SHA:

`39c2705fa18d7d487803f8a37b8fe79df639daaa`

Original freeze date: 2026-10-04

This candidate remains an immutable historical record. It was superseded by `m20-review-candidate-2` before any external review submission existed, because M12 metrology/evidence tooling changed afterward.

Evidence or review targeting candidate 1 must not be relabeled as candidate 2 evidence.

---

## Original candidate-1 record

Candidate ID: `m20-review-candidate-1`

Frozen software/specification Git SHA:

`39c2705fa18d7d487803f8a37b8fe79df639daaa`

Freeze date: 2026-10-04

## Purpose

This commit is the immutable target for the first M20 external-review cycle.

External review evidence submitted for M20 gate satisfaction must target both the candidate ID and exact Git SHA above. Review of `main`, a later commit, or an older commit does not satisfy this candidate's gate.

If a blocking review finding requires changes to code/specification semantics, this candidate remains immutable. The project must create a new numbered review candidate and obtain/reconcile the affected reviews against that new target.

## Exact-candidate checks

GitHub Actions on the candidate commit:

- `m-time-ci` run 37209167850 — SUCCESS
- `m-time-prestandard` run 37209167923 — SUCCESS

The prestandard run correctly reports the v1 program as BLOCKED because external reviews and physical evidence are not yet complete.

## Core-validation continuity

The immediately preceding v0.14 internal validation baseline was commit:

`2c94255049666f543c4afc629586778ab541ef6c`

At that baseline, the manually dispatched compatibility, Antikythera calibration, M-Clock, hardware-readiness and prestandard workflows were successful.

The only repository changes from that baseline through this candidate are evidence-admission/workflow/documentation files for M11/M12 and M20. No `crates/**` implementation file, Antikythera model equation, protocol specification, calibration coefficient, validated interval, or calendar/worship algorithm changed in that two-commit interval.

This continuity statement is a repository-diff fact, not a substitute for external review.

## Candidate boundaries

The candidate does not claim:

- physical M-Clock existence;
- physical PPS/holdover accuracy;
- independent external reproduction;
- external security approval;
- external scientific approval;
- external historical reconstruction approval;
- external revelation/textual-boundary approval;
- v1.0 readiness.

Those remain explicit M11/M12/M20 gates.

## Review intake

Reviewers should use:

- `reviews/external/README.md`
- `reviews/external/review-template.json`
- `reviews/external/review.schema.json`
- `reviews/external/requirements.json`

The repository admission checker is:

`python scripts/external_review_readiness.py`

A report can be valuable even if its verdict is FAIL, INCONCLUSIVE or PASS_WITH_FINDINGS. Such reports remain evidence but do not satisfy a v1 gate until blocking findings are resolved and the exact accepted candidate receives a qualifying PASS review.

