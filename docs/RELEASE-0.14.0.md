# M-Time v0.14.0 — Antikythera Systems Integration

Date: 2026-10-04

Status: **SOFTWARE/RESEARCH MILESTONES M7–M19 INTEGRATED; PHYSICAL/EXTERNAL GATES REMAIN**

## Architecture

```text
Software Antikythera
        ↓
native M-Time / MTS-2
        ↓
M-Clock
        ↓
calendar / worship / observation / authority
```

JPL/IERS/UTC/GNSS remain calibration, interoperability or realization layers.

## Internal milestones completed

- M7: bibliography-backed historical evidence/reconstruction registry.
- M8: versioned correction architecture, ablation, out-of-sample fit, V2 Experimental.
- M9: canonical MTS-2 binary/JSON state.
- M10: explicit uncertainty budgets and criterion-margin semantics.
- M13: native M-Time calendar/worship application path and executable demos.
- M14: MOBS-1 / MAUTH-1 observation-authority audit separation.
- M15: second-language Python conformance plus differential/malformed corpus.
- M16: fail-closed 1850–2149 validated digital interval.
- M17: revelation semantic ontology with numerical non-injection.
- M18: cosmology sandbox isolated from operational clock.
- M19: SDK/FFI/schema/interoperability/conformance/RFC/governance surfaces.

## M8/M16 quantitative result

```text
full 1850–2149, 3600 monthly epochs

Digital V1:
P95 = 0.302376434°
MAX = 0.427321722°

Digital V2 Experimental:
P95 = 0.253539543°
MAX = 0.314474632°
```

V2 improves pre-training, training, validation and future partitions, but remains explicitly experimental.

## M15 conformance

```text
135 Digital V1 differential vectors PASS
135 Digital V2 differential vectors PASS
MTS-2 binary re-encoding PASS
malformed packet corpus PASS
Python SDK PASS
C ABI build PASS
public conformance runner PASS
```

This is implementation independence inside the project, not unaffiliated external reproduction.

## Still open by design

M11: physical M-Clock bench build.

M12: measured PPS/oscillator/holdover metrology.

M20: external/pre-standard v1 gates.

The repository now includes device/metrology crates, evidence manifest templates, bench protocol and a hardware-readiness workflow, but **does not claim a manufactured or measured device**.

The v1 readiness checker correctly remains:

```text
M-Time v1 readiness: BLOCKED
```

Outstanding evidence includes physical realization, measured metrology, unaffiliated reproduction, and external scientific/security/historical/textual-boundary reviews.

M-Time therefore remains a 0.x research prototype and does not claim replacement of UTC/BIPM/IERS/JPL or religious authority.
