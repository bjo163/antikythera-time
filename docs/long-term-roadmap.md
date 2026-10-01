# U-Time Long-Term Roadmap

## Mission

Build a reproducible astronomical time protocol and validation framework.

U-Time is not a claim of one absolute universal clock. The target is:

```text
shared SI duration
+ declared epoch
+ declared time scale
+ declared reference frame
+ uncertainty
+ falsifiable astronomical models
```

## v0.1 — Core Coordinate
- SI second as the physical unit.
- J2000.0 as coordinate epoch.
- integer elapsed duration.

## v0.2 — Evidence Boundary
- separate metrology, historical engineering and theological/conceptual references.
- define "universal protocol" rather than "absolute cosmic clock".

## v0.3 — Scale Safety + Website
- explicit TT/TAI/UTC/TDB/TCB metadata.
- refuse fake cross-scale conversion.
- browser evidence dashboard.

## v0.4 — External Validation
- NASA/GSFC phase-zero.
- JPL Horizons reference.
- MAE/RMSE/max-error reporting.

## v0.5 — Calibration + Holdout
- bounded mean-period/epoch calibration.
- untouched 2013–2026 holdout.
- reject training-only improvements.

## v0.6 — Antikythera Anomaly / Residual Engine
- fixed anomalistic month: 27.554551 d.
- fixed draconic month: 27.212220 d.
- no free-frequency search.
- fit only amplitudes/phases of physically declared cycles.
- accept only if untouched holdout improves.
- expose Saros 223 synodic ≈ 239 anomalistic ≈ 242 draconic diagnostic.

## v0.7 — Eclipse Engine
- build a node + syzygy eclipse-candidate model.
- implement Saros/Exeligmos indexing.
- compare against NASA eclipse catalogs.
- measure event-time/type recall and false positives.
- distinguish "eclipse cycle recurrence" from full modern eclipse prediction.

## v0.8 — Solar + Planetary Cosmos
- Sun longitude and seasonal calendar.
- zodiac/ecliptic display.
- five classical planets as an explicit model layer.
- compare reconstructed cycle/gear hypotheses against JPL ephemerides.
- label each planetary component CONFIRMED / RECONSTRUCTED / SPECULATIVE.

## v0.9 — Relativistic Time
- validated TT↔TDB and TCB↔TDB transformations.
- IAU/IERS/SOFA reference vectors.
- proper-time examples for observer worldlines.
- never silently convert coordinate time to proper time.

## v1.0 — U-Time Specification Candidate
- normative data schema.
- reference JavaScript implementation.
- versioned test vectors.
- reproducibility manifest.
- uncertainty propagation.
- published scientific validation report.
- independent implementation compatibility tests.

## v1.1 — Multi-Observer / Spacecraft Time
- Earth geocenter, barycenter and spacecraft observer contexts.
- observer metadata and clock-path declarations.
- proper-time versus coordinate-time demonstrations.

## v1.2 — Astronomical Age API
- age = event_B - event_A with explicit scale/frame/uncertainty.
- stellar/planetary ages remain observational/model estimates.
- separate numerical duration precision from origin-event uncertainty.

## v1.3 — Cosmology Chronology Lab
This is where "age of the universe" belongs.

It must not be inferred from Antikythera cycles.

Inputs should include a declared cosmological model and observational parameters
(e.g. expansion history / Hubble parameter and density parameters, or published CMB-derived model fits).

Output:

```text
cosmic age estimate
+ cosmological model
+ parameter set
+ observational source
+ uncertainty
```

The currently standard cosmological estimate is about 13.8 billion years, but this is a model-dependent astrophysical inference, not a reading from a universal clock.

## v1.4 — Historical Antikythera Reconstruction Layer
- encode published gear trains and tooth counts.
- implement lunar anomaly pin-and-slot models.
- front/back dial render.
- Metonic, Callippic, Saros, Exeligmos and Games dial.
- compare alternative scholarly reconstructions.
- preserve uncertainty where fragments are missing.

## v2.0 — Standardization / Independent Review
- formal specification.
- public reference dataset.
- independent reviewers and reproducibility challenges.
- multiple implementations.
- no "universal standard" claim until external adoption and validation exist.

## Scientific rule

Every layer must answer:

1. What is measured?
2. What is conventional?
3. What is modeled?
4. What is reconstructed?
5. What is uncertain?
6. What external reference can falsify it?
