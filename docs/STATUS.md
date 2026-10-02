# M-Time Implementation Status

Date: 2026-10-02  
Version: **v0.2 Hardening Research Prototype**  
Internal status: **HIGH-PRECISION FLAGSHIP PATH COMPLETE**

M-Time is a Rust-first temporal interoperability framework. This status does **not** claim international standard adoption, religious/fiqh authority, or replacement of BIPM/IAU/IERS/JPL infrastructure.

## Final v0.2 internal gates

| Gate | Result | Reference |
|---|---|---|
| Rust workspace CI | **PASS** | Actions run 36958335307 |
| Rust tests | **60 passed / 0 failed** | same run |
| Offline JPL DE440/SPK provider | PASS | 36956737541 |
| Integrated topocentric Hijri state vs Horizons | **PASS** | 36958335289 |
| Topocentric acceptance threshold | **0.001° / 3.6 arcsec** | same run |
| SHA-256 / Ed25519 verification | PASS | 36956837579 |
| WASM multi-profile engine | PASS | 36957501866 |
| Indonesia 1447 H replay | PASS | core CI |
| Indonesia–Türkiye Shawwal 1447 corpus | COMPLETE | source-linked corpus |
| Revelation no-numerical-prior invariant | PASS | core CI |
| Planck-like cosmology inference | PASS | core CI |

## v0.2 high-precision reference path

DE440/SPK ICRF vector oracle at JD TDB 2461119.0:

~~~text
Sun max component difference   0.000025597 km
Moon max component difference  0.002110027 km
~~~

Fixed Jakarta topocentric oracle at 2026-03-19 10:00 UTC:

~~~text
M-Time altitude   17.062943893901°
Horizons altitude 17.062815000000°
altitude residual 0.000128893901°
azimuth residual  0.000152801998°
~~~

The previous GMST-only diagnostic had about 0.3146° altitude error. IAU 2006/2000A plus IERS UT1/polar motion reduces the fixed-oracle residual to sub-arcsecond scale.

## Integrated HilalEngine

The mtime-hilal crate now executes:

~~~text
UTC + observer + IERS EOP + DE440
→ TT / TDB / UT1
→ IAU 2006/2000A topocentric transform
→ Moon altitude + geocentric elongation
→ HijriAstronomicalState
~~~

The integrated state itself passes the 0.001° Horizons altitude gate. It also provides an explicit local-sunset solver whose Sun-center altitude definition is caller-provided rather than hidden.

## Source integrity

mtime-integrity provides SHA-256 content hashes and Ed25519 signature verification. A signature proves integrity/authentication relative to a key; it does not prove scientific, legal, or theological correctness.

## Multi-profile interoperability

Bundled source-linked profile components include:

- MABIMS_ID_2026;
- DIYANET_1978_VISIBILITY.

The Diyanet 5°/8° criterion is stored as a visibility component only. Current official methodology also states additional regional/timing conditions, stored explicitly as additional_calendar_conditions. A candidate-site threshold pass is not a complete Diyanet month-start decision.

## Cross-jurisdiction validation

Machine-readable corpus: data/hijri/shawwal-1447-id-tr.json.

Documented official outcomes:

- Indonesia: 1 Shawwal 1447 H = **21 March 2026**;
- Türkiye/Diyanet: 1 Shawwal 1447 H = **20 March 2026**.

The corpus preserves methodology/profile, observation scope/evidence, jurisdiction and authority as separate causal layers without ranking the religious methods.

## What is now internally complete

For research-prototype purposes, M-Time can now trace:

~~~text
physical / coordinate time
→ validated high-precision celestial geometry
→ HijriAstronomicalState
→ versioned calendar profile
→ criterion result
→ observation / rukyat
→ jurisdiction
→ authority
→ final calendar result
→ ExplainDifference
~~~

This is the core original mission in executable form.

## Open production / external gates

1. Broad multi-epoch and multi-location astronomy oracle matrix.
2. Operational atmospheric/refraction and local-horizon validation.
3. Automatic execution of the additional international-profile conditions preserved in policy metadata.
4. Live signed institutional ingestion and key registries.
5. Larger multi-country, multi-decade historical replay corpus.
6. Unaffiliated implementation and expert review.
7. Formal standardization/adoption.
8. Migration from bjo163/antikythera-time to a dedicated M-Time repository.

## Next-stage priority

Do not redesign the ontology. Prioritize broad validation, complete policy execution, signed source ingestion, historical falsification, independent implementation, peer review, and external standardization discussion.