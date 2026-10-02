# M-Time 0.2 Protocol Draft

Status: **research protocol draft / hardening prototype**.

M-Time 0.2 keeps the 0.1 semantic separations and adds normative metadata for high-precision astronomical reference paths, profile scope, and source-artifact integrity.

## 1. Core invariant

A temporal/calendar result MUST preserve the distinction:

```text
physical / coordinate time
!= astronomical state
!= calendar criterion
!= observation / rukyat
!= jurisdiction
!= authority decision
!= final calendar result
```

Agreement between layers does not collapse them into one fact.

## 2. Astronomical source identity

An astronomical state SHOULD identify:

- ephemeris provider and release/kernel;
- coordinate frame;
- time scale used to evaluate the ephemeris;
- observer coordinates/datum for topocentric quantities;
- Earth-orientation source when UT1/polar motion affects the result;
- whether the quantity is geometric, astrometric, apparent, or refracted;
- uncertainty/quality class.

For the v0.2 high-precision reference path:

```text
JPL DE440 SPK / ICRF
+ TDB evaluation
+ IERS EOP
+ IAU 2006/2000A celestial-to-terrestrial transformation
+ WGS84 observer
= airless geometric topocentric state
```

An atmospheric refraction model, when used, MUST be declared separately.

## 3. Hijri geometry semantics

Profiles MUST define the geometry semantics of every criterion.

The MABIMS Indonesia 2026 profile uses:

- lunar altitude: topocentric;
- elongation: geocentric center-to-center;
- altitude threshold: >= 3 degrees;
- elongation threshold: >= 6.4 degrees.

A value with incompatible geometry semantics MUST NOT silently satisfy the profile.

## 4. Calendar profile scope

A profile MUST declare its spatial scope when the rule cannot be evaluated from one site alone.

Normative v0.2 scopes:

- `SINGLE_STATE`: evaluate one supplied astronomical state.
- `GLOBAL_ANY_SITE`: the profile passes if at least one evaluated candidate site satisfies all profile clauses.

A single-site evaluation of a `GLOBAL_ANY_SITE` profile is only a candidate-site result. It MUST NOT be represented as the final global profile outcome unless the required search domain has been evaluated.

## 5. Bundled research profiles

### MABIMS_ID_2026

Source-linked Indonesian profile implementing PMA No. 1/2026 criterion semantics. Observation/rukyat and Sidang Isbat remain separate records.

### DIYANET_1978_GLOBAL

Source-linked computational representation of the Diyanet/Türkiye 1978 visibility criterion:

- topocentric altitude >= 5 degrees;
- geocentric angular separation >= 8 degrees;
- global-any-site scope.

This representation is an interoperability profile. It is not a declaration that one fiqh methodology is scientifically or religiously mandatory.

## 6. Source artifact integrity

When source documents, observation payloads, or authority decisions are ingested, an implementation SHOULD record a content digest.

M-Time v0.2 defines:

- SHA-256 content digest;
- signature state `UNSIGNED` or `VERIFIED_ED25519`;
- signing-key identifier when a signature verifies.

A valid digital signature authenticates the supplied payload relative to the supplied key. It does not by itself establish the scientific truth, legal authority, or theological correctness of the content.

## 7. External-reference gates

A high-precision implementation SHOULD publish reproducible comparisons against external reference products.

M-Time v0.2 ships two fixed oracle classes:

1. DE440/SPK geocentric Sun/Moon vectors compared with JPL Horizons in the same ICRF frame.
2. Airless topocentric Moon azimuth/elevation using DE440 + IERS + IAU 2006/2000A compared with JPL Horizons observer output.

Acceptance thresholds MUST NOT be widened merely to obtain a passing build.

## 8. ExplainDifference

A resolution comparison SHOULD identify, at minimum:

```text
PhysicalInput
Ephemeris
Observer
Criterion
Observation
Jurisdiction
Authority
SourceVersion
```

If two results use the same astronomical state but different profile rules, the disagreement MUST be classified as a criterion difference rather than an astronomical disagreement.

## 9. Cosmic chronology

Cosmic age remains `INFERRED` and model-dependent. It MUST NOT be serialized as an absolute universal coordinate-time instant.

## 10. Revelation ontology

Revelation-text mappings remain `TEXTUAL_REFERENCE / CONCEPTUAL`.

Removing the revelation ontology MUST NOT alter numerical astronomy, calendar geometry, time-scale conversion, or cosmological integration results.

## 11. Status boundary

M-Time 0.2 is an internally reproducible research protocol. It is not:

- a replacement for the SI second or UTC;
- an adopted IAU/BIPM/ISO standard;
- a religious ruling;
- a replacement for JPL operational ephemerides;
- an absolute cosmic clock.
