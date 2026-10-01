# Official DESI DR2 Posterior Reproduction

Date: 2026-10-01

Workflow run: `36825466646`

Artifact:
- name: `desi-official-age-posterior`
- ID: `11144394424`
- SHA256: `5f8b4ee7a27fcc92cc8dc0dd70fca779f6a689d44f59450f23614120beb08aa2`

## Dataset lock

The runner is locked to DESI DR2's baseline CMB combination:

- Planck 2018 low-l TT;
- Planck 2018 low-l EE;
- Planck NPIPE high-l CamSpec TTTEEE;
- Planck/ACT DR6 lensing.

It never substitutes another high-l likelihood merely because the directory also matches "CMB".

## Flat ΛCDM — DESI DR2 BAO + CMB

- official chain rows: 59,891
- independent engine validation subset: 6,000
- engine subset age: `13.788578 ± 0.015643 Gyr`
- engine subset 68%: `[13.772381, 13.804347] Gyr`
- official full-chain Cobaya age: `13.788868 ± 0.015691 Gyr`
- official full-chain 68%: `[13.773143, 13.804565] Gyr`
- matched engine - Cobaya mean: `-3.8097e-4 Gyr`
- 1 Myr agreement gate: **PASS**

## Curved ΛCDM — DESI DR2 BAO + CMB

- official chain rows: 37,441
- engine validation subset: 6,000
- engine subset age: `13.702282 ± 0.044965 Gyr`
- engine subset 68%: `[13.658313, 13.745750] Gyr`
- official full-chain Cobaya age: `13.703507 ± 0.045204 Gyr`
- official full-chain 68%: `[13.659442, 13.746958] Gyr`
- matched engine - Cobaya mean: `-3.6860e-4 Gyr`
- 1 Myr agreement gate: **PASS**

## CPL w0waCDM — DESI DR2 BAO + CMB + DESY5

- official chain rows: 61,168
- engine validation subset: 6,000
- engine subset age: `13.758517 ± 0.018839 Gyr`
- engine subset 68%: `[13.739825, 13.776983] Gyr`
- official full-chain Cobaya age: `13.759532 ± 0.019103 Gyr`
- official full-chain 68%: `[13.740557, 13.778614] Gyr`
- matched engine - Cobaya mean: `-3.7662e-4 Gyr`
- 1 Myr agreement gate: **PASS**

## Interpretation

The engine does not force exact equality with the official Cobaya derived-age calculation. It exposes the residual and requires the weighted mean discrepancy to remain below the declared 0.001-Gyr validation threshold.

The posterior values are conditional on the stated cosmological model and observational combination. They are not an absolute cosmic clock.
