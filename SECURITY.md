# Security and Data Integrity

U-Time handles scientific data and public external endpoints; it does not require secrets for its public reference workflows.

Report security issues privately through the repository owner where possible.

## Data-integrity rules

- do not commit credentials or API tokens;
- downloaded reference datasets must record source/provenance;
- release manifests use SHA-256;
- CI reference failures must not be hidden by silently widening thresholds;
- untrusted conformance JSON is treated as data, not executable code.
