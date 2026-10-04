# M-Time v0.3.0 — Provider-Wired Policy Context

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE PROVIDER-WIRED RESEARCH PROTOTYPE**

## Increment over v0.2.2

### 1. Provider-derived Diyanet policy context

M-Time can now derive the additional represented Diyanet conditions from typed providers rather than requiring callers to manually fill policy booleans.

The flow is:

```text
threshold-passing candidate sites
→ AmericasMainlandProvider
→ mainland condition

computed conjunction instant (UT1)
+ WellingtonFajrProvider
→ conjunction-before-fajr condition

threshold result
+ provider-derived additional conditions
→ complete represented policy result
```

Unknown geospatial or worship-time evidence remains `UNKNOWN`.

### 2. Explicit conjunction-before-fajr comparison

`mtime-worship` now provides a strict UT1 comparison helper for conjunction vs a typed Fajr event. Equality does not satisfy "before".

### 3. Delivery automation hardening

The dev→main release pipeline now evaluates the complete unreleased range `origin/main..HEAD` for SemVer/changelog generation, so staged `[skip version]` commits are still represented in the next release. After promotion, `dev` is fast-forwarded to the release merge.

A main-history guard also checks that pushes to `main` follow the dev→main merge topology.

## Gates

- Rust workspace tests: **80 passed / 0 failed**.
- Rust/Python compatibility: **PASS**.
- Clippy: **PASS**.
- Layering invariant: **PASS**.
- Existing high-precision astronomy/reference gates remain part of the post-release validation set.

## Scientific / policy boundary

v0.3.0 does not ship a production global landmass polygon dataset and does not hard-code a universal Wellington fajr angle. Those inputs remain explicit, versioned provider responsibilities.

The official Diyanet 1447/2026 statement represented by this profile requires the 5°/8° visibility criteria together with qualifying visibility on North/South American mainland and conjunction before Wellington imsak.
