# Cosmic Age Results — U-Time v0.7-alpha

All quantities below are **model-dependent cosmological inferences**, not absolute-clock readings.

## Nominal parameter-set outputs

| Preset / model | Nominal engine age (Gyr) | Numerical integration error (Gyr) |
|---|---:|---:|
| Planck 2018 base flat ΛCDM | 13.795174514 | 3.93e-11 |
| DESI DR2 + CMB flat ΛCDM | 13.788217233 | 3.82e-11 |
| DESI DR2 + CMB curved ΛCDM | 13.703041674 | 3.79e-11 |
| DESI DR2 + CMB + DESY5 w0waCDM | 13.758158507 | 4.44e-11 |

Planck's published derived-age reference for the selected base-ΛCDM combination is approximately `13.797 ± 0.023 Gyr`.

## Official DESI DR2 full-chain age posteriors

The following values are calculated from the official Cobaya chain `age` column across the full chain. The engine independently recomputed a deterministic 6000-sample subset and compared it sample-by-sample against that official derived quantity.

| Dataset / model | Official full-chain age mean ± SD (Gyr) | 68% interval (Gyr) | Engine subset mean ± SD (Gyr) | Engine - Cobaya matched mean (Gyr) | 1 Myr gate |
|---|---:|---:|---:|---:|---|
| DESI DR2 + CMB flat ΛCDM | 13.788868 ± 0.015691 | [13.773143, 13.804565] | 13.788578 ± 0.015643 | -3.8097e-4 | PASS |
| DESI DR2 + CMB curved ΛCDM | 13.703507 ± 0.045204 | [13.659442, 13.746958] | 13.702282 ± 0.044965 | -3.6860e-4 | PASS |
| DESI DR2 + CMB + DESY5 w0waCDM | 13.759532 ± 0.019103 | [13.740557, 13.778614] | 13.758517 ± 0.018839 | -3.7662e-4 | PASS |

Reproduction workflow: GitHub Actions run `36825466646`.

Artifact:
- ID `11144394424`
- SHA256 `5f8b4ee7a27fcc92cc8dc0dd70fca779f6a689d44f59450f23614120beb08aa2`.

The sub-Myr residual is reported rather than zeroed out. It demonstrates close agreement with the official derived background-age calculation under the project's declared reduced cosmological model.

## Level-1 uncertainty remains diagnostic

Independent-Gaussian propagation is still available for demonstration, but it is not substituted for the correlated official posterior where official chains exist.

This is especially important for Planck: independent sampling of quoted marginal H0/Ωm errors does not reproduce the published derived-age uncertainty because the parameters are correlated.

## Current DESI 2026 boundary

DESI DR2 Results IV (2026 Lyα full-shape/AP) is recorded in `docs/cosmology-references.md`. It is not mixed into the 2025 baseline-CMB/SN chains above. A new age posterior requires a compatible joint posterior/model combination.

## Semantic boundary

```text
Antikythera cycle phase != cosmic age
UTime timestamp          != cosmic age
CosmicAgeEstimate        = observations + model + Friedmann integral + uncertainty
```
