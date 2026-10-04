# M6 — Antikythera Accuracy Program I

M6 characterizes Software Antikythera error before any new digital correction term is introduced.

## Calibration range

```text
1900-01 through 2100-12
2412 monthly epochs
DE440 short kernel reference
J2000 TT dynamic baseline
```

The analysis records both:

- **absolute angular residual** — model dial versus DE440 J2000-ecliptic longitude;
- **dynamic residual** — change from the J2000 baseline versus DE440 change from the same baseline.

This prevents a static frame/epoch offset from being confused with long-term dynamical drift.

## Phase-sensitive sampling

The M6 analyzer also evaluates targeted mean-cycle epochs for:

- new / first-quarter / full / last-quarter lunar phase;
- lunar perigee / apogee phase;
- solar perihelion / aphelion anomaly phase.

These samples are deliberately generated from independent compact cycle relations and then compared against DE440. They are diagnostic samples, not claims that those mean-cycle timestamps are exact physical events.

## Machine-readable artifacts

The workflow stores:

```text
monthly-1900-2100.csv
targeted-phases.csv
phase-bins.csv
summary.csv
report.txt
source-sha256.txt
```

`phase-bins.csv` bins digital lunar dynamic error against:

- synodic phase;
- anomalistic phase;
- draconic phase.

Each dimension uses 24 phase bins and publishes count, mean, P95 and maximum residual.

`summary.csv` publishes mean/P50/P95/P99/max for historical and digital Sun/Moon/phase metrics.

## M6 rule

No new correction term is permitted merely because a residual looks large. M6 first measures and decomposes the error. M8 will introduce explicit, versioned correction architecture only after these patterns are understood.
