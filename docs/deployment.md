# Deployment Status

## Static package

Workflow: `static-site`

Successful run: `36825497347`.

The artifact contains:

```text
index.html
site.js
styles.css
src/**
```

Cosmology calculations have a client-side fallback and therefore work on static hosting.

JPL validation/calibration/residual endpoints under `/api/**` require serverless hosting such as Vercel.

## GitHub Pages

An automated Pages deploy was attempted. GitHub returned:

```text
Resource not accessible by integration
```

when the connected GitHub App attempted to create/enable the Pages site.

Therefore the remaining activation is an owner/repository-admin action:

1. GitHub repository Settings;
2. Pages;
3. set Source to GitHub Actions;
4. rerun/deploy a Pages workflow or publish the packaged static artifact.

The source is already static-host compatible.

## Vercel

The repository includes `vercel.json` and serverless functions in `api/`.

The active Vercel connector in this session did not expose a working project-creation/deploy action for this repository. Importing `bjo163/antikythera-time` as a Vercel project is therefore an account-level action.

No deployment token or secret is committed to the repository.
