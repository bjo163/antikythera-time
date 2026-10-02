# M-Time Implementation Status

Date: 2026-10-03  
Version: **v0.2.2 Hardening Research Prototype**  
Internal status: **HIGH-PRECISION FLAGSHIP PATH + POLICY/INTEGRITY HARDENING COMPLETE**

M-Time is a Rust-first temporal interoperability framework. This status does **not** claim international standard adoption, religious/fiqh authority, or replacement of BIPM/IAU/IERS/JPL infrastructure.

## Final v0.2.2 internal gates

| Gate | Result | Reference |
|---|---|---|
| Rust workspace CI | **PASS** | Actions run 37054820440 |
| Rust tests | **74 passed / 0 failed** | same run |
| Rust↔Python compatibility | **PASS** | Actions run 37054820611 |
| Offline JPL DE440/SPK provider | PASS | prior SPK reference gates |
| Fixed Jakarta topocentric reference | PASS | prior topocentric reference gate |
| 9-case Horizons topocentric oracle matrix | **9/9 PASS** | Actions run 37054820402 |
| Matrix physical pointing threshold | **0.001°** | same run |
| Matrix maximum direction residual | **~0.000257° (~0.93 arcsec)** | same run |
| Executable Diyanet additional calendar conditions | PASS | core CI |
| SHA-256 / Ed25519 verification | PASS | core CI |
| Validity/revocation-aware trusted key registry | PASS | core CI |
| Surveyed local-horizon interpolation | PASS | core CI |
| Observer-height geometric horizon dip | PASS | core CI |
| Indonesia 1447 H replay | PASS | core CI |
| Indonesia–Türkiye Shawwal 1447 corpus | COMPLETE | source-linked corpus |
| Revelation no-numerical-prior invariant | PASS | core CI |
| Planck-like cosmology inference | PASS | core CI |

## Broad astronomy oracle matrix

The matrix uses official DE440 short SPK + current IERS finals.all and compares M-Time against JPL Horizons AIRLESS topocentric Moon coordinates.

Cases:

- Ramadan 1447 boundary epoch × Jakarta / Ankara / Makkah;
- Shawwal 1447 boundary epoch × Jakarta / Ankara / Makkah;
- Dhulhijjah 1447 boundary epoch × Jakarta / Ankara / Makkah.

Maximum measured physical horizon-direction residual:

```text
~0.000257° ≈ 0.93 arcsec
```

All 9 cases pass the fixed 0.001° gate.

Raw azimuth is still reported, but the broad matrix uses spherical sky-direction error as the physical pointing metric because azimuth becomes ill-conditioned near zenith. This was discovered empirically in the Dhulhijjah/Makkah case and fixed without weakening the physical threshold.

## Diyanet current-policy execution

The profile no longer stores current methodology beyond 5°/8° as opaque metadata only.

Executable conditions now include:

1. a qualifying visibility event on North or South American mainland;
2. conjunction before Wellington/New Zealand fajr.

The evaluator therefore distinguishes:

```text
5°/8° threshold component
+
global-site scope
+
Americas-mainland condition
+
Wellington-fajr timing condition
=
complete represented policy result
```

Missing policy context produces UNKNOWN rather than a false complete decision.

## Integrity hardening

`mtime-integrity` now supports:

- SHA-256 content hashes;
- Ed25519 verification;
- institution/key identifiers;
- validity windows;
- explicit revocation status;
- duplicate-key rejection;
- source-artifact binding to observation/authority records.

A valid signature authenticates content relative to a trusted key registry. It does not establish astronomical, legal, theological, or observational truth.

## Local-horizon hardening

`mtime-astro` now provides:

- surveyed obstruction altitude vs azimuth;
- circular interpolation through north;
- observer-height geometric horizon dip;
- explicit local-horizon clearance.

Local horizon and atmospheric refraction remain separate inputs. They are not silently merged into the astronomical altitude used by a calendar profile.

## High-precision flagship path

```text
UTC + observer + IERS EOP + DE440
→ TT / TDB / UT1
→ IAU 2006/2000A topocentric transform
→ Moon altitude + geocentric elongation
→ HijriAstronomicalState
→ versioned calendar profile
→ criterion / policy result
→ observation / jurisdiction / authority
→ final CalendarResult
→ ExplainDifference
```

## What is internally complete

For research-prototype purposes, M-Time can now:

- compute a high-precision offline Sun/Moon reference path;
- validate topocentric geometry against Horizons over a small multi-site/multi-epoch matrix;
- execute MABIMS and represented Diyanet policy conditions without collapsing them into one rule;
- preserve observation, authority and source-integrity layers separately;
- explain why two temporal/calendar outcomes differ.

## Open production / external gates

These remain deliberately **OPEN**:

1. Much broader multi-year and global astronomy oracle matrix.
2. Field validation of atmospheric refraction and surveyed local-horizon profiles.
3. Automatic geospatial determination of “American mainland” from visibility maps rather than caller-supplied policy context.
4. Automatic Wellington fajr computation wired directly into the Diyanet policy evaluator with a versioned worship profile.
5. Live institutional public-key registries and signed production ingestion.
6. Larger multi-country, multi-decade historical replay corpus.
7. Unaffiliated implementation and expert review.
8. Formal standardization/adoption.
9. Migration from `bjo163/antikythera-time` to a dedicated M-Time repository.

## Next-stage priority

Do not redesign the ontology.

Priority order:

1. wire policy contexts to computed astronomy/worship/geospatial providers;
2. broaden validation matrix;
3. signed real-source ingestion;
4. historical falsification across jurisdictions;
5. independent implementation and peer review;
6. external standardization discussion.
