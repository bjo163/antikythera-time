# U-Time 2.0.0 Release Record

Date: 2026-10-02

Status: **STANDARDIZATION_CANDIDATE**

## Final gates

- package/protocol version: `2.0.0`;
- full automated test gate: **103/103 PASS**;
- final test run: `36914542152`;
- JavaScript/Python/Rust compatibility: **PASS**;
- IERS EOP ingestion: **PASS**;
- SOFA reference vectors: **PASS**;
- ERFA TDB−TT benchmark: **PASS**;
- NASA Saros recurrence validation: **PASS**;
- NASA published Besselian-element evaluator: **PASS**;
- JPL planetary benchmark: **PASS**;
- official DESI posterior reproduction: **PASS**;
- conformance corpus: **PASS**;
- GitHub Pages v2 dashboard: **SUCCESS**.

## Standardization bundle

Workflow: `v2-release`

Successful run: `36914464594`

Artifact:
- name: `utime-2.0.0-standardization-candidate`
- artifact ID: `11189056329`
- artifact ZIP size: `1,235,757 bytes`

Candidate tarball SHA-256:

```text
adc96cd9744c9f3c0110fc7a746e4e4db82d07443383b66991ea74b12544bc7c
```

The release workflow also produces a per-file SHA-256 manifest.

## Normative assets

- `spec/UTIME-2.0.md`
- `spec/utime-v2.schema.json`
- `spec/v2-conformance-corpus.json`

## Implementations

- JavaScript primary implementation;
- independent Python numerical checker;
- independent Rust numerical checker.

## Scientific boundary

This software/protocol version is complete as an **internal standardization candidate**.

It is not an adopted international standard. Formal external peer review, unaffiliated implementation and standards-body/community adoption remain external processes.
