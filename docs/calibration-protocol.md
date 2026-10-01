# U-Time v0.5 Calibration Protocol

## Purpose

The calibration engine tests whether a bounded correction to the simple mean-lunation model improves prediction on data that was not used during fitting.

## Reference

- Target: Moon (301)
- Observer: Earth geocenter
- Source: NASA/JPL Horizons
- Quantities: illuminated fraction (#10) and Sun-Target-Observer phase angle (#24)
- Sampling: every 30 days, 2000-01-01 through 2027-01-01

## Split

- Training: before 2013-01-01
- Holdout: 2013-01-01 and later

The holdout samples are never passed to the optimizer.

## Parameters allowed to move

- Mean synodic period: baseline ±180 seconds
- Phase-zero epoch: NASA/GSFC New Moon 2000-01-06 18:14 UT ±12 hours

The bounds prevent the calibration layer from turning into an arbitrary curve fitter.

## Objective

Normalized combined score:

```text
illumination_RMSE / 100 + phase_angle_RMSE / 180
```

Lower is better.

## Acceptance rule

```text
if holdout_score(calibrated) < holdout_score(baseline):
    ACCEPT calibration
else:
    REJECT calibration
```

Training improvement alone is never sufficient.

## Scientific boundary

A successful calibration would show that a bounded mean-cycle correction generalizes across the chosen holdout period. It would not prove a universal absolute clock, replace JPL ephemerides, or establish that all lunar perturbations are captured by a single corrected period.
