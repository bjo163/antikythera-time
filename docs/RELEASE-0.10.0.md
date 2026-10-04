# M-Time v0.10.0 — Seasonal Solar Oracle + Live Lab

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE SEASONAL SOLAR-ORACLE + LIVE-LAB RESEARCH PROTOTYPE**

## Increment over v0.9.0

### 1. Four-season Wellington Sun oracle

M-Time now independently validates the Diyanet-specific -18° Wellington solar path against JPL Horizons in January, March, June and September 2026.

```text
4 / 4 PASS
max direction residual = 0.000173324462°
max -18° target residual = 0.000060°
direction gate = 0.001°
```

### 2. Apparent-Sun hardening

The first seasonal run failed by ~0.0059° (~21 arcsec). No threshold was weakened.

M-Time added an observer-style apparent solar vector with:

- iterative solar light-time;
- Earth barycentric velocity;
- first-order annual aberration.

Geometric Sun vectors remain separate for geocentric center-to-center elongation.

### 3. IERS fixed-width parser hardening

Official finals.all rows can contain flag-prefixed negative UT1 values and compact date fields. The parser now handles these forms and remains interpolation-safe.

### 4. M-Time Live Lab

The GitHub Pages surface is rebuilt as a current dashboard with:

- live release / commit information;
- latest public GitHub Actions state;
- current oracle metrics;
- Rust/WASM criterion comparison;
- policy-provider results;
- integrity/falsification status;
- docs and machine-readable evidence links.

GitHub Pages remains the deployment platform. Vercel is unnecessary while the app remains static/WASM with public data.

### Gates

- Rust workspace tests: **100 passed / 0 failed**;
- Rust/Python compatibility: PASS;
- official IERS product parser: PASS;
- 21-case boundary Moon oracle: PASS;
- 21-case multi-year Moon oracle: PASS;
- 4-case seasonal Sun oracle: PASS;
- existing Wellington/Diyanet replay: PASS;
- geospatial provider: PASS;
- historical falsification: PASS;
- source ingestion: PASS.

## External Horizons coverage

```text
21 Hijri-boundary Moon cases
+ 21 multi-year Moon cases
+ 4 seasonal Wellington Sun cases
= 46 active external JPL Horizons comparisons
```

## Boundary

v0.10.0 materially hardens the solar worship-time path and public observability. It does not yet provide independent implementation by another team, a multi-decade multi-country historical corpus, or formal standards adoption.
