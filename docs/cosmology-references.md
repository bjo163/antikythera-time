# Cosmology Reference Sets

## Planck 2018 final CMB ΛCDM

Primary publication:

- Planck Collaboration, *Planck 2018 results. VI. Cosmological parameters*, A&A 641, A6 (2020), DOI 10.1051/0004-6361/201833910.
- Baseline parameter tables report approximately H0 = 67.36 ± 0.54 km s^-1 Mpc^-1, Ωm = 0.3153 ± 0.0073 and age = 13.797 ± 0.023 Gyr for the stated base-ΛCDM combination.

The local preset uses H0 and Ωm and derives today's small radiation term explicitly; flat dark energy density is then derived from closure. The published Planck age is a validation reference, not a hardcoded output target.

## DESI DR2

Key DR2 BAO cosmology publication:

- DESI Collaboration, *DESI DR2 Results II: Measurements of Baryon Acoustic Oscillations and Cosmological Constraints*, Physical Review D 112, 083515 (2025), arXiv:2503.14738.

Table V provides distinct model/data combinations. Presets remain separate rather than mixing parameters:

- DESI DR2 + CMB, flat ΛCDM: Ωm = 0.3027 ± 0.0036; H0 = 68.17 ± 0.28 km s^-1 Mpc^-1.
- DESI DR2 + CMB, ΛCDM+ΩK: Ωm = 0.3034 ± 0.0037; H0 = 68.50 ± 0.33; ΩK = 0.0023 ± 0.0011.
- DESI DR2 + CMB + DESY5, w0waCDM: Ωm = 0.3191 ± 0.0056; H0 = 66.74 ± 0.56; w0 = -0.752 ± 0.057; wa = -0.86 (+0.23/-0.20).

DESI's official DR2 publications page also lists later 2026 DR2 Lyα/AP cosmology work. Those products are not automatically substituted for the above Table-V combinations; each inference must identify its exact publication, model and dataset combination.

## Uncertainty warning

Published parameters are correlated. Level-1 independent Gaussian Monte Carlo is implemented only as a diagnostic. Faithful reproduction of published age posteriors requires covariance or the released cosmology chains. DESI released DR2 cosmology chains/data products in October 2025; the engine includes a posterior-chain adapter for later ingestion.
