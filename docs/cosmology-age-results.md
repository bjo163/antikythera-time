# Cosmic Age Results — U-Time v0.7-alpha

These are outputs of the current engine using the named parameter presets. They are **model-dependent inferences**, not absolute-clock readings.

| Preset / model | Nominal engine age (Gyr) | Numerical integration error (Gyr) | Parameter uncertainty status |
|---|---:|---:|---|
| Planck 2018 base flat ΛCDM | 13.795174514 | 3.93e-11 | Published Planck derived age: 13.797 ± 0.023 Gyr |
| DESI DR2 + CMB flat ΛCDM | 13.788217233 | 3.82e-11 | official correlated age posterior not yet ingested |
| DESI DR2 + CMB curved ΛCDM | 13.703041674 | 3.79e-11 | official correlated age posterior not yet ingested |
| DESI DR2 + CMB + DESY5 w0waCDM | 13.758158507 | 4.44e-11 | official correlated age posterior not yet ingested |

## Level-1 diagnostic only

If the quoted 1σ parameter errors are sampled as **independent Gaussians** (5000 deterministic seeded draws), the engine obtains approximate σ(age):

- Planck: 0.143 Gyr
- DESI DR2+CMB flat ΛCDM: 0.073 Gyr
- DESI DR2+CMB curved ΛCDM: 0.082 Gyr
- DESI DR2+CMB+DESY5 w0waCDM: 0.199 Gyr

These are **not authoritative posterior uncertainties** because cosmological parameters are correlated. The Planck discrepancy illustrates the point: its published derived age uncertainty is about 0.023 Gyr, much narrower than the independent-Gaussian diagnostic.

## Current 2026 DESI DR2 update

DESI DR2 Results IV (2026 Lyα full-shape/AP) is recorded in `docs/cosmology-references.md`. It is not mixed into the 2025 Table-V presets. A dedicated cosmic-age posterior from Results IV remains OPEN pending ingestion of an internally compatible joint posterior/covariance/chain.

## Boundary

```text
Antikythera cycle phase != cosmic age
UTime timestamp          != cosmic age
CosmicAgeEstimate        = model + parameters + Friedmann integral + uncertainties
```
