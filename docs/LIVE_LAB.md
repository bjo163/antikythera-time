# M-Time Live Lab — GitHub Pages

v0.10.0 upgrades the public static surface from the old v0.2 demo to a current research dashboard.

## Why GitHub Pages is enough now

The public application consists of:

```text
HTML + CSS + JavaScript
+ Rust compiled to WebAssembly
+ public GitHub REST reads
+ static validation/docs/corpus files
```

No server-side secret is required. Therefore GitHub Pages is the simpler deployment target and stays coupled to the repository's `main` commit.

The Pages workflow creates a `build-info.json` containing:

- workspace version;
- deployed commit SHA;
- deployment timestamp;
- branch.

The browser also performs best-effort public GitHub API reads for:

- latest release;
- latest `main` workflow runs.

If public API rate limits are reached, the site falls back to the metrics embedded in the deployed build.

## What the Live Lab shows

- release / commit / WASM version;
- live CI/oracle workflow states;
- 46 current external Horizons checks;
- MABIMS vs Diyanet site-component WASM evaluator;
- Diyanet geospatial + Wellington-fajr provider path;
- source trust semantics;
- historical falsification model;
- links to current validation documents and machine-readable corpus.

## When Vercel becomes useful

Vercel is **not required** for the current public lab.

A server platform becomes useful if M-Time later adds:

- authenticated accounts;
- protected API keys or secrets;
- database-backed observation submissions;
- server-side ephemeris jobs;
- long-running compute;
- scheduled ingestion/monitoring independent of GitHub Actions;
- private institutional connectors.

Until then, GitHub Pages minimizes moving parts while providing a reproducible public build directly from `main`.
