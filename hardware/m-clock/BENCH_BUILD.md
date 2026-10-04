# M11 Physical M-Clock Bench Build Plan

Repository status: **software-ready; physical build not yet evidenced**.

## Pre-build gate

Before assembly, complete the vendor-neutral hardware qualification process in:

`hardware/m-clock/QUALIFICATION.md`

Machine-readable planning files:

- `qualification.schema.json`
- `qualification-template.json`
- optional real `qualification.json`

Validate with:

`python scripts/mclock_qualification_readiness.py`

A `BENCH_READY` qualification record means only that a real component selection and test campaign are documented. It does **not** prove the device exists or works.

## Minimum bench chain

```text
GNSS/PPS or traceable lab reference
→ timestamp capture
→ characterized oscillator / holdover
→ Rust-capable controller
→ M-Time / Software Antikythera
→ MCLOCK-1 + visible lock state
→ display/log output
```

## Assembly preflight

Before collecting evidence:

- verify all component model/serial IDs against the qualification record;
- freeze firmware Git SHA and flashed-binary SHA-256;
- freeze the exact metrology analyzer Git SHA;
- record wiring/schematic revision;
- document PPS electrical path and timestamp capture interface;
- document cable/configurable receiver delays;
- verify reference-source provenance/calibration record;
- verify temperature sensor identity;
- confirm storage space for immutable logs;
- confirm lock/holdover/stale indicators are visible in output.

## Required physical evidence before M11 closes

- board/controller model and serial/build identity;
- GNSS/PPS receiver identity;
- oscillator/RTC identity;
- wiring/schematic;
- firmware hash;
- boot/self-test log;
- live PPS-lock log;
- network-disconnected holdover demonstration;
- display/photo or captured hardware output;
- qualification record matching the actual build;
- immutable metrology/report package required by M12.

The `mtime-device` crate defines lock/holdover/stale/self-test semantics but is not itself evidence that hardware exists.
