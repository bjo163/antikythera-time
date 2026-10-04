# M13 Native M-Time Calendar and Worship Integration

Calendar and worship layers can now carry a native `MTS-2` identity.

## Hijri

`NativeMTimeHijriInput` binds:

- the native M-Time instant/profile;
- independently computed astronomical geometry.

`CalendarProfile::evaluate_native` evaluates exactly the same versioned criterion as before. It does not convert the Antikythera cycle state into a hidden fiqh rule.

`criterion_margins` exposes signed distance to each threshold.

## Worship

`NativeMTimeSolarEvent` binds a solar event to:

- an M-Time instant;
- an explicit worship profile.

UT1 solar-event geometry remains reference data. M-Time does not pretend UT1 is derived from the Antikythera dial.

This gives the application path:

```text
M-Time physical instant
+
astronomical/observer geometry
+
versioned criterion/profile
→ application result
```
