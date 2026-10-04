# Multi-Year Topocentric Oracle — v0.5.0

M-Time now runs a second external JPL Horizons topocentric regression suite that is independent of Hijri policy decisions.

## Fixed regression epochs

- 2024-03-01 12:00 UTC — JD 2460371.0
- 2025-03-01 12:00 UTC — JD 2460736.0
- 2026-03-01 12:00 UTC — JD 2461101.0

These are astronomy regression epochs, not month-start determinations.

## Sites

The same seven-site geographic spread used by the global boundary matrix:

- Jakarta
- Ankara
- Makkah
- Wellington
- New York
- Santiago
- Cape Town

Total: **21 multi-year cases**.

## Result

```text
21 / 21 PASS
max_direction_error_deg=0.000336345642
threshold_deg=0.001000000000
```

Worst case:

```text
Y2026_MAR01_12UTC_ANKARA
direction_error_deg=0.000336345642
```

Approximately 1.21 arcsec.

## Combined external coverage

```text
21 global Hijri-boundary cases
+ 21 global multi-year geometry cases
= 42 JPL Horizons comparisons
```

The active physical gate remains unchanged at 0.001° spherical horizon-direction separation.
