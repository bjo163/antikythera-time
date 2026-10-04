# M-Time Implementation Status

Date: 2026-10-04  
Version: **v0.9.0 Computed Wellington Worship-Time Research Prototype**  
Internal status: **COMPUTED WELLINGTON DIYANET FAJR + VERSIONED GEOSPATIAL/HISTORICAL/SOURCE/ORACLE HARDENING COMPLETE**

M-Time is a Rust-first temporal interoperability framework. This status does **not** claim international standard adoption, religious/fiqh authority, or replacement of BIPM/IAU/IERS/JPL infrastructure.

## Final v0.9.0 internal gates

| Gate | Result | Reference |
|---|---|---|
| Rust workspace CI | **PASS** | Actions run 37054820440 |
| Rust tests | **98 passed / 0 failed** | geospatial-provider CI |
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
| Natural Earth Americas-mainland provider | **PASS** | Actions run 37194423011 |
| Natural Earth source SHA-256 pin | **PASS** | `9e0729ee...35d9` |
| Diyanet geospatial provider integration | **PASS** | mainland/island/unknown live-data gate |
| Diyanet Wellington -18° fajr provider | **PASS** | Actions run 37194975490 |
| Shawwal 1447 conjunction-before-Wellington-fajr | **TRUE** | computed DE440 + IERS |
| Wellington fajr solver residual | **-0.000067779102°** | same run |
| Conjunction-to-fajr separation | **15.434722 h** | same run |
| SHA-256 / Ed25519 verification | PASS | core CI |
| Validity/revocation-aware trusted key registry | PASS | core CI |
| Real IERS source ingestion | **PASS** | Actions run 37193413659 |
| Real DE440 source ingestion | **PASS** | Actions run 37193413659 |
| Fail-closed signed-source contract | **6/6 PASS** | Actions run 37193413659 |
| Source-backed observation binding | PASS | core CI |
| Source-backed authority binding | PASS | core CI |
| Surveyed local-horizon interpolation | PASS | core CI |
| Observer-height geometric horizon dip | PASS | core CI |
| Indonesia 1447 H replay | PASS | core CI |
| Historical falsification engine | **7/7 PASS** | Actions run 37193741501 |
| Existing Indonesia replay regression | **4/4 PASS** | Actions run 37193741501 |
| Indonesia–Türkiye Shawwal 1447 divergence replay | **REPRODUCED** | explicit layer differences |
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

## Auditable real-source ingestion

v0.6.0 adds an explicit ingestion contract containing:

- source ID and institution ID;
- canonical URL and media type;
- retrieval instant;
- computed SHA-256;
- optional expected SHA-256 pin;
- signature policy: allow unsigned or require trusted Ed25519;
- detached signature metadata;
- signer institution matching;
- trusted-key validity/revocation enforcement;
- source-backed observation and authority records.

The CI path now downloads and ingests two real official scientific artifacts:

```text
IERS_FINALS_ALL_IAU2000
sha256 = 9fbc14ae5e71de96cc1e6b43b54a547acc80c7a6ce910c0209ad6230ff3d30cd

NAIF_DE440_SHORT
sha256 = c1c7feeab882263fc493a9d5a5b2ddd71b54826cdf65d8d17a76126b260a49f2
```

Both were retrieved from their canonical official URLs during Actions run 37193413659.

Those live upstream artifacts are explicitly recorded as `signature_verified=false` because the current ingestion run did not receive an institutional detached Ed25519 signature plus trusted institutional key. M-Time does not convert TLS retrieval or source reputation into a cryptographic signature claim.

The signed-source path itself is fail-closed: missing required signatures, hash mismatch, signer/institution mismatch, unknown keys, expired keys and revoked keys are rejected.

## Computed Wellington/Diyanet worship-time provider

v0.9.0 removes the remaining fixed Wellington-fajr fixture from the production validation path.

Diyanet's published current imsak methodology uses astronomical dawn at a Sun altitude of **-18°**. M-Time now represents that as the versioned worship profile:

```text
DIYANET_IMSAK_FAJR_MINUS_18
```

and computes the event with:

```text
DE440 Sun vector
+ IERS EOP
+ Wellington observer
+ IAU 2006/2000A topocentric transform
+ -18° rising solar crossing
= Wellington fajr SolarEvent
```

