# Test Report — U-Time 1.0.0

Date: 2026-10-01

## Core unit/integration gate

Final v1.0 full test gate:

- GitHub Actions run: `36831062747`
- Result: SUCCESS


```text
tests 85
pass  85
fail  0
```

The suite was expanded cumulatively; earlier time/astronomy/cosmology tests were not replaced.

## Relativistic-time validation

- six IAU SOFA reference vectors;
- TT↔TCG roundtrip;
- TDB↔TCB roundtrip;
- explicit TT↔TDB dtr boundary;
- precision-preserving two-part JD type.

Result: PASS.

## Automatic TDB−TT / ERFA benchmark

Workflow: `erfa-dtr-benchmark`

Successful run: `36829987304`.

```text
samples              164
max abs error         34.679807 µs
mean abs error        14.898798 µs
RMS error             17.171827 µs
threshold             100 µs
result                PASS
```

## NASA eclipse recurrence validation

The Saros 139 recurrence engine uses NASA's published Saros period and NASA GSFC greatest-eclipse reference epochs for 2024, 2042 and 2060.

Declared gate: first two recurrences remain within 30 minutes of reference greatest-eclipse TT.

Result: PASS.

This validates recurrence behavior, not full Besselian eclipse geometry.

## JPL planetary benchmark

Workflow: `jpl-planetary-benchmark`

Successful run: `36830365898`.

15 comparisons against JPL Horizons geometric heliocentric ecliptic vectors:

```text
Mercury  0.7–3.4 thousand km
Venus    8.1–12.3 thousand km
Mars     8.2–45.8 thousand km
Jupiter  0.61–1.58 million km
Saturn   2.03–3.85 million km
```

All declared body-specific gates: PASS.

## Cosmology / official DESI posterior validation

Workflow: `official-posterior`

Reference successful run: `36825466646`.

The full official Cobaya age posterior is summarized; a deterministic 6000-sample subset is recomputed independently through the project's Friedmann integrator.

All three supported DESI combinations pass the 0.001-Gyr weighted-mean agreement threshold.

## v1 independent implementation compatibility

Workflow: `v1-compatibility`

Successful run: `36830567705`.

Independent JavaScript/Python checks:

- TT→TCG: PASS;
- Planck age: PASS;
- one-Saros recurrence: PASS;
- JPL approximate Mars x/y/z: PASS.

Overall: **PASS**.

## Public site

GitHub Pages deployment is active:

```text
https://bjo163.github.io/antikythera-time/
```

## Scientific interpretation

Passing tests establish internal reproducibility and agreement with the declared external references at the declared accuracy class.

They do not establish international-standard adoption or overall superiority to NASA/JPL operational systems.
