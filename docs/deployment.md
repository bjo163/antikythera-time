# Deployment Status

## GitHub Pages

**Status: DEPLOYED**

Successful v1 Pages workflow:

- Run: `36831062744`
- Commit: `69539b0ffb8fdd0e44de6481dde123b4036c2174`
- Conclusion: `success`

Canonical site URL:

```text
https://bjo163.github.io/antikythera-time/
```

The deployed static site contains:

```text
index.html
site.js
styles.css
src/**
```

Cosmology calculations have a client-side fallback and therefore work directly on GitHub Pages.

## Server-backed APIs

JPL validation/calibration/residual endpoints under `/api/**` still require serverless hosting such as Vercel.

The GitHub Pages site remains useful as the public static research dashboard, while Vercel remains the preferred future target for the full API-backed experience.

## Security

No deployment token or secret is committed to the repository.
