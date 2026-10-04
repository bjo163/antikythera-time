# M7 Historical Reconstruction Fidelity

M7 makes historical claims auditable without freezing one scholarly reconstruction as unquestionable fact.

## Profiles

```text
ANTIKYTHERA_CONSERVATIVE_REAR_DIALS_V1
ANTIKYTHERA_FREETH_2021_FRONT_COSMOS_V1
```

The conservative profile contains the evidence-backed Metonic, Saros and lunar-anomaly components.

The 2021 front-cosmos profile additionally includes inscription-constrained planetary display/reconstruction components, explicitly labelled `ReconstructedModel`.

## Evidence sources

- Freeth et al., Nature 444 (2006), DOI 10.1038/nature05357.
- Freeth et al., Nature 454 (2008), DOI 10.1038/nature07130.
- Freeth et al., Scientific Reports 11 (2021), DOI 10.1038/s41598-021-84310-w.

The machine-readable registry is `data/antikythera/historical-evidence-v1.json`.

## M8 foundation included in this change

The 14 existing compact digital lunar-series terms are now represented as named `CorrectionTerm` records with:

- stable ID;
- coefficient;
- argument family;
- evidence class;
- default enabled state;
- provenance label.

This is a structural refactor only. It preserves the v0.13 numerical formula exactly.

Every term can now be disabled independently for ablation experiments.

No new correction coefficient is activated by M7/M8 foundation.
