# Digital Antikythera V1 Public Computational Specification

This document is the public algorithm surface for independent implementations.

## Epoch

`d = JD_TT - 2451545.0`

Angles are degrees and are normalized to [0,360).

## Solar longitude

```text
T = d / 36525
L0 = 280.46646 + 0.98564736 d
M  = 357.52911 + 0.98560028 d

C = (1.914602 - 0.004817 T - 0.000014 T²) sin(M)
  + (0.019993 - 0.000101 T) sin(2M)
  + 0.000289 sin(3M)

lambda_sun = L0 + C
```

## Lunar longitude

```text
L  = 218.3164477 + 13.17639648 d
M  = 134.9633964 + 13.06499295 d
D  = 297.8501921 + 12.19074912 d
F  = 93.272095  + 13.22935024 d
Ms = 357.52911  + 0.98560028 d
```

Digital V1 correction:

```text
+6.289 sin(M)
+1.274 sin(2D-M)
+0.658 sin(2D)
+0.214 sin(2M)
-0.186 sin(Ms)
-0.059 sin(2D-2M)
-0.057 sin(2D-Ms-M)
+0.053 sin(2D+M)
+0.046 sin(2D-Ms)
+0.041 sin(Ms-M)
-0.035 sin(D)
-0.031 sin(Ms+M)
-0.015 sin(2F-2D)
+0.011 sin(2D-4M)
```

`lambda_moon = L + correction`

## Cyclic state

Periods in days:

```text
tropical      365.2421897
synodic       29.530588853
sidereal      27.321661547
anomalistic   27.55454988
draconic      27.212220817
Metonic       6939.688
Saros         6585.3223
Exeligmos     19755.9669
node regression 6798.383
```

Synodic/sidereal/anomalistic/draconic phases use their listed epoch angles in the conformance implementation. Metonic/Saros/Exeligmos use elapsed-day modulo period.

This specification intentionally contains no JPL ephemeris lookup table.
