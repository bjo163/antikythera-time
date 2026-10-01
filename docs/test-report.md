# Test Report — U-Time 2.0.0

Date: 2026-10-02

## Final core gate

GitHub Actions run: `36914542152`

```text
tests 103
pass  103
fail  0
```

## Cross-language conformance

JavaScript / Python / Rust compatibility: **PASS**.

Domains independently recomputed:

- TT→TCG;
- Planck flat-ΛCDM age;
- Saros recurrence;
- Mars J2000 approximate vector.

## External-reference gates

| Gate | Result |
|---|---|
| IAU SOFA time vectors | PASS |
| ERFA TDB−TT, 164 samples 1900–2100 | PASS |
| IERS finals.all IAU2000 ingestion | PASS |
| NASA Saros 139 recurrence | PASS |
| NASA 2026 Besselian t0 evaluator | PASS |
| JPL Horizons planetary vectors, 15 comparisons | PASS |
| official DESI DR2 posterior reproduction | PASS |

## ERFA dtr benchmark

```text
max absolute error   34.679807 µs
mean absolute error  14.898798 µs
RMS error            17.171827 µs
gate                 100 µs
```

## IERS ingestion

Workflow run `36913489054` parsed 2,386 EOP rows; the latest observed entry in that fetched snapshot was dated 2026-09-08.

## Release integrity

v2 release bundle run: `36914464594`.

Tarball SHA-256:

`adc96cd9744c9f3c0110fc7a746e4e4db82d07443383b66991ea74b12544bc7c`

## Interpretation

These gates demonstrate reproducibility and agreement with declared reference products at declared accuracy classes. They do not constitute external standards adoption or an operational replacement for NASA/JPL systems.
