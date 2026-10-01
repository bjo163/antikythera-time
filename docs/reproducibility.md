# Cosmology Reproducibility Report

## Determinism

- numerical integration is deterministic adaptive Simpson quadrature;
- absolute/relative tolerances, depth, evaluations and integration error are returned;
- independent/covariance Monte Carlo uses a seeded PRNG;
- posterior chains retain their official sample weights;
- the official DESI reproduction runner downloads the named public Cobaya chains and applies the same code used by the API.

## Core reproduction

```bash
npm test
```

Final gate on 2026-10-01:

```text
tests 55
pass  55
fail  0
```

## Official DESI DR2 posterior reproduction

Workflow:

```text
.github/workflows/official-posterior.yml
```

Runner:

```text
node scripts/official-desi-posterior.mjs
```

The runner:

1. uses DESI's official public DR2 Cobaya repository;
2. locks the CMB combination to Planck low-l TT/EE + Planck NPIPE high-l CamSpec TTTEEE + Planck/ACT DR6 lensing;
3. discovers the matching chain directories;
4. reads every official chain row for the official derived-age posterior;
5. independently recomputes a deterministic 6000-sample subset through U-Time's Friedmann integrator;
6. compares the engine age with Cobaya's derived `age`;
7. fails when the absolute weighted mean age offset exceeds 0.001 Gyr.

Reference successful run: `36825466646`.

## Static site reproduction

The site does not require a build tool.

```text
index.html
site.js
styles.css
src/**
```

The `static-site` workflow packages these files as an Actions artifact. Cosmic-age inference has a browser fallback, so the Cosmology Chronology Lab remains functional on static hosting. JPL-backed endpoints still require serverless/API hosting.

## API examples

```text
/api/cosmology-age?preset=planck2018
/api/cosmology-age?preset=desi-dr2-lcdm-cmb
/api/cosmology-age?preset=desi-dr2-w0wa-cmb-desy5
/api/cosmology-reference?dataset=planck2018
POST /api/cosmology-chain
```

## Reproducibility boundary

A reproducible calculation is not the same as universal adoption. External peer review, independent implementations and standards adoption are future external gates.
