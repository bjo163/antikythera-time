# U-Time v0.8 — Relativistic Time Core

## Objective

Add standards-aligned coordinate-time transformations without turning U-Time into an absolute cosmic clock.

## Coordinate semantics

```text
TT  : terrestrial time, geocentric context
TCG : geocentric coordinate time / GCRS
TDB : barycentric dynamical time / BCRS-scaled coordinate
TCB : barycentric coordinate time / BCRS
```

The implementation uses a separate `CoordinateTime` two-part Julian-Date type. This is important because the legacy `UTime.nsSinceJ2000` storage is a TT-centered protocol coordinate and must not simply be relabelled as TCB/TDB.

## Implemented canonical relations

### TT ↔ TCG

IAU 2000 Resolution B1.9 using `L_G = 6.969290134e-10`.

### TDB ↔ TCB

IAU 2006 Resolution B3 using:

```text
L_B  = 1.550519768e-8
TDB0 = -6.55e-5 s
```

### TT ↔ TDB

The canonical transformation is implemented **only with an explicit caller-supplied `dtr = TDB-TT`**.

The project does not invent `dtr`. SOFA documents it as a quasi-periodic relativistic term that depends on the adopted solar-system ephemeris/model; the dominant annual component is about 1.7 ms.

## Validation

Six reference vectors from the IAU SOFA validation program are embedded as tests:

- TT → TCG
- TCG → TT
- TCB → TDB
- TDB → TCB
- TDB → TT with dtr
- TT → TDB with dtr

Tolerance: `1e-12 day` on the tested second JD component.

## Boundary

This makes U-Time more standards-compatible. It does **not** make the project operationally superior to JPL Horizons, SPICE, or NASA mission navigation systems.
