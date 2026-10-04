# Historical Falsification — v0.7.0

M-Time treats historical replay as a falsification exercise, not as a search for confirming examples.

## Verdicts

### REPRODUCED

The represented computation and authority outcome are consistent, or a cross-jurisdiction outcome difference is accompanied by explicit represented causal-layer differences.

### FALSIFIED

A represented claim fails the replay. Examples:

- computed month action conflicts with the recorded authority decision;
- calendar output changes while no represented criterion, observation, jurisdiction, authority, observer, ephemeris, or physical-input layer changes.

### INCOMPLETE

The replay cannot safely decide. Examples:

- computed action is unknown;
- represented policy requires additional rule/observation context that is absent.

M-Time does not convert missing evidence into a pass.

## v0.7 executable corpus

### Indonesia 1447 H

- Ramadan: **REPRODUCED**
- Shawwal: **REPRODUCED**
- Dhulhijjah: **REPRODUCED**

These use the existing source-linked Kementerian Agama pilot corpus and preserve astronomy, observation, and authority as separate layers.

### Indonesia vs Türkiye/Diyanet — Shawwal 1447 H

Official outcomes differ by one civil day.

The executable replay classifies this as **REPRODUCED**, because the represented layers explicitly differ in criterion/profile, observation evidence/scope, jurisdiction, authority, and observer state.

This is not a claim that either religious methodology is superior or that one jurisdiction's local astronomy should replace another's.

## Negative controls

The suite includes deliberately false/incomplete cases:

- counterfactual Indonesia authority reversal → **FALSIFIED**;
- unexplained calendar date flip → **FALSIFIED**;
- unknown computed action → **INCOMPLETE**.

These controls are important: without them, a replay system could accidentally be built to always "explain" whatever outcome it receives.

## Gate

Actions run **37193741501**:

```text
historical_falsification: 7 passed / 0 failed
existing indonesia_1447: 4 passed / 0 failed
```

Machine-readable corpus:

```text
data/hijri/historical-falsification-v0.7.json
```

## Next falsification expansion

The engine is now ready for broader cases. Production-grade historical validation still requires more jurisdictions, more years/decades, source snapshots, and independent review.


## v0.11 source-backed expansion

The v0.11 corpus adds a second historical replay layer for official web sources across Indonesia, Singapore and Malaysia.

This layer is intentionally conservative:

- `MET + begin next day` -> REPRODUCED;
- `NOT_MET + complete to 30` -> REPRODUCED;
- explicit contradiction -> FALSIFIED;
- insufficient published criterion detail -> INCOMPLETE.

It does not infer exact 3° / 6.4° geometry merely from an official date or a qualitative horizon-duration statement.

Coverage:

```text
21 real cases
3 jurisdictions
3 civil years
3 Hijri years
3 negative/incomplete controls
```

Machine gate:

```text
real_cases=21
jurisdictions=3
civil_years=3
reproduced=12
falsified_controls=2
incomplete=10
```

Real incomplete cases are not failures; they document missing evidence required for a safe replay.

See `docs/HISTORICAL_MULTICOUNTRY.md` and `data/hijri/historical-falsification-v0.11.json`.
