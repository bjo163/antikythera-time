# M-Time Temporal State Specification 2.0 (MTS-2)

Status: research-prototype wire specification.

## Identity

Magic: `MTS2`

Schema string: `MTS-2`

## Instant identity and ordering

The canonical instant identity is:

```text
linear_si_nanoseconds_from_j2000_tt : signed i128
```

Ordering and equality of **instants** use this linear coordinate.

Cyclic fields are model/profile state attached to the instant; they are not used to redefine instant ordering.

## Canonical fields

- profile ID;
- signed i128 SI nanoseconds from J2000 TT;
- TT Julian Date;
- TDB Julian Date;
- reference uncertainty seconds;
- four dial angles: Sun, Moon, Moon-Sun phase, lunar node;
- eight normalized cycle phases: solar-year, synodic, sidereal, anomalistic, draconic, Metonic, Saros, Exeligmos.

## JSON rule

The i128 coordinate is encoded as a decimal **string**, never an IEEE-754 JSON number.

## Binary rule

Big-endian canonical layout:

```text
"MTS2" magic
u16 profile byte length
UTF-8 profile bytes
i128 linear coordinate
f64 TT
f64 TDB
f64 reference uncertainty
4 × f64 dial
8 × f64 cycle phase
```

All f64 fields use IEEE-754 bit patterns in big-endian byte order.

## Time-scale conversion contracts

- TT is the native linear-coordinate realization used by MTS-2.
- TDB is an interoperability/reference coordinate and carries model uncertainty.
- UTC is an input/output civil adapter and depends on leap-second data.
- TAI is a continuous atomic reference adapter.
- UT1 requires Earth-rotation reference data and is not inferred from the Antikythera cycles.
- leap/discontinuity behavior belongs to the adapter scale, not to the M-Time linear SI coordinate.

## Versioning

MTS-2 readers must reject unknown magic/layouts rather than guessing.

Future additive JSON extensions must not alter the meaning of the canonical linear coordinate.
