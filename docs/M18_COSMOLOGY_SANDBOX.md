# M18 Cosmological Chronology Sandbox

Cosmic-age research remains explicitly model-dependent and research-only.

M18 adds versioned cosmology profiles:

- `PLANCK_2018_FLAT_LCDM_REFERENCE`;
- `H0_73_SENSITIVITY_ONLY` (sensitivity scenario, not a preferred-fit claim).

The same age-inference engine demonstrates model dependence.

`CosmicEpochMapping` always has:

```text
research_only = true
operational_mtime_coordinate = None
```

Repository layering tests prohibit `mtime-timescales`, `mtime-antikythera`, `mtime-temporal` and `mtime-clock` from depending on `mtime-cosmology`.

Therefore a change in inferred age of the universe cannot change today's M-Time/M-Clock state.
