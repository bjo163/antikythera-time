# M-Time — Mīqāt Temporal Protocol

**Status:** Rust-first **software-only Antikythera Systems Integration Research Prototype**.

M-Time is a temporal interoperability framework. It does **not** invent a new physical second, replace UTC, choose a fiqh position, or claim an absolute cosmic clock. Its flagship use-case is explainable Hijri/worship-calendar resolution.

Core rule: physical time, astronomy, calendar criteria, observation/rukyat, jurisdiction, authority decisions, cosmic inference, historical reconstruction, and revelation-text concepts are separate semantic layers.

## v0.14.0 highlights

- **M7–M10 and M13–M19 internal software milestones complete**; hardware realization has been removed from scope, while M20 external software-review/reproduction gates remain deliberately open.
- M8 V2 Experimental improves full-span 1850–2149 lunar P95 from **0.302376434° → 0.253539543°** and max from **0.427321722° → 0.314474632°**, while default remains V1.
- MTS-2 independent Python conformance covers **135 V1 + 135 V2** differential vectors plus malformed packets.
- Public Python SDK, C ABI/header, JSON schemas, signed-bundle spec, compatibility policy, threat review, RFC and governance are present.
- Native M-Time calendar/worship demos preserve geometry/profile/observation/authority separation.
- MCLOCK-1 is retained strictly as a software clock-state serialization/view; physical clock hardware is outside project scope.
- v1 readiness remains **BLOCKED** on unaffiliated reproduction and external review evidence.
- **1900–2100 M6 calibration:** 2,412 monthly epochs plus 15,676 targeted lunar/solar phase-anomaly samples.
- M6 calibration now uses an explicit **IAU 2006 mean ecliptic of date** reference, preventing fixed-J2000 frame rotation from being misclassified as Antikythera dynamical error.
- Digital Sun 1900–2100 max residual: **0.009572456° absolute / 0.010229337° dynamic**.
- Digital Moon 1900–2100: **0.305809037° P95 / 0.427321722° max absolute; 0.528604874° max dynamic**.
- Moon-Sun phase: **0.304293570° P95 / 0.424730056° max**.
- Worst lunar phase-bin mean currently occurs in **draconic phase 0.5833–0.6250** at **0.232686478°**, making node/draconic structure a candidate for M8 ablation testing—not yet an accepted correction.
- Machine-readable M6 CSV artifacts are published by the calibration workflow.
- **Software Antikythera is now a first-class core:** virtual cycles, gear relations/trains, dials and synchronized machine state.
- Separate `ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1` and `MTIME_DIGITAL_ANTIKYTHERA_V1` profiles.
- 12-epoch 2026 DE440 calibration: digital max residuals **0.014132662° Sun**, **0.504873428° Moon**, **0.277593562° Moon-Sun phase**.
- New `mtime-temporal` native state combines a continuous SI coordinate with solar/synodic/sidereal/anomalistic/draconic/Metonic/Saros/Exeligmos phases.
- New `mtime-clock` crate and **MCLOCK-1** packet expose M-Time as a software-facing clock-state surface.
- CLI + WASM + GitHub Pages Live Lab now expose the current Software Antikythera/M-Clock state.
- **115 Rust tests / 0 failures** on the Phase-5 staging gate.
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
- New source-backed corpus covers **21 real cases** across Indonesia, Singapore and Malaysia over **2024–2026 / 1445–1447 H**.
- Corpus gate reports **12 reproduced real cases**, **9 evidence-limited real cases kept incomplete**, and **2/2 contradiction controls falsified**.
- 2025 Indonesia–Singapore Ramadan and Dhulhijjah one-day divergences are preserved explicitly instead of normalized away.
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
- Historical v0.11 regression remains active alongside the Antikythera/M-Time software gates.
- Indonesia 1447 H Ramadan/Syawal/Zulhijjah replay corpus.
- ExplainDifference preserves astronomy vs criterion vs rukyat vs authority.

## Flagship profiles

`MABIMS_ID_2026`: topocentric Moon altitude ≥ 3° AND geocentric center-to-center elongation ≥ 6.4°. Observation and Sidang Isbat authority decisions remain separate records.

`DIYANET_1978_VISIBILITY`: the 5° altitude / 8° separation visibility component, evaluated across candidate sites. The current official methodology also states additional regional/timing conditions (including an Americas-mainland condition and conjunction-before-Wellington-Fajr condition); therefore a 5°/8° site pass is **not serialized as a complete Diyanet month-start decision**. In v0.3.0 those conditions can be derived through typed geospatial and Wellington-fajr providers instead of manually supplied booleans.

## Scientific boundaries

Antikythera supplies a computational grammar—cycle/ratio/state/recurrence—not an external standards authority and not a direct cosmic-age clock. Cosmic age remains a model-dependent inference. Revelation texts remain TEXTUAL_REFERENCE / CONCEPTUAL and supply no hidden numerical physics priors.

See `CHARTER.md`, `spec/MTIME-0.2.md`, `docs/BLUEPRINT.md`, and `docs/STATUS.md`.


## Software-only scope

M-Time is intentionally software-only. Physical M-Clock hardware, embedded controllers, GNSS/PPS receivers, oscillators, RTCs, bench prototypes and hardware metrology are not project requirements and are not v1 gates.

The active chain is:

```text
Software Antikythera
→ native M-Time
→ software protocols / SDK / Live Lab
→ calendar / worship / observation / authority applications
```

MCLOCK-1 remains a software serialization/view of M-Time state.
