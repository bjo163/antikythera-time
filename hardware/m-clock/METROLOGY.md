# M12 M-Clock Metrology Protocol

Repository status: **analysis pipeline executable; physical measurements not yet collected**.

## Required physical runs

- PPS latency/jitter;
- oscillator frequency offset;
- 1 h holdover;
- 6 h holdover;
- 24 h holdover;
- 72 h holdover;
- temperature sweep or natural thermal profile;
- GNSS-loss/reacquisition;
- power-cycle recovery.

The five timing datasets used by the machine-readable suite are:

1. PPS latency/jitter;
2. holdover >= 1 h;
3. holdover >= 6 h;
4. holdover >= 24 h;
5. holdover >= 72 h.

Thermal, GNSS-loss/reacquisition and power-cycle evidence remain separate physical logs/captures in the evidence manifest.

## Strict CSV contract

Header is exactly:

```text
elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha
```

Rules:

- at least two rows per file;
- finite elapsed/offset values;
- elapsed time is non-negative and non-decreasing;
- optional temperature, when present, must be finite;
- reference, device and firmware identity must be non-empty;
- identity must remain constant inside a CSV;
- the same identity must be used across all five suite datasets;
- 1 h / 6 h / 24 h / 72 h duration gates use **last elapsed minus first elapsed**, not merely the largest timestamp.

The strict format intentionally does not support quoted commas in identifiers. Evidence IDs should be stable machine identifiers without commas.

## Reproducible Rust analysis

Generate the metrology suite report with:

```bash
cargo run --release -p mtime-metrology -- suite \
  "$(git rev-parse HEAD)" \
  path/to/pps.csv \
  path/to/holdover-1h.csv \
  path/to/holdover-6h.csv \
  path/to/holdover-24h.csv \
  path/to/holdover-72h.csv \
  > hardware/m-clock/physical-evidence/metrology-report.json
```

The report schema is:

`hardware/m-clock/metrology-suite.schema.json`

Report identity:

`mtime-mclock-metrology-suite-1`

The first CLI argument is the exact Git SHA of the analysis code. The report records both the crate version and this commit so a later reviewer can identify the analyzer precisely.

## Reported statistics

For each dataset the Rust tool reports:

- number of samples;
- start/end elapsed time and measured duration;
- mean offset;
- population standard deviation of raw offset;
- ordinary-least-squares intercept;
- fitted frequency drift in ppm;
- population standard deviation of residuals around the fitted line;
- maximum absolute offset;
- measured temperature range when available.

The field `jitter_stddev_ns` is the fit-residual standard deviation, not the raw offset standard deviation. This prevents a clean linear oscillator drift from being mislabeled as random jitter.

The suite also reports the maximum absolute offset and maximum absolute fitted drift across the five datasets.

## Important interpretation boundary

The generated report always declares:

`measurement_characterization_only_no_accuracy_class_assigned`

A report is **not** an accuracy-class certificate. Numerical thresholds for a future physical accuracy class require real data, a stated use case, uncertainty review and an explicit governance decision.

## Evidence admission

For real physical evidence:

1. preserve all five raw CSV files;
2. generate `metrology-report.json` with the Rust CLI;
3. SHA-256 pin the raw CSVs and report in `physical-evidence/manifest.json`;
4. run `python scripts/hardware_readiness.py`;
5. have the measured error budget reviewed separately before M12 closes.

The physical-evidence validator cross-checks report identity, source paths, durations and device/reference/firmware identity against the manifest.

## Synthetic fixtures

`hardware/m-clock/fixtures/` contains synthetic CSVs solely for CI.

They prove that the software pipeline executes. They are not physical measurements and cannot satisfy M11/M12.
