# Multi-Country Historical Falsification — v0.11.0

M-Time v0.11 expands historical falsification from a small Indonesia/Türkiye pilot into a source-backed corpus spanning three Southeast Asian jurisdictions and three years.

## Scope

```text
jurisdictions:
- Indonesia (ID)
- Singapore (SG)
- Malaysia (MY)

civil years:
- 2024
- 2025
- 2026

Hijri years:
- 1445
- 1446
- 1447

real cases: 21
controls: 3
```

Events are Ramadan, Shawwal and Dhulhijjah month starts where suitable official sources are available.

## Evidence rule

A published date is not treated as proof that M-Time's represented criterion reproduced the decision.

Each record separately stores:

- jurisdiction;
- event;
- official month start;
- source authority;
- official source URL;
- published/source-asserted criterion status;
- observation status;
- authority action;
- expected replay verdict.

The source-backed audit is deliberately narrow:

```text
criterion MET
+ authority BEGIN NEXT DAY
= REPRODUCED

criterion NOT_MET
+ authority COMPLETE TO 30
= REPRODUCED

criterion/action contradiction
= FALSIFIED

criterion evidence UNKNOWN
= INCOMPLETE
```

## Gate result

Staging Actions run **37197344093**:

```text
real_cases=21
jurisdictions=3
civil_years=3

reproduced=12
falsified_controls=2
incomplete=10
```

The ten incomplete records consist of nine real source-limited cases plus one explicit unknown-evidence control.

No real record is labeled FALSIFIED in this corpus revision.

## Preserved cross-jurisdiction divergences

### Ramadan 1446 H

```text
Indonesia  -> 2025-03-01
Singapore  -> 2025-03-02
```

Indonesia's official source reports that parts of western Indonesia reached the MABIMS threshold and that two sworn Aceh observers reported the crescent.

Singapore's official source reports altitude 4.3° and angular distance 5.1°, explicitly below the agreed MABIMS criterion.

The one-day difference is therefore preserved with local jurisdiction/evidence context.

### Dhulhijjah 1446 H

```text
Indonesia  -> 2025-05-28
Singapore  -> 2025-05-29
```

For Indonesia the authority decision and accepted sighting are recorded, but v0.11 deliberately keeps the source-backed criterion field UNKNOWN rather than extrapolating a national criterion state from a local geometry excerpt.

Singapore's official release completes Zulkaedah to 30 days.

Both source-backed replays therefore retain the evidence limitations instead of inventing a universal conclusion.

## Official source families

Indonesia records use official Kementerian Agama domains.

Singapore records use Majlis Ugama Islam Singapura / Office of the Mufti releases on `muis.gov.sg`.

Malaysia's 1445 H records use the official Keeper of the Rulers' Seal / Conference of Rulers moon-sighting history.

The exact URLs are stored per record in:

```text
data/hijri/historical-falsification-v0.11.json
```

## Boundary

This is not yet a multi-decade independent historical validation.

The next expansion should:

1. snapshot/hash the cited historical sources;
2. add older years under their historically correct criteria;
3. add more jurisdictions;
4. distinguish policy versions by effective date;
5. add independent implementation/review.
