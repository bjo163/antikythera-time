# M-Clock — Phase 5 Reference Prototype

M-Clock is the first device-facing representation of M-Time's native state.

## Executable stack

- `mtime-antikythera`: virtual astronomical machine;
- `mtime-temporal`: linear SI + cyclic M-Time state;
- `mtime-clock`: versioned `MCLOCK-1` packet and renderer;
- `mtime-wasm`: browser API;
- `mtime-cli`: deterministic command-line clock surface;
- GitHub Actions `m-time-mclock` conformance gate;
- Live Lab real-time panel.

## Clock packet

The packet carries:

```text
packet/status/profile identity
linear SI nanoseconds from J2000 TT
TT / TDB interoperability coordinates
uncertainty
Sun / Moon / lunar phase / lunar node
solar year
synodic / sidereal / anomalistic / draconic
Metonic / Saros / Exeligmos
```

The i128-scale linear coordinate is serialized as a decimal string so JavaScript and embedded consumers do not silently lose nanosecond identity.

## Hardware status

Phase 5 is complete as an executable software/reference-design prototype. This repository does **not** claim that a physical M-Clock has already been manufactured, metrologically certified, or approved for religious/civil authority use.

The next physical step is a GNSS/PPS-disciplined bench prototype followed by oscillator/holdover/error-budget measurement.
