# M6 Reference-Frame Semantics

The first 1900–2100 M6 run exposed a common Sun/Moon longitude drift roughly matching precession-scale frame rotation.

The comparison had mixed:

```text
Software Antikythera longitude
(mean ecliptic/equinox-of-date style coefficients)

with

DE440 Cartesian state
rotated only to fixed J2000 ecliptic
```

M6 now compares in an explicit IAU 2006 mean-ecliptic-of-date reference:

```text
DE440 ICRF/GCRS vector
        ↓
IAU 2006 precession-bias
        ↓
mean equator/equinox of date
        ↓
mean obliquity of date
        ↓
mean ecliptic of date
```

This changes only calibration reference-frame semantics. It does **not** add a numerical correction to `mtime-antikythera`.

The original v0.12 12-epoch values remain frozen as historical regression data. M6 long-horizon metrics use the explicit mean-ecliptic-of-date frame.
