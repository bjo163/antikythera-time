# M-Time — Master Issue Backlog

Status: blueprint backlog for the Rust-first M-Time program.  
Naming: M-Time = Mīqāt Temporal Protocol.  
Primary principle: separate physical time, astronomy, calendar rules, observations, jurisdiction, authority decisions, cosmological inference, historical reconstruction, and revelation-text concepts.

## EPIC A — Research Charter & Project Reset
- [ ] M-001 Freeze `antikythera-time` as legacy research archive; document trusted vs experimental outputs.
- [ ] M-002 Create new `m-time` Rust-first repository/workspace.
- [x] M-003 Adopt official project name `M-Time — Mīqāt Temporal Protocol`.
- [x] M-004 Publish one-page mission statement and non-goals.
- [x] M-005 Freeze evidence states.
- [x] M-006 Freeze quality classes.
- [x] M-007 Define scientific/religious boundary policy.
- [x] M-008 Define calendar-neutrality policy.
- [x] M-009 Define provenance/source-version requirements in core types.
- [x] M-010 Define uncertainty/unknown policy in research charter and diff engine.
- [x] M-011 Adopt normative MUST/SHOULD/MAY language in spec draft.
- [x] M-012 Rust computational core + JS/WASM application architecture adopted.

## EPIC B — Rust Temporal Kernel
- [x] M-020 Cargo workspace and CI.
- [x] M-021 `DurationSI`.
- [x] M-022 two-part `CoordinateTime`.
- [x] M-023 typed `TimeScale`.
- [x] M-024 typed `ReferenceFrame`.
- [ ] M-025 general Observer trait; Earth/site and spacecraft contracts exist but need unified trait.
- [~] M-026 uncertainty types: absolute implemented; covariance/reference pending.
- [x] M-027 evidence/provenance types.
- [x] M-028 cross-scale conversion requires explicit functions; implicit TT->UTC rejected in provider.
- [~] M-029 J2000 vectors partly represented; expand.
- [x] M-030 UTC↔TAI↔TT for leap-second era 1972+.
- [x] M-031 TT↔TCG and TDB↔TCB canonical transforms.
- [x] M-032 explicit dtr interface.
- [ ] M-033 compact automatic dtr provider not yet ported to M-Time Rust.
- [x] M-034 SOFA reference-vector harness.
- [x] M-035 WASM crate skeleton.
- [ ] M-036 generated TypeScript bindings.
- [x] M-037 independent Python checker.
- [~] M-038 JS/Python golden checks exist; full Rust-vs-legacy benchmark pending.

## EPIC C — IERS / Earth Orientation / Observer
- [x] M-050 IERS `finals.all (IAU2000)` parser in Rust.
- [~] M-051 provenance fields exist; retrieval timestamp/hash in release artifact pending.
- [x] M-052 xp/yp/UT1−UTC interpolation.
- [x] M-053 reject outside EOP coverage.
- [x] M-054 WGS84 site representation through astronomy layer.
- [~] M-055 geodetic→topocentric geometry delegated to validated provider; expose direct public helper later.
- [~] M-056 spacecraft contract exists in legacy; M-Time unified observer trait pending.
- [ ] M-057 observer/worldline provenance extension.
- [x] M-058 IERS fixture/live ingestion tests.
- [ ] M-059 EOP snapshot hash/version manifest.

## EPIC D — Antikythera-RS Computational Grammar
- [x] M-070 generic `Cycle`.
- [ ] M-071 rational gear composition graph.
- [x] M-072 phase/modular state.
- [x] M-073 Metonic metadata.
- [x] M-074 Saros.
- [x] M-075 Exeligmos.
- [x] M-076 anomalistic month.
- [x] M-077 draconic month.
- [ ] M-078 lunar anomaly correction interface.
- [x] M-079 evidence manifest categories.
- [ ] M-080 tooth-count evidence encoding.
- [ ] M-081 alternative reconstruction profiles.
- [ ] M-082 residual interface.
- [x] M-083 Saros recurrence golden test.
- [x] M-084 explicit no-cosmic-age-from-cycle rule.

## EPIC E — Celestial State / Ephemeris Abstraction
- [x] M-100 `EphemerisProvider` trait.
- [x] M-101 JPL Horizons validation adapter/workflows.
- [x] M-102 pinned deterministic offline provider adapter (`solar-ephemeris=0.2.0`) with independent JPL gates.
- [x] M-103 Sun state path.
- [x] M-104 Moon state path.
- [x] M-105 topocentric transformation.
- [x] M-106 explicit equatorial state; ecliptic internals used.
- [x] M-107 bracketed conjunction solver.
- [x] M-108 local sunset solver; convention remains provenance-sensitive.
- [x] M-109 topocentric lunar altitude.
- [x] M-110 geocentric elongation.
- [x] M-111 illumination fraction.
- [x] M-112 lunar age + geometric moon-lag metric.
- [ ] M-113 pluggable refraction/limb convention interface.
- [x] M-114 external acceptance gates.
- [x] M-115 frozen 9-case JPL fixture for offline CI.

