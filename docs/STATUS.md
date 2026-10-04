# M-Time Implementation Status

Date: 2026-10-04  
Version: **v0.5.0 Global + Multi-Year Oracle Research Prototype**  
Internal status: **GLOBAL + MULTI-YEAR HORIZONS ORACLE + PROVIDER-WIRED POLICY/INTEGRITY HARDENING COMPLETE**

M-Time is a Rust-first temporal interoperability framework. This status does **not** claim international standard adoption, religious/fiqh authority, or replacement of BIPM/IAU/IERS/JPL infrastructure.

## Final v0.5.0 internal gates

| Gate | Result | Reference |
|---|---|---|
| Rust workspace CI | **PASS** | Actions run 37054820440 |
| Rust tests | **80 passed / 0 failed** | provider-wiring CI |
| Rust↔Python compatibility | **PASS** | Actions run 37054820611 |
| Offline JPL DE440/SPK provider | PASS | prior SPK reference gates |
| Fixed Jakarta topocentric reference | PASS | prior topocentric reference gate |
| 21-case Horizons topocentric oracle matrix | **21/21 PASS** | Actions run 37192295790 |
| Matrix physical pointing threshold | **0.001°** | same run |
| Matrix maximum direction residual | **0.000359059275° (~1.29 arcsec)** | same run |
| Matrix worst case | **Ramadan × Wellington** | same run |
| 21-case multi-year Horizons oracle | **21/21 PASS** | Actions run 37192584075 |
| Multi-year maximum residual | **0.000336345642° (~1.21 arcsec)** | same run |
| Multi-year worst case | **2026-03-01 12:00 UTC × Ankara** | same run |
| Combined external Horizons coverage | **42 cases** | 21 boundary + 21 multi-year |
| Executable Diyanet additional calendar conditions | PASS | core CI |
| Provider-derived Americas/Wellington policy context | **PASS** | provider-wiring CI |
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

- Ramadan 1447 boundary epoch × Jakarta / Ankara / Makkah / Wellington / New York / Santiago / Cape Town;
- Shawwal 1447 boundary epoch × the same seven sites;
- Dhulhijjah 1447 boundary epoch × the same seven sites.

This covers Southeast Asia, Türkiye, Hijaz, Oceania, North America, South America, and Africa.

Maximum measured physical horizon-direction residual:

```text
0.000359059275° ≈ 1.29 arcsec
```

Worst case: Ramadan × Wellington.

All 21 cases pass the fixed 0.001° gate. The matrix workflow now emits an explicit case count, maximum residual, and fixed threshold summary in its artifact.

Raw azimuth is still reported, but the broad matrix uses spherical sky-direction error as the physical pointing metric because azimuth becomes ill-conditioned near zenith. This was discovered empirically in the Dhulhijjah/Makkah case and fixed without weakening the physical threshold.

## Multi-year astronomy oracle

A second external matrix now checks the same seven sites at fixed geometry-regression epochs on 2024-03-01, 2025-03-01, and 2026-03-01 at 12:00 UTC. These dates are deliberately astronomical regression epochs, not Hijri calendar decisions.

Result:

```text
21 / 21 PASS
max_direction_error_deg = 0.000336345642
threshold_deg = 0.001000000000
```

Worst case: 2026-03-01 12:00 UTC × Ankara (~1.21 arcsec).

Combined with the 21-case 1447 H boundary matrix, M-Time now has 42 external JPL Horizons topocentric direction comparisons in the active regression suite.

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

As of v0.3.0, the complete represented rule can also be evaluated through typed providers instead of manually entered booleans. Threshold-passing site IDs feed an `AmericasMainlandProvider`, while a `WellingtonFajrProvider` supplies a computed imsak/fajr event. The conjunction-vs-fajr comparison is explicit on UT1 Julian dates. Unknown provider evidence remains UNKNOWN.

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
- validate topocentric geometry against Horizons over a seven-site / three-epoch global matrix;
- execute MABIMS and represented Diyanet policy conditions without collapsing them into one rule;
- preserve observation, authority and source-integrity layers separately;
- explain why two temporal/calendar outcomes differ.

## Open production / external gates

These remain deliberately **OPEN**:

1. Further temporal expansion beyond the current 2024-2026 / 42-case active Horizons suite, including more years and more intra-year epochs.
2. Field validation of atmospheric refraction and surveyed local-horizon profiles.
3. Production-grade geospatial determination of “American mainland” backed by an authoritative/versioned polygon or visibility-map dataset.
4. Production Wellington imsak/fajr provider backed by an explicitly versioned authoritative worship-time method; v0.3.0 wires the provider interface but does not hard-code a universal fajr angle.
5. Live institutional public-key registries and signed production ingestion.
6. Larger multi-country, multi-decade historical replay corpus.
7. Unaffiliated implementation and expert review.
8. Formal standardization/adoption.
9. Migration from `bjo163/antikythera-time` to a dedicated M-Time repository.

## Next-stage priority

Do not redesign the ontology.

Priority order:

1. signed real-source ingestion;
2. historical falsification across jurisdictions;
3. broaden the now-global/multi-year validation matrix further;
4. production geospatial and worship-time providers;
5. independent implementation and peer review;
6. external standardization discussion.
