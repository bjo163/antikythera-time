# U-Time Governance

## Status

U-Time 2.0 is an open **standardization candidate**, not an adopted international standard.

## Normative changes

A change is normative when it modifies:

- required record semantics;
- evidence or quality classes;
- schema requirements;
- golden-vector meaning/tolerance;
- transformation requirements;
- conformance criteria.

Normative changes require:

1. a documented rationale;
2. backward-compatibility analysis;
3. updated schema/spec;
4. updated conformance corpus;
5. all language implementations passing;
6. no weakened failing reference threshold solely to obtain a green build.

## Versioning

- PATCH: documentation/non-semantic corrections.
- MINOR: backward-compatible additions.
- MAJOR: incompatible semantic/schema changes.

## Evidence rule

External reference disagreement is data. It must be reported before a model or threshold is changed.

## Authority boundary

BIPM, IAU/IERS/SOFA, NASA/JPL and observational collaborations remain authorities for their respective reference standards/data. U-Time does not override them.

## Adoption

Repository maintainers may publish U-Time versions. Only relevant external standards bodies/communities can confer external standard adoption.
