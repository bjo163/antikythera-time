# M19 Interoperability Adapters

M-Time does not replace transport protocols merely to be different.

```text
NTP / PTP / GNSS-PPS
        ↓
explicit reference adapter
        ↓
TT / TAI / UTC / UT1 as applicable
        ↓
MTS-2
        ↓
Software Antikythera cyclic state
```

Every adapter output must expose protocol/version, upstream source, lock/holdover state when applicable, uncertainty estimate, and leap/EOP data version when relevant.

No adapter may silently infer calendar, worship, observation, or authority state.
