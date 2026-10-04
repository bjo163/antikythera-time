# Software Antikythera Core — Phase 2

M-Time now treats the Antikythera computational idea as a first-class core model rather than a decorative validator.

## Two profiles

### Historical reconstruction

`ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1`

This profile keeps historically evidenced/reconstructed cycle relations explicit. It is intentionally not presented as an exact replica because only part of the original mechanism survives and front-cosmos reconstruction remains model-dependent.

### M-Time digital Antikythera

`MTIME_DIGITAL_ANTIKYTHERA_V1`

This is the generalized software descendant. It preserves interlocking solar/lunar cycle state while allowing explicitly modern digital correction terms.

## Core cycles

The machine exposes synchronized state for:

- tropical/solar cycle;
- synodic lunar cycle;
- sidereal lunar cycle;
- anomalistic lunar cycle;
- draconic lunar cycle;
- Metonic cycle;
- Saros cycle;
- Exeligmos cycle;
- lunar node;
- lunar phase.

## Historical relations kept explicit

```text
235 synodic months / 19 years
254 sidereal months / 235 synodic months
223 synodic months / Saros
3 Saros / Exeligmos
node relations: -12/223 and reconstructed compact -5/93 model
```

Evidence labels distinguish surviving evidence, strongly indicated reconstruction, reconstructed models, hypotheses, modern references, and modern digital corrections.

## Virtual mechanism

Software primitives now include:

```text
Cycle
RationalRelation
SignedRelation
VirtualGear
GearMesh
GearTrain
DialState
AntikytheraMachine
AntikytheraState
```

The digital machine is driven by elapsed TT days but produces a multidimensional cyclic state rather than only a scalar timestamp.

## Boundary

JPL, IERS and modern civil time are not dependencies of this crate. They remain outside the Antikythera core and are used by separate calibration/interop layers.

Historical reconstruction references include the surviving/reconstructed Metonic, Saros, lunar-anomaly and front-cosmos research described by Freeth et al. The code does not claim that every digital correction coefficient existed in the ancient device.
