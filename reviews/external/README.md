# M20 External Review Evidence Intake

This directory defines the evidence boundary for the external parts of the M-Time v1 exit program.

It does **not** manufacture external review. It makes real review submissions auditable, immutable enough for repository use, and unable to satisfy a v1 gate merely because somebody edited a status string.

## Authoritative boundary

`data/v1-gates.json` remains the authoritative v1 gate list.

This directory provides the evidence needed to justify changing an external gate to `PASS`.

A gate may be marked `PASS` only when the corresponding external-review category is machine-classified as `SATISFIED` by:

`python scripts/external_review_readiness.py`

`scripts/v1_readiness.py` cross-checks that rule.

## Frozen candidate first

Before soliciting a gate-satisfying review, pin the candidate in:

`reviews/external/requirements.json`

Both fields must be immutable candidate identifiers:

- `candidate.release`
- `candidate.git_sha`

While the candidate is `UNSET`, no external submission can satisfy a v1 gate.

This prevents an old review of a moving branch from being reused as approval of a different system.

## Required review categories

1. independent reimplementation / reproduction;
2. security;
3. scientific;
4. historical Antikythera reconstruction;
5. revelation/textual-boundary review.

Each category maps to exactly one gate in `data/v1-gates.json`.

## Submission workflow

Copy `review-template.json` to a unique JSON file under:

`reviews/external/submissions/`

Then provide:

- unique `review_id`;
- category;
- exact candidate release + Git SHA;
- reviewer name, public identity, organization and relationship disclosure;
- conflict-of-interest declaration;
- review methodology;
- every required topic for that category;
- reviewer verdict;
- report artifact and SHA-256;
- optional supporting artifacts and SHA-256 values.

For `independent_reproduction`, also provide the independent implementation repository/language, conformance command/result, and declare whether project code was shared.

## Two distinct decisions

The external reviewer supplies a **review verdict**.

The repository performs a separate **evidence admission**.

Valid states:

- `SUBMITTED_UNREVIEWED` — report exists but repository admission has not happened;
- `ADMITTED` — report identity/integrity/scope has been admitted;
- `REJECTED` — submission was rejected as evidence.

A gate is satisfied only when all of these are true:

- candidate is pinned;
- submission targets that exact candidate;
- status is `ADMITTED`;
- reviewer verdict is `PASS`;
- relationship class is allowed by the category;
- all required topics are explicitly covered;
- report and supporting artifacts match their SHA-256 pins;
- admission actor is distinct from the external reviewer identity;
- independent reproduction, when applicable, declares no shared project code and a passing conformance result.

`PASS_WITH_FINDINGS`, `FAIL`, and `INCONCLUSIVE` are retained as useful evidence but do not satisfy a v1 gate.

## What automation cannot prove

The validator can enforce declarations, hashes, candidate identity and separation of recorded roles. It cannot prove that a human is truly independent, competent, conflict-free, or that a scientific/security judgment is correct.

Those are governance and peer-review responsibilities, not facts that a JSON schema can manufacture.

Conflicting or negative reviews must remain in the repository as explicit evidence; they must not be deleted merely to produce a clean readiness result.
