# M-Time v0.4.0 — Global Oracle Expansion

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE GLOBAL-ORACLE RESEARCH PROTOTYPE**

## Increment over v0.3.0

### 1. 21-case global Horizons matrix

The topocentric oracle expands from three sites to seven:

```text
Jakarta
Ankara
Makkah
Wellington
New York
Santiago
Cape Town
```

across three 1447 H boundary epochs:

```text
Ramadan
Shawwal
Dhulhijjah
```

Total: **21 external JPL Horizons comparisons**.

### 2. Fixed physical gate retained

The threshold is unchanged:

```text
direction_error_deg <= 0.001°
```

No tolerance weakening was introduced.

### 3. Result

```text
21 / 21 PASS
maximum = 0.000359059275°
worst case = RAMADAN_WELLINGTON
```

The maximum is approximately 1.29 arcsec.

### 4. Machine-readable matrix summary

The CI artifact now records:

- case count;
- maximum direction error;
- threshold;
- all per-case residuals;
- source SHA-256 values for DE440 and IERS EOP inputs.

## Boundaries

v0.4.0 closes the immediate geographic-expansion step, not the full multi-year validation program. Multi-year epochs remain an open production/external gate.
