# Antikythera → DE440 Calibration — Phase 3

The Antikythera core remains independent of JPL. Calibration is performed from the reference side.

## Matrix

Twelve evenly spaced TT epochs across 2026 are evaluated.

For each epoch:

1. `ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1` produces its cyclic Sun/Moon state.
2. `MTIME_DIGITAL_ANTIKYTHERA_V1` produces the generalized corrected state.
3. DE440 produces geocentric Sun/Moon vectors.
4. Reference vectors are transformed to J2000 ecliptic longitude.
5. Relative solar/lunar motion and Moon-Sun phase are compared.

Relative-motion comparison avoids treating a frame-zero offset as a dynamical error.

## Calibration philosophy

Historical reconstruction has deliberately broad baselines because it represents a reconstructed ancient computational model.

The digital profile has tighter baselines and is allowed explicit modern correction terms.

The calibration workflow is fail-closed and stores its DE440 source hash and full residual report as a CI artifact.

## Current prototype gates

```text
historical Sun relative motion <= 4.5°
historical Moon relative motion <= 8.0°
historical Moon-Sun phase       <= 8.0°

digital Sun relative motion     <= 0.25°
digital Moon relative motion    <= 1.5°
digital Moon-Sun phase          <= 1.5°
```

These are Phase-3 model-characterization gates, not the final M-Time accuracy specification. Future releases should drive the digital residuals down rather than loosen them.