For the Shawwal 1447 policy replay:

```text
published conjunction:
2026-03-19 01:24 UTC

computed Wellington fajr:
JD(UT1) = 2461119.201438614167

conjunction -> fajr:
15.434722 hours

conjunction_before_wellington_fajr = true
```

The root-solver residual is -0.000067779102°.

This computed `SolarEvent` directly implements the `WellingtonFajrProvider` boundary used by the Diyanet calendar-policy evaluator.

The current Diyanet imsak method and Wellington public prayer-time page are retained as explicit provenance; M-Time does not treat one universal fajr angle as binding on other institutions or profiles.

## Versioned Americas-mainland geospatial provider

v0.8.0 replaces the placeholder site classifier in the production path with a versioned GeoJSON-backed provider.

Dataset:

```text
Natural Earth ne_110m_land
version: 5.1.2
Git blob SHA-1: 04811d72fff2701ec67587e30ad8942675b511e3
SHA-256: 9e0729ee253ca7d7a5c4ae9395fb1902264c5377c52e224d13dd85010e2835d9
```

The provider identifies the connected American mainland landmass by an anchor inside continental North America and performs point-in-polygon classification with hole handling.

Live validation:

```text
NEW_YORK    true
SANTIAGO    true
PANAMA      true
MEXICO_CITY true
ANCHORAGE   true

HAVANA      false
HONOLULU    false
GREENLAND   false
WELLINGTON  false
JAKARTA     false
USHUAIA     false
```

The Diyanet policy integration therefore produces:

```text
New York threshold pass + Wellington timing pass  -> complete rule TRUE
Havana threshold pass + Wellington timing pass    -> complete rule FALSE
Wellington threshold pass                         -> complete rule FALSE
unknown site                                      -> UNKNOWN
```

The GeoJSON is also passed through M-Time's auditable source-ingestion layer.

This is a versioned policy-classification provider, not a meter-level coastline survey. Natural Earth 110m is intentionally coarse and should not be used to make sub-kilometer coastal-boundary claims.

## Historical falsification

v0.7.0 adds executable replay verdicts:

```text
REPRODUCED
FALSIFIED
INCOMPLETE
```

A represented historical resolution is `FALSIFIED` when its computed month action conflicts with the recorded authority decision under the represented profile.

It is `INCOMPLETE` when computation is unknown or the represented rule explicitly requires additional context that is not available.

Cross-jurisdiction replay is `REPRODUCED` when a differing calendar outcome is accompanied by explicit represented differences such as criterion/profile, observation evidence, jurisdiction, or authority.

Negative controls prove the engine does not accept unexplained date flips or counterfactual authority reversals.

The v0.7 corpus includes:

- Indonesia Ramadan 1447 H — reproduced;
- Indonesia Shawwal 1447 H — reproduced;
- Indonesia Dhulhijjah 1447 H — reproduced;
- Indonesia vs Türkiye/Diyanet Shawwal 1447 H — differing outcome reproduced with explicit layer differences;
- counterfactual authority flip — falsified;
- unexplained calendar date flip — falsified;
- unknown computation — incomplete.

Machine-readable corpus: `data/hijri/historical-falsification-v0.7.json`.

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
3. Higher-resolution geospatial refinement beyond the current pinned Natural Earth 110m mainland provider for near-coast/border edge cases.
4. Broader official Wellington/Diyanet prayer-time replay across more dates and edge seasons; v0.9.0 computes the -18° method directly for the Shawwal 1447 policy case.
5. Live institutional detached-signature feeds and public-key registries for upstream authority/observation sources; v0.6.0 implements the fail-closed ingestion machinery and live hash-recorded source acquisition.
6. Larger multi-country, multi-decade historical replay corpus; v0.7.0 establishes the falsification engine and first cross-jurisdiction executable corpus.
7. Unaffiliated implementation and expert review.
8. Formal standardization/adoption.
9. Migration from `bjo163/antikythera-time` to a dedicated M-Time repository.

## Next-stage priority

Do not redesign the ontology.

Priority order:

1. expand historical falsification to more countries and decades;
2. institutional signed authority/observation source feeds;
3. broaden the now-global/multi-year validation matrix further;
4. higher-resolution geospatial edge-case validation and broader worship-time replay;
5. independent implementation and peer review;
6. external standardization discussion.
