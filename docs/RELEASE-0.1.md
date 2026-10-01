# M-Time v0.1 — Research Prototype Release Record

Date: 2026-10-02

## Release meaning

M-Time v0.1 is the first internally complete end-to-end implementation of the original research architecture:

```text
physical / coordinate time
→ celestial state
→ versioned calendar profile
→ criterion result
→ rukyat observation
→ jurisdiction
→ authority decision
→ calendar result
→ ExplainDifference
```

It also preserves separate Antikythera, cosmology, worship-time and revelation-text layers.

## Verified internal gates

- Rust CI: PASS — run `36925573620`
- 45 Rust tests passed, 0 failed
- Rust/Python compatibility: PASS — `36925573662`
- WASM: PASS — `36925431257`
- GitHub Pages: PASS — `36925431295`
- IERS EOP workflow: PASS — `36923484500`
- JPL reference workflow: PASS — `36924790531`

## Flagship evidence

The Indonesia 1447 H pilot demonstrates three distinct situations:

- Ramadan: MABIMS thresholds fail; no accepted sighting; official month starts after completion rule.
- Syawal: altitude may approach/pass 3° at some locations while national maximum elongation remains below 6.4°; no confirmed sighting; official outcome remains separately recorded.
- Zulhijjah: national minima satisfy both MABIMS thresholds; accepted sightings exist; official decision is still stored as its own authority record.

Agreement does not collapse the layers. Disagreement can be causally decomposed.

## Claim boundary

M-Time v0.1 does **not** claim:
- a new physical second;
- replacement of UTC;
- an absolute universal simultaneity;
- one scientifically mandated fiqh/calendar method;
- that Antikythera calculates the age of the Universe;
- that scripture supplies physical constants;
- production-grade replacement for JPL/SOFA/IERS;
- international or religious-authority standard adoption.

## Release verdict

**Internally complete as a research prototype. Externally unvalidated as a standard.**
