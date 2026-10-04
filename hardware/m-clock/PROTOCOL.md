# M-Clock Protocol — MCLOCK-1

Canonical Rust representation: `mtime_clock::MClockPacket`.

## Mandatory identity

- packet version;
- research/production status;
- M-Time state version;
- Antikythera profile ID.

## Mandatory temporal fields

- linear SI nanoseconds from J2000 TT;
- TT JD;
- TDB JD;
- TDB-TT model offset;
- uncertainty.

## Mandatory cycle fields

- Sun longitude;
- Moon longitude;
- Moon-Sun phase;
- lunar node;
- solar-year phase;
- synodic phase;
- sidereal phase;
- anomalistic phase;
- draconic phase;
- Metonic phase;
- Saros phase;
- Exeligmos phase.

The linear nanosecond coordinate is JSON text, not IEEE-754 Number.

Future calendar, worship, observation and authority extensions must remain distinguishable from the physical/cyclic state.