## EPIC F — Hijri Geometry Engine
- [x] M-130 `HijriAstronomicalState`.
- [x] M-131 conjunction.
- [x] M-132 local sunset.
- [x] M-133 topocentric altitude, explicitly separated from geocentric altitude.
- [x] M-134 geocentric elongation.
- [ ] M-135 arc-of-light/width metrics.
- [x] M-136 lunar age and geometric lag.
- [~] M-137 multi-site supported structurally; global scanner pending.
- [x] M-138 site/timezone/geodetic provenance fields.
- [~] M-139 CLI summaries exist; polished human astronomy summary pending.
- [x] M-140 JPL 9-case corpus + KHGT event/geometry validation.

## EPIC G — Calendar Profile DSL
- [x] M-150 language-neutral `CalendarProfile`.
- [x] M-151 version/effective date.
- [x] M-152 composite All/Any criteria.
- [x] M-153 threshold operators/units/metric semantics.
- [x] M-154 MABIMS Indonesia 2026 3° / 6.4°.
- [~] M-155 30-day completion represented in authority workflow; exact generic profile action DSL pending.
- [x] M-156 observation separate from astronomy criterion.
- [ ] M-157 tabular Hijri profile.
- [ ] M-158 configurable local-rukyat profile.
- [ ] M-159 generic global-rukyat profile.
- [x] M-160 KHGT Muhammadiyah 2026 specialized primary-source profile.
- [ ] M-161 signed/auditable profile bundle.
- [x] M-162 astronomy core does not depend on a calendar profile.

## EPIC H — Rukyat / Observation Evidence
- [x] M-180 `ObservationReport`.
- [x] M-181 core site/org/time/instrument/weather/horizon fields.
- [x] M-182 positive/negative/inconclusive/invalidated.
- [x] M-183 raw report separate from authority acceptance.
- [x] M-184 attachment hash field.
- [x] M-185 modeled prediction separate from report by architecture.
- [ ] M-186 observation quality scoring.
- [ ] M-187 official rukyat importer.
- [ ] M-188 privacy policy.

## EPIC I — Jurisdiction & Authority
- [x] M-200 `Jurisdiction`.
- [x] M-201 `AuthorityDecision`.
- [x] M-202 source document/provenance.
- [x] M-203 decision time/authority/cited observation fields.
- [x] M-204 official vs computed separated.
- [x] M-205 simultaneous authorities can coexist via traces.
- [~] M-206 Indonesia Kemenag represented in 1447 cases; reusable authority profile pending.
- [x] M-207 selected Ramadan/Syawal/Zulhijjah 1447 cases represented.
- [ ] M-208 signed decision records.

## EPIC J — ExplainDifference
- [x] M-220 canonical diff categories.
- [x] M-221 compare resolution records.
- [x] M-222 shared sky event can be represented without false physical diff.
- [x] M-223 criterion differences.
- [x] M-224 observation differences.
- [x] M-225 jurisdiction/authority differences.
- [x] M-226 machine-readable causal diff.
- [x] M-227 plain-language explanation.
- [x] M-228 INCOMPLETE marker for unknown causes.
- [x] M-229 real Ramadan/Syawal/Zulhijjah 1447 acceptance tests.
- [ ] M-230 flagship UI.

## EPIC K — Worship Resolution Products
- [x] M-250 Ramadan boundary type + historical trace.
- [x] M-251 Syawal boundary + end-to-end 1447 government resolution.
- [x] M-252 Zulhijjah boundary + historical comparison trace.
- [x] M-253 prayer-time remains separate boundary.
- [ ] M-254 fasting sunrise/sunset profile engine.
- [ ] M-255 ICS/calendar export.
- [~] M-256 official/computed/observed separation exists in model/CLI; UI pending.
- [ ] M-257 regional dashboard.

## EPIC L — Revelation Temporal Ontology
- [~] M-280 typed schema exists; source-edition metadata needs expansion.
- [x] M-281 Qur’an 2:189.
- [x] M-282 Qur’an 10:5.
- [x] M-283 Qur’an 55:5.
- [x] M-284 Qur’an 21:33.
- [x] M-285 Genesis 1:14 witness.
- [x] M-286 Psalms 104:19 witness.
- [x] M-287 Mark 13:32 witness.
- [ ] M-288 source edition/translation/access provenance expansion.
- [x] M-289 interpretation-category enum.
- [x] M-290 CI dependency rule proves revelation crate cannot enter Hijri/cosmology numerical pipeline.
- [x] M-291 numerology prohibited by charter.
- [ ] M-292 qualified scholar review.

