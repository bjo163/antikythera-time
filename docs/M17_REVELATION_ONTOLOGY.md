# M17 Revelation Temporal Ontology

M17 formalizes textual semantics while making numerical non-injection machine-testable.

## Corpus classes

- Qur'an references;
- Torah/Hebrew-Bible comparative witness;
- Psalms comparative witness;
- Gospel comparative witness.

The comparative witnesses are explicitly marked `NeedsExternalScholarReview`; M-Time does not claim that its software metadata resolves textual/theological scholarship.

## Concepts

The ontology classifies semantic concepts such as:

- day/night;
- Sun;
- Moon;
- lunar phases;
- months/years;
- reckoning;
- appointed times;
- celestial motion;
- epistemic limits.

Every V2 entry has:

- stable ID;
- canonical reference;
- source-language identifier;
- translation-provenance rule;
- semantic concepts;
- review status;
- semantic note.

`numerical_physics_value` is required to remain `None`.

A unit test and repository dependency guard enforce that revelation cannot numerically change the physical M-Time state.
