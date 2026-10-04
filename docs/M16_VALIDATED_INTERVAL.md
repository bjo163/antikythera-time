# M16 Validated Interval Contract

The digital profiles have a fail-closed validated API:

`AntikytheraMachine::state_at_tt_validated`

Current research interval:

```text
start inclusive: 1850-01-01 TT (JD 2396758.5)
end exclusive:   2150-01-01 TT (JD 2506331.5)
```

This interval corresponds to the M16 1850–2149 monthly falsification program using DE440s.

The unrestricted `state_at_tt` remains available for research/extrapolation, but callers that require a validated-domain guarantee must use `state_at_tt_validated`.

Historical reconstruction has no modern-accuracy validated interval and therefore returns `NoValidatedInterval` from the validated API.
