# M-Time Bootstrap Status

Status: Rust-first vertical-slice implementation on staging branch `m-time-bootstrap`.

## Proven complete in CI

- Rust Cargo workspace compiles/tests.
- Core integer duration and two-part coordinate time.
- Typed time scales / reference frames.
- Antikythera cycle primitives and evidence categories.
- MABIMS Indonesia 2026 threshold profile.
- Hijri astronomical-state schema.
- Observation / jurisdiction / authority records.
- TemporalResolution separation.
- ExplainDifference causal categories and plain-language output.
- Historical Syawal 1447 H Indonesia representation.
- IERS finals.all IAU2000 Rust ingestion against live official product.
- Cosmology inference remains a separate type and rejects Antikythera→Big-Bang semantics.
- Revelation ontology has `NUMERICAL_INFLUENCE=false`.
- WASM binding skeleton.
- Independent JavaScript and Python golden checks.

## Deliberately incomplete

These are not hidden:
- no production-grade local Sun/Moon ephemeris yet;
- no independent conjunction/sunset solver yet;
- no topocentric hilal geometry validation against JPL yet;
- no official rukyat ingestion pipeline yet;
- no 20-case historical corpus yet;
- no standalone `m-time` GitHub repository because the active connector cannot create repositories.

## Next scientific gate

The next milestone is not another schema. It is:

```text
offline/validated Sun+Moon provider
→ conjunction
→ local sunset
→ topocentric altitude
→ geocentric elongation
→ JPL/observatory validation
→ Hijri resolution
```
