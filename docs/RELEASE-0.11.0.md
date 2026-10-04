# M-Time v0.11.0 — Multi-Country Historical Falsification

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE 3-COUNTRY SOURCE-BACKED HISTORICAL FALSIFICATION RESEARCH PROTOTYPE**

## Increment over v0.10.0

v0.11 adds a machine-readable historical corpus covering Indonesia, Singapore and Malaysia across 2024–2026 / 1445–1447 H.

### Corpus

```text
21 real official-source cases
3 jurisdictions
3 civil years
3 Hijri years
+ 3 controls
```

### Gate

```text
12 REPRODUCED
2 FALSIFIED controls
10 INCOMPLETE
```

Nine of the incomplete cases are real historical records whose official source does not expose enough represented criterion evidence. M-Time deliberately keeps those cases incomplete instead of converting an official date into a scientific/policy replay pass.

### Divergence preservation

Two 2025 Indonesia–Singapore one-day differences are explicit machine records:

```text
Ramadan 1446:
ID 2025-03-01
SG 2025-03-02

Dhulhijjah 1446:
ID 2025-05-28
SG 2025-05-29
```

### Live Lab

The public GitHub Pages lab reads the v0.11 JSON corpus directly and reports corpus size and verdict distribution.

### Regression gates

- Rust workspace: **103 passed / 0 failed**;
- Rust/Python compatibility: PASS;
- source-backed historical corpus: PASS;
- legacy historical falsification: PASS;
- prior Moon/Sun oracle, geospatial, Wellington and source-ingestion gates remain in the release pipeline.

## Boundary

v0.11 is multi-country and multi-year, but not yet multi-decade or independently reproduced by an unaffiliated implementation.
