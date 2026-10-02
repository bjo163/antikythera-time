# M-Time v0.2 — Hardening Research Prototype Release Record

Date: 2026-10-02

Status: **INTERNALLY REPRODUCIBLE HARDENING PROTOTYPE**

## What changed from v0.1

v0.1 proved the semantic architecture end-to-end. v0.2 hardens the flagship astronomy/calendar path:

- offline JPL DE440/SPK evaluation in Rust;
- DE440 ↔ Horizons ICRF vector oracle;
- WGS84 observer parallax;
- IAU 2006/2000A + IERS Earth-orientation transform;
- sub-arcsecond fixed-epoch agreement with Horizons;
- integrated UTC + site + IERS + DE440 → HijriAstronomicalState;
- explicit refraction model boundary;
- SHA-256 / Ed25519 source-integrity layer;
- multiple source-linked calendar-profile semantics;
- first Indonesia–Türkiye cross-jurisdiction Shawwal corpus;
- protocol/schema draft 0.2;
- Rust/WASM multi-profile demonstration.

## Release gates

- Rust CI: **60 passed / 0 failed** — run 36958335307
- DE440/SPK external oracle: **PASS** — run 36956737541
- integrated topocentric Horizons oracle: **PASS** — run 36958335289
- active topocentric gate: **0.001°**
- SHA-256 / Ed25519 integrity tests: **PASS** — run 36956837579
- WASM multi-profile engine: **PASS**
- final Pages deployment is required before the release branch is frozen.

## Scientific meaning

M-Time now has a high-precision airless geometric path for constructing a Hijri astronomical state from standard time, Earth orientation, a JPL ephemeris kernel, and a terrestrial observer.

It still does not turn astronomy into a religious/legal decision. Profiles, observations, jurisdiction, and authority remain separate layers.

## Claim boundary

This release is not:

- a new definition of the second;
- a replacement for UTC;
- an absolute universal clock;
- an operational replacement for JPL/SOFA/IERS;
- a ruling on which Hijri methodology is religiously correct;
- a proof that Antikythera directly encodes cosmic age;
- a claim that revelation texts supply physical constants;
- a formally adopted standard.

## Verdict

**The original M-Time concept is now implemented as a high-precision research prototype. The remaining path is broader validation, institutional ingestion, independent review, and external standardization — not another conceptual rewrite.**