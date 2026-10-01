# Cosmology Reproducibility Report

## Determinism

- numerical integration is deterministic adaptive Simpson quadrature;
- tolerances, max depth and max evaluations are returned in every age estimate;
- independent/covariance Monte Carlo uses an explicit seeded PRNG;
- posterior chains are evaluated sample-by-sample without tuning to a target age.

## Numerical validation

Required automated checks:

1. explicit H0 unit conversion to s^-1;
2. Einstein-de Sitter analytic limit `2/(3H0)`;
3. flat matter+Λ analytic age check when radiation is disabled;
4. Planck published-age sanity check;
5. integration convergence across several tolerances;
6. CPL reduction to ΛCDM at `(w0,wa)=(-1,0)`;
7. curvature path and invalid-input rejection;
8. deterministic uncertainty propagation;
9. semantic separation from `UTime`;
10. phase-vs-age conceptual invariant.

## Reproduction command

```bash
npm test
```

Reference API examples after deployment:

```text
/api/cosmology-age?preset=planck2018&uncertainty=none
/api/cosmology-age?preset=desi-dr2-lcdm-cmb&uncertainty=none
/api/cosmology-age?preset=desi-dr2-w0wa-cmb-desy5&uncertainty=none
/api/cosmology-reference?dataset=planck2018
```
