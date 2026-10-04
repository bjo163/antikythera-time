# M-Time Long-Term Roadmap — M6 to M20

Status baseline: v0.12.0

Core goal:

```text
Antikythera computational idea
        ↓
Software Antikythera
        ↓
native M-Time temporal system
        ↓
M-Clock
        ↓
calendar / worship / observation / authority applications
```

Modern references such as JPL DE, IERS, BIPM/UTC and GNSS are calibration, interoperability and falsification layers. They must not silently replace the Software Antikythera core.

## Non-negotiable invariants

1. `mtime-antikythera` must not depend on JPL, IERS, UTC feeds, calendar policy, worship policy, authority or revelation crates.
2. Historical reconstruction and M-Time digital correction profiles remain separately identifiable.
3. Every digital correction term must be named, versioned, testable and removable.
4. Calibration failures are investigated; thresholds are not widened merely to make CI green.
5. "More accurate" is always qualified by a metric, interval, reference frame and uncertainty.
6. Native M-Time keeps both linear SI duration and cyclic astronomical state.
7. Calendar criterion, observation, jurisdiction and authority stay separate from physical/cyclic state.
8. Revelation may guide temporal semantics and ethics but must not inject unverified physical constants.
9. Unknown evidence remains unknown.
10. A v1.0 claim requires independent implementation and physical-device validation.

---

## M6 — Antikythera Accuracy Program I

Target: v0.13.x

Goal: understand the current digital residuals instead of hiding them.

### Todo

- [ ] Build a 1900–2100 calibration matrix with monthly epochs.
- [ ] Add dense new/full/quarter Moon sampling.
- [ ] Add perihelion/aphelion and lunar perigee/apogee sampling.
- [ ] Separate absolute frame offset from dynamical phase error.
- [ ] Produce Sun, Moon and Moon-Sun residual time series.
- [ ] Decompose lunar residual by synodic, anomalistic and draconic phase.
- [ ] Add residual histograms and percentile metrics.
- [ ] Store machine-readable calibration artifacts.
- [ ] Define P50/P95/P99/max accuracy metrics.
- [ ] Freeze v0.12 residuals as regression baselines.

Exit gate: every residual can be reproduced from a versioned profile and calibration artifact.

---

## M7 — Historical Antikythera Reconstruction Fidelity

Target: v0.14.x

Goal: make historical reconstruction auditable rather than a loose approximation.

### Todo

- [ ] Create a bibliography-backed component/evidence registry.
- [ ] Represent surviving evidence vs inferred gearing vs hypothetical reconstruction.
- [ ] Encode documented Metonic, Saros and lunar-anomaly relations as explicit topology.
- [ ] Add reconstruction-profile version IDs.
- [ ] Support competing scholarly reconstruction profiles where evidence is uncertain.
- [ ] Add provenance per gear/train/relation.
- [ ] Add archaeological uncertainty notes to machine outputs.
- [ ] Build historical-dial visualization in Live Lab.
- [ ] Add tests proving historical profile never imports modern correction coefficients.
- [ ] Publish reconstruction assumptions as machine-readable JSON.

Exit gate: every historical mechanism claim maps to an evidence label and source record.

---

## M8 — Digital Antikythera Correction Architecture

Target: v0.15.x

Goal: improve precision while preserving the Antikythera-style independent machine.

### Todo

- [ ] Introduce `CorrectionTerm` and `CorrectionRegistry`.
- [ ] Make every correction independently switchable.
- [ ] Separate secular, periodic and epoch-offset corrections.
- [ ] Fit residual terms without embedding JPL tables.
- [ ] Add coefficient provenance and training/calibration interval.
- [ ] Add out-of-sample validation interval.
- [ ] Reject correction sets that improve training but degrade validation materially.
- [ ] Add correction ablation reports.
- [ ] Add deterministic coefficient serialization.
- [ ] Compare compact rational-cycle models against polynomial/periodic corrections.

Exit gate: digital precision improves measurably and every improvement is attributable to explicit terms.

---

## M9 — M-Time Native Temporal Specification 2.0

Target: v0.16.x

Goal: make M-Time a defined temporal coordinate/state system, not only internal structs.

### Todo

- [ ] Formalize M-Time epoch semantics.
- [ ] Specify linear coordinate units and overflow range.
- [ ] Specify cyclic-state normalization.
- [ ] Define canonical binary and JSON encodings.
- [ ] Define ordering/equality rules for M-Time instants.
- [ ] Define conversion contracts to TT/TDB/TAI/UTC/UT1.
- [ ] Define leap/discontinuity semantics explicitly.
- [ ] Add round-trip conformance vectors.
- [ ] Add schema/version negotiation.
- [ ] Publish `MTS-2` specification.

Exit gate: two independent serializers can exchange an M-Time state losslessly.

---

## M10 — Uncertainty & Accuracy Budget Engine

Target: v0.17.x

Goal: every meaningful M-Time result carries an honest uncertainty budget.

### Todo

