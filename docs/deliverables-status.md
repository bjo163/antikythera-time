# Deliverables Status

Scope source: Cosmic Chronology research brief.

| Deliverable | Status | Repository output |
|---|---|---|
| 1. Repository audit | COMPLETE | `docs/repository-audit.md` |
| 2. Scientific feasibility report | COMPLETE | `docs/cosmology-feasibility.md` |
| 3. Architecture decision record | COMPLETE | `docs/adr-001-cosmology-separation.md` |
| 4. Cosmology engine | COMPLETE | `src/cosmology/**` |
| 5. ΛCDM engine | COMPLETE | flat + curved in `models.js`, `age.js` |
| 6. w0waCDM engine | COMPLETE | CPL model in `models.js` |
| 7. Uncertainty engine | COMPLETE | independent, covariance, posterior-chain paths |
| 8. Validation suite | COMPLETE | 55/55 final test gate + official-chain gate |
| 9. APIs | COMPLETE | age, reference, posterior chain |
| 10. Web UI | COMPLETE | separated Cosmic Chronology Lab + static fallback |
| 11. Qur'anic conceptual map | COMPLETE | `docs/quranic-celestial-computation-map.md` |
| 12. Updated README | COMPLETE | root README |
| 13. Updated roadmap | COMPLETE | `docs/long-term-roadmap.md` |
| 14. Test report | COMPLETE | `docs/test-report.md` |
| 15. Reproducibility report | COMPLETE | `docs/reproducibility.md` |
| Claim/result/evidence matrix | COMPLETE | `docs/claim-matrix.md` |
| Quantitative cosmic ages | COMPLETE | `docs/cosmology-age-results.md` |
| Official DESI posterior reproduction | COMPLETE | `docs/official-desi-posterior.md` |

## Public deployment

GitHub Pages is now **COMPLETE**. Successful deploy run: `36826609111`.

Canonical public site: `https://bjo163.github.io/antikythera-time/`

Full server-backed `/api/**` endpoints still require a serverless host such as Vercel.

## External gates that remain

### Scientific standard adoption
A repository cannot self-declare itself a universal standard. Independent implementation, peer review, external reproducibility and standards-community adoption remain external future gates.

### Future research roadmap
The v0.8 relativistic coordinate-time core is now implemented and reference-validated. Remaining roadmap work includes the full automatic TDB−TT provider, eclipse engine, planetary reconstruction, and full historical Antikythera reconstruction. They are not required to make the present cosmological chronology implementation internally complete.


## v0.8 relativistic extension

| Deliverable | Status | Repository output |
|---|---|---|
| Separate coordinate-time type | COMPLETE | `src/relativity/coordinate-time.js` |
| TT↔TCG canonical transform | COMPLETE | `src/relativity/transforms.js` |
| TDB↔TCB canonical transform | COMPLETE | `src/relativity/transforms.js` |
| TT↔TDB canonical wrapper | COMPLETE WITH EXPLICIT dtr | `src/relativity/transforms.js` |
| SOFA reference-vector validation | COMPLETE | `src/relativity/validation.js`, `test/relativity.test.js` |
| Automatic ephemeris-dependent TDB−TT provider | OPEN v0.8.1 | intentionally not approximated |
