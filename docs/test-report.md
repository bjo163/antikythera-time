# Test Report — U-Time v0.7-alpha

Date: 2026-10-01

## Final unit/integration gate

GitHub Actions run:

- Run: 36825749256
- Result: SUCCESS

```text
tests 55
pass  55
fail  0
```

The original astronomy/time tests were preserved. Cosmology/Cobaya coverage was added rather than replacing earlier gates.

## Cosmology coverage

- explicit Mpc and H0 SI conversion;
- deterministic adaptive Simpson quadrature;
- radiation density;
- flatness consistency without silent renormalization;
- explicit/derived curvature;
- CPL dark-energy evolution;
- CPL reduction to ΛCDM at w0=-1, wa=0;
- negative/NaN/unphysical parameter rejection;
- Einstein-de Sitter analytic limit;
- flat matter+Λ analytic solution;
- Planck published-age sanity reference;
- numerical convergence across tolerances;
- CosmicAgeEstimate/UTime semantic separation;
- JSON scientific-boundary serialization;
- phase != absolute age invariant;
- Antikythera/Big-Bang semantic misuse rejection;
- API query parsing;
- dataset/model-specific presets;
- percentile statistics;
- seeded independent and covariance-aware Monte Carlo;
- H0, Ωm, ΩΛ, w0 and wa sensitivity;
- weighted posterior-chain evaluation;
- Cobaya chain parsing;
- engine-vs-Cobaya age reproduction.

## Official DESI posterior validation

Workflow: `official-posterior`

Baseline-CMB full-chain run:

- Run: 36825466646
- Result: SUCCESS
- Artifact: `desi-official-age-posterior`
- Artifact ID: 11144394424
- ZIP SHA256: `5f8b4ee7a27fcc92cc8dc0dd70fca779f6a689d44f59450f23614120beb08aa2`

For every supported DESI chain, the engine recomputed a deterministic 6000-sample subset and compared it against the official Cobaya derived `age` column. The declared agreement gate is 0.001 Gyr (1 Myr) on the weighted mean offset.

All three supported DESI combinations passed.

## Static site package

Workflow: `static-site`

- Run: 36825497347
- Result: SUCCESS

The static site is packaged as an Actions artifact. Public GitHub Pages publication remains an account/repository-administration action because the connected GitHub App cannot enable Pages.
