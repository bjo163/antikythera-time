# ADR-001: Cosmology Is a Separate Semantic Layer

**Status:** accepted

## Decision

Create `src/cosmology/` and the type `CosmicAgeEstimate`. Do not extend `UTime` with a cosmological origin.

## Rationale

`UTime` is a coordinate representation around J2000 with declared scales/frames. Cosmic age is an inferred FLRW duration from a model and observational parameters. Conflating them would turn a model-dependent inference into a false universal timestamp.

## Consequences

- No `nsSinceJ2000` in `CosmicAgeEstimate`.
- No TT/TAI/UTC/TDB/TCB label on the Big-Bang origin.
- Cosmology APIs report model, parameters, provenance, numerical integration error, parameter uncertainty and model dependence.
- Antikythera and Qur'anic material can appear in conceptual/provenance documentation but never as numerical priors for H0, Ωm, w0 or wa.
