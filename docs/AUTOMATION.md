# M-Time Dev/Main Automation

Only `dev` and `main` are part of the active delivery flow.

## Branch roles

- `dev` is the development branch.
- `main` is the promoted release branch.
- The automated pipeline never targets any other branch.

## Automatic release flow

Every non-bot push to `dev` runs `.github/workflows/dev-pipeline.yml`:

1. Determine the SemVer bump.
2. Update `[workspace.package].version` in the root `Cargo.toml`.
3. Prepend an entry to `CHANGELOG.md`.
4. Commit the version bump back to `dev`.
5. Run rustfmt, workspace tests, clippy, the layering invariant, the Python reference, and a WASM release build.
6. Create or update a `dev -> main` release pull request.
7. Merge the PR automatically when those gates pass.
8. Create tag `vX.Y.Z` on the merge commit.
9. Create a GitHub Release with generated notes.
10. Dispatch compatibility, WASM, IERS, JPL, SPK, topocentric reference/matrix, and Pages workflows on `main`.

## SemVer rules

- `BREAKING CHANGE:` or a Conventional Commit header with `!` -> major.
- `feat:` -> minor.
- Everything else -> patch.
- `[release:major]`, `[release:minor]`, or `[release:patch]` overrides automatic detection.
- Manual workflow dispatch can force `patch`, `minor`, or `major`.

All crates inherit `version.workspace = true`, so one root workspace version bump updates the whole Rust workspace logically.

## Bootstrap / internal escape hatch

`[skip version]` prevents the automatic release job for a push. It is reserved for automation/bootstrap commits and is also used on the bot-generated version commit to prevent recursion.

## Main protection

The workflow only auto-promotes `dev` to `main`. Repository-level branch protection is still recommended so humans cannot accidentally push directly to `main`; that setting is separate from these workflow files.
