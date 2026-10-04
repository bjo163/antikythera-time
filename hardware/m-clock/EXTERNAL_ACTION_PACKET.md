# M11/M12 External Bench Action Packet

Repository state: **software-ready, physical evidence absent**.

## Bench operator must provide

1. Complete `physical-evidence` manifest with real component identities.
2. Firmware binary hash and Git SHA.
3. PPS lock capture and timestamp path description.
4. Network-disconnected holdover demonstration.
5. Metrology CSV:

```text
elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha
```

6. Separate datasets for 1h, 6h, 24h and 72h holdover.
7. GNSS loss/reacquisition and power-cycle logs.
8. Reference-device calibration/provenance.

## Completion rule

M11 can close only after a physical device exists and evidence is committed/reviewed.

M12 can close only after the physical datasets are analyzed by `mtime-metrology` and a measured error budget is published.

Synthetic fixtures are useful for software tests but do **not** count as physical evidence.
