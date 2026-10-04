# Physical M-Clock Evidence Directory

This directory intentionally contains **templates only** until a real device is built.

Do not replace `status: TEMPLATE_NOT_MEASUREMENT` with a completion claim unless the named files actually exist and came from a physical bench run.

Required evidence:

- device/controller identity;
- GNSS/PPS receiver identity;
- oscillator and RTC identity;
- firmware Git SHA + binary SHA-256;
- wiring/schematic;
- boot/self-test log;
- PPS lock log;
- offline holdover log;
- physical display capture;
- metrology CSV with reference-device provenance.

M11 remains open until these exist.

M12 additionally requires measured PPS latency/jitter, frequency offset, 1h/6h/24h/72h holdover, thermal behavior, GNSS loss/reacquisition and power-cycle recovery.
