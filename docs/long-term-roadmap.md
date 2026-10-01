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

## v0.7 — Cosmic Chronology Foundation — IMPLEMENTED

- separate `src/cosmology/` semantic layer;
- `CosmicAgeEstimate`, never UTime;
- adaptive Friedmann age integration;
- flat/non-flat ΛCDM;
- CPL w0waCDM;
- Planck 2018 + DESI DR2 reference adapters;
- numerical, independent, covariance and posterior-chain uncertainty paths;
- analytic/numerical falsification tests;
- Cosmic Chronology web lab;
- textual/conceptual Qur'anic map outside numerical inference.

## v0.7.1 — Posterior Reproduction

- ingest official Planck/DESI released covariance or MCMC chains;
- reproduce posterior cosmic-age distributions;
- compare engine percentiles against published derived parameters;
- record chain hashes / provenance.

## v0.8 — Eclipse Engine

- syzygy + node candidate model;
- Saros/Exeligmos indexing;
- NASA eclipse-catalog validation;
- recall, timing residuals and false-positive rates.

## v0.9 — Solar + Planetary Cosmos

- Sun/ecliptic longitude;
- five classical planets;
- compare alternative Antikythera gear reconstructions;
- JPL ephemeris holdout validation.

## v0.10 — Relativistic Time

- validated TT↔TDB and TDB↔TCB transformations;
- IAU/IERS/SOFA reference vectors;
- proper-time examples with declared observer worldlines.

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

- official posterior-chain ingestion at scale;
- model-dependent cosmic-age distributions;
- ΛCDM versus extensions without declaring a theological or political “winner”;
- model evidence/fit results reported only from appropriate external analyses.

## v1.4 — Historical Antikythera Reconstruction

- documented gear trains/tooth counts;
- pin-and-slot lunar anomaly;
- front/back dials;
- Metonic, Callippic, Saros, Exeligmos, Games;
- alternative scholarly reconstructions with uncertainty labels.

## v2.0 — Independent Review / Standardization

- frozen spec candidate;
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
