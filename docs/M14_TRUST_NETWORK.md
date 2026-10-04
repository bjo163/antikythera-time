# M14 Observation and Authority Trust Network

M14 adds versioned audit packets without merging observation and authority.

## Observation packet MOBS-1

Contains report/site/institution identity, optional instrument-calibration ID, optional local-horizon profile, weather summary, attachment hashes and optional source-integrity status.

Unknown calibration/signature metadata stays `None`; it is not invented.

Duplicate report IDs carrying conflicting status/site/evidence are detected.

## Authority audit MAUTH-1

Contains decision/authority/jurisdiction identity, cited profile, cited observation IDs, and optional source hash/signature status.

The resolver can verify that cited observation IDs exist in the provided observation set.

A source signature authenticates the artifact relative to a key registry; it still does not prove the astronomical, legal, or theological conclusion.
