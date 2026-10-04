# M13 Native M-Time Application Demos

Two executable examples prove the native application path.

## Hijri criterion

```bash
cargo run -p mtime-hijri --example native_mtime_calendar
```

The example begins with `MTimeEngine::digital()`, binds an independently produced astronomical geometry state, evaluates the versioned MABIMS profile, and prints signed criterion margins.

Observation and authority remain explicitly separate.

## Worship / fasting

```bash
cargo run -p mtime-worship --example native_fasting
```

The example binds explicit UT1 solar events to native M-Time states and a versioned worship profile.

The demo intentionally does not infer UT1 solar geometry from the Antikythera dials.
