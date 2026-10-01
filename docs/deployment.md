# Deployment Status

## GitHub Pages

**Status: DEPLOYED**

Successful Pages workflow:

- Run: `36826609111`
- Commit: `7fb2f05003c62522aeb6ac3ec632a7d52040da31`
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
