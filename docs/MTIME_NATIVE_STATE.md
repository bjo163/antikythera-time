# Native M-Time Temporal State — Phase 4

Phase 4 introduces `mtime-temporal`, the first native M-Time coordinate/state layer.

## Why this is not another Unix timestamp

M-Time stores two orthogonal views of one instant:

### Linear coordinate

```text
linear_si_nanoseconds_from_j2000_tt
```

This is a continuous TT-based SI-duration coordinate relative to J2000.

### Cyclic coordinate

The same state carries the synchronized Software Antikythera vector:

```text
solar year phase
synodic lunar phase
sidereal lunar phase
anomalistic lunar phase
draconic lunar phase
Metonic phase
Saros phase
Exeligmos phase
solar longitude dial
lunar longitude dial
lunar phase dial
lunar node dial
```

Therefore M-Time does not define an instant only as "seconds since epoch". It defines an instant as linear duration plus an astronomical-cycle state.

## Reference interoperability

`ReferenceTimes` carries TT and TDB coordinates and their modeled uncertainty.

UTC can be used as an input adapter through the existing leap-second-aware timescale layer. UTC does not define the Antikythera machine architecture; it is an interoperable civil input.

## Profile separation

The same linear instant may be evaluated by:

- `ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1`
- `MTIME_DIGITAL_ANTIKYTHERA_V1`

The linear coordinate must remain identical; only modeled cyclic outputs may differ.

This separation is an invariant tested by the crate.
