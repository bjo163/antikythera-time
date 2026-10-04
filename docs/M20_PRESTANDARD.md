# M20 Pre-Standard / v1.0 Exit Program

M20 is intentionally a **blocking milestone**.

The machine-readable v1 gate file is:

`data/v1-gates.json`

The v1 checker:

`python scripts/v1_readiness.py`

must compute readiness from evidence statuses. The current correct result is:

```text
M-Time v1 readiness: BLOCKED
```

This is a feature, not a failure.

Repository-level software can prepare the specification, protocols, conformance suite, evidence contracts and hardware/metrology tooling. It cannot honestly manufacture physical measurements, external scientific judgment, independent institutional reproduction, or specialist historical/textual review.

## External review intake

M20 external evidence is governed by:

- `reviews/external/requirements.json` — category → v1 gate mapping and required scope;
- `reviews/external/review.schema.json` — machine-readable submission contract;
- `reviews/external/review-template.json` — neutral template, never evidence;
- `reviews/external/submissions/*.json` — real external submissions when they exist;
- `python scripts/external_review_readiness.py` — fail-closed evidence classifier.

The external categories are:

1. unaffiliated independent reproduction;
2. external security review;
3. external scientific review;
4. external historical Antikythera reconstruction review;
5. external revelation/textual-boundary review.

## Candidate freeze rule

External approval is meaningful only against a specific immutable candidate.

Before a submission can satisfy a gate, `reviews/external/requirements.json` must pin:

- candidate release identity;
- exact 40-character Git commit SHA.

While the candidate remains `UNSET`, the external-review readiness result remains blocked even if reports are present.

## Evidence integrity and separation

A gate-satisfying submission must:

- target the exact pinned candidate;
- identify the reviewer and organization;
- declare relationship/conflict-of-interest status;
- cover every required topic for its category;
- carry a reviewer verdict of `PASS`;
- pin its report/supporting files by SHA-256;
- be admitted by an actor distinct from the external reviewer identity;
- satisfy category-specific independence rules.

For independent reproduction specifically, the submission must declare no shared M-Time implementation code and must report a passing public conformance result.

A `PASS_WITH_FINDINGS`, `FAIL`, `INCONCLUSIVE`, rejected review, review of an old commit, or unreviewed submission is preserved as evidence but cannot satisfy the corresponding v1 gate.

## Anti-self-approval linkage

`scripts/v1_readiness.py` cross-checks every external v1 gate.

Changing an external gate to `PASS` without a machine-classified `SATISFIED` external evidence package fails the checker.

This prevents a silent JSON-only promotion. It still cannot prove the truth of a person's independence or the correctness of expert judgment; those remain governance and peer-review responsibilities.

## Exit rule

The project must remain on 0.x until:

- M11 has real physical build evidence;
- M12 has reviewed measured metrology/error-budget evidence;
- all M20 external-review categories satisfy their evidence gates;
- the authoritative `data/v1-gates.json` and all linked evidence agree.

No maintainer preference, feature count, marketing deadline or theological/scientific conviction may bypass these evidence requirements.
