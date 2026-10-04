# M11/M12 External Bench Action Packet

Repository state: **software-ready, physical evidence absent**.

This packet is the handoff boundary between repository work and a real bench experiment. It is intentionally strict so that synthetic fixtures, screenshots without provenance, or short test runs cannot silently become "hardware validation."

## 1. Build the physical chain

Minimum chain:

`GNSS/PPS or traceable lab reference → timestamp capture → characterized oscillator/holdover → Rust-capable controller → Software Antikythera / M-Time → MCLOCK-1 → display/log output`

Record exact component identities and serial/build IDs.

## 2. Freeze firmware identity

Record:

- firmware Git SHA;
- SHA-256 of the exact flashed binary;
- reproducible build instructions.

The Git SHA and binary hash are both required because source identity alone does not prove which binary was flashed.

## 3. Capture physical evidence

Provide immutable files for:

- wiring/schematic;
- boot/self-test;
- PPS lock;
- visible physical display output;
- thermal behavior;
- GNSS loss/reacquisition;
- power-cycle recovery.

Every artifact path is pinned by SHA-256 in `physical-evidence/manifest.json`.

## 4. Collect metrology CSVs

Header must be exactly:

`elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha`

Submit separate datasets for:

- PPS latency/jitter;
- >= 1 h holdover;
- >= 6 h holdover;
- >= 24 h holdover;
- >= 72 h holdover.

Each dataset must identify the same physical device, firmware and reference source used by the manifest. Duration is measured as last elapsed timestamp minus first elapsed timestamp.

## 5. Generate the measured suite report

Run:

```bash
cargo run --release -p mtime-metrology -- suite \
  "$(git rev-parse HEAD)" \
  <pps.csv> <1h.csv> <6h.csv> <24h.csv> <72h.csv> \
  > hardware/m-clock/physical-evidence/metrology-report.json
```

The tool records its exact analysis Git SHA and reports offset statistics, residual jitter, linear drift ppm, maximum absolute offset and thermal envelope. It deliberately does **not** assign an accuracy class.

Add `metrology-report.json` to the manifest evidence with its SHA-256.

## 6. Document reference provenance

The reference source must have:

- stable ID;
- description;
- traceability statement;
- calibration/provenance record.

"GPS time" or "lab clock" without identity/provenance is insufficient for a physical accuracy claim.

## 7. Create the real manifest

Copy `manifest-template.json` to `manifest.json`, fill every required field and set:

`status: MEASURED_UNREVIEWED`

Do not mark it accepted yourself merely because collection completed.

## 8. Run admission gate

Run:

`python scripts/hardware_readiness.py`

The gate verifies:

- evidence schema/version;
- non-empty hardware identity;
- firmware SHA formats;
- artifact existence;
- SHA-256 integrity;
- exact metrology CSV contract;
- real measured duration for 1h/6h/24h/72h;
- report schema/source/identity linkage;
- review-state consistency.

## Completion rule

M11 can close only after a physical device exists, its evidence package is admitted, and the build evidence is reviewed.

M12 can close only after the physical datasets are analyzed by `mtime-metrology`, a measured error budget is published, and the metrology review is accepted.

A passing admission script does **not** automatically make v1 ready. M20 still requires unaffiliated reproduction plus external security, scientific, historical-reconstruction and revelation/textual-boundary review.
