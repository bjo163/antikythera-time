# M20-B — Unaffiliated Independent Reproduction Packet

Category: `independent_reproduction`  
Issue: #39  
Candidate: `m20-review-candidate-3`  
Candidate Git SHA: `f5a14d8d00367117a6ae9f98ce6cac25f2f262e9`

## Independence rule

The gate requires an unaffiliated implementation. Do not copy, translate, bind, or call M-Time implementation code under `crates/**`, the C FFI, WASM build, or the repository's Python reference implementation.

Using the public specifications, schemas, frozen test vectors, and malformed-input corpus as test material is allowed.

The external submission must declare `shared_project_code = false`.

## Required topics

- `mts2_wire_conformance`
- `digital_antikythera_state`
- `public_conformance_vectors`
- `malformed_packet_behavior`

## Minimum reproduction

Implement from the public written specifications:

1. Digital Antikythera V1 solar longitude.
2. Digital Antikythera V1 lunar longitude.
3. Moon-Sun phase longitude.
4. lunar-node state.
5. eight normalized cycle phases.
6. signed i128 SI-nanosecond coordinate from J2000 TT.
7. MTS-2 binary decoding/encoding.
8. rejection of malformed/truncated/unknown-layout packets.

Compare the independent outputs to the frozen vector artifact.

## Public inputs

Primary:

- `spec/ANTIKYTHERA-DIGITAL-V1.md`
- `spec/MTIME-2.0.md`
- `spec/RFC-0001-MTIME-1.0.md`
- reviewer-kit `vectors/mts2-conformance.txt`
- reviewer-kit `vectors/malformed-mts2.txt`

The vector file is produced from the frozen candidate as a compatibility oracle. A reviewer should also inspect the equations and not treat matching the project's own oracle as independent proof that the equations are scientifically correct.

## Required report evidence

Report at minimum:

- independent repository URL;
- implementation language/runtime;
- dependency list;
- exact command used for conformance;
- number of valid vectors evaluated;
- numeric tolerance policy;
- binary round-trip result;
- malformed-corpus result;
- any ambiguities encountered in the written specification;
- any case where independent interpretation differs from the candidate output.

A PASS requires all required topics to be covered and the independent conformance result to be PASS.

This lane tests reproducibility of the public computational contract. It is not the external scientific review.
