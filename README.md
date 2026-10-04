# M-Time — Mīqāt Temporal Protocol

**Status:** Rust-first **v0.10.0 Seasonal Solar Oracle + Live Lab Research Prototype**.

M-Time is a temporal interoperability framework. It does **not** invent a new physical second, replace UTC, choose a fiqh position, or claim an absolute cosmic clock. Its flagship use-case is explainable Hijri/worship-calendar resolution.

Core rule: physical time, astronomy, calendar criteria, observation/rukyat, jurisdiction, authority decisions, cosmic inference, historical reconstruction, and revelation-text concepts are separate semantic layers.

## v0.10.0 highlights

- Pure-Rust offline JPL DE440/SPK provider.
- Integrated `mtime-hilal` engine: `UTC + observer + IERS + DE440 → HijriAstronomicalState`.
- External DE440 ↔ JPL Horizons vector oracle.
- WGS84 lunar topocentric parallax.
- IAU 2006/2000A celestial→terrestrial transform with IERS polar motion.
- Fixed Jakarta topocentric oracle agrees with Horizons at sub-arcsecond level and is gated at 0.001°.
- 21-case Horizons boundary matrix: Ramadan/Syawal/Zulhijjah × Jakarta/Ankara/Makkah/Wellington/New York/Santiago/Cape Town, **21/21 PASS**; max = 0.000359059275° (~1.29 arcsec).
- 21-case 2024/2025/2026 multi-year geometry matrix over the same seven sites, **21/21 PASS**; max = 0.000336345642° (~1.21 arcsec).
- 4-case Wellington seasonal Sun oracle against JPL Horizons, **4/4 PASS**; max direction residual = **0.000173324462° (~0.624 arcsec)**.
- Combined active external topocentric coverage: **46 JPL Horizons comparisons** (42 Moon + 4 Sun).
- Explicit optional atmospheric-refraction model; airless geometry remains separately available.
- Surveyed local-horizon profile, horizon obstruction interpolation, and observer-height geometric dip utilities remain separate from calendar criteria.
- SHA-256 + Ed25519 source-artifact integrity primitives plus validity/revocation-aware trusted-key registry.
- Fail-closed real-source ingestion contract with source/institution identity, canonical URL, retrieval time, optional hash pin, detached signature policy, signer-institution matching, and explicit unsigned/trusted states.
- CI ingests live official IERS finals.all and NAIF DE440 artifacts and preserves their exact SHA-256 records; both remain explicitly unsigned unless an institutional detached signature and trusted key are provided.
- Observation and authority records can now bind directly to auditable source-ingestion records.
- Historical replay engine emits explicit `REPRODUCED`, `FALSIFIED`, or `INCOMPLETE` verdicts instead of treating every historical fit as success.
- Indonesia Ramadan/Shawwal/Dhulhijjah 1447 H replays reproduce the represented profile/authority outcomes.
- Indonesia–Türkiye/Diyanet Shawwal 1447 H divergence is reproduced with explicit criterion/observation/jurisdiction/authority differences.
- Negative controls prove counterfactual authority flips and unexplained date flips are falsified.
- New `mtime-geospatial` crate provides pinned Natural Earth v5.1.2 mainland classification with SHA-256 verification.
- Americas-mainland policy classification distinguishes continental New York/Santiago/Panama/Mexico City/Anchorage from Havana/Honolulu/Greenland and non-Americas sites.
- Diyanet policy evaluation now accepts the real GeoJSON-backed provider directly; unknown site IDs remain UNKNOWN.
- Versioned Diyanet imsak profile uses the institution's published -18° astronomical-dawn criterion.
- High-precision Wellington fajr is computed from DE440 + IERS + IAU topocentric geometry rather than supplied as a fixed boolean/event fixture.
- Observer-style Sun path now includes solar light-time + first-order annual aberration; geometric Sun vectors remain separate for elongation semantics.
- IERS finals.all parser handles fixed-width negative UT1 tokens and compact date fields used by the official product.
- Shawwal 1447 replay computes conjunction-before-Wellington-fajr = true with a 15.434722-hour separation.
- Indonesia MABIMS/PMA No. 1/2026 source-linked profile.
- Türkiye Diyanet 1978/2016/2026 global profile: 5°/8° visibility plus executable Americas-mainland and conjunction-before-Wellington-Fajr conditions.
- Typed provider wiring derives Diyanet policy context from threshold-passing sites plus geospatial/Wellington-fajr providers; missing evidence stays UNKNOWN.
- GitHub Pages **M-Time Live Lab** replaces the stale v0.2 demo and shows live release/Actions health, oracle metrics, trust/falsification status, and the Rust/WASM profile evaluator.
- Rust/WASM profile comparison surface.
- 100 Rust tests / 0 failures on the v0.10.0 seasonal-solar regression gate.
- Indonesia 1447 H Ramadan/Syawal/Zulhijjah replay corpus.
- ExplainDifference preserves astronomy vs criterion vs rukyat vs authority.

## Flagship profiles

`MABIMS_ID_2026`: topocentric Moon altitude ≥ 3° AND geocentric center-to-center elongation ≥ 6.4°. Observation and Sidang Isbat authority decisions remain separate records.

`DIYANET_1978_VISIBILITY`: the 5° altitude / 8° separation visibility component, evaluated across candidate sites. The current official methodology also states additional regional/timing conditions (including an Americas-mainland condition and conjunction-before-Wellington-Fajr condition); therefore a 5°/8° site pass is **not serialized as a complete Diyanet month-start decision**. In v0.3.0 those conditions can be derived through typed geospatial and Wellington-fajr providers instead of manually supplied booleans.

## Scientific boundaries

Antikythera supplies a computational grammar—cycle/ratio/state/recurrence—not a metrological authority and not a direct cosmic-age clock. Cosmic age remains a model-dependent inference. Revelation texts remain TEXTUAL_REFERENCE / CONCEPTUAL and supply no hidden numerical physics priors.

See `CHARTER.md`, `spec/MTIME-0.2.md`, `docs/BLUEPRINT.md`, and `docs/STATUS.md`.