- [ ] Define uncertainty categories: numerical/model/reference/observer/environment/policy.
- [ ] Propagate uncertainty through linear time conversions.
- [ ] Propagate model uncertainty through Antikythera dials.
- [ ] Add calibration-derived residual envelopes.
- [ ] Distinguish random vs systematic/model uncertainty.
- [ ] Add criterion-margin uncertainty for Hijri decisions.
- [ ] Add stale-reference-data uncertainty flags.
- [ ] Add M-Clock confidence/status fields.
- [ ] Add explainable uncertainty breakdown API.
- [ ] Prevent "high precision" labels when uncertainty data is missing.

Exit gate: no public precision claim lacks a machine-readable uncertainty basis.

---

## M11 — M-Clock Physical Bench Prototype

Target: v0.18.x hardware-alpha

Goal: turn MCLOCK-1 into a real laboratory device.

### Todo

- [ ] Select a Rust-capable bench controller.
- [ ] Select GNSS receiver with PPS.
- [ ] Select RTC and characterized TCXO/OCXO/holdover source.
- [ ] Define PPS capture interface and timestamp path.
- [ ] Implement device boot/self-test state.
- [ ] Implement lock/holdover/stale-reference indicators.
- [ ] Implement offline Software Antikythera state.
- [ ] Render linear + cyclic dials on physical display.
- [ ] Implement signed reference/profile bundle updates.
- [ ] Publish BOM, wiring, firmware build and reproducibility guide.

Exit gate: a physical device can produce MCLOCK packets and display cycle state while disconnected from the network after synchronization.

---

## M12 — M-Clock Metrology & Holdover

Target: v0.19.x hardware-beta

Goal: measure the physical clock instead of assuming it is accurate.

### Todo

- [ ] Measure PPS input latency/jitter.
- [ ] Measure oscillator frequency offset.
- [ ] Characterize 1 h / 6 h / 24 h / 72 h holdover.
- [ ] Test thermal drift.
- [ ] Test reboot/power-loss recovery.
- [ ] Test GNSS-loss behavior.
- [ ] Add calibration constants with provenance.
- [ ] Compare against laboratory/reference time source where available.
- [ ] Publish error budget and plots.
- [ ] Define hardware accuracy classes.

Exit gate: M-Clock timing accuracy is expressed by measured data, not design expectation.

---

## M13 — Calendar & Worship on Native M-Time

Target: v0.20.x

Goal: applications consume native M-Time instead of treating UTC as the conceptual core.

### Todo

- [ ] Make Hijri engine accept `MTimeState`.
- [ ] Make worship-event engine accept `MTimeState`.
- [ ] Preserve observer geometry and atmospheric inputs separately.
- [ ] Add local solar-time view derived from native state.
- [ ] Add criterion margin + uncertainty output.
- [ ] Add profile-version provenance to every result.
- [ ] Add cross-jurisdiction ExplainDifference from one physical state.
- [ ] Demonstrate Ramadan/Shawwal/Dhulhijjah workflows from M-Time state.
- [ ] Add no-hidden-UTC-assumption tests.
- [ ] Keep official authority decisions separate from computed candidates.

Exit gate: flagship calendar/worship demos can begin from M-Time state and explain every policy transformation.

---

## M14 — Observation & Authority Trust Network

Target: v0.21.x

Goal: connect physical/cyclic computation to real observations without collapsing evidence into authority.

### Todo

- [ ] Version observation packet format.
- [ ] Add instrument/site calibration metadata.
- [ ] Add signed image/report hashes.
- [ ] Add observer/institution key registry.
- [ ] Add revocation and expiry handling.
- [ ] Add weather/horizon metadata.
- [ ] Add duplicate/conflict detection.
- [ ] Keep observation verdict separate from authority action.
- [ ] Add multi-source authority provenance.
- [ ] Add public audit trail format.

Exit gate: every authority/calendar result can trace back to distinct computational, observational and institutional evidence.

---

## M15 — Independent Reimplementation & Conformance

Target: v0.22.x

Goal: prove the Rust implementation is not self-confirming.

### Todo

- [ ] Freeze MTS/MCLOCK conformance vectors.
- [ ] Build independent second implementation in a different language.
- [ ] Do not share implementation algorithms beyond the public spec.
- [ ] Compare linear coordinates bit-for-bit where defined.
- [ ] Compare cyclic state within declared tolerances.
- [ ] Compare calibration residuals independently.
- [ ] Add differential fuzzing.
- [ ] Add malformed-packet and edge-case corpus.
- [ ] Publish discrepancies rather than hiding them.
- [ ] Invite unaffiliated reproduction.

Exit gate: independent implementation passes the public conformance suite.

---

## M16 — Multi-Century Celestial Falsification

Target: v0.23.x

Goal: test whether the digital machine remains stable outside the 2026 calibration window.

### Todo

