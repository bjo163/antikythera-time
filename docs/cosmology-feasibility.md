# Scientific Feasibility: Cosmic Chronology Engine

## Research question

Can U-Time's computational/validation architecture be extended into a model-dependent cosmic chronology engine?

## Result

**YES, with strict semantic separation.** The reusable idea is computational architecture, not an Antikythera astronomical period and not a civil/relativistic time scale.

```text
observations -> cosmological parameters -> E(a) -> Friedmann age integral -> age distribution
```

For scale factor `a`:

```text
t0 = H0^-1 ∫[0,1] da / (a E(a))
```

The implementation integrates after the deterministic substitution `a=x²`, which regularizes the supported early-time matter/radiation behavior at the lower endpoint.

## ΛCDM

```text
E(a)^2 = Ωr a^-4 + Ωm a^-3 + Ωk a^-2 + ΩΛ
```

Flat mode sets Ωk=0. Inconsistent flat inputs are never silently renormalized; a declared consistency diagnostic warns or rejects.

## CPL w0waCDM

DESI DR2 uses the Chevallier-Polarski-Linder form:

```text
w(a) = w0 + wa(1-a)
```

Integrating the energy-conservation equation `dρ/da = -3(1+w(a))ρ/a` gives:

```text
ρDE(a)/ρDE,0 = a^[-3(1+w0+wa)] exp[-3 wa (1-a)]
```

and therefore:

```text
E(a)^2 = Ωr a^-4 + Ωm a^-3 + Ωk a^-2
       + ΩDE a^[-3(1+w0+wa)] exp[-3 wa (1-a)]
```

This is implemented independently rather than copied as an unchecked prompt formula.

## Why phase is not age

A periodic state satisfies `θ(t)=θ(t+nT)`. Measuring phase determines a state modulo the period; it cannot determine the integer cycle count `n`. Antikythera therefore supports recurrence computation, not a Big-Bang timestamp.
