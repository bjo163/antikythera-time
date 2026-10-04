# M8 Digital Correction Architecture

M8 separates **correction architecture** from **correction acceptance**.

## Existing terms

The 14 compact lunar-series terms that already existed in the digital profile are now structured as `CorrectionTerm` records.

Every term has:

- stable ID;
- coefficient;
- argument family;
- evidence = `ModernDigitalCorrection`;
- default-enabled flag;
- provenance label.

The v0.13 default numerical output is preserved.

## Ablation

The M8 workflow disables each term independently and records the 2000–2100 validation P95/max residual change.

Artifact:

`ablation.csv`

This answers: "what happens if this term is removed?"

## Experimental candidate fitting

Candidate basis functions are intentionally small and interpretable:

- sin/cos draconic phase;
- sin/cos 2× draconic phase;
- sin/cos anomalistic phase;
- sin/cos synodic phase.

Coefficients are fitted only on 1900–1999 monthly residuals.

Validation uses 2000–2100.

A candidate is accepted for **further investigation** only if:

1. training P95 improves;
2. validation P95 improves;
3. validation maximum does not worsen by more than 5%.

Even an accepted experimental candidate is **not automatically activated** in the default M-Time profile.

Artifact:

`candidate-fit.csv`

The report explicitly asserts:

`default_profile_changed=false`

This prevents calibration-oracle data from silently becoming an ephemeris table inside `mtime-antikythera`.
