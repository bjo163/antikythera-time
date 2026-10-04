# M-Time v0.13.0 — Antikythera Accuracy Program I

Date: 2026-10-04

Status: **MULTI-CENTURY DIGITAL ANTYKITHERA ERROR CHARACTERIZATION COMPLETE**

## Goal

v0.13.0 does not add a new correction term. It measures the existing digital machine across a much wider interval so later corrections can be evidence-driven.

## Coverage

```text
1900–2100 monthly epochs = 2,412
targeted phase/anomaly epochs = 15,676
```

Targeted diagnostics include:

- new / first-quarter / full / last-quarter mean lunar phase;
- lunar perigee / apogee mean anomaly phase;
- solar perihelion / aphelion anomaly phase.

## Reference-frame correction

The initial long-range run compared model-of-date longitude coefficients to a fixed J2000 ecliptic reference and therefore exposed precession-scale frame rotation as if it were dynamical error.

v0.13 adds an explicit reference transform:

```text
DE440 ICRF/GCRS
→ IAU 2006 precession-bias
→ mean equator/equinox of date
→ mean obliquity
→ mean ecliptic of date
```

No Antikythera coefficient was changed.

## Digital profile results

```text
Sun absolute:
P50 0.002362880°
P95 0.006319944°
P99 0.007860385°
MAX 0.009572456°

Sun dynamic MAX
0.010229337°

Moon absolute:
P50 0.105735848°
P95 0.305809037°
P99 0.373034617°
MAX 0.427321722°

Moon dynamic:
P50 0.132205667°
P95 0.360167526°
P99 0.447837600°
MAX 0.528604874°

Moon-Sun phase:
P50 0.105996365°
P95 0.304293570°
P99 0.373298749°
MAX 0.424730056°
```

## Phase decomposition

Worst-bin mean digital lunar dynamic error:

```text
synodic     0.148989455°
anomalistic 0.159840566°
draconic    0.232686478°
```

The draconic bin is the strongest current diagnostic signal. It is not yet treated as causal evidence.

## Artifacts

The GitHub Actions calibration run publishes:

- monthly-1900-2100.csv
- targeted-phases.csv
- phase-bins.csv
- summary.csv
- report.txt
- source SHA-256

## Regression status

Core CI, Rust/Python compatibility, SPK reference, topocentric reference, 21-case boundary matrix, 21-case multi-year matrix and seasonal Wellington Sun oracle remain green on the M6 changes.

## Next

M7 will improve historical-reconstruction provenance in parallel. M8 will introduce an explicit correction architecture and use M6 diagnostics for ablation experiments.

No v0.13 result changes the separation:

```text
Software Antikythera core
≠
JPL reference oracle
≠
calendar/worship policy
≠
revelation semantics
```
