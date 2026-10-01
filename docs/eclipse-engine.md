# v0.9 Eclipse / Saros Engine

The implemented engine is a **recurrence-family engine**, not a full Besselian eclipse geometry solver.

NASA GSFC gives the current Saros relationship as approximately:

```text
223 synodic months   = 6585.3223 d
239 anomalistic      ≈ same interval
242 draconic         ≈ same interval
```

The engine therefore propagates a known eclipse by a constant Saros or Exeligmos interval.

## NASA Saros 139 validation

Seed:

- 2024-04-08 total solar eclipse;
- Saros 139;
- greatest eclipse JD(TT/TDT) ≈ 2460409.26300.

References:

- 2042-04-20 JD ≈ 2466994.595481;
- 2060-04-30 JD ≈ 2473579.92400.

The constant-Saros predictor remains within 30 minutes for these first two recurrences, while the non-zero residual is preserved as model drift.

## Boundary

This validates the recurrence architecture historically relevant to Antikythera.

It does **not** compute:

- Besselian elements;
- local contact times;
- path width;
- gamma/magnitude from physical shadow geometry;
- lunar limb topography.

NASA/JPL/modern ephemeris methods remain the reference for those quantities.
