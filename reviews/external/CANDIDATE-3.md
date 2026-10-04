# M20 External Review Candidate 3

Status: **CURRENT / FROZEN FOR EXTERNAL REVIEW**

Candidate ID: `m20-review-candidate-3`

Frozen Git SHA:

`f5a14d8d00367117a6ae9f98ce6cac25f2f262e9`

Release: `v0.16.0`

Freeze date: 2026-10-05

## Scope

Candidate 3 is the first M20 candidate under M-Time's software-only scope.

It contains Software Antikythera, native M-Time/MTS-2, MCLOCK-1 as a software state surface, uncertainty, calibration/falsification, SDK/ABI/WASM/Python interoperability, calendar/worship, observation/authority/provenance, historical reconstruction, revelation ontology and isolated cosmology research.

## Validation

Release pipeline run `37238100834` completed SUCCESS, including:

- Rust workspace tests;
- Clippy;
- layering invariant;
- Python reference;
- WASM release build;
- dev → main promotion;
- v0.16.0 tag and GitHub Release;
- post-release validation dispatch.

Successful exact-candidate post-release runs include:

- main history guard `37238168816`;
- compatibility `37238170127`;
- Antikythera calibration `37238171436`;
- MCLOCK software packet `37238172809`;
- WASM `37238174127`;
- IERS EOP `37238175448`;
- JPL reference `37238176917`;
- SPK reference `37238178271`;
- topocentric reference `37238179749`;
- topocentric matrix `37238181218`;
- topocentric multiyear `37238182658`;
- source ingestion `37238184290`;
- historical falsification `37238185863`;
- geospatial mainland `37238187414`;
- Wellington Fajr `37238188811`;
- Wellington seasonal oracle `37238190367`;
- prestandard `37238191820`;
- GitHub Pages `37238193386`.

## Remaining v1 blockers

Candidate 3 does not claim:

- unaffiliated independent reproduction;
- external security approval;
- external scientific approval;
- external historical-reconstruction approval;
- external revelation/textual-boundary approval;
- international standards adoption;
- religious authority;
- replacement of UTC/BIPM/IERS/JPL.

The machine-readable v1 result remains BLOCKED until the five external evidence lanes are satisfied.

## Immutability

External review must target both:

- `m20-review-candidate-3`
- `f5a14d8d00367117a6ae9f98ce6cac25f2f262e9`

If a review finding changes reviewed software semantics, freeze a new numbered candidate and re-review affected lanes.
