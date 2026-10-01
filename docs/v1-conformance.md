# U-Time 1.0 Conformance Report

Status: **Specification Candidate**

## Normative assets

- `spec/UTIME-1.0.md`
- `spec/utime-v1.schema.json`
- `spec/golden-vectors.json`

## Implementations

### JavaScript reference
Primary repository implementation.

### Independent Python checker
`reference-python/utime_v1.py`

The Python implementation independently computes:

- TT→TCG;
- Planck flat-ΛCDM age integral;
- one-Saros recurrence;
- JPL approximate Mars J2000 vector.

It does not call the JavaScript implementation.

## Compatibility gate

Workflow: `v1-compatibility`

Successful run: `36830567705`.

Checks:

- TT→TCG: PASS;
- Planck age: PASS;
- Saros recurrence: PASS;
- Mars x/y/z: PASS.

Overall: **PASS**.

## Meaning of v1.0

v1.0 freezes the project's protocol contract and reference outputs.

It does **not** mean:

- IAU/BIPM/NASA endorsement;
- international standards adoption;
- superiority to operational NASA/JPL systems;
- completion of every possible astronomical model.

Those are external or future research questions.
