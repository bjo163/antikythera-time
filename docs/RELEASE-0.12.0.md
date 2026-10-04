# M-Time v0.12.0 — Antikythera Genesis / Phase 1–5

Date: 2026-10-04

Status: **SOFTWARE/RESEARCH PROTOTYPE — PHASES 1–5 COMPLETE**

## Why this release matters

v0.12.0 re-centers M-Time on the project's original architecture:

```text
Antikythera computational idea
        ↓
Software Antikythera
        ↓
native M-Time state
        ↓
M-Clock
```

JPL, IERS and civil/reference timescales remain calibration/interoperability layers rather than hidden conceptual replacements for the machine.

## Phase 2 — Software Antikythera

`mtime-antikythera` now includes:

- Cycle;
- rational/signed historical relations;
- VirtualGear;
- GearMesh;
- GearTrain;
- DialState;
- AntikytheraMachine;
- AntikytheraState;
- historical reconstruction profile;
- generalized digital profile.

The state tracks solar, synodic, sidereal, anomalistic, draconic, Metonic, Saros, Exeligmos, lunar-phase and node cycles.

## Phase 3 — DE440 calibration

12 TT epochs across 2026:

```text
Historical reconstruction max:
Sun relative  1.996415486°
Moon relative 3.701058086°
phase         3.437724488°

Digital Antikythera max:
Sun relative  0.014132662°
Moon relative 0.504873428°
phase         0.277593562°
```

Calibration is performed from the reference side. `mtime-antikythera` does not depend on JPL.

## Phase 4 — native M-Time

`mtime-temporal` defines a native state with:

```text
linear_si_nanoseconds_from_j2000_tt
+
Software Antikythera cycle vector
+
TT/TDB reference interoperability
+
uncertainty
```

## Phase 5 — M-Clock

`mtime-clock` defines versioned packet `MCLOCK-1`.

Surfaces:

- Rust library;
- deterministic CLI renderer;
- WASM API;
- GitHub Pages Live Lab;
- M-Clock conformance workflow;
- hardware reference architecture;
- protocol and safety rules.

## Gates

- Rust workspace: **115 passed / 0 failed**;
- clippy: PASS;
- layering invariant: PASS;
- M-Clock packet/conformance: PASS;
- wasm32 release build: PASS;
- Antikythera DE440 calibration: PASS;
- previous scientific/historical/integrity gates remain active.

## Boundary

"Phase 1–5 complete" means **software/research prototype complete**.

Still open:

- manufactured physical M-Clock;
- oscillator/PPS/holdover metrology;
- larger multi-year/century Antikythera calibration;
- independent second implementation;
- field testing;
- security review;
- external expert review;
- institutional or international standardization.
