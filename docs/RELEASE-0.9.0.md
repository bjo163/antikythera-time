# M-Time v0.9.0 — Computed Wellington Worship-Time Provider

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE COMPUTED WORSHIP-TIME POLICY RESEARCH PROTOTYPE**

## Increment over v0.8.0

v0.9.0 replaces the fixed Wellington-fajr fixture in the Diyanet policy path with a high-precision computed event.

### Versioned Diyanet worship profile

```text
DIYANET_IMSAK_FAJR_MINUS_18
Sun altitude = -18°
direction = rising
```

### Computation

```text
DE440
+ IERS EOP
+ Wellington observer
+ TT/TDB/UT1
+ IAU 2006/2000A
→ topocentric Sun altitude
→ -18° rising crossing
```

### Shawwal 1447 result

```text
conjunction = 2026-03-19 01:24 UTC
conjunction_to_fajr = 15.434722 h
conjunction_before_wellington_fajr = true
solver residual = -0.000067779102°
```

### Gate hardening

External-source workflows now use `set -euo pipefail`, preventing failed validation commands piped through `tee` from being reported as successful.

### Regression gates

- core Rust CI: PASS;
- Rust/Python compatibility: PASS;
- 21-case boundary oracle: PASS;
- 21-case multi-year oracle: PASS;
- fixed topocentric reference: PASS;
- Natural Earth mainland provider: PASS;
- real-source ingestion: PASS;
- Wellington/Diyanet computed fajr: PASS.

## Boundary

v0.9.0 computes the relevant Diyanet Wellington-fajr condition but does not claim that the -18° worship profile is universal across Islamic institutions or fiqh methodologies.
