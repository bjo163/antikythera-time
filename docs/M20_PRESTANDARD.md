# M20 Pre-Standard / v1.0 Exit Program

M20 is intentionally a **blocking software milestone**.

The authoritative machine-readable gate file is:

`data/v1-gates.json`

The checker:

`python scripts/v1_readiness.py`

must compute readiness from evidence statuses. The current correct result is:

```text
M-Time v1 readiness: BLOCKED
```

The reason is external software/review evidence, not missing hardware.

## Software-only scope

M-Time does not require a physical M-Clock, embedded controller, GNSS/PPS receiver, oscillator, RTC, bench prototype or hardware metrology program.

`MCLOCK-1` remains only as a software clock-state serialization/view.

## External review intake

M20 external evidence is governed by:

- `reviews/external/requirements.json`;
- `reviews/external/review.schema.json`;
- `reviews/external/review-template.json`;
- `reviews/external/submissions/*.json`;
- `python scripts/external_review_readiness.py`.

Required external categories:

1. unaffiliated independent software reproduction;
2. external software/security review;
3. external scientific review;
4. external historical Antikythera reconstruction review;
5. external revelation/textual-boundary review.

## Candidate freeze rule

Gate-satisfying external evidence must target one immutable candidate release and exact Git SHA.

A review of a moving branch, old candidate or later commit does not silently transfer to another candidate.

## Evidence integrity

A qualifying submission must:

- target the exact pinned candidate;
- identify reviewer/organization and relationship;
- disclose conflicts of interest;
- cover every required category topic;
- carry an eligible reviewer verdict;
- pin report/supporting artifacts by SHA-256;
- pass separate repository evidence admission;
- satisfy independence rules for the unaffiliated reproduction lane.

Negative, inconclusive and PASS_WITH_FINDINGS reports remain first-class evidence. They are not deleted merely to obtain readiness.

## Anti-self-approval linkage

`scripts/v1_readiness.py` cross-checks external v1 gates against admitted evidence.

Editing `data/v1-gates.json` alone cannot convert a missing external review into a valid PASS.

## Exit rule

The project remains on 0.x until:

- the public software specification/conformance surface is stable;
- unaffiliated reproduction satisfies its gate;
- all external review categories satisfy their evidence gates;
- the authoritative gate file and evidence packages agree.

No maintainer preference, feature count, marketing deadline or theological/scientific conviction may bypass these evidence requirements.
