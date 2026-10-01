# SOFA-Derived Work Notice

The U-Time v0.8 relativistic time layer uses constants, equations, reference vectors and computational structure derived from the IAU SOFA software/documentation.

It **does not itself constitute software provided by or endorsed by the IAU SOFA Board**.

The implementation is rewritten in JavaScript, uses U-Time-specific types, provenance and uncertainty metadata, and deliberately uses function names that do not use the reserved `iau` or `sofa` prefixes.

Relevant upstream material:

- IAU SOFA current release: 2023-10-11.
- Time Scale and Calendar Tools cookbook.
- canonical TT/TCG, TCB/TDB and TT/TDB transformations.
- IAU 2000 Resolution B1.9.
- IAU 2006 Resolution B3.

The SOFA software licence permits derived work subject to attribution, clear marking of derived status, non-misrepresentation and naming requirements. See https://www.iausofa.org/terms-and-conditions.

## Deliberate difference

U-Time v0.8 does **not** copy the complete SOFA `Dtdb` periodic model into this repository.

For TT↔TDB, the caller must provide `dtr = TDB-TT` with provenance/uncertainty. This prevents the project from presenting a compact or incomplete periodic approximation as a standards-grade result.

A future provider may use:

- an intact/validated SOFA/ERFA implementation;
- an authoritative time ephemeris;
- another explicitly validated equivalent.

Until then, automatic TT↔TDB is intentionally incomplete rather than silently approximate.
