# Computed Wellington / Diyanet Fajr — v0.9.0

M-Time now computes the Wellington imsak/fajr event used by the represented Diyanet Shawwal policy condition.

## Method provenance

Diyanet's current public methodology states that imsak begins at astronomical dawn, represented by the Sun approaching **18° below the horizon**.

M-Time encodes this as:

```text
profile_id = DIYANET_IMSAK_FAJR_MINUS_18
target solar altitude = -18°
event = rising / FajrThreshold
```

Sources:

- https://kurul.diyanet.gov.tr/tr/video/imsak-nedir-ne-zaman-baslar/019d0027-3361-75de-b072-bc87fbe71859
- https://vakithesaplama.diyanet.gov.tr/imsak.php
- https://namazvakitleri.diyanet.gov.tr/tr-TR/16638/wellington-namaz-vakitleri

This profile is Diyanet-specific. It does not replace other worship-time profiles.

## High-precision event path

```text
UTC bracket
+ Wellington observer (174.7772 E, 41.2889 S)
+ current IERS finals.all
+ JPL DE440
+ TT/TDB/UT1
+ IAU 2006/2000A
→ topocentric Sun altitude
→ rising -18° root
→ SolarEvent(FajrThreshold)
```

## Shawwal 1447 policy replay

Diyanet publishes conjunction at:

```text
2026-03-19 01:24 UTC
```

M-Time computes the following Wellington dawn on 20 March local time.

Actions run **37194975490**:

```text
profile_id=DIYANET_IMSAK_FAJR_MINUS_18
target_altitude_deg=-18
wellington_fajr_utc_unix=1773939004
wellington_fajr_jd_ut1=2461119.201438614167
solver_residual_deg=-0.000067779102
conjunction_utc_unix=1773883439
conjunction_to_fajr_hours=15.434722
conjunction_before_wellington_fajr=true
```

The policy condition is therefore derived from computed astronomy rather than a caller-supplied boolean.

## CI hardening discovered during this milestone

The first Wellington workflow exposed a shell-pipeline weakness: a failing command piped to `tee` could inherit `tee`'s zero exit code.

The affected external-source workflows were hardened with:

```text
set -euo pipefail
```

so validation failures now propagate correctly.

## Boundary

This release validates the Diyanet -18° method for the relevant Shawwal 1447 Wellington policy case. It does not yet provide a multi-season, multi-year oracle against every published Wellington prayer-time entry.
