# Deliverables Status — U-Time 1.0.0

## Core platform

| Deliverable | Status |
|---|---|
| SI/J2000 integer time core | COMPLETE |
| UTC/TAI/TT handling | COMPLETE WITH DECLARED LEAP-TABLE WINDOW |
| TT/TCG canonical transform | COMPLETE |
| TDB/TCB canonical transform | COMPLETE |
| TT/TDB explicit-dtr transform | COMPLETE |
| Automatic geocentric TDB−TT approximation | COMPLETE / APPROXIMATE |
| ERFA external benchmark | COMPLETE / PASS |

## Antikythera / astronomy

| Deliverable | Status |
|---|---|
| Metonic/Saros cycle layer | COMPLETE |
| Lunar calibration/holdout validation | COMPLETE |
| Anomalistic/draconic residual layer | COMPLETE |
| Saros/Exeligmos eclipse recurrence | COMPLETE |
| NASA Saros 139 validation | COMPLETE / PASS |
| Full Besselian eclipse geometry | FUTURE v1.2 |
| JPL approximate classical-planet model | COMPLETE |
| JPL Horizons planetary benchmark | COMPLETE / PASS |
| Full historical planetary gear reconstruction | FUTURE v1.3 |

## Cosmology

| Deliverable | Status |
|---|---|
| Separate CosmicAgeEstimate | COMPLETE |
| flat/curved ΛCDM | COMPLETE |
| CPL w0waCDM | COMPLETE |
| numerical/covariance/posterior uncertainty | COMPLETE |
| official DESI posterior reproduction | COMPLETE / PASS |
| Qur'anic conceptual map outside numerical pipeline | COMPLETE |

## v1 protocol

| Deliverable | Status |
|---|---|
| U-Time 1.0 normative specification | COMPLETE |
| JSON Schema | COMPLETE |
| Golden vectors | COMPLETE |
| Evidence/provenance ontology | COMPLETE |
| JavaScript reference implementation | COMPLETE |
| Independent Python compatibility implementation | COMPLETE |
| Cross-language compatibility gate | COMPLETE / PASS |
| GitHub Pages research dashboard | COMPLETE |

## External gates that remain

### International standard adoption
NOT CLAIMED. Requires external review, multiple third-party implementations and relevant standards-community adoption.

### NASA/JPL replacement
NOT A GOAL and NOT CLAIMED. High-precision ephemerides, SPICE, mission navigation and orbit determination remain reference operational capabilities.

### Full Antikythera digital twin
The existing software adopts documented cycle/computation principles and selected planetary modelling. A fragment-by-fragment, gear-by-gear scholarly reconstruction remains future research.
