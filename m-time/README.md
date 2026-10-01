# M-Time — Mīqāt Temporal Protocol

Rust-first temporal interoperability research program.

## Mission

M-Time does **not** invent a new physical second and does **not** replace UTC. It separates and reconnects physical time, astronomical state, calendar criteria, actual observation, jurisdiction, authority decisions, historical reconstruction, cosmological inference, and textual/conceptual references.

Flagship problem: explainable Hijri/worship-time resolution.

## Current vertical slice

```text
Duration / Coordinate semantics
        ↓
Antikythera cycle grammar
        ↓
HijriAstronomicalState
        ↓
versioned CalendarProfile (MABIMS first)
        ↓
CriterionResult
        ↓
ObservationReport
        ↓
AuthorityDecision
        ↓
TemporalResolution
        ↓
ExplainDifference(A, B)
```

## Non-goals

- no absolute cosmic `now`;
- no replacement of SI/UTC/BIPM/IAU/IERS/NASA/JPL;
- no declaration that one fiqh/calendar profile is religiously correct;
- no use of scripture numbers as hidden scientific constants;
- no claim that Antikythera directly computes the age of the Universe.

## Run

```bash
cargo test --workspace
cargo run -p mtime-cli -- conformance
cargo run -p mtime-cli -- mabims --altitude 3.1 --elongation 6.5
```

See `spec/MTIME-0.1.md` and `docs/RESEARCH_CHARTER.md`.
