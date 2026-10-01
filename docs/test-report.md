# Test Report — U-Time v0.7-alpha

Date: 2026-10-01

## CI gate

GitHub Actions run for commit `d94d3ba98abbd188349525373d86d1c42914bd5c`:

```text
tests 52
pass  52
fail  0
```

The 25 pre-cosmology astronomy/time tests remain present. The cosmology suite adds 27 checks.

## Cosmology coverage

- explicit Mpc and H0 SI conversion;
- deterministic adaptive Simpson quadrature;
- radiation density;
- flatness consistency without silent renormalization;
- explicit and derived curvature;
- CPL dark-energy evolution;
- CPL reduction to ΛCDM at w0=-1, wa=0;
- negative/NaN/unphysical parameter rejection;
- Einstein-de Sitter analytic limit;
- flat matter+Λ analytic solution;
- Planck published-age sanity reference;
- integration convergence;
- CosmicAgeEstimate/UTime semantic separation;
- JSON scientific-boundary serialization;
- phase ≠ absolute age;
- Antikythera/Big-Bang semantic misuse rejection;
- API query parsing;
- dataset/model-specific presets;
- percentile statistics;
- seeded independent Monte Carlo;
- seeded covariance-aware Monte Carlo;
- H0, Ωm, ΩΛ, w0 and wa sensitivity;
- posterior-chain weighted evaluation.

## Remaining validation work

A Level-3 posterior reproduction against official Planck/DESI chains is still open. Until those chains are ingested, independent-Gaussian uncertainty is labelled diagnostic and must not be confused with the actual correlated published posterior.
