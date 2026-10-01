# Antikythera Time — U-Time v0.3-alpha

Research prototype for a universal **time representation protocol** with an SI-second core, explicit astronomical coordinates, Antikythera-inspired cycles, and an evidence dashboard.

> U-Time does **not** claim one absolute clock for the whole universe.

## Protocol

```text
U-Time = SI duration + epoch + time scale + reference frame + uncertainty
```

Current implementation:
- SI nanoseconds stored as JavaScript `BigInt`
- J2000.0 = JD 2451545.0 TT
- explicit TT / TAI / UTC / TDB / TCB labels
- explicit reference frame
- uncertainty field
- exact TT ↔ TAI relation
- current UTC → TT path bounded by an IERS leap-second validation window
- Antikythera-inspired Metonic (235 lunar months) and Saros (223 lunar months) cycle engine
- live NASA/JPL Horizons reference endpoint for the Moon

## Evidence levels

### Validated metrology / astronomical standards
- BIPM SI second: https://www.bipm.org/en/si-base-units/second
- TT / TAI relationship: https://www.bipm.org/en/-/cctf-recommendation-2017-3
- IERS Bulletin C leap-second information: https://datacenter.iers.org/
- J2000.0: IAU definition, JD 2451545.0 TT

### External ephemeris reference
- NASA/JPL Horizons: https://ssd.jpl.nasa.gov/horizons/

The dashboard can request current Moon illuminated fraction and phase angle from Horizons. Horizons is an **external reference**, not the definition of U-Time.

### Historical engineering
Antikythera is adopted at the level of its computational principle:

```text
astronomical period -> ratio/cycle -> phase/index -> readable prediction
```

Implemented structures:
- Metonic cycle: 235 lunar months
- Saros cycle: 223 lunar months
- idealized gear-ratio primitive

The project does not claim that the ancient device defined SI time, and the software zero-point is J2000 rather than a reconstructed ancient dial epoch.

### Qur'anic conceptual references
Qur'an 10:5, 55:5 and 21:33 may be studied as textual/conceptual references to celestial regularity, reckoning and motion. They are not used as experimental metrology or as substitutes for astronomical observation.

## Accuracy boundary

Implemented now:
- TT ↔ TAI: exact conventional offset used by the protocol layer
- current UTC offset: 37 seconds, explicitly bounded to the current verified IERS window

Intentionally **not** implemented yet:
- TDB ↔ TT high-accuracy periodic transformation
- TCB ↔ TDB relativistic transformation
- full IERS Earth-orientation path
- ephemeris-grade lunar phase prediction inside U-Time itself

Those remain blocked rather than silently approximated.

## Run

```bash
npm test
npm run demo
```

Node.js 20+. No runtime dependencies.

## Website

The repository contains a static evidence dashboard in `index.html` plus a Vercel serverless endpoint at `/api/horizons`.

The dashboard separates:
- **VALIDATED**
- **WINDOWED**
- **MODEL**
- **PENDING**

so the project does not confuse computational precision with scientific validation.

## v0.3-alpha test gate

Current test suite: **12 tests**, covering J2000, Julian day/year, BigInt duration, scale mismatch protection, Metonic/Saros structure, gear ratios, exact TT/TAI roundtrip, current UTC conversion, and expiry of the leap-second validation window.

## Next scientific gates

1. Add authoritative reference vectors for TT/TDB/TCB.
2. Compare cycle-model lunar predictions against JPL Horizons across many epochs.
3. Quantify error distributions instead of presenting a single example.
4. Add proper-time demonstrations with declared worldlines/reference frames.
5. Add uncertainty propagation to every derived astronomical result.