- [ ] Expand Sun/Moon calibration across centuries supported by reference ephemerides.
- [ ] Add eclipse epochs as high-information tests.
- [ ] Add lunar-node and perigee-sensitive epochs.
- [ ] Add ancient/modern epoch partitions.
- [ ] Track extrapolation error vs time from calibration interval.
- [ ] Define supported temporal validity range.
- [ ] Fail closed outside the validated range.
- [ ] Compare historical vs digital profiles separately.
- [ ] Publish long-horizon residual maps.
- [ ] Add regression gates for worst-case drift.

Exit gate: M-Time publishes a validated temporal domain and explicit out-of-range behavior.

---

## M17 — Revelation Temporal Ontology

Target: v0.24.x

Goal: preserve the project's scriptural motivation without contaminating physical measurement.

### Todo

- [ ] Build versioned temporal-concept ontology for Qur'anic references.
- [ ] Record original-language text reference IDs and translation provenance.
- [ ] Classify concepts: day/night, Sun, Moon, phases, months, years, reckoning, appointed times.
- [ ] Add interpretation notes as non-numerical semantic metadata.
- [ ] Enforce no-dependency path from scientific crates to revelation.
- [ ] Add scholar-review fields without treating review as physics evidence.
- [ ] Add comparative revelation layer only as separately sourced research.
- [ ] Add tests proving ontology cannot alter numerical clock state.
- [ ] Expose references through ExplainTime rather than core computation.
- [ ] Publish boundary statement.

Exit gate: revelation is useful to explain design semantics but cannot change an empirical result.

---

## M18 — Cosmological Chronology Sandbox

Target: v0.25.x research-only

Goal: continue the original cosmic-age research goal without destabilizing operational time.

### Todo

- [ ] Keep `mtime-cosmology` dependency-isolated from operational clock crates.
- [ ] Add multiple cosmological model profiles.
- [ ] Represent posterior/model uncertainty explicitly.
- [ ] Add reproducible age-of-universe inference notebooks/tests.
- [ ] Compare model dependence rather than publish one absolute age.
- [ ] Define CosmicEpoch mappings as research metadata only.
- [ ] Prevent cosmology updates from changing MTS/MCLOCK state.
- [ ] Document philosophical vs operational-time distinction.
- [ ] Add independent reference datasets.
- [ ] Publish falsification conditions for cosmology claims.

Exit gate: cosmic chronology can evolve independently with zero effect on operational M-Time.

---

## M19 — Public SDK, Protocol & Interoperability

Target: v0.30.x

Goal: make M-Time usable by external software without adopting hidden assumptions.

### Todo

- [ ] Stable Rust API surface.
- [ ] C ABI or portable FFI.
- [ ] JavaScript/WASM SDK.
- [ ] Python SDK/reference implementation.
- [ ] MCLOCK/MTS JSON schemas.
- [ ] NTP/PTP/GNSS adapter design.
- [ ] Signed bundle distribution format.
- [ ] Backward-compatibility policy.
- [ ] API threat/security review.
- [ ] Public conformance test runner.

Exit gate: an external application can consume M-Time while knowing profile/version/uncertainty/provenance.

---

## M20 — Pre-Standard / M-Time 1.0 Exit Program

Target: v1.0.0 only after gates pass

Goal: decide whether M-Time is mature enough to call itself a stable temporal system.

### Mandatory v1.0 gates

- [ ] Software Antikythera core specification frozen.
- [ ] Historical reconstruction evidence registry independently reviewed.
- [ ] Digital correction architecture independently reviewed.
- [ ] Long-horizon accuracy domain published.
- [ ] Uncertainty engine complete.
- [ ] Native M-Time specification stable.
- [ ] MCLOCK protocol stable.
- [ ] Physical M-Clock prototype measured.
- [ ] Independent second implementation passes.
- [ ] Scientific regression/falsification corpus passes.
- [ ] Security/provenance review passes.
- [ ] Calendar/worship layer demonstrates transparent profile separation.
- [ ] Revelation boundary review passes.
- [ ] No claim of replacing UTC/BIPM/IERS/JPL without external standards process.
- [ ] Public RFC/specification and governance process published.

Exit gate: **v1.0 is blocked until all mandatory gates are independently evidenced.**

---

# Priority chain

Do not execute milestones merely by version number. The critical dependency chain is:

```text
M6 residual understanding
  ↓
M8 correction architecture
  ↓
M10 uncertainty
  ↓
M9 stable native specification
  ↓
M15 independent implementation
  ↓
M11/M12 physical clock + metrology
  ↓
M20 v1.0 decision
```

M7, M13, M14, M16, M17, M18 and M19 can advance in parallel where they do not violate this chain.

# Definition of project success

The goal is not to make a visually impressive astronomical clock.

Success means:

```text
Software Antikythera is an independent, explainable cyclic machine
        +
native M-Time has explicit linear/cyclic semantics
        +
modern references can falsify/calibrate it
        +
M-Clock can realize it physically with a measured error budget
        +
applications can use it without hidden calendar/religious assumptions
        +
an independent implementation reproduces it
```

Only after those conditions are met should M-Time consider a stable v1.0 claim.
