# M-Time v0.2.2 — Policy, Oracle-Matrix & Integrity Hardening

Date: 2026-10-03

Status: **INTERNALLY REPRODUCIBLE HIGH-PRECISION RESEARCH PROTOTYPE**

## Increment over v0.2.1

### 1. Diyanet policy execution
The current profile can now execute represented conditions beyond the 5°/8° visibility component:

- visibility/ru'yet condition on North or South American mainland;
- conjunction-before-Wellington-fajr condition.

Unknown context stays UNKNOWN instead of being serialized as a complete decision.

### 2. Broad topocentric reference matrix
Nine fixed oracle cases now compare M-Time directly with JPL Horizons:

- Ramadan / Shawwal / Dhulhijjah 1447 boundary epochs;
- Jakarta / Ankara / Makkah.

All 9 cases pass the 0.001° physical sky-direction gate.

Maximum direction residual observed:

```text
~0.000257° ≈ 0.93 arcsec
```

A near-zenith Makkah case demonstrated that raw azimuth difference is not a stable physical error metric near the zenith. The gate was corrected to spherical horizon-direction separation while retaining the same 0.001° physical threshold.

### 3. Trusted signing-key registry
Source integrity now includes:

- Ed25519 public-key registry;
- institution and key ID;
- validity windows;
- revocation state;
- duplicate-key prevention.

### 4. Local horizon
The astronomy layer now supports:

- surveyed horizon obstruction by azimuth;
- circular interpolation;
- geometric observer-height horizon dip;
- explicit horizon clearance.

These are kept separate from the underlying astronomical coordinate and calendar criterion.

## Gates

- Rust CI: **74 passed / 0 failed** — run `37054820440`.
- Rust/Python compatibility: **PASS** — run `37054820611`.
- 9-case Horizons oracle matrix: **PASS** — run `37054820402`.
- Existing SPK / topocentric / WASM / historical-corpus gates remain intact.

## Scientific boundary

M-Time v0.2.2 is not:

- a replacement for UTC/BIPM/IAU/IERS/JPL;
- a declaration that one Hijri methodology is religiously superior;
- proof that a signature establishes truth;
- a production-ready global atmospheric/horizon model;
- an adopted standard.

It is a stronger research prototype whose flagship temporal-interoperability path now has broader external astronomy validation and more complete policy/integrity semantics.
