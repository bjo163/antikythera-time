# Antikythera Time — U-Time v0.6-alpha

U-Time is an experimental astronomical time-representation protocol with an SI-time core, explicit time scales/reference frames, Antikythera-inspired cycle computation, and falsifiable validation against NASA/JPL data.

> Universal protocol ≠ absolute cosmic clock.

## Core protocol

```text
U-Time = SI duration + epoch + time scale + reference frame + uncertainty
```

Astronomy workflow:

```text
cycle model
-> external reference
-> measured error
-> bounded calibration
-> fixed physical residual cycles
-> untouched holdout
-> accept / reject
```

## v0.6 — Residual Astronomy Engine

v0.6 adds a physically constrained residual layer inspired by the way Antikythera combined astronomical cycles.

Fixed periods only:

- anomalistic month: 27.554551 days
- draconic month: 27.212220 days
- Saros diagnostic: 223 synodic ≈ 239 anomalistic ≈ 242 draconic months

The engine does **not** search arbitrary frequencies. It fits only sine/cosine amplitudes for the declared physical periods, after the v0.5 bounded mean-period/epoch calibration.

Acceptance requires improvement on untouched 2013–2026 holdout data.

## Current test gate

25 automated tests cover:

- J2000 and SI duration
- TT/TAI relation
- UTC leap-second table
- Metonic/Saros structure
- NASA-calibrated mean lunar model
- JPL parser
- error statistics
- bounded calibration
- anti-overfit behavior
- Saros 223/239/242 coherence
- synthetic recovery of anomalistic/draconic residual harmonics
- residual holdout generalization

## APIs after Vercel deployment

```text
/api/horizons?date=<ISO>
/api/validation?year=2026
/api/calibrate
/api/residuals
```

## What Antikythera contributes

Antikythera contributes the computational architecture:

```text
astronomical cycle -> mechanical ratio -> phase/state -> prediction
```

Historically supported functions include solar/lunar calendrical information, lunar anomaly, Metonic calendar reckoning, Saros eclipse prediction, and games/calendar dials. Planetary display is supported by inscriptions, while exact lost gearing remains reconstructed/debated.

It does **not** contain evidence for atomic time, relativity, the Hubble expansion rate, the Big Bang epoch, or the age of the universe.

See:

- `docs/antikythera-function-map.md`
- `docs/long-term-roadmap.md`
- `docs/calibration-protocol.md`

## Long-term direction

The roadmap runs through:

- v0.7 eclipse engine
- v0.8 solar + planetary Cosmos
- v0.9 relativistic time-scale validation
- v1.0 U-Time specification candidate
- v1.2 astronomical age API
- v1.3 cosmology chronology lab
- v1.4 historical Antikythera reconstruction
- v2.0 independent validation / standardization

The age of the universe belongs to the **cosmology chronology layer**, not the Antikythera cycle engine.

## Sources

- BIPM SI second: https://www.bipm.org/en/si-base-units/second
- NASA/GSFC Moon phases: https://eclipse.gsfc.nasa.gov/phase/phasecat.html
- NASA eclipse periodicity / Saros: https://eclipse.gsfc.nasa.gov/LEsaros/LEperiodicity.html
- JPL Horizons: https://ssd.jpl.nasa.gov/horizons/
- Freeth et al., Nature 444 (2006)
- Freeth et al., Nature 454 (2008)
- Seiradakis & Edmunds, Nature Astronomy 2 (2018)
- Freeth et al., Scientific Reports 11 (2021)

## Boundaries still enforced

Not yet treated as validated:

- full TT↔TDB transformation
- TDB↔TCB relativistic transformation
- full IERS Earth-orientation chain
- ephemeris-grade internal lunar orbit
- full historical reconstruction of every Antikythera gear
- cosmological age inference

Those remain explicit future gates rather than hidden approximations.
