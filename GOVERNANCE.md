# M-Time Governance

M-Time is currently a research project, not an international standard or religious authority.

## Change classes

### Scientific / machine changes

Changes to Software Antikythera equations, correction coefficients, validated intervals, time-scale semantics or uncertainty envelopes require:

- versioned specification change;
- reproducible tests;
- calibration/falsification evidence;
- no silent threshold weakening.

### Historical reconstruction changes

Must carry evidence labels and source records. Surviving evidence, inference and reconstruction must not be merged.

### Calendar / worship policy changes

Must identify institution/jurisdiction/profile version and remain separate from physical state.

### Revelation ontology changes

May change semantic/explanation metadata only. They may not alter physical numerical state.

## Protocol compatibility

Published protocol/profile IDs are immutable. Breaking meaning requires a new major protocol ID.

## v1.0 authority

The repository may not tag v1.0 solely by maintainer preference. All machine-readable v1 gates must be satisfied, including physical metrology and external review/reproduction evidence.

## Disagreement policy

Conflicting observations, scholarly reconstructions, authority decisions or external reviews are retained as explicit competing records rather than normalized into false consensus.


## Repository promotion topology

Development changes land on `dev` first. Promotion to `main` must use a pull request from `dev` and a real merge commit.

Direct feature-branch squash/rebase promotion to `main` is not an accepted release topology because it removes the parent relationship audited by `.github/workflows/main-history-guard.yml`.

If `main` receives an emergency direct commit, `dev` must first be fast-forwarded to that exact history, then a subsequent `dev -> main` merge-commit promotion must restore the audited topology. History must not be rewritten merely to make the guard green.
