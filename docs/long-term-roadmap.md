# U-Time Long-Term Roadmap

## Mission

Build a reproducible time/astronomy framework where every quantity declares whether it is measured, conventional, modeled, inferred, reconstructed or speculative.

## Completed foundation

- **v0.1** — SI-duration + J2000 coordinate core.
- **v0.2** — evidence boundary: scientific standards vs historical/textual inspiration.
- **v0.3** — explicit time-scale/frame safety + website.
- **v0.4** — NASA/JPL lunar external validation.
- **v0.5** — bounded calibration + untouched holdout.
- **v0.6** — anomalistic/draconic fixed-cycle residual engine.
- **v0.7** — separate Cosmic Chronology Engine.
- **v0.7.1** — official DESI DR2 posterior reproduction.
- **v0.8** — relativistic coordinate-time core.

## v0.8 — Relativistic Time Core — IMPLEMENTED

- separate two-part-Julian-Date `CoordinateTime` type;
- explicit TCG time scale;
- TT↔TCG canonical relation from IAU 2000 B1.9;
- TDB↔TCB canonical relation from IAU 2006 B3;
- canonical TT↔TDB wrapper with explicit `dtr = TDB-TT`;
- six IAU SOFA reference vectors;
- no invented automatic `dtr` model;
- derived-work/licensing notice;
- live Pages dashboard status.

## v0.8.1 — Full TDB−TT Provider

- integrate an intact/validated SOFA/ERFA-compatible `Dtdb` provider or authoritative time ephemeris;
- include topocentric observer parameters and UT1 where appropriate;
- cross-check against reference vectors and JPL/SPICE time conversion behavior;
- keep provider provenance and uncertainty explicit.

## v0.9 — Eclipse Engine

- syzygy + node candidate model;
- Saros/Exeligmos indexing;
- NASA eclipse-catalog validation;
- recall, timing residuals and false-positive rates.

## v0.10 — Solar + Planetary Cosmos

- Sun/ecliptic longitude;
- five classical planets;
- compare alternative Antikythera gear reconstructions;
- JPL ephemeris holdout validation.

## v1.0 — U-Time Specification Candidate

- normative data schema;
- versioned test vectors;
- reference JavaScript implementation;
- uncertainty contract;
- independent implementation compatibility tests.

## v1.1 — Multi-Observer / Spacecraft Time

- geocenter/barycenter/spacecraft contexts;
- coordinate versus proper time;
- observer trajectory metadata.

## v1.2 — Astronomical Age API

- event-to-event age with explicit scale/frame;
- inferred stellar/planetary ages with provenance + uncertainty;
- numerical precision separated from uncertain origin epoch.

## v1.3 — Cosmology Posterior / Model Comparison

- additional official posterior-chain ingestion as datasets evolve;
- model-dependent cosmic-age distributions;
- compare models without collapsing model dependence into one absolute age.

## v1.4 — Historical Antikythera Reconstruction

- documented gear trains/tooth counts;
- pin-and-slot lunar anomaly;
- front/back dials;
- Metonic, Callippic, Saros, Exeligmos, Games;
- alternative scholarly reconstructions with uncertainty labels.

## v2.0 — Independent Review / Standardization

- frozen specification candidate;
- public reference datasets;
- third-party reproducibility challenge;
- multiple independent implementations;
- external review before any claim of standard adoption.

## Permanent scientific rule

Every layer must state:

1. what was observed/measured;
2. what is a convention;
3. what is modeled/inferred;
4. what is reconstructed/speculative;
5. what uncertainty applies;
6. what external evidence could falsify it.
