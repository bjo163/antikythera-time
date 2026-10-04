# M-Clock Hardware Reference Architecture

Status: **reference design — not yet manufactured/certified**

```text
traceable GNSS/PPS or laboratory reference
                |
                v
        oscillator / holdover
                |
                v
        Rust-capable controller
          |              |
          v              v
linear M-Time      Software Antikythera
          \              /
           \            /
             MCLOCK-1
                |
                v
        display / network output
```

## Research-bench prototype

Recommended capability classes:

- embedded Linux/Rust-capable board;
- GNSS receiver with PPS;
- TCXO/OCXO or equivalent characterized local oscillator;
- RTC/holdover path;
- nonvolatile storage for signed reference/profile bundles;
- display capable of civil/reference time plus cyclic dials.

No commercial component is normative. Hardware must be selected and tested against PPS latency, oscillator drift, thermal range, power interruption and data-age requirements.

## Embedded demonstrator

A later MCU implementation may omit large ephemeris/reference datasets and consume signed precomputed bundles, but it must preserve packet/profile identity and stale-data indicators.
