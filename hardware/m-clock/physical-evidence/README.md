# Physical M-Clock Evidence Directory

This directory is a **fail-closed evidence intake boundary** for M11/M12.

The repository may contain templates and synthetic fixtures for software testing, but they do **not** count as proof that a physical M-Clock exists or has measured timing performance.

## Files

- `manifest.schema.json` — machine-readable v2 evidence contract.
- `manifest-template.json` — empty template; its status must remain `TEMPLATE_NOT_MEASUREMENT`.
- `manifest.json` — optional real bench submission. Do not create it until physical evidence exists.
- referenced logs, captures, schematics and CSV datasets — immutable evidence artifacts.

## Admission states

- `MEASURED_UNREVIEWED` — physical artifacts are present, hashes and minimum dataset durations pass, but no reviewer has accepted them.
- `REVIEWED_ACCEPTED` — an identified reviewer has accepted the evidence package.
- `REVIEWED_REJECTED` — evidence exists but failed review.
- `TEMPLATE_NOT_MEASUREMENT` — template only; never satisfies M11/M12.

The admission validator intentionally does **not** close GitHub issues or flip `data/v1-gates.json`. Evidence admission and milestone/governance decisions are separate actions.

## Required physical identity

The manifest records:

- device/controller model and serial/build identity;
- GNSS/PPS receiver identity;
- oscillator identity;
- RTC identity;
- firmware Git SHA and binary SHA-256;
- build instructions;
- traceable reference-time-source identity and calibration record;
- physical build date.

## Required immutable evidence

Every referenced artifact has a SHA-256 pin:

- wiring/schematic;
- boot/self-test log;
- PPS lock log;
- physical display capture;
- thermal log;
- GNSS loss/reacquisition log;
- power-cycle log.

## Required metrology datasets

CSV header is exactly:

`elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha`

Required datasets:

- PPS latency/jitter;
- holdover >= 1 h;
- holdover >= 6 h;
- holdover >= 24 h;
- holdover >= 72 h.

The validator checks file existence, SHA-256, CSV structure and minimum elapsed duration. It does not claim that passing data is scientifically good enough; that requires metrology review and a published error budget.

## Local validation

Run:

`python scripts/hardware_readiness.py`

Expected state before a real bench submission:

`M11/M12 repository readiness: PASS`
`physical_evidence: MISSING_BY_DESIGN`
`milestone_status: SOFTWARE_READY_EXTERNAL_BENCH_REQUIRED`

M11 remains open until a real physical device is built and reviewed. M12 remains open until measured datasets are analyzed and a measured error budget is accepted.
