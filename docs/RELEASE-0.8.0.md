# M-Time v0.8.0 — Versioned Geospatial Policy Provider

Date: 2026-10-04

Status: **INTERNALLY REPRODUCIBLE VERSIONED GEOSPATIAL-POLICY RESEARCH PROTOTYPE**

## Increment over v0.7.0

v0.8.0 replaces the placeholder Americas-mainland classifier with a real, versioned geospatial provider.

### Pinned source

```text
Natural Earth ne_110m_land v5.1.2
Git blob SHA-1:
04811d72fff2701ec67587e30ad8942675b511e3

SHA-256:
9e0729ee253ca7d7a5c4ae9395fb1902264c5377c52e224d13dd85010e2835d9
```

### New crate

```text
mtime-geospatial
```

Capabilities:

- version-pinned GeoJSON parsing;
- SHA-256 validation;
- Polygon and MultiPolygon support;
- point-in-polygon classification;
- hole exclusion;
- site registry with explicit coordinates;
- connected-American-mainland selection;
- direct adapter into the Diyanet policy evaluator.

### Live validation

Actions run **37194423011** passes mainland/island/non-Americas classification and Diyanet integration using the pinned dataset.

### Gates

- workspace Rust tests: **98 passed / 0 failed**;
- clippy: PASS;
- layering invariant: PASS;
- Rust/Python compatibility: PASS;
- pinned Natural Earth SHA-256: PASS;
- geospatial classification integration: PASS.

## Boundary

The 110m Natural Earth geometry is deliberately coarse. v0.8.0 closes the placeholder-classifier gap but does not claim meter-level coastline or cadastral precision.
