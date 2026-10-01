# Final Research Report — Antikythera Time / Cosmic Chronology

## Research question

Can the U-Time project's Antikythera-inspired computation and validation architecture be extended into a scientifically defensible model-dependent Cosmic Chronology Engine?

## Result

**Supported with limits.**

The architecture can be extended when the historical inspiration and modern physical inference are kept semantically separate.

```text
Antikythera-inspired architecture:
physical phenomenon -> numerical relation -> computational model -> prediction

Modern cosmic chronology:
observations -> cosmological parameters -> E(a) -> Friedmann integral
-> age posterior -> uncertainty -> validation
```

The historical Antikythera Mechanism does not determine the age of the Universe.

## Architecture result

`UTime` remains a J2000-centered astronomical time-coordinate object.

`CosmicAgeEstimate` is a separate semantic type with:

- quantity = age_of_universe;
- status = MODEL_DEPENDENT_INFERENCE;
- seconds / Julian years / Gyr;
- cosmological model and parameters;
- observational provenance;
- integration settings/error;
- parameter uncertainty;
- model dependence;
- explicit scientific-boundary flags.

## Numerical methods

The engine implements:

- explicit H0 conversion from km s^-1 Mpc^-1 to s^-1;
- deterministic adaptive Simpson integration;
- variable transform `a=x²` for robust lower-end integration in the supported backgrounds;
- flat ΛCDM;
- curved ΛCDM;
- CPL w0waCDM;
- independent, covariance-aware and posterior-chain uncertainty propagation.

## Falsification results

Automated validation includes:

- dimensional H0 check;
- Einstein-de Sitter analytic age;
- flat matter+Λ analytic solution;
- Planck age sanity check;
- numerical convergence;
- curvature and CPL limits;
- parameter sensitivity;
- invalid-input rejection;
- phase != age invariant;
- semantic rejection of Antikythera -> Big-Bang claims.

Final software gate: **55/55 PASS**.

## Official-chain validation

Official DESI DR2 Cobaya chains were ingested from the DESI public data server. Full-chain official age posteriors were summarized, while a deterministic 6000-sample subset was independently recomputed through this project's Friedmann integrator.

All supported models passed the declared 1-Myr weighted-mean agreement threshold.

See `docs/official-desi-posterior.md`.

## Knowledge boundary

- Antikythera: historical astronomical computation / recurrence — not cosmic-age evidence.
- Qur'anic verses: textual/conceptual research layer — not numerical cosmology priors.
- Planck/DESI: observational parameter/posterior provenance.
- Friedmann/CPL equations: physical model.
- Cosmic age: inferred/model-dependent quantity.
- Numerical integration error, parameter uncertainty and model dependence remain separate.

## Conclusion

The project does not establish an absolute universal cosmic clock.

It does establish a transparent, reproducible implementation of model-dependent cosmic-age inference that can be independently checked, falsified and extended.
