# M-Time v0.7.0 — Historical Falsification

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE HISTORICAL-FALSIFICATION RESEARCH PROTOTYPE**

## Increment over v0.6.0

v0.7.0 adds an executable historical replay/falsification layer.

### Verdict model

```text
REPRODUCED
FALSIFIED
INCOMPLETE
```

The engine refuses to collapse unknown data into success.

### Positive historical replays

```text
Indonesia Ramadan 1447     REPRODUCED
Indonesia Shawwal 1447     REPRODUCED
Indonesia Dhulhijjah 1447  REPRODUCED
ID vs TR Shawwal 1447      REPRODUCED with explicit differing layers
```

### Negative controls

```text
counterfactual authority flip  FALSIFIED
unexplained civil-date flip    FALSIFIED
unknown computation            INCOMPLETE
```

### Gates

Actions run 37193741501:

- historical falsification tests: **7 / 7 PASS**;
- existing Indonesia 1447 regression: **4 / 4 PASS**.

Workspace gate:

- Rust tests: **95 passed / 0 failed**;
- clippy: PASS;
- layering invariant: PASS;
- Rust/Python compatibility: PASS.

## Boundary

The current historical corpus is still small and concentrated in 1447 H / 2026. v0.7.0 establishes the falsification machinery and first executable cross-jurisdiction case; it does not yet satisfy a multi-country, multi-decade external-validation standard.
