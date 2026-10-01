# U-Time Long-Term Roadmap

## Mission

Build a reproducible time/astronomy framework where every quantity declares whether it is measured, conventional, modeled, inferred, reconstructed or speculative.

## Completed through v1.0

- **v0.1** — SI-duration + J2000 coordinate core.
- **v0.2** — evidence boundary: scientific standards vs historical/textual inspiration.
- **v0.3** — explicit time-scale/frame safety + website.
- **v0.4** — NASA/JPL lunar external validation.
- **v0.5** — bounded calibration + untouched holdout.
- **v0.6** — anomalistic/draconic residual engine.
- **v0.7** — separate Cosmic Chronology Engine.
- **v0.7.1** — official DESI DR2 posterior reproduction.
- **v0.8** — TT/TCG/TDB/TCB relativistic coordinate-time core.
- **v0.8.1** — automatic approximate geocentric TDB−TT provider benchmarked against ERFA.
- **v0.9** — NASA-validated Saros/Exeligmos recurrence engine.
- **v0.10** — JPL approximate planetary-position layer + Horizons benchmark.
- **v1.0** — protocol/schema/golden-vector freeze + independent Python compatibility gate.

## v1.0 — SPECIFICATION CANDIDATE — IMPLEMENTED

Normative assets:

- `spec/UTIME-1.0.md`
- `spec/utime-v1.schema.json`
- `spec/golden-vectors.json`

Reference layers:

- JavaScript primary implementation;
- independent Python compatibility checker.

Validation includes:

- IAU SOFA reference vectors;
- ERFA TDB−TT benchmark;
- NASA eclipse recurrence references;
- JPL Horizons planetary-vector benchmark;
- Planck/DESI cosmology/posterior checks;
- JS↔Python golden-vector compatibility.

## Next after v1.0

### v1.1 — Higher-accuracy time/observer providers
- full SOFA/ERFA-equivalent topocentric `Dtdb` provider;
- UT1/EOP provider;
- spacecraft observer/worldline interfaces;
- proper-time examples.

### v1.2 — Full eclipse geometry
- Besselian elements;
- local contact circumstances;
- ΔT/EOP provenance;
- NASA catalog comparison over large historical/future samples.

### v1.3 — Planetary Digital Twin
- alternate published Antikythera planetary gear reconstructions;
- front-dial visualization;
- JPL ephemeris residual distributions rather than three-epoch benchmark only.

### v1.4 — Independent review
- third-party implementation;
- public conformance challenge;
- archived benchmark datasets;
- research paper/preprint.

## v2.0 — External standardization candidate

A repository MUST NOT self-declare international standard adoption. v2.0 requires external review, multiple independent implementations and relevant standards/community engagement.

## Permanent scientific rule

Every layer must state:

1. what was observed/measured;
2. what is a convention;
3. what is modeled/inferred;
4. what is reconstructed/speculative;
5. what uncertainty applies;
6. what external evidence could falsify it.
