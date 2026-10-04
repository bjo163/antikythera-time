# RFC-0001 — M-Time 1.0 Candidate Architecture

Status: **DRAFT / NOT READY FOR v1.0**

## Core proposition

M-Time represents an instant with both:

1. a continuous SI-duration coordinate anchored to J2000 TT;
2. a synchronized Software Antikythera cyclic astronomical state.

## Architecture

```text
Software Antikythera
        ↓
MTS-2 native temporal state
        ↓
M-Clock / SDK
        ↓
calendar / worship / observation / authority applications
```

Modern ephemerides, IERS data, UTC/TAI/UT1, GNSS and laboratory references are calibration, interoperability or realization layers. They do not silently replace the Antikythera computational core.

## Required properties

- profile/version identity;
- lossless instant identity;
- explicit uncertainty;
- reproducible calibration;
- fail-closed validated domain;
- historical-vs-modern model separation;
- policy/observation/authority separation;
- revelation/cosmology dependency isolation;
- independent implementation conformance;
- measured physical realization before standards claims.

## Current v1 blockers

- physical M-Clock build;
- PPS/oscillator/holdover metrology;
- unaffiliated independent reproduction;
- external security review;
- external scientific/reconstruction review;
- external review of textual/revelation boundary metadata.

Until those blockers are evidenced, M-Time remains 0.x research software.
