# M-Time Implementation Status

Date: 2026-10-02  
Version: **v0.1 Research Prototype**  
Internal status: **END-TO-END PROTOTYPE COMPLETE**

M-Time is a Rust-first temporal interoperability framework. This status does **not** claim production astronomical accuracy, international standard adoption, or religious/fiqh authority.

## Internal gates

| Gate | Result | Reference |
|---|---|---|
| Rust workspace CI | PASS | Actions run 36925573620 |
| Rust tests | **45 passed / 0 failed** | same run |
| Rust↔Python compatibility | PASS | Actions run 36925573662 |
| WASM build | PASS | Actions run 36925431257 |
| Public Pages build/deploy | PASS | Actions run 36925431295 |
| IERS EOP ingestion workflow | PASS | Actions run 36923484500 |
| JPL Horizons reference workflow | PASS | Actions run 36924790531 |
| Indonesia 1447 H replay tests | PASS | core CI |
| Revelation no-numerical-prior invariant | PASS | core CI |
| Planck-like cosmology inference | PASS | core CI |

## Implemented end-to-end layers

### 1. Physical / coordinate time
- integer/typed temporal primitives;
- explicit time scale/frame semantics;
- UTC/TAI/TT over a declared leap-second support window;
- TT↔TCG and TDB↔TCB canonical transforms;
- explicit TT↔TDB dtr provider contract;
- compact approximate geocentric dtr provider;
- SOFA-style reference-vector tests.

### 2. Earth orientation / observer
- IERS finals.all parser;
- observed-vs-predicted EOP evidence;
- interpolation with outside-coverage rejection;
- UT1−UTC handling;
- Earth observer semantics.

### 3. Antikythera computational grammar
- cycle/ratio/recurrence primitives;
- Metonic/Saros/Exeligmos and lunar-cycle concepts;
- evidence-labelled historical reconstruction boundary;
- explicit rule that periodic cycles do not determine cosmic age.

### 4. Astronomy / ephemeris abstraction
- Sun/Moon state interfaces;
- equatorial/ecliptic semantics;
- angular separation;
- observer horizon geometry;
- JPL Horizons parsers;
- strict distinction between topocentric altitude and geocentric center-to-center elongation.

### 5. Hijri resolution
- `HijriAstronomicalState`;
- geometry-semantics guard;
- versioned calendar profiles;
- MABIMS Indonesia 2026 profile;
- month-completion action;
- final `CalendarResult` as a first-class record.

### 6. Rukyat / authority
- observation-report schema;
- observation summary;
- jurisdiction;
- authority decision;
- computed / observed / official layers remain separate.

### 7. ExplainDifference
- compares temporal resolutions;
- preserves shared astronomy;
- identifies criterion / observation / authority / jurisdiction / source differences;
- provides a plain-language causal explanation.

### 8. Worship time
- versioned solar-threshold profile;
- bracketed solar-altitude crossing solver;
- fasting-window record;
- prayer-time/lunar-month semantics remain separate.

### 9. Historical real-world validation
The Indonesia 1447 H pilot corpus covers:
- Ramadan 1447 H;
- Syawal 1447 H;
- Zulhijjah 1447 H.

The tests preserve the invariant:

`profile result != observation report != official authority decision`.

### 10. Cosmology
- flat/curved/CPL-ready model structure;
- adaptive Friedmann age integration;
- Planck-like reference test;
- cosmic age explicitly marked model-dependent, not an absolute cosmic clock.

### 11. Revelation ontology
- Qur'an 2:189, 10:5, 55:5, 21:33 conceptual nodes;
- Genesis 1:14;
- Psalm 104:19;
- Mark 13:32;
- all are TEXTUAL_REFERENCE / CONCEPTUAL;
- invariant: no textual reference supplies a numerical scientific prior.

### 12. Distribution
- CLI demo;
- Rust↔Python compatibility checker;
- WASM crate;
- public web UI / GitHub Pages workflow.

## Flagship problem status

The original flagship question is now representable:

> Why can two communities/authorities produce different Hijri calendar outcomes even when observing the same physical universe?

M-Time can represent and compare:
- physical/astronomical input;
- ephemeris source;
- observer semantics;
- calendar criterion;
- actual observation/rukyat;
- jurisdiction;
- authority decision;
- final calendar outcome.

This is the first concrete completion of the original M-Time mission.

## Internal scope now considered complete

The repository has enough architecture and executable behavior to stop treating M-Time as a conceptual prototype. The next work is **hardening and external validation**, not another redesign.

## Open scientific / production gates

These remain deliberately **OPEN**:

1. **High-precision offline Sun/Moon ephemeris**  
   Current architecture can validate against JPL, but M-Time does not yet ship a production-grade integrated offline lunar/solar ephemeris.

2. **Production topocentric parallax/refraction pipeline**  
   Geometry semantics are explicit, but a full high-accuracy atmospheric/refraction and lunar-parallax chain still requires dedicated validation.

3. **Signed external ingestion**  
   Observation and authority schemas exist; authenticated/signed machine-to-machine ingestion from external authorities/observatories is not yet productionized.

4. **Broader historical corpus**  
   Indonesia 1447 H is a successful pilot, not yet a multi-country, multi-decade validation dataset.

5. **Independent external review**  
   Falak experts, religious/calendar authorities, historians of astronomy, metrologists, and textual scholars have not yet independently reviewed/adopted the protocol.

6. **Formal standardization**  
   M-Time is not an IAU/BIPM/ISO or religious-authority standard. That cannot be completed by repository code alone.

## Next-stage principle

Do not expand features merely to increase version numbers.

Priority order:
1. astronomical accuracy;
2. real authority/observation provenance;
3. multi-country Hijri disagreement replay;
4. third-party implementation;
5. peer review;
6. standardization discussion.

See GitHub Issues #8–#17 for the M-Time epic structure.
