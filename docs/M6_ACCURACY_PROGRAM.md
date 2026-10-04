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

- **absolute angular residual** — model dial versus DE440 transformed into the IAU 2006 mean ecliptic of date;
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


## Final M6 result

Actions run: `37202646134`

```text
monthly_epochs  = 2412
targeted_epochs = 15676
```

Digital profile:

```text
Sun absolute P50/P95/P99/MAX
0.002362880 / 0.006319944 / 0.007860385 / 0.009572456°

Sun dynamic MAX
0.010229337°

Moon absolute P50/P95/P99/MAX
0.105735848 / 0.305809037 / 0.373034617 / 0.427321722°

Moon dynamic P50/P95/P99/MAX
0.132205667 / 0.360167526 / 0.447837600 / 0.528604874°

Moon-Sun phase P50/P95/P99/MAX
0.105996365 / 0.304293570 / 0.373298749 / 0.424730056°
```

Worst lunar dynamic-error phase bins by maximum-containing bin:

```text
synodic:
phase 0.9167–0.9583
mean 0.148989455°
P95  0.354336172°

anomalistic:
phase 0.0417–0.0833
mean 0.159840566°
P95  0.378174330°

draconic:
phase 0.5833–0.6250
mean 0.232686478°
P95  0.437391361°
```

The draconic-conditioned bin has the largest mean residual of the three diagnostics. This makes lunar-node/draconic structure the first M8 candidate for **ablation testing**, not an automatic correction.

### Important frame finding

The first M6 run used fixed-J2000 ecliptic reference longitudes and produced an apparent ~1.4°/century common Sun/Moon drift. That was a calibration-frame mismatch. The final M6 run precesses the DE440 ICRF/GCRS vector into the IAU 2006 mean equator/equinox of date and rotates it with mean obliquity of date before comparison.

No `mtime-antikythera` coefficient was changed to obtain the corrected M6 numbers.
