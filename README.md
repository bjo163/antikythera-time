# Antikythera Time — U-Time v0.7-alpha

U-Time is an experimental astronomical time-representation and validation project. v0.7 fast-tracks a **separate model-dependent Cosmic Chronology Lab** without redefining UTime as time since the Big Bang.

> Universal protocol ≠ absolute cosmic clock.

## Two explicitly separated layers

### Solar-System / Antikythera-inspired layer

```text
SI duration + J2000 + time scale/frame
-> astronomical cycles
-> prediction
-> external JPL reference
-> error / holdout validation
```

This layer contains Metonic/Saros, lunar calibration and physically constrained lunar residuals.

### Cosmology chronology layer

```text
observational dataset
-> cosmological parameters
-> FLRW/Friedmann model E(a)
-> numerical age integral
-> CosmicAgeEstimate
-> numerical error + parameter uncertainty + model dependence
```

CosmicAgeEstimate is **not** UTime and has status `MODEL_DEPENDENT_INFERENCE`.

## Supported cosmology

- flat ΛCDM with radiation;
- non-flat ΛCDM with explicit or explicitly-derived Ωk;
- CPL `w0waCDM`, `w(a)=w0+wa(1-a)`;
- deterministic adaptive Simpson integration with reported tolerances/error;
- independent-parameter Monte Carlo;
- covariance-aware Monte Carlo;
- posterior-chain age evaluation.

## Observational presets

- Planck 2018 final base-ΛCDM reference;
- DESI DR2 BAO + CMB flat ΛCDM;
- DESI DR2 BAO + CMB curved ΛCDM;
- DESI DR2 BAO + CMB + DESY5 w0waCDM.

Presets are never mixed across incompatible model/data combinations.

## Cosmology APIs

```text
GET  /api/cosmology-age?preset=planck2018
GET  /api/cosmology-reference?dataset=planck2018
POST /api/cosmology-chain
```

Explicit-parameter example:

```text
/api/cosmology-age?model=flat-lcdm&H0=67.4&OmegaM=0.315&OmegaLambda=0.6849
```

The response carries `scientificBoundary.antikytheraDirectlyDeterminesCosmicAge=false` and `scriptureUsedAsNumericalPrior=false`.

## Validation gates

Cosmology tests include:

- H0 unit conversion;
- Einstein-de Sitter `2/(3H0)`;
- flat matter+Λ analytic solution;
- Planck age sanity check;
- tolerance convergence;
- curvature;
- CPL→ΛCDM reduction;
- invalid densities/NaN/negative H0;
- parameter sensitivity;
- seeded independent/covariance uncertainty;
- posterior-chain adapter;
- UTime/CosmicAge semantic separation;
- phase ≠ absolute age invariant;
- rejection of Antikythera→Big-Bang semantic misuse.

Existing Solar-System tests remain intact. Final CI gate: **55 tests / 55 pass / 0 fail**.

## Documentation

- `docs/repository-audit.md`
- `docs/cosmology-feasibility.md`
- `docs/adr-001-cosmology-separation.md`
- `docs/cosmology-references.md`
- `docs/quranic-celestial-computation-map.md`
- `docs/ontology.md`
- `docs/claim-matrix.md`
- `docs/reproducibility.md`
- `docs/test-report.md`
- `docs/research-integrity.md`
- `docs/long-term-roadmap.md`
- `docs/official-desi-posterior.md`
- `docs/cosmology-age-results.md`
- `docs/deliverables-status.md`
- `docs/deployment.md`
- `docs/final-research-report.md`


## Official DESI posterior reproduction

The `official-posterior` GitHub Actions workflow downloads DESI DR2 public Cobaya chains, locks the documented baseline CMB likelihood combination, summarizes the full official derived-age posterior, independently recomputes a deterministic 6000-sample subset through the U-Time Friedmann integrator, and enforces a 0.001-Gyr (1 Myr) mean-agreement gate.

Final supported full-chain posteriors:

- DESI DR2 + CMB flat ΛCDM: **13.788868 ± 0.015691 Gyr**;
- DESI DR2 + CMB curved ΛCDM: **13.703507 ± 0.045204 Gyr**;
- DESI DR2 + CMB + DESY5 w0waCDM: **13.759532 ± 0.019103 Gyr**.

All three engine-vs-Cobaya agreement gates pass. See `docs/official-desi-posterior.md` for intervals, sample counts, run ID and artifact hash.

## Scientific boundaries

Antikythera contributes a computational idea:

```text
physical phenomenon -> period/relation -> computational model -> prediction
```

Modern cosmology contributes:

```text
cosmic observation -> physical parameters -> expansion function -> integration -> inferred age distribution
```

No Antikythera cycle, Qur'anic number, TT/UTC/TDB coordinate, or desired 13.8-Gyr target is used to tune cosmological parameters.
