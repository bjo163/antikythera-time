# M12 M-Clock Metrology Protocol

Repository status: **analysis software ready; measurements not yet collected**.

## Required runs

- PPS latency/jitter;
- oscillator frequency offset;
- 1 h holdover;
- 6 h holdover;
- 24 h holdover;
- 72 h holdover;
- temperature sweep or natural thermal profile;
- GNSS-loss/reacquisition;
- power-cycle recovery.

## CSV sample format

```text
elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha
```

The `mtime-metrology` crate computes mean offset, standard-deviation jitter, max absolute offset and fitted frequency drift in ppm.

Measured results must be stored as immutable artifacts with reference-device provenance before any physical accuracy class is published.
