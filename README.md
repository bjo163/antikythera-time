# M-Time — Mīqāt Temporal Protocol

**Status:** Rust-first **v0.2 Hardening Research Prototype**.

M-Time is a temporal interoperability framework. It does **not** invent a new physical second, replace UTC, choose a fiqh position, or claim an absolute cosmic clock. Its flagship use-case is explainable Hijri/worship-calendar resolution.

Core rule: physical time, astronomy, calendar criteria, observation/rukyat, jurisdiction, authority decisions, cosmic inference, historical reconstruction, and revelation-text concepts are separate semantic layers.

## v0.2 highlights

- Pure-Rust offline JPL DE440/SPK provider.
- External DE440 ↔ JPL Horizons vector oracle.
- WGS84 lunar topocentric parallax.
- IAU 2006/2000A celestial→terrestrial transform with IERS polar motion.
- Fixed Jakarta topocentric oracle agrees with Horizons at sub-arcsecond level and is gated at 0.001°.
- Explicit optional atmospheric-refraction model; airless geometry remains separately available.
- SHA-256 + Ed25519 source-artifact integrity primitives.
- Indonesia MABIMS/PMA No. 1/2026 source-linked profile.
- Türkiye Diyanet 1978 global-any-site 5°/8° source-linked profile.
- Rust/WASM profile comparison surface.
- Indonesia 1447 H Ramadan/Syawal/Zulhijjah replay corpus.
- ExplainDifference preserves astronomy vs criterion vs rukyat vs authority.

## Flagship profiles

`MABIMS_ID_2026`: topocentric Moon altitude ≥ 3° AND geocentric center-to-center elongation ≥ 6.4°. Observation and Sidang Isbat authority decisions remain separate records.

`DIYANET_1978_GLOBAL`: candidate sunset states use altitude ≥ 5° AND elongation ≥ 8°, evaluated under a global-any-site scope. It is represented as a distinct calendar profile, not as a claim that one fiqh methodology is scientifically mandatory.

## Scientific boundaries

Antikythera supplies a computational grammar—cycle/ratio/state/recurrence—not a metrological authority and not a direct cosmic-age clock. Cosmic age remains a model-dependent inference. Revelation texts remain TEXTUAL_REFERENCE / CONCEPTUAL and supply no hidden numerical physics priors.

See `CHARTER.md`, `spec/MTIME-0.1.md`, `docs/BLUEPRINT.md`, and `docs/STATUS.md`.
