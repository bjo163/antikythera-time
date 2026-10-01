# Antikythera Time — U-Time 2.0.0

## Live research lab

https://bjo163.github.io/antikythera-time/

U-Time 2.0 is an experimental **astronomical-time standardization candidate**. It is not an adopted international standard and does not claim one absolute universal clock.

## Core protocol

Every v2 record exposes:

```text
value
+ semantic quantity
+ scale / frame / observer or model context
+ evidence state
+ quality class
+ uncertainty
+ provenance / reference data
+ algorithm / validity
+ transformation history
```

Normative assets:

- `spec/UTIME-2.0.md`
- `spec/utime-v2.schema.json`
- `spec/v2-conformance-corpus.json`

## Status

```text
VERSION                    2.0.0
STATUS                     STANDARDIZATION_CANDIDATE
CORE TESTS                 103 / 103 PASS
LANGUAGE COMPATIBILITY     JAVASCRIPT / PYTHON / RUST PASS
PUBLIC LAB                 LIVE
INTERNATIONAL ADOPTION     NOT CLAIMED
NASA/JPL REPLACEMENT       NO
ABSOLUTE COSMIC CLOCK      NO
```

## Time / relativity / observer

Implemented:

- UTC / TAI / TT;
- TT ↔ TCG canonical IAU relation;
- TDB ↔ TCB canonical IAU relation;
- canonical TT ↔ TDB with explicit dtr;
- compact automatic geocentric TDB−TT provider;
- six SOFA reference vectors;
- ERFA TDB−TT benchmark;
- IERS finals.all IAU2000 ingestion;
- UT1−UTC and polar-motion interpolation;
- WGS84 Earth observer geometry;
- spacecraft observer state contract;
- weak-field proper-time-rate demonstration.

ERFA benchmark for the compact automatic dtr provider:

```text
164 samples · 1900–2100
max |error|   34.680 µs
mean |error|  14.899 µs
RMS error     17.172 µs
gate          100 µs
PASS
```

Official IERS ingestion workflow parsed 2,386 rows with the latest observed snapshot in that run dated 2026-09-08.

## Astronomy / Antikythera

Implemented:

- Metonic / Saros / Exeligmos;
- lunar mean model + JPL validation;
- bounded calibration and untouched holdout;
- anomalistic/draconic residual engine;
- NASA Saros 139 recurrence validation;
- evaluator for published NASA Besselian polynomial elements;
- JPL-published approximate planetary positions for Mercury–Saturn;
- Horizons planetary benchmark;
- evidence-labelled Antikythera digital-twin manifest.

The Besselian layer evaluates published NASA elements; it does not claim independent generation of Besselian elements from a full high-precision Sun/Moon ephemeris.

Historical Antikythera components are labelled as surviving evidence, strongly indicated reconstruction, reconstructed model or hypothetical feature.

## Planetary benchmark

15 JPL Horizons vector comparisons pass their declared approximate-model gates.

Observed error ranges:

```text
Mercury  0.7–3.4 thousand km
Venus    8.1–12.3 thousand km
Mars     8.2–45.8 thousand km
Jupiter  0.61–1.58 million km
Saturn   2.03–3.85 million km
```

These are lower-accuracy planetary models. JPL Horizons/integrated ephemerides remain the high-precision reference.

## Cosmic Chronology

Cosmic age is represented as a model-dependent inference, never as an absolute coordinate-time instant.

Implemented:

- flat ΛCDM;
- curved ΛCDM;
- CPL w0waCDM;
- deterministic numerical integration;
- independent/covariance uncertainty propagation;
- posterior-chain evaluation;
- official DESI DR2 chain reproduction.

Official full-chain age results reproduced by the project include:

- DESI DR2 + CMB flat ΛCDM: **13.788868 ± 0.015691 Gyr**;
- DESI DR2 + CMB curved ΛCDM: **13.703507 ± 0.045204 Gyr**;
- DESI DR2 + CMB + DESY5 w0waCDM: **13.759532 ± 0.019103 Gyr**.

## Three-language conformance

Independent calculations are run in:

- JavaScript;
- Python;
- Rust.

Compatibility domains:

- TT → TCG;
- Planck flat-ΛCDM age;
- Saros recurrence;
- Mars J2000 approximate vector.

The v2 compatibility workflow passes across all three implementations.

## Evidence / quality semantics

Evidence:

```text
OBSERVED · MEASURED · CALCULATED · MODELED
INFERRED · RECONSTRUCTED · SPECULATIVE · TEXTUAL_REFERENCE
```

Quality:

```text
REFERENCE · HIGH_PRECISION · APPROXIMATE
RECONSTRUCTION · CONCEPTUAL
```

Textual/Qur'anic material remains `TEXTUAL_REFERENCE / CONCEPTUAL` and is never used as a hidden numerical prior.

## Conformance

CLI:

```bash
npm run conformance
```

API on serverless deployment:

```text
POST /api/conformance
```

A public GitHub issue template is provided for independent conformance reports/discrepancies.

## Reproducibility / release

The `v2-release` workflow:

1. runs the full test suite;
2. validates the conformance corpus;
3. runs JS/Python/Rust golden-vector compatibility;
4. generates SHA-256 hashes for normative/reference files;
5. creates the `utime-2.0.0-standardization-candidate` bundle.

See:

- `benchmarks/v2-reference-results.json`
- `docs/standardization-readiness.md`
- `docs/preprint.md`
- `GOVERNANCE.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CITATION.cff`

## Position relative to external authorities

U-Time consumes and validates against authoritative external work:

- BIPM for SI metrology;
- IAU/IERS/SOFA for astronomical reference/time conventions;
- NASA/JPL for eclipse/solar-system references;
- Planck/DESI for cosmological observations/posteriors.

It does not supersede those systems.

## What v2 means

`2.0.0` means the repository is internally packaged for independent evaluation as a **standardization candidate**.

It does **not** mean external peer review, independent community adoption, or formal standards-body approval has already happened.
