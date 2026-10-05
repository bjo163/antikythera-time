# External Reviewer Quickstart

Current frozen target is defined by `reviews/external/requirements.json`.

At the time of this guide:

- candidate: `m20-review-candidate-3`
- release baseline: M-Time v0.16.0
- Git SHA: `f5a14d8d00367117a6ae9f98ce6cac25f2f262e9`
- scope: software-only

This guide is for real external reviewers and independent reproducers. It does not turn a project-team review into external evidence.

## 1. Choose one review lane

- #39 — unaffiliated independent reproduction
- #40 — external software/security review
- #41 — external scientific review
- #42 — external historical Antikythera reconstruction review
- #43 — external revelation/textual-boundary review

Each lane has a packet under `reviews/external/packets/`.

You may review more than one lane, but each submitted review record remains category-specific.

## 2. Build the frozen review kit

From a clone of the repository:

```bash
git fetch --all --tags
bash scripts/build_review_kit.sh
```

Default output:

```text
/tmp/mtime-m20-review-kit/
/tmp/mtime-m20-review-kit.tar.gz
/tmp/mtime-m20-review-kit.tar.gz.sha256
```

The builder reads the exact candidate release/SHA from `reviews/external/requirements.json`, creates a detached worktree at that SHA, and packages only the public review inputs.

Verify the extracted kit:

```bash
python3 /tmp/mtime-m20-review-kit/verify-kit.py
```

The kit contains `SHA256SUMS` and `CANDIDATE.txt`.

## 3. Independence rule for #39

If you are doing unaffiliated independent reproduction:

- implement from the written specs and kit vectors;
- use a separate repository;
- do not copy, translate, bind, import or call M-Time implementation code;
- do not use the repository's Python reference implementation as your implementation;
- declare `shared_project_code = false`.

For a stronger independence claim, work from the extracted review kit and avoid reading `crates/**` entirely.

Matching vectors is a reproducibility result, not by itself a scientific validation.

## 4. Volunteer / coordinate

Open the GitHub issue form:

`.github/ISSUE_TEMPLATE/external-review-volunteer.yml`

The volunteer issue is coordination only. It never counts as a PASS verdict.

Do not publish secrets, private credentials or confidential employer information.

## 5. Produce the review

Your report should state:

- reviewer identity and organization;
- relationship to the project and conflicts of interest;
- exact candidate release + Git SHA;
- methodology and environment;
- every required topic from the category packet;
- findings, including negative or inconclusive findings;
- final verdict: PASS, PASS_WITH_FINDINGS, FAIL or INCONCLUSIVE.

Do not weaken a threshold merely to obtain PASS.

## 6. Submit evidence

Use:

- `reviews/external/review-template.json`
- `reviews/external/review.schema.json`

Place report files under `reviews/external/reports/` and the JSON intake record under `reviews/external/submissions/`.

SHA-256 pin every report/supporting artifact referenced by the JSON record.

A submission starts as `SUBMITTED_UNREVIEWED`. Repository evidence admission is a separate step, and the external reviewer cannot self-admit the record.

## 7. Local validation

Before opening a PR:

```bash
python3 scripts/review_packet_readiness.py
python3 scripts/external_review_readiness.py
python3 scripts/v1_readiness.py
```

A valid external report may still produce FAIL, INCONCLUSIVE or PASS_WITH_FINDINGS. Those outcomes remain evidence and are not deleted to make readiness green.

## What is currently blocking v1

Only the five external software evidence lanes above.

There is no hardware/device/metrology v1 requirement.
