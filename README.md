# Antikythera Time — U-Time 1.0.0

## Live site

https://bjo163.github.io/antikythera-time/

U-Time is an experimental **time/astronomy protocol specification candidate** that combines:

- SI/J2000 time representation;
- explicit UTC/TAI/TT/TCG/TDB/TCB semantics;
- Antikythera-inspired cycle computation;
- NASA/JPL external validation;
- model-dependent Cosmic Chronology;
- evidence/provenance/uncertainty labels.

> Universal protocol ≠ absolute cosmic clock.

## U-Time 1.0 specification candidate

Normative assets:

- `spec/UTIME-1.0.md`
- `spec/utime-v1.schema.json`
- `spec/golden-vectors.json`

Protocol records distinguish:

```text
value
+ scale / frame / model context
+ uncertainty
+ evidence state
+ provenance
+ semantic boundary
```

Evidence states:

```text
OBSERVED · MEASURED · CALCULATED · MODELED
INFERRED · RECONSTRUCTED · SPECULATIVE · TEXTUAL_REFERENCE
```

## v1 validation summary

### Core automated tests

**85 tests / 85 pass / 0 fail** before final documentation freeze.

Coverage includes metrology, lunar/Saros models, cosmology, official DESI posterior reproduction, relativistic coordinate time, automatic TDB−TT, eclipse recurrence, planetary approximation and v1 protocol records.

### Relativistic coordinate time

- TT ↔ TCG: IAU 2000 B1.9;
- TDB ↔ TCB: IAU 2006 B3;
- TT ↔ TDB: canonical wrapper with explicit `dtr=TDB−TT`;
- six IAU SOFA reference vectors: PASS.

A compact automatic geocentric TDB−TT provider is also available and explicitly labelled approximate.

ERFA benchmark, 1900–2100:

```text
164 samples
max |error|   34.680 µs
mean |error|  14.899 µs
RMS error     17.172 µs
gate          100 µs
result        PASS
```

### Eclipse recurrence

The v0.9 engine implements Saros/Exeligmos recurrence rather than pretending to be a full Besselian solver.

NASA Solar Saros 139 reference check:

- 2042 recurrence: < 30 min timing residual;
- 2060 recurrence: < 30 min timing residual;
- recurrence gate: PASS.

### Planetary Cosmos

Implements JPL SSD's published **Approximate Positions of the Planets** Table-1 Keplerian model for 1800–2050.

Bodies: Mercury, Venus, Earth/EM-barycenter approximation, Mars, Jupiter, Saturn.

15 external comparisons against JPL Horizons all pass declared model-specific gates.

Observed vector-error ranges:

```text
Mercury  0.7–3.4 thousand km
Venus    8.1–12.3 thousand km
Mars     8.2–45.8 thousand km
Jupiter  0.61–1.58 million km
Saturn   2.03–3.85 million km
```

This is a lower-accuracy display/research model. Horizons/integrated ephemerides remain the high-precision reference.

### Independent implementation compatibility

A separate Python implementation recomputes four v1 golden domains without calling the JavaScript implementation:

- TT→TCG;
- Planck flat-ΛCDM cosmic age;
- Saros recurrence;
- Mars J2000 approximate vector.

Workflow `v1-compatibility`: **PASS**.

## Cosmic Chronology

Cosmic age remains semantically separate from `UTime`.

Supported:

- flat ΛCDM;
- curved ΛCDM;
- CPL `w0waCDM`;
- deterministic numerical integration;
- independent/covariance uncertainty;
- posterior-chain evaluation.

Official DESI DR2 full-chain results reproduced in the project:

- DESI DR2 + CMB flat ΛCDM: **13.788868 ± 0.015691 Gyr**;
- DESI DR2 + CMB curved ΛCDM: **13.703507 ± 0.045204 Gyr**;
- DESI DR2 + CMB + DESY5 w0waCDM: **13.759532 ± 0.019103 Gyr**.

The independent Friedmann engine passes a 1-Myr weighted-mean agreement gate against the official Cobaya-derived age on matched chain samples.

## Antikythera boundary

Antikythera contributes the computational architecture:

```text
astronomical phenomenon
→ period / ratio
→ state / recurrence
→ prediction
→ external validation
```

It does **not** historically provide:

- atomic seconds;
- relativistic coordinate time;
- JPL-style planetary ephemerides;
- Hubble parameters;
- a Big-Bang date;
- an exact age of the Universe.

## Qur'anic research boundary

Qur'anic celestial/reckoning material is stored as a textual/conceptual research layer.

It is never used as a numerical prior for H₀, Ωm, ΩΛ, w₀, wₐ, eclipse timing or planetary position.

## NASA/JPL positioning

U-Time is **not more advanced than NASA/JPL overall**.

NASA/JPL remains substantially more mature for:

- integrated high-precision ephemerides;
- SPICE;
- spacecraft navigation;
- orbit determination;
- operational mission geometry.

U-Time's contribution is a transparent cross-domain protocol/integration architecture with explicit epistemic status, provenance and uncertainty.

See `docs/engine-positioning.md`.

## Documentation

Key documents:

- `spec/UTIME-1.0.md`
- `docs/v1-conformance.md`
- `docs/dtr-benchmark.md`
- `docs/eclipse-engine.md`
- `docs/planetary-cosmos.md`
- `docs/relativistic-time-core.md`
- `docs/cosmology-age-results.md`
- `docs/official-desi-posterior.md`
- `docs/quranic-celestial-computation-map.md`
- `docs/claim-matrix.md`
- `docs/test-report.md`
- `docs/reproducibility.md`
- `docs/long-term-roadmap.md`

## Status

```text
VERSION                    1.0.0
STATUS                     SPECIFICATION CANDIDATE
INTERNATIONAL STANDARD     NO
NASA/JPL REPLACEMENT       NO
ABSOLUTE COSMIC CLOCK      NO
PUBLIC REPRODUCIBLE LAB    YES
```
