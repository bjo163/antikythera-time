# M-Time Protocol Draft 0.1

Status: research draft.

Core entities: `PhysicalInstant`, `DurationSI`, `Observer`, `AstronomicalState`, `CalendarProfile`, `CriterionResult`, `ObservationReport`, `Jurisdiction`, `AuthorityDecision`, `CalendarDate`, `CosmicInference`, `TextualReference`.

Mandatory separations:
- AstronomicalState != CriterionResult
- CriterionResult != ObservationReport
- ObservationReport != AuthorityDecision
- AuthorityDecision != PhysicalTruth
- CosmicInference != CoordinateTime
- TextualReference != MeasuredScientificParameter

Evidence states: OBSERVED / MEASURED / CALCULATED / MODELED / INFERRED / RECONSTRUCTED / SPECULATIVE / TEXTUAL_REFERENCE.

Quality classes: REFERENCE / HIGH_PRECISION / APPROXIMATE / RECONSTRUCTION / CONCEPTUAL.

MABIMS profile draft: topocentric Moon altitude >= 3 degrees and geocentric elongation >= 6.4 degrees; month-completion and observation/authority remain separate layers.
