# M20 Pre-Standard / v1.0 Exit Program

M20 is intentionally a **blocking milestone**.

The machine-readable gate file is:

`data/v1-gates.json`

The checker:

`python scripts/v1_readiness.py`

must compute readiness from evidence statuses. The current correct result is:

```text
M-Time v1 readiness: BLOCKED
```

This is a feature, not a failure.

Repository-level software can prepare the specification, protocols, conformance suite and hardware/metrology tooling. It cannot honestly manufacture external scientific review, independent institutional reproduction, or physical PPS/holdover measurements.

The project must remain on 0.x until those gates have real evidence.
