# U-Time: A Reproducible Cross-Domain Protocol for Astronomical Time, Validation, and Model-Dependent Cosmic Chronology

## Abstract

U-Time is an experimental interchange protocol and validation framework for astronomical time-related quantities. It combines explicit SI-based time semantics, relativistic coordinate-time transformations, observer/Earth-orientation provenance, evidence-labelled historical astronomical models, external NASA/JPL validation, and model-dependent cosmological age inference. U-Time does not propose an absolute universal clock. Instead, it requires values to carry scale/frame/model context, uncertainty, evidence state, quality class and provenance. Version 2.0 is packaged as a standardization candidate with JSON Schema, conformance corpus, golden vectors, JavaScript/Python/Rust implementations and reproducible external-reference workflows.

## 1. Motivation

Astronomical software often exchanges numbers whose interpretation depends on time scale, reference frame, observer, ephemeris/model and external data version. U-Time treats those semantics as first-class protocol data.

The Antikythera Mechanism motivates the computational pattern of mapping astronomical cycles/relations into a reproducible state/prediction, not a claim that ancient gearing contained modern metrology, relativity or cosmology.

## 2. Protocol architecture

```text
value
+ quantity
+ scale/frame/observer or model
+ evidence state
+ quality class
+ uncertainty
+ provenance/reference data
+ algorithm/validity
+ transformation history
```

## 3. Time and relativity

The implementation distinguishes civil/atomic/terrestrial and geocentric/barycentric coordinate-time semantics. TT↔TCG and TDB↔TCB are checked against IAU SOFA reference vectors. A compact geocentric TDB−TT approximation is benchmarked against ERFA and remains explicitly approximate.

IERS Earth-orientation products provide UT1−UTC and polar-motion provenance for observer-dependent applications.

## 4. Astronomy validation

Lunar and cycle models expose error against JPL Horizons. Saros recurrence is validated against NASA eclipse references but is not called full eclipse prediction. Published NASA Besselian elements can be evaluated without claiming independent element generation. A JPL-published lower-accuracy planetary model is benchmarked against Horizons and remains validity-bounded.

## 5. Historical reconstruction

Antikythera digital-twin metadata separates surviving evidence from strongly indicated reconstructions, reconstructed models and hypothetical features. This prevents front-gear reconstruction hypotheses from being presented as surviving historical fact. Published work supports lunar/solar calendrical functions and inscriptions describing the five classical planets; substantial front gearing is necessarily reconstructed.

## 6. Cosmic chronology

Cosmic age is a model-dependent inference, not a coordinate-time instant. ΛCDM/curved/CPL backgrounds, uncertainty propagation and official DESI chain reproduction are implemented. Qur'anic celestial/reckoning material is kept in a textual/conceptual layer and never supplies numerical cosmological priors.

## 7. Reproducibility

The project ships:

- normative v2 specification/schema;
- conformance corpus;
- golden vectors;
- JavaScript reference implementation;
- independent Python and Rust checkers;
- SOFA/ERFA/NASA/JPL/IERS/DESI workflows;
- SHA-256 release manifest.

## 8. Limitations

U-Time does not replace BIPM/IAU/IERS/SOFA standards, NASA/JPL integrated ephemerides/SPICE, or operational mission-navigation systems. The NASA Besselian evaluator consumes published element sets rather than independently generating them from a high-precision lunar/solar ephemeris. Historical planetary gearing remains partially reconstructed. External peer review and standards adoption remain future external processes.

## 9. Conclusion

The principal contribution is a reproducible semantic and validation contract: every scientific quantity must reveal what it is, how it was produced, its uncertainty/validity and what evidence supports it. Version 2.0 is therefore a standardization candidate for independent evaluation, not a self-declared international standard.
