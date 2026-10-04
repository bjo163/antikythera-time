# M11 Hardware Qualification and Bench Campaign

Repository status: **qualification framework ready; no real hardware selected in-repository**.

This document defines what must be known before a physical M-Clock build can be treated as an evidence-producing bench.

It is deliberately vendor-neutral. A commercial part is acceptable only if its actual interface, timing behavior, provenance and environmental limits are documented for the specific evidence build.

## 1. Qualification boundary

Hardware selection is not the same thing as physical validation.

The repository recognizes three states:

- `TEMPLATE_NOT_SELECTION` — empty planning template only;
- `SELECTED_UNVERIFIED` — real components have been selected/identified, but the complete bench path has not yet passed preflight;
- `BENCH_READY` — the selection record is complete enough to begin the physical campaign.

Even `BENCH_READY` does **not** satisfy M11. M11 requires the real assembled device and admitted physical evidence.

## 2. Minimum physical chain

```text
traceable reference or GNSS/PPS
        ↓
reference receiver / instrument
        ↓
electrical PPS path
        ↓
timestamp capture on controller
        ↓
local oscillator / RTC holdover
        ↓
Software Antikythera + native M-Time
        ↓
MCLOCK state + lock/holdover/stale status
        ↓
display and immutable measurement logs
```

The qualification record must describe every boundary in that path.

## 3. Controller qualification

Record at minimum:

- manufacturer/vendor;
- exact model;
- serial/build identifier;
- CPU/architecture;
- operating environment;
- Rust/toolchain support;
- timestamp input interface used for PPS;
- timestamp resolution or counter characteristics;
- display/output interface;
- nonvolatile storage path;
- power input and recovery behavior.

The controller must expose enough information to distinguish software timestamping from a hardware capture path. If the intended PPS timestamp path is software-only, that limitation must be explicit.

## 4. GNSS/PPS or reference-input qualification

Record:

- exact receiver/instrument identity;
- reference type: GNSS-PPS, laboratory frequency/time standard, or other traceable source;
- electrical PPS level/interface;
- stated pulse characteristics;
- acquisition/lock indicators;
- antenna/reference-input path;
- known configurable delays, cable delays or receiver timing offsets;
- provenance/calibration plan.

A label such as "GPS module" is not sufficient.

## 5. Oscillator and RTC qualification

Record:

- exact oscillator and RTC identities;
- oscillator class, e.g. crystal/TCXO/OCXO or equivalent;
- nominal frequency;
- manufacturer stability/tolerance information when available;
- control/discipline interface, if any;
- temperature operating range;
- power-loss behavior;
- RTC backup behavior;
- how holdover will be measured when reference lock is removed.

Manufacturer specifications are planning inputs, not measured M-Time performance.

## 6. Reference instrument qualification

The bench must identify what is used to decide that the M-Clock is early/late.

Record:

- reference instrument/source ID;
- description;
- traceability plan;
- calibration/provenance record plan;
- capture method;
- expected measurement resolution;
- how common delays are controlled or characterized.

The final physical evidence manifest must replace plans/placeholders with actual provenance records.

## 7. Power and thermal qualification

Before the campaign, document:

- nominal input voltage/current;
- power supply identity;
- expected brownout/power-cycle behavior;
- intended thermal range;
- temperature sensor source;
- whether natural ambient variation or controlled chamber testing is used;
- whether oscillator warm-up is required and how long the campaign waits before lock measurements.

## 8. Required campaign

The machine-readable campaign requires these runs:

- `boot_self_test`;
- `pps_lock`;
- `pps_latency_jitter`;
- `holdover_1h`;
- `holdover_6h`;
- `holdover_24h`;
- `holdover_72h`;
- `thermal_profile`;
- `gnss_loss_reacquisition`;
- `power_cycle_recovery`;
- `offline_mclock_output`.

Each run has an explicit planned duration or event count, output artifact type, and acceptance evidence description.

## 9. Machine-readable files

Template:

`hardware/m-clock/qualification-template.json`

Schema:

`hardware/m-clock/qualification.schema.json`

Optional real selection record:

`hardware/m-clock/qualification.json`

Validation:

```bash
python scripts/mclock_qualification_readiness.py
```

Expected repository-only state:

```text
M11 hardware qualification framework: PASS
hardware_selection: MISSING_BY_DESIGN
qualification_status: EXTERNAL_SELECTION_REQUIRED
```

Do not create `qualification.json` with invented serial numbers or placeholder hardware merely to turn the gate green.

## 10. Handoff to physical evidence

A complete qualification record is an input to the bench, not its result.

After assembly and measurements:

1. freeze firmware and analyzer Git SHAs;
2. collect the real evidence and CSVs;
3. generate the metrology report;
4. populate `physical-evidence/manifest.json`;
5. run both readiness validators;
6. obtain review.

Only that evidence path can satisfy M11/M12.
