# M11 Physical M-Clock Bench Build Plan

Repository status: **software-ready; physical build not yet evidenced**.

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

## Required physical evidence before M11 closes

- board/controller model and serial/build identity;
- GNSS/PPS receiver identity;
- oscillator/RTC identity;
- wiring/schematic;
- firmware hash;
- boot/self-test log;
- live PPS-lock log;
- network-disconnected holdover demonstration;
- display/photo or captured hardware output.

The `mtime-device` crate defines lock/holdover/stale/self-test semantics but is not itself evidence that hardware exists.
