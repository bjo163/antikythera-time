# v0.10 Planetary Cosmos Layer

## Model

The implementation follows JPL Solar System Dynamics' published **Approximate Positions of the Planets** Table 1 and Keplerian recipe, valid for 1800–2050.

Implemented bodies:

- Mercury;
- Venus;
- Earth/EM-barycenter approximation;
- Mars;
- Jupiter;
- Saturn.

Outputs are heliocentric J2000-ecliptic vectors and derived longitude/latitude. A geocentric subtraction helper is also provided.

## External benchmark

Workflow: `jpl-planetary-benchmark`

Successful run: `36830365898`.

Fifteen comparisons were made against JPL Horizons geometric heliocentric ecliptic vectors at three epochs.

Measured vector errors:

```text
Mercury : 0.7–3.4 thousand km
Venus   : 8.1–12.3 thousand km
Mars    : 8.2–45.8 thousand km
Jupiter : 0.61–1.58 million km
Saturn  : 2.03–3.85 million km
```

All declared model-specific benchmark thresholds passed.

## Interpretation

This is useful for an Antikythera-style planetary display and low-accuracy educational/research visualization.

It is **not** a replacement for JPL integrated ephemerides or Horizons. JPL itself describes these formulas as lower accuracy and directs high-precision work to Horizons.
