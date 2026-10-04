# M-Time v0.5.0 — Global + Multi-Year Oracle

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE GLOBAL + MULTI-YEAR ORACLE RESEARCH PROTOTYPE**

## Increment over v0.4.0

v0.4.0 expanded geographic coverage to seven sites across three 1447 H boundary epochs.

v0.5.0 adds a second external oracle dimension: **time**.

### Multi-year epochs

```text
2024-03-01 12:00 UTC
2025-03-01 12:00 UTC
2026-03-01 12:00 UTC
```

across:

```text
Jakarta
Ankara
Makkah
Wellington
New York
Santiago
Cape Town
```

### Result

```text
21 / 21 multi-year PASS
max multi-year residual = 0.000336345642°
worst = 2026-03-01 12:00 UTC × Ankara
```

The fixed 0.001° physical direction gate remains unchanged.

Combined active external validation:

```text
21 global boundary cases
+ 21 global multi-year cases
= 42 JPL Horizons comparisons
```

## Delivery automation

The new multi-year workflow is part of post-release validation, so future releases automatically re-run both the global boundary matrix and the multi-year matrix.

## Boundary

This materially improves multi-year coverage but does not constitute multi-decade independent validation. Broader years, additional seasonal epochs, independent implementations, and external review remain open.
