# U-Time 1.0 Specification Candidate

Status: **SPECIFICATION CANDIDATE**, not an adopted international standard.

Normative words MUST, SHOULD and MAY are used in the RFC-style sense.

## 1. Principle

A U-Time record MUST distinguish numerical value from:

- time scale / reference frame;
- physical or model context;
- uncertainty;
- evidence state;
- provenance.

A record MUST NOT imply one absolute universal "now".

## 2. Evidence state

Every record MUST use one of:

`OBSERVED | MEASURED | CALCULATED | MODELED | INFERRED | RECONSTRUCTED | SPECULATIVE | TEXTUAL_REFERENCE`.

These states MUST NOT be silently collapsed.

## 3. Coordinate-time instant

A relativistic astronomical coordinate-time instant SHOULD use a precision-preserving two-part Julian Date:

```json
{
  "kind":"instant",
  "quantity":"coordinate_time",
  "value":{"d1":2451545.0,"d2":0.0,"scale":"TT","frame":"GCRS"}
}
```

TT/TCG correspond to a geocentric context; TDB/TCB correspond to a barycentric context.

Relabelling a numeric counter from one scale to another without a transformation is prohibited.

## 4. Duration

Duration is an elapsed SI-time quantity and MUST be separate from calendar presentation.

## 5. Model result

Astronomical cycle, eclipse recurrence and planetary approximation results MUST carry model identity, validity interval and provenance.

A Saros recurrence result MUST NOT be represented as a full Besselian eclipse prediction.

The JPL approximate planetary element model MUST be labelled lower-accuracy and validity-bounded.

## 6. Cosmic age

Cosmic age MUST use an inference record, not a coordinate-time instant.

It MUST include:

- cosmological model;
- parameter set/posterior provenance;
- uncertainty when available;
- boundary that it is not an absolute cosmic clock.

No Antikythera cycle or textual/scriptural number may be used as a hidden numerical cosmology prior.

## 7. Relativistic transforms

v1 requires:

- TT↔TCG canonical transform;
- TDB↔TCB canonical transform;
- TT↔TDB transform accepting an explicit `dtr=TDB-TT`;
- optional automatic dtr provider whose accuracy class and provenance are explicit.

The built-in compact automatic provider is `APPROXIMATE_GEOCENTRIC`; standards-grade work SHOULD use a validated SOFA/ERFA/time-ephemeris provider.

## 8. Validation contract

Implementations SHOULD ship versioned golden vectors.

A conforming implementation MUST NOT weaken a failed external-reference test merely to obtain a passing build.

## 9. Current model boundaries

- Antikythera layer: historical/computational inspiration and recurrence models.
- NASA/JPL: external ephemeris/eclipse reference.
- IAU SOFA/IERS: fundamental-astronomy/time-reference authority.
- Planck/DESI: cosmological observation/posterior provenance.
- Qur'anic material: textual/conceptual research layer only.

## 10. Conformance

A U-Time v1 implementation is protocol-compatible if it:

1. emits records compatible with `utime-v1.schema.json`;
2. preserves semantic boundaries above;
3. passes the normative golden vectors within their declared tolerances;
4. exposes provenance and uncertainty/status metadata.
