# Global Topocentric Oracle Matrix — v0.4.0

M-Time validates its airless topocentric Moon direction against JPL Horizons using the same DE440 + IERS EOP high-precision path used by the flagship engine.

## Matrix dimensions

Three Hijri-boundary reference epochs in 1447 H:

- Ramadan
- Shawwal
- Dhulhijjah

Seven geographic sites:

- Jakarta — Southeast Asia
- Ankara — Türkiye
- Makkah — Hijaz
- Wellington — Oceania
- New York — North America
- Santiago — South America
- Cape Town — Africa

Total: **21 cases**.

## Gate

The physical spherical horizon-direction separation must satisfy:

```text
direction_error_deg <= 0.001°
```

This is deliberately based on sky-direction separation rather than raw azimuth difference, because azimuth is ill-conditioned close to zenith.

## v0.4.0 result

```text
case_count=21
max_direction_error_deg=0.000359059275
threshold_deg=0.001000000000
```

Result: **21/21 PASS**.

Worst measured case:

```text
RAMADAN_WELLINGTON
direction_error_deg=0.000359059275
```

That is approximately 1.29 arcsec and remains below the fixed 0.001° gate.

## Remaining validation work

This release expands geographic breadth. It does not yet close the multi-year validation requirement. The next astronomy-validation increment should add independent epochs from additional Hijri/Gregorian years while preserving the same sites and physical gate.
