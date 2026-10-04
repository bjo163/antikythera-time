# M-Time Phases 1–5

## Phase 1 — Build the laboratory

**Status: implemented research infrastructure.**

Timescales, IERS EOP, DE440/SPK, Horizons oracles, calendar/worship policy layers, source integrity, authority, historical falsification, automated release and Live Lab.

## Phase 2 — Build the machine

**Status: implemented prototype.**

`mtime-antikythera` now contains virtual cycles, gear relations/trains, dials, synchronized state and separate historical vs digital profiles.

## Phase 3 — Calibrate the machine

**Status: implemented baseline calibration.**

Twelve 2026 DE440 epochs compare historical and digital Antikythera dynamics. JPL is outside the core dependency graph.

Current max residuals:

```text
historical Sun relative  1.996415486°
historical Moon relative 3.701058086°
historical phase         3.437724488°

digital Sun relative     0.014132662°
digital Moon relative    0.504873428°
digital phase            0.277593562°
```

These are baseline model-characterization numbers, not final accuracy claims.

## Phase 4 — Derive M-Time

**Status: implemented prototype.**

`mtime-temporal` combines a continuous TT/SI linear coordinate with the Software Antikythera cycle vector.

## Phase 5 — M-Clock

**Status: executable reference prototype.**

`mtime-clock`, `MCLOCK-1`, CLI/WASM/Live Lab rendering, conformance CI and hardware reference architecture are implemented.

A physical manufactured/certified device remains an external engineering milestone.

## Meaning of "complete"

Phases 1–5 are complete at **software/research-prototype level**. They are not equivalent to global standardization, independent reproduction, hardware certification or institutional adoption.
