# M-Time v0.2 High-Precision Astronomy Validation

Date: 2026-10-02

## DE440/SPK vector oracle

Reference: JPL Horizons geometric vectors, ICRF/frame plane, TDB.

Fixed epoch: JD TDB 2461119.0.

~~~text
Sun max component error   = 0.000025597 km
Moon max component error  = 0.002110027 km
~~~

Workflow: m-time-spk-reference, successful run 36956737541.

## Topocentric Moon oracle

Fixed public observer: longitude 106.8272° E, latitude -6.1754°, height 8 m; epoch 2026-03-19T10:00:00Z.

Reference: JPL Horizons observer quantity #4, airless.

M-Time chain:

~~~text
DE440 SPK / ICRF
+ UTC→TT/TDB
+ IERS UT1−UTC / polar motion
+ IAU 2006/2000A celestial-to-terrestrial transform
+ WGS84 observer / parallax
→ airless local horizon
~~~

Result:

~~~text
legacy GMST-only altitude = 16.748171341635°
M-Time IAU altitude       = 17.062943893901°
Horizons altitude         = 17.062815000000°
altitude residual         = 0.000128893901°
azimuth residual          = 0.000152801998°
~~~

The active gate is 0.001° (3.6 arcsec) on altitude and azimuth.

Workflow: m-time-topocentric-reference, successful integrated run 36958335289.

## Integrated Hijri state

The same workflow now runs mtime-hilal::HilalEngine, not only low-level geometry.

~~~text
integrated_state_altitude_deg = 17.062943893901
integrated_altitude_error_deg = 0.000128893901
integrated_elongation_deg     = 5.149538573229
quality                       = HighPrecision
~~~

This demonstrates that the actual HijriAstronomicalState construction path inherits the validated topocentric geometry.

## Interpretation

This is strong evidence for the fixed oracle and its declared semantics. It is not yet a claim of sub-arcsecond accuracy for every date, location, atmosphere, ephemeris version, or apparent-place convention.

The next astronomy validation milestone is a multi-epoch, multi-latitude oracle matrix.