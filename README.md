# Antikythera Time — U-Time v0.5-alpha

U-Time is an experimental time-representation protocol with an SI-time core, explicit astronomical coordinates, Antikythera-inspired cycles, and a falsifiable lunar validation/calibration lab.

> Universal protocol ≠ absolute cosmic clock.

## Protocol

```text
U-Time = SI duration + epoch + time scale + reference frame + uncertainty
```

The astronomy layer follows:

```text
cycle model -> external reference -> measured error -> bounded calibration -> untouched holdout test
```

## v0.5 fast-track

- SI nanosecond / J2000 / TT core retained.
- Explicit leap-second history for UTC↔TAI inside the declared support window.
- Metonic 235-month and Saros 223-month Antikythera-inspired cycle engine.
- Mean lunar model calibrated to NASA/GSFC New Moon 2000-01-06 18:14 UT.
- JPL Horizons validation for illuminated fraction (#10) and phase angle (#24).
- Bounded optimizer for mean period and phase-zero epoch.
- Training/holdout split: pre-2013 versus 2013–2026.
- Calibration is rejected unless the untouched holdout score improves.
- 22 automated tests, including synthetic recovery and anti-fake-improvement tests.

## Run

```bash
npm test
npm run demo
```

## APIs after Vercel deployment

```text
/api/horizons?date=<ISO>
/api/validation?year=2026
/api/calibrate
```

## Calibration limits

```text
mean synodic period correction: ±180 seconds
phase-zero epoch correction:    ±12 hours
```

Objective:

```text
illumination_RMSE / 100 + phase_angle_RMSE / 180
```

The optimizer sees only the training set. A lower training error is not enough; the holdout must also improve.

See `docs/calibration-protocol.md`.

## Evidence

- BIPM SI second: https://www.bipm.org/en/si-base-units/second
- NASA/GSFC Moon phases: https://eclipse.gsfc.nasa.gov/phase/phasecat.html
- JPL Horizons API: https://ssd-api.jpl.nasa.gov/doc/horizons.html
- JPL Horizons manual: https://ssd.jpl.nasa.gov/horizons/manual.html

## Boundaries still enforced

Not implemented as validated transformations:
- full TT↔TDB
- TDB↔TCB relativistic conversion
- full IERS Earth-orientation chain
- ephemeris-grade internal lunar orbit
- full mechanical reconstruction of every Antikythera gear and dial epoch

These remain explicit future gates rather than hidden approximations.
