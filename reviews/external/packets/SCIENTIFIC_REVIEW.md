# M20-D — External Scientific Review Packet

Category: `scientific`  
Issue: #41  
Candidate: `m20-review-candidate-2`  
Candidate Git SHA: `799e70d87bd3b4a918986a3f8877b9ca44a13696`

## Required topics

- `calibration_methodology`
- `reference_frames`
- `residual_metrics`
- `validated_interval`
- `uncertainty_budget`
- `falsification_design`

## Central question

Does M-Time clearly separate its Software Antikythera computational model from the modern reference systems used to calibrate, characterize, falsify, interoperate with, or physically realize it?

A reviewer is explicitly invited to reject any claim that overstates what the current data can support.

## Review targets

### calibration_methodology

Inspect how the Digital Antikythera equations and correction terms were compared with modern ephemeris/reference data, including training vs validation separation.

### reference_frames

Inspect time scales, coordinate/reference frames, topocentric transforms, and whether hidden UTC/JPL/IERS assumptions leak into the Antikythera core.

### residual_metrics

Inspect the chosen angular/time error metrics, percentile summaries, tolerances, and whether they are meaningful for the stated claims.

### validated_interval

Inspect the evidence for the current tested/validated interval and ensure conclusions are not silently extrapolated outside it.

### uncertainty_budget

Inspect the distinction between numeric precision, reference uncertainty, model uncertainty, observer/environment uncertainty, and policy margins.

### falsification_design

Inspect negative controls, out-of-sample intervals, historical/future partitions, and whether the program can actually produce FAIL/INCOMPLETE outcomes rather than always confirming itself.

## Baseline documents

- `docs/M6_ACCURACY_PROGRAM.md`
- `docs/ANTIKYTHERA_CALIBRATION.md`
- `docs/M10_UNCERTAINTY.md`
- `docs/M16_LONGSPAN_FALSIFICATION.md`
- `docs/M16_VALIDATED_INTERVAL.md`
- `spec/ANTIKYTHERA-DIGITAL-V1.md`
- `spec/MTIME-2.0.md`

## Boundary

This review is not asked to validate physical M-Clock performance; M11/M12 require real bench evidence separately.

A scientific PASS must not be interpreted as proof that M-Time is a universal or superior world time standard. It means the reviewed candidate's stated scientific/model claims survived the declared review scope.
