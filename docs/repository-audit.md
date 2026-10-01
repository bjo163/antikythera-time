# Repository Audit Before Cosmology

## Existing validated / convention-backed core

- SI-second unit foundation.
- J2000.0 TT coordinate epoch.
- explicit time-scale and reference-frame metadata.
- exact conventional TT = TAI + 32.184 s relation.
- table-backed UTC/TAI handling inside the declared leap-second support window.

These are time-coordinate/metrology facilities. None is a Big-Bang clock.

## Existing experimental astronomy

- mean synodic lunar model.
- bounded lunar period/epoch calibration.
- anomalistic/draconic residual model.
- JPL Horizons validation and holdout philosophy.

These are Solar-System astronomical models, not cosmological chronology.

## Existing historical reconstruction layer

- Antikythera-inspired Metonic/Saros cycle representation.
- documented computational analogy: physical period → numerical ratio/cycle → phase/state → prediction.

The project does not have a full reconstruction of every lost Antikythera gear train.

## Reusable abstractions

Safe to reuse:

1. explicit model identity;
2. explicit parameter provenance;
3. numerical method metadata;
4. uncertainty separated from numerical error;
5. model → external reference → validation;
6. machine-readable evidence/status labels.

## Abstractions that MUST NOT be reused semantically

- `UTime.nsSinceJ2000` as time since the Big Bang;
- TT, TAI, UTC, TDB, TCB as a cosmological origin coordinate;
- Metonic, Saros, anomalistic, draconic or other periodic phase as an absolute cosmic age estimator;
- lunar calibration as a cosmological prior.

Cosmological age therefore uses the separate semantic type `CosmicAgeEstimate` with status `MODEL_DEPENDENT_INFERENCE`.
