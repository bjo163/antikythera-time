# Antikythera Time — U-Time v0.4-alpha Validation Lab

U-Time is an experimental **time representation protocol** combining a modern SI-time core with explicit astronomical coordinates and an Antikythera-inspired cycle layer.

> It does not claim one absolute clock for the universe, and it does not claim that a mean lunar cycle is an ephemeris.

## Protocol

```text
U-Time = SI duration + epoch + time scale + reference frame + uncertainty
```

v0.4 adds:

```text
astronomical model -> external reference -> measured error
```

## v0.4 additions

- Historical TAI−UTC leap-second table from 1972 through the current verified window.
- UTC → TT conversion using the applicable table offset.
- Mean lunar-cycle model calibrated to NASA/GSFC New Moon **2000-01-06 18:14 UT**.
- NASA/JPL Horizons observer-table parser.
- `/api/validation?year=YYYY` monthly Moon comparison.
- Illumination MAE/RMSE/max and phase-angle MAE/RMSE/max.
- 20 automated tests.

## Evidence hierarchy

### Metrology
- BIPM SI second: https://www.bipm.org/en/si-base-units/second
- TT = TAI + 32.184 s.
- IERS/BIPM leap-second history.
- J2000.0 = JD 2451545.0 TT.

### Lunar calibration + external validation
- NASA/GSFC Six Millennium Moon Phase Catalog: https://eclipse.gsfc.nasa.gov/phase/phasecat.html
- NASA/JPL Horizons: https://ssd.jpl.nasa.gov/horizons/

The simple lunar model uses the NASA/GSFC New Moon as phase zero and advances with the project mean synodic period. It is compared with Horizons illuminated fraction (#10) and Sun-Target-Observer phase angle (#24).

Non-zero error is expected because real lunar motion is perturbed and individual lunations vary.

### Antikythera adoption

```text
astronomical period -> ratio/cycle -> phase/index -> prediction -> verification
```

Implemented historical structures:
- Metonic: 235 lunar months.
- Saros: 223 lunar months.
- idealized gear-ratio primitive.

This is not yet a full reconstruction of every known Antikythera gear train or ancient dial zero-point.

### Qur'anic conceptual references
Qur'an 10:5, 55:5 and 21:33 may be studied as conceptual references to celestial regularity, reckoning and motion. They are not used as experimental metrology or replacements for observation.

## Run

```bash
npm test
npm run demo
```

Node.js 20+.

## Validation API

After Vercel deployment:

```text
/api/validation?year=2026
```

returns monthly JPL comparisons and summary statistics.

## Accuracy boundary

Implemented:
- SI duration core.
- J2000 coordinate origin.
- TT ↔ TAI fixed relation.
- UTC ↔ TAI table inside declared support window.
- testable mean lunar model.
- live JPL comparison path.

Still withheld:
- full TT ↔ TDB periodic transformation.
- TCB ↔ TDB relativistic transformation.
- full IERS Earth-orientation path.
- ephemeris-grade internal Moon orbit.
- full Antikythera mechanical reconstruction.

Those are future scientific gates, not hidden approximations.
