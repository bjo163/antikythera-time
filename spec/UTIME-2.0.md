# U-Time 2.0 — Standardization Candidate

Status: **STANDARDIZATION CANDIDATE**. This document is not an adopted IAU, BIPM, ISO or other international standard.

## 1. Scope

U-Time 2.0 defines an interchange contract for astronomical/cosmological time-related quantities. It does not define one absolute universal "now".

Every record MUST expose:

```text
value
+ semantic quantity
+ time scale / frame / observer or model context
+ evidence state
+ quality class
+ uncertainty
+ provenance
+ algorithm/reference-data identity
+ validity
+ transformation history when applicable
```

## 2. Evidence state

Normative states:

`OBSERVED | MEASURED | CALCULATED | MODELED | INFERRED | RECONSTRUCTED | SPECULATIVE | TEXTUAL_REFERENCE`.

## 3. Quality class

Normative quality classes:

- `REFERENCE` — standards/reference transformation or posterior/reference result;
- `HIGH_PRECISION` — quantitatively benchmarked high-precision implementation;
- `APPROXIMATE` — validity-bounded engineering/educational model;
- `RECONSTRUCTION` — historical reconstruction;
- `CONCEPTUAL` — non-numerical conceptual/textual material.

Approximate models MUST state validity.

## 4. Coordinate time

Astronomical coordinate-time instants SHOULD use a two-part Julian Date. An instant MUST state time scale and reference frame.

A transformation record SHOULD preserve a `transformChain` identifying conversions such as:

`UTC -> TAI -> TT -> TDB`.

UT1 use MUST cite an EOP provider/version or snapshot.

## 5. Observer

Topocentric or spacecraft-dependent quantities SHOULD include an observer object.

Earth observer coordinates MUST state datum. Spacecraft observers MUST state frame, state vector and epoch.

## 6. Reference data

External reference data SHOULD identify source, dataset/product and retrieval/version information.

Examples:

- IERS finals.all / Bulletin A;
- IAU SOFA/ERFA validation;
- JPL Horizons/ephemeris;
- NASA/GSFC eclipse elements;
- Planck/DESI chains.

## 7. Astronomy model boundaries

- Saros recurrence MUST NOT be labelled full eclipse geometry.
- A published Besselian-element evaluator MUST NOT be labelled independent Besselian-element generation.
- JPL approximate planetary elements MUST remain validity/accuracy-labelled.
- Antikythera reconstructed gearing MUST expose evidence category.

## 8. Cosmic chronology

Cosmic age MUST be `kind=inference`, MUST identify a cosmological model/parameter source, and MUST NOT be represented as a coordinate-time instant.

## 9. Textual/conceptual layer

A textual/scriptural reference MUST use `status=TEXTUAL_REFERENCE` and `quality=CONCEPTUAL`, and MUST NOT silently supply a numerical scientific prior.

## 10. Conformance

A conforming implementation MUST:

1. emit records satisfying the v2 JSON schema and semantic checker;
2. pass normative golden vectors within declared tolerances;
3. preserve evidence/quality/provenance boundaries;
4. identify approximate or reconstructed models as such;
5. not weaken failed reference thresholds merely to obtain a passing build.

## 11. Standardization status

"U-Time 2.0" is the software/protocol version. "Standardization Candidate" means the project is packaged for independent evaluation. External review/adoption remains outside repository control.
