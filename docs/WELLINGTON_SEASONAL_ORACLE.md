# Wellington Seasonal Solar Oracle — v0.10.0

M-Time validates the Diyanet-specific Wellington -18° worship-time astronomy path against JPL Horizons across four seasonal epochs.

## Cases

```text
SUMMER_JAN15  — 2026-01-15 local-date dawn
AUTUMN_MAR20  — 2026-03-20 local-date dawn
WINTER_JUN21  — 2026-06-21 local-date dawn
SPRING_SEP22  — 2026-09-22 local-date dawn
```

Observer:

```text
Wellington
longitude = 174.7772°
latitude  = -41.2889°
```

Profile target:

```text
DIYANET_IMSAK_FAJR_MINUS_18
rising Sun altitude = -18°
```

## Independent oracle path

For each season:

1. load official DE440 and IERS finals.all;
2. compute the apparent Sun vector;
3. solve the rising -18° crossing;
4. query JPL Horizons Sun quantity #4 at that solved UTC instant;
5. compare topocentric sky direction;
6. require direction error ≤ 0.001°;
7. require Horizons elevation to remain within 0.0015° of -18°.

Actions run **37196296276**:

```text
SUMMER_JAN15
direction_error_deg = 0.000173324462
horizons_target_residual_deg = 0.000045

AUTUMN_MAR20
direction_error_deg = 0.000167592092
horizons_target_residual_deg = 0.000041

WINTER_JUN21
direction_error_deg = 0.000138319236
horizons_target_residual_deg = 0.000060

SPRING_SEP22
direction_error_deg = 0.000141818973
horizons_target_residual_deg = 0.000003
```

Summary:

```text
4 / 4 PASS
max direction residual = 0.000173324462° ≈ 0.624 arcsec
max target residual = 0.000060°
fixed direction gate = 0.001°
```

## Falsification-driven hardening

The initial oracle failed in summer by roughly:

```text
0.0059° ≈ 21 arcsec
```

The threshold was not relaxed.

The discrepancy matched the scale of annual aberration. The solar observer path was therefore split explicitly from the geometric Sun path:

```text
geometric Sun
→ center-to-center elongation semantics

apparent Sun
→ light-time
→ Earth barycentric velocity
→ first-order annual aberration
→ observer/worship-time geometry
```

After that correction the seasonal cases pass the existing 0.001° physical-direction gate.

## IERS parser hardening

Seasonal expansion also exposed official fixed-width finals.all forms such as:

```text
I-0.0106308
```

and compact calendar-date tokens.

The parser now recognizes the MJD field by its numeric domain and accepts flag-prefixed negative UT1 values. Official-product and workspace gates pass.

## Boundary

This oracle validates astronomical geometry for the -18° Diyanet profile. It does not claim the -18° criterion is universal, nor does it substitute for a large historical replay of Diyanet's published Wellington timetable.
