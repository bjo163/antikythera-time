# M-Clock — Software Clock-State Surface

M-Clock is a **software-facing representation** of M-Time's native state. It is not a hardware product, hardware roadmap, or metrology program.

## Executable stack

- `mtime-antikythera`: virtual astronomical machine;
- `mtime-temporal`: linear SI + cyclic M-Time state;
- `mtime-clock`: versioned `MCLOCK-1` software packet and renderer;
- `mtime-wasm`: browser API;
- `mtime-cli`: deterministic command-line clock surface;
- GitHub Actions `m-time-mclock` conformance gate;
- Live Lab real-time panel.

## Clock packet

The software packet carries:

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

The i128-scale linear coordinate is serialized as a decimal string so JavaScript consumers do not silently lose nanosecond identity.

## Scope boundary

`MCLOCK-1` is a serialization/view of software state only.

M-Time does not require a physical clock, GNSS/PPS receiver, oscillator, RTC, embedded controller, bench prototype, or hardware accuracy class. Physical realization is outside the active project scope.

The accuracy claims that remain relevant are claims about software model outputs, conversions, astronomical reference comparisons, uncertainty, reproducibility and conformance.
