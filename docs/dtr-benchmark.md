# Automatic TDB−TT Benchmark

Provider: `NASA_TP_2022_SIMPLE_TDB_TT`

Reference: ERFA/PyERFA `dtdb`, which is SOFA-derived.

Benchmark window: 1900–2100, quarterly samples every five years.

Successful workflow: `erfa-dtr-benchmark` run `36829987304`.

Results:

```text
samples                164
max absolute error     34.679807 microseconds
mean absolute error    14.898798 microseconds
RMS error              17.171827 microseconds
acceptance threshold   100 microseconds
result                  PASS
```

## Meaning

The compact built-in provider is suitable as an explicitly labelled geocentric approximation within the benchmarked regime.

It is not identical to the full SOFA/ERFA `Dtdb` model and must not be advertised as such.

High-precision applications should supply a higher-accuracy provider through the canonical TT↔TDB interface.
