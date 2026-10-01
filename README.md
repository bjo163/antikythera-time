# Antikythera Time — U-Time v0.2 (JavaScript MVP)

Experimental time protocol combining a **modern metrology core** with an
**Antikythera-inspired astronomical cycle layer**.

> Status: research prototype, not a replacement for UTC, TAI, TT, TDB, TCB, IERS,
> BIPM, IAU, JPL ephemerides, or legal/civil time standards.

## Core claim

U-Time does **not** claim an absolute clock for the universe.

```text
U-Time = elapsed SI time + epoch + time scale + reference frame + uncertainty
```

v0.2 uses SI nanoseconds, J2000.0 (JD 2451545.0 TT), JavaScript BigInt,
explicit time-scale/reference-frame metadata, and an uncertainty field.

## What is actually adopted from Antikythera?

The project adopts the **computational principle**, not the ancient mechanism as a
modern metrological clock:

```text
astronomical period -> ratio/cycle -> phase/index -> readable prediction
```

The cycle engine includes the Metonic structure (235 lunar months / about 19 years),
the Saros structure (223 lunar months), and an idealized gear-ratio primitive.
The software phase is referenced to J2000 for reproducibility; it is **not** claimed
to reproduce the historical zero-point of an Antikythera dial.

## Evidence hierarchy

### A. Metrology / physics
- BIPM SI second: https://www.bipm.org/en/si-base-units/second
- BIPM history of the second: https://www.bipm.org/en/history-si/second
- Validated astronomical scale/reference-system transformations should follow IAU/IERS.

### B. Historical engineering
- Freeth et al., Nature 444 (2006), DOI 10.1038/nature05357.
- Freeth et al., Nature 454 (2008), DOI 10.1038/nature07130.

These support Antikythera as a geared astronomical calculator with Metonic and
Saros/eclipse-cycle functions. They do not establish an absolute universal clock.

### C. Qur'anic conceptual references
Qur'an 10:5, 55:5 and 21:33 can be studied as textual/conceptual references to
celestial regularity, reckoning and motion. They are not treated here as experimental
metrology or substitutes for astronomical measurement.

## Run

```bash
npm test
npm run demo
```

Requires Node.js 20+. No runtime dependencies.

## Accuracy boundary

v0.2 intentionally refuses fake TT -> TDB/TCB/UTC conversion. Correct transformations
need validated IAU/IERS algorithms and, where applicable, Earth-orientation or
leap-second data.

The mean synodic month in the cycle layer is a cycle-model constant, not an ephemeris
and not an exact prediction of every observed lunar phase.

## Goal for v0.3

1. IAU/IERS-backed time-scale conversions.
2. Reference vectors from recognized astronomical software/data.
3. Ephemeris-backed Moon phase and eclipse validation.
4. Relativistic proper-time demonstrations with declared observer paths.
5. Reproducible uncertainty propagation.
