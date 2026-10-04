# M10 Uncertainty and Accuracy Budget

M-Time distinguishes precision of representation from uncertainty of knowledge/model.

## Categories

- numerical;
- reference;
- model;
- observer;
- environment;
- policy margin;
- data age.

Units are explicit and are never combined across seconds/degrees.

The initial digital angular model envelope is derived from M6 monthly 1900–2100 P95 calibration:

```text
Sun       0.006319944°
Moon      0.305809037°
Moon-Sun  0.304293570°
```

These are **model-characterization envelopes**, not universal Gaussian standard deviations.

M-Clock now exposes:

- reference time uncertainty;
- uncertainty status;
- M6 Sun/Moon/phase P95 model envelopes.

A future result with missing environment/observer/policy uncertainty must report incompleteness rather than silently presenting high precision.
