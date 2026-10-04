# M-Time Long-Term Roadmap — Software-Only Scope

Status baseline: post-v0.15.0 scope reset.

Core goal:

```text
Antikythera computational idea
        ↓
Software Antikythera
        ↓
native M-Time temporal system
        ↓
software protocols / SDK / Live Lab
        ↓
calendar / worship / observation / authority applications
```

M-Time is a **software system**. Physical clock hardware, embedded controllers, GNSS/PPS receivers, oscillators, RTCs, bench prototypes and hardware metrology are outside the active project scope.

Modern references such as JPL DE, IERS, BIPM/UTC and GNSS may be used as calibration, interoperability or falsification references. They must not silently replace the Software Antikythera core.

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
10. A v1.0 claim requires independent software reproduction plus external scientific, security, historical-reconstruction and textual-boundary review.

---

## M6 — Antikythera Accuracy Program I

Goal: characterize current digital residuals instead of hiding them.

Exit gate: every residual can be reproduced from a versioned profile, declared reference frame and calibration artifact.

## M7 — Historical Antikythera Reconstruction Fidelity

Goal: make historical reconstruction auditable rather than a loose approximation.

Exit gate: every historical mechanism claim maps to an evidence label and source record.

## M8 — Digital Antikythera Correction Architecture

Goal: improve precision while preserving the Antikythera-style independent machine.

Exit gate: precision improvements are attributable to explicit, removable, versioned correction terms with out-of-sample validation.

## M9 — M-Time Native Temporal Specification 2.0

Goal: make M-Time a defined temporal coordinate/state system, not only internal structs.

Exit gate: independent serializers can exchange an M-Time state losslessly under the public specification.

## M10 — Uncertainty & Accuracy Budget Engine

Goal: every meaningful M-Time result carries an honest uncertainty basis.

Exit gate: no public precision claim lacks machine-readable uncertainty/provenance.

## M13 — Calendar & Worship on Native M-Time

Goal: applications consume native M-Time instead of treating UTC as the conceptual core.

Exit gate: flagship calendar/worship demos begin from M-Time state and explain every policy transformation.

## M14 — Observation & Authority Trust Network

Goal: connect computation to real observations without collapsing evidence into authority.

Exit gate: every authority/calendar result traces to distinct computational, observational and institutional evidence.

## M15 — Independent Reimplementation & Conformance

Goal: prove the Rust implementation is not self-confirming.

Required work includes public conformance vectors, independent implementation, malformed-input coverage, discrepancy publication and unaffiliated reproduction.

Exit gate: an unaffiliated implementation passes the public conformance suite against the frozen software candidate.

## M16 — Multi-Century Celestial Falsification

Goal: test whether the digital machine remains stable outside the original calibration window.

Exit gate: M-Time publishes a validated temporal domain and explicit out-of-range behavior.

## M17 — Revelation Temporal Ontology

Goal: preserve scriptural motivation without contaminating numerical computation.

Exit gate: revelation remains useful semantic/explanatory metadata and cannot alter empirical numerical state.

## M18 — Cosmological Chronology Sandbox

Goal: continue cosmic-age/chronology research without destabilizing operational M-Time.

Exit gate: cosmology can evolve independently with zero effect on operational M-Time.

## M19 — Public SDK, Protocol & Interoperability

Goal: make M-Time usable by external software without hidden assumptions.

Deliverables include stable Rust API, C ABI, WASM, Python compatibility/reference implementation, MTS/MCLOCK software schemas, signed bundle format, compatibility policy, threat review and public conformance tools.

Exit gate: external software can consume M-Time while knowing profile, version, uncertainty and provenance.

## M20 — Pre-Standard / M-Time 1.0 Exit Program

Goal: decide whether the **software system** is mature enough to call itself stable.

Mandatory v1 gates:

- Software Antikythera core specification frozen.
- Historical reconstruction evidence independently reviewed.
- Digital correction architecture/scientific methodology independently reviewed.
- Long-horizon validated domain published.
- Uncertainty engine complete.
- Native M-Time specification stable.
- MCLOCK retained only as a software state/serialization surface.
- Unaffiliated independent implementation/reproduction passes.
- Scientific regression/falsification corpus passes.
- External security/provenance review passes.
- Calendar/worship layer demonstrates transparent profile separation.
- Revelation/textual-boundary review passes.
- Public RFC/specification and governance process published.
- No claim of replacing UTC/BIPM/IERS/JPL without an external standards process.

Exit gate: **v1.0 remains blocked until all mandatory software/external-review gates are independently evidenced.**

---

# Priority chain

```text
M6 residual understanding
  ↓
M8 correction architecture
  ↓
M10 uncertainty
  ↓
M9 stable native specification
  ↓
M15 unaffiliated reproduction
  ↓
M16 long-horizon falsification
  ↓
M20 external review / v1.0 decision
```

M7, M13, M14, M17, M18 and M19 may advance in parallel where they do not violate this chain.

# Definition of project success

Success means:

```text
Software Antikythera is an independent, explainable cyclic machine
        +
native M-Time has explicit linear/cyclic semantics
        +
modern references can falsify/calibrate it without becoming the core
        +
applications can use it without hidden calendar/religious assumptions
        +
an unaffiliated implementation reproduces it
        +
external reviewers can audit its scientific/security/historical/textual boundaries
```

No hardware realization is required for project completion or v1.0.