## EPIC M — Cosmic Chronology
- [x] M-310 flat ΛCDM Rust engine.
- [ ] M-311 curved ΛCDM.
- [ ] M-312 CPL w0waCDM.
- [x] M-313 deterministic quadrature/error estimate.
- [ ] M-314 posterior ingestion.
- [~] M-315 Planck-like age sanity vector; full published-posterior reproduction not yet ported.
- [ ] M-316 official DESI chain reproduction in Rust M-Time.
- [x] M-317 cosmic inference separate from coordinate time.
- [x] M-318 semantic Antikythera→Big-Bang rejection.
- [ ] M-319 model-comparison layer.

## EPIC N — Specification & Conformance
- [x] M-340 `MTIME-0.1.md`.
- [ ] M-341 canonical JSON/CBOR schema.
- [ ] M-342 serde representation.
- [~] M-343 relativistic SOFA vectors exist in Rust but need spec publication.
- [x] M-344 Antikythera Saros vector.
- [x] M-345 frozen JPL Hijri geometry fixture.
- [x] M-346 MABIMS boundary vectors.
- [~] M-347 historical traces exist as tests/docs; canonical serialized record pending.
- [~] M-348 cosmic sanity vector.
- [x] M-349 independent JS checker (TypeScript generation pending).
- [x] M-350 independent Python checker.
- [~] M-351 Rust/WASM/JS/Python CI exists for current golden subset.
- [ ] M-352 standalone conformance CLI.
- [ ] M-353 public conformance API.

## EPIC O — Product / WASM / API
- [x] M-370 `mtime-wasm` crate.
- [~] M-371 core WASM exposure started.
- [x] M-372 MABIMS evaluation exposed to WASM.
- [ ] M-373 ExplainDifference WASM export.
- [ ] M-374 generated TS types.
- [ ] M-375 API.
- [x] M-376 Rust CLI bootstrap.
- [ ] M-377 map.
- [ ] M-378 provenance inspector.
- [ ] M-379 authority/profile comparison UI.
- [ ] M-380 export bundle.

## EPIC P — Historical Validation Corpus
- [~] M-400 three 1447 boundaries represented; target remains >=20 Indonesian cases.
- [ ] M-401 official documents/hashes archive.
- [x] M-402 reproducible astronomy references exist for current cases.
- [x] M-403 MABIMS rule represented for selected cases.
- [x] M-404 rukyat separate in current traces.
- [x] M-405 official decisions represented for current traces.
- [ ] M-406 >=5 cross-country cases.
- [~] M-407 Muhammadiyah vs Kemenag current multi-organization corpus; expand to >=5.
- [ ] M-408 domain-expert review.
- [ ] M-409 full validation report.

## EPIC Q — Research / Governance
- [x] M-430 blueprint/research charter frozen for alpha.
- [~] M-431 blueprint paper exists; methods paper after implementation freeze.
- [ ] M-432 Hijri interoperability case-study paper.
- [ ] M-433 Antikythera-RS methods paper.
- [ ] M-434 revelation ontology paper with qualified collaborators.
- [ ] M-435 governance/normative-change process.
- [ ] M-436 privacy/security policy.
- [ ] M-437 external conformance challenge.
- [ ] M-438 independent external implementation.
- [ ] M-439 advisory/reviewer group.
- [x] M-440 no international-standard claim without external adoption.

## EPIC R — Flagship Delivery
- [~] M-460 core + mission/spec draft completed on staging; standalone repo/legacy admin freeze pending.
- [x] M-461 time/Antikythera core + WASM skeleton passing CI.
- [x] M-462 Sun/Moon/observer + Hijri geometry validated against JPL/KHGT references.
- [x] M-463 MABIMS 2026 profile + generic criterion tree.
- [~] M-464 observation + authority + ExplainDifference end-to-end represented; import/UI pending.
- [ ] M-465 >=20-case pilot report and public M-Time demo.

## Mission-Accomplished Gate
- [~] Same fully specified input produces interoperable Rust/WASM/JS/Python results for current golden subset; expand to whole protocol.
- [x] Core time/calendar records carry explicit scale/profile/provenance semantics.
- [x] Physical astronomical facts are separable from calendar criteria.
- [x] Calculations are separable from rukyat observations.
- [x] Observations are separable from authority decisions.
- [x] Differing Hijri traces return structured causes or UNKNOWN.
- [~] Ramadan/Syawal/Zulhijjah 1447 pilot traces are represented; official ingestion/replay corpus should expand.
- [x] Revelation ontology has zero numerical dependency path into Hijri/cosmology.
- [x] Cosmic chronology is separate/model-dependent; full posterior port remains.
- [x] Antikythera reconstructions expose evidence level.
- [ ] Third-party implementation from public spec alone has not yet been demonstrated.

Legend: [x] implemented/validated in alpha; [~] partial; [ ] open.
