# M-Time — Mīqāt Temporal Protocol

Rust-first temporal interoperability research program.

## Mission

M-Time does not replace the SI second or UTC. It makes time-related statements explicit across:
physical/coordinate time, astronomy, calendar profiles, observations, jurisdiction/authority,
uncertainty, provenance, cosmological inference and textual/conceptual references.

## Flagship acceptance test

Given the same sky state, M-Time must be able to evaluate multiple Hijri profiles and explain
exactly why two calendar outcomes differ without deciding which fiqh/theology is correct.

## Bootstrap scope

This branch implements the first vertical slice:

1. typed temporal/evidence/provenance primitives;
2. Antikythera-inspired cycle grammar;
3. Hijri astronomical-state input schema;
4. versioned MABIMS 3° / 6.4° profile;
5. observation and authority records;
6. explain-difference engine;
7. Rust CI.

Astronomical ephemeris production is intentionally abstracted out at this stage.
