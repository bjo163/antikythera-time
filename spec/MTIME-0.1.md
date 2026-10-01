# M-Time 0.1 Protocol Draft

Status: **research draft**.

A temporal result MUST carry enough information to answer: what quantity, when, in what scale/frame, for which observer, using which model/profile, with what uncertainty, supported by what evidence/provenance, and — when calendrical — under which observation/jurisdiction/authority context.

## Required separations

- AstronomicalState != CriterionResult
- CriterionResult != ObservationReport
- ObservationReport != AuthorityDecision
- AuthorityDecision != physical fact
- CosmicAgeInference != CoordinateTime
- TextualReference != NumericalScientificPrior

## Hijri result pipeline

physical time -> ephemeris -> observer geometry -> HijriAstronomicalState -> CalendarProfile -> CriterionResult -> ObservationReports[] -> Jurisdiction -> AuthorityDecision -> CalendarResult -> ExplainDifference

## MABIMS Indonesia 2026 profile

- topocentric lunar altitude >= 3°;
- geocentric elongation >= 6.4°;
- if the criterion is not met, complete the current month to 30 days.

Observation/rukyat and the official Sidang Isbat decision remain distinct records.
