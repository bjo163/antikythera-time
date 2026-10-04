#!/usr/bin/env python3
"""M20 external-review evidence admission and readiness checker.

The machine can validate identity declarations, scope, target commit, artifact
integrity and admission state. It cannot prove that a reviewer is genuinely
independent or that the review judgment is scientifically correct.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
REVIEW_ROOT = ROOT / "reviews" / "external"
REQUIREMENTS_PATH = REVIEW_ROOT / "requirements.json"
SCHEMA_PATH = REVIEW_ROOT / "review.schema.json"
TEMPLATE_PATH = REVIEW_ROOT / "review-template.json"
SUBMISSIONS_DIR = REVIEW_ROOT / "submissions"
V1_GATES_PATH = ROOT / "data" / "v1-gates.json"

EXPECTED_REQUIREMENTS_SCHEMA = "mtime-external-review-requirements-1"
EXPECTED_REVIEW_SCHEMA = "mtime-external-review-1"
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
GIT_SHA_RE = re.compile(r"^[0-9a-f]{40}$")

EXPECTED_CATEGORIES = {
    "independent_reproduction",
    "security",
    "scientific",
    "historical_reconstruction",
    "revelation_textual_boundary",
}

EXPECTED_GATE_IDS = {
    "external-independent-reproduction",
    "external-security-review",
    "external-scientific-review",
    "external-historical-reconstruction-review",
    "revelation-boundary-review",
}

RELATIONSHIPS = {
    "UNAFFILIATED",
    "EXTERNAL_DISCLOSED_RELATIONSHIP",
    "PROJECT_TEAM",
    "UNKNOWN",
}

VERDICTS = {"PASS", "PASS_WITH_FINDINGS", "FAIL", "INCONCLUSIVE"}
STATUSES = {"SUBMITTED_UNREVIEWED", "ADMITTED", "REJECTED"}


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def require_nonempty(value: object, field: str) -> str:
    assert isinstance(value, str) and value.strip(), f"{field} must be non-empty"
    return value.strip()


def validate_sha256(value: object, field: str) -> str:
    digest = require_nonempty(value, field).lower()
    assert SHA256_RE.fullmatch(digest), f"{field} must be lowercase SHA-256 hex"
    return digest


def safe_repo_path(value: object, field: str) -> Path:
    raw = require_nonempty(value, field)
    path = (ROOT / raw).resolve()
    assert ROOT == path or ROOT in path.parents, f"{field} escapes repository root"
    assert path.exists() and path.is_file(), f"{field} does not exist: {raw}"
    return path


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def validate_artifact(ref: object, field: str) -> None:
    assert isinstance(ref, dict), f"{field} must be an object"
    path = safe_repo_path(ref.get("path"), f"{field}.path")
    expected = validate_sha256(ref.get("sha256"), f"{field}.sha256")
    actual = file_sha256(path)
    assert actual == expected, (
        f"{field} SHA-256 mismatch: expected {expected}, got {actual}"
    )


def validate_requirements() -> tuple[dict[str, dict[str, Any]], dict[str, str]]:
    data = load_json(REQUIREMENTS_PATH)
    assert data.get("schema") == EXPECTED_REQUIREMENTS_SCHEMA
    requirements = data.get("requirements")
    assert isinstance(requirements, list) and requirements, "requirements must be non-empty"

    by_category: dict[str, dict[str, Any]] = {}
    gate_to_category: dict[str, str] = {}

    for item in requirements:
        assert isinstance(item, dict), "requirement entries must be objects"
        category = require_nonempty(item.get("category"), "requirements[].category")
        gate_id = require_nonempty(item.get("v1_gate_id"), "requirements[].v1_gate_id")
        assert category not in by_category, f"duplicate review category: {category}"
        assert gate_id not in gate_to_category, f"duplicate v1 gate mapping: {gate_id}"

        relationships = item.get("allowed_relationships")
        assert isinstance(relationships, list) and relationships
        assert set(relationships) <= RELATIONSHIPS
        assert "PROJECT_TEAM" not in relationships
        assert "UNKNOWN" not in relationships

        topics = item.get("required_topics")
        assert isinstance(topics, list) and topics
        assert len(topics) == len(set(topics)), f"duplicate required topic in {category}"
        assert all(isinstance(topic, str) and topic.strip() for topic in topics)

        by_category[category] = item
        gate_to_category[gate_id] = category

    assert set(by_category) == EXPECTED_CATEGORIES, "external review category set mismatch"
    assert set(gate_to_category) == EXPECTED_GATE_IDS, "external v1 gate mapping mismatch"

    gates = load_json(V1_GATES_PATH).get("gates")
    assert isinstance(gates, list)
    actual_gate_ids = {g.get("id") for g in gates if isinstance(g, dict)}
    missing = EXPECTED_GATE_IDS - actual_gate_ids
    assert not missing, f"external review requirements reference missing v1 gates: {sorted(missing)}"

    return by_category, gate_to_category


def candidate_target() -> tuple[str | None, str | None]:
    data = load_json(REQUIREMENTS_PATH)
    candidate = data.get("candidate")
    assert isinstance(candidate, dict), "candidate must be an object"
    release = candidate.get("release")
    git_sha = candidate.get("git_sha")

    if release == "UNSET" and git_sha == "":
        return None, None

    release_text = require_nonempty(release, "candidate.release")
    sha_text = require_nonempty(git_sha, "candidate.git_sha").lower()
    assert GIT_SHA_RE.fullmatch(sha_text), "candidate.git_sha must be a 40-character Git SHA"
    assert release_text != "UNSET", "candidate.release must be pinned when git_sha is pinned"
    return release_text, sha_text


def validate_template() -> None:
    assert SCHEMA_PATH.exists(), "external review JSON schema missing"
    schema = load_json(SCHEMA_PATH)
    assert schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema"
    template = load_json(TEMPLATE_PATH)
    assert template.get("schema") == EXPECTED_REVIEW_SCHEMA
    assert template.get("status") == "TEMPLATE_NOT_REVIEW"
    assert template.get("category") == ""
    assert template.get("admission", {}).get("result") == "NOT_REVIEWED"


def validate_submission(
    path: Path,
    requirements: dict[str, dict[str, Any]],
    pinned_release: str | None,
    pinned_sha: str | None,
) -> tuple[str, str, str]:
    data = load_json(path)
    assert data.get("schema") == EXPECTED_REVIEW_SCHEMA, f"{path}: schema mismatch"

    status = data.get("status")
    assert status in STATUSES, f"{path}: invalid status"

    review_id = require_nonempty(data.get("review_id"), f"{path}: review_id")
    category = require_nonempty(data.get("category"), f"{path}: category")
    assert category in requirements, f"{path}: unknown category {category}"
    requirement = requirements[category]

    target = data.get("target")
    assert isinstance(target, dict), f"{path}: target must be an object"
    release = require_nonempty(target.get("release"), f"{path}: target.release")
    git_sha = require_nonempty(target.get("git_sha"), f"{path}: target.git_sha").lower()
    assert GIT_SHA_RE.fullmatch(git_sha), f"{path}: target.git_sha must be a 40-character Git SHA"

    reviewer = data.get("reviewer")
    assert isinstance(reviewer, dict), f"{path}: reviewer must be an object"
    reviewer_name = require_nonempty(reviewer.get("name"), f"{path}: reviewer.name")
    reviewer_identity = require_nonempty(
        reviewer.get("public_identity"), f"{path}: reviewer.public_identity"
    )
    require_nonempty(reviewer.get("organization"), f"{path}: reviewer.organization")
    relationship = reviewer.get("relationship_to_project")
    assert relationship in RELATIONSHIPS, f"{path}: invalid relationship_to_project"
    require_nonempty(
        reviewer.get("conflict_of_interest_declaration"),
        f"{path}: reviewer.conflict_of_interest_declaration",
    )

    review = data.get("review")
    assert isinstance(review, dict), f"{path}: review must be an object"
    require_nonempty(review.get("reviewed_at"), f"{path}: review.reviewed_at")
    verdict = review.get("verdict")
    assert verdict in VERDICTS, f"{path}: invalid reviewer verdict"
    require_nonempty(review.get("summary"), f"{path}: review.summary")

    covered_topics = review.get("covered_topics")
    assert isinstance(covered_topics, list), f"{path}: covered_topics must be an array"
    assert len(covered_topics) == len(set(covered_topics)), f"{path}: duplicate covered topic"
    required_topics = set(requirement["required_topics"])
    missing_topics = required_topics - set(covered_topics)
    assert not missing_topics, f"{path}: missing required topics: {sorted(missing_topics)}"

    methodology = review.get("methodology")
    assert isinstance(methodology, dict), f"{path}: methodology must be an object"
    for field in ("scope", "method", "environment", "reproduction_steps"):
        require_nonempty(methodology.get(field), f"{path}: review.methodology.{field}")

    evidence = data.get("evidence")
    assert isinstance(evidence, dict), f"{path}: evidence must be an object"
    validate_artifact(evidence.get("report"), f"{path}: evidence.report")
    supporting = evidence.get("supporting_artifacts")
    assert isinstance(supporting, list), f"{path}: supporting_artifacts must be an array"
    for index, artifact in enumerate(supporting):
        validate_artifact(artifact, f"{path}: evidence.supporting_artifacts[{index}]")

    reproduction = data.get("reproduction")
    if category == "independent_reproduction":
        assert isinstance(reproduction, dict), f"{path}: reproduction object required"
        require_nonempty(
            reproduction.get("implementation_repository"),
            f"{path}: reproduction.implementation_repository",
        )
        require_nonempty(
            reproduction.get("implementation_language"),
            f"{path}: reproduction.implementation_language",
        )
        assert reproduction.get("shared_project_code") is False, (
            f"{path}: independent reproduction cannot share project implementation code"
        )
        require_nonempty(
            reproduction.get("conformance_command"),
            f"{path}: reproduction.conformance_command",
        )
        assert reproduction.get("conformance_result") in {"PASS", "FAIL", "NOT_RUN"}
    else:
        assert reproduction is None, f"{path}: reproduction must be null for {category}"

    admission = data.get("admission")
    assert isinstance(admission, dict), f"{path}: admission must be an object"
    admission_result = admission.get("result")

    if status == "SUBMITTED_UNREVIEWED":
        assert admission_result == "NOT_REVIEWED", f"{path}: unreviewed status/admission mismatch"
        classification = "PENDING"
    elif status == "REJECTED":
        assert admission_result == "REJECTED", f"{path}: rejected status/admission mismatch"
        require_nonempty(admission.get("admitted_by"), f"{path}: admission.admitted_by")
        require_nonempty(admission.get("admitted_at"), f"{path}: admission.admitted_at")
        require_nonempty(admission.get("notes"), f"{path}: admission.notes")
        classification = "REJECTED"
    else:
        assert admission_result == "ADMITTED", f"{path}: admitted status/admission mismatch"
        admitted_by = require_nonempty(
            admission.get("admitted_by"), f"{path}: admission.admitted_by"
        )
        require_nonempty(admission.get("admitted_at"), f"{path}: admission.admitted_at")
        assert admitted_by.casefold() not in {
            reviewer_name.casefold(),
            reviewer_identity.casefold(),
        }, f"{path}: external reviewer cannot self-admit the evidence"

        candidate_matches = (
            pinned_release is not None
            and pinned_sha is not None
            and release == pinned_release
            and git_sha == pinned_sha
        )
        relationship_allowed = relationship in set(requirement["allowed_relationships"])
        reproduction_pass = (
            category != "independent_reproduction"
            or reproduction.get("conformance_result") == "PASS"
        )

        if (
            candidate_matches
            and relationship_allowed
            and verdict == "PASS"
            and reproduction_pass
        ):
            classification = "SATISFIED"
        elif not candidate_matches:
            classification = "ADMITTED_WRONG_OR_UNPINNED_CANDIDATE"
        elif not relationship_allowed:
            classification = "ADMITTED_RELATIONSHIP_NOT_ALLOWED"
        elif verdict != "PASS":
            classification = "ADMITTED_NONPASS_VERDICT"
        else:
            classification = "ADMITTED_REPRODUCTION_NOT_PASSING"

    return review_id, category, classification


def evaluate_external_reviews() -> dict[str, Any]:
    requirements, gate_to_category = validate_requirements()
    validate_template()
    pinned_release, pinned_sha = candidate_target()

    per_category: dict[str, list[str]] = {category: [] for category in requirements}
    review_ids: set[str] = set()

    if SUBMISSIONS_DIR.exists():
        for path in sorted(SUBMISSIONS_DIR.glob("*.json")):
            review_id, category, classification = validate_submission(
                path,
                requirements,
                pinned_release,
                pinned_sha,
            )
            assert review_id not in review_ids, f"duplicate review_id: {review_id}"
            review_ids.add(review_id)
            per_category[category].append(classification)

    category_state: dict[str, str] = {}
    for category, states in per_category.items():
        if "SATISFIED" in states:
            state = "SATISFIED"
        elif "PENDING" in states:
            state = "PENDING"
        elif any(s.startswith("ADMITTED_") for s in states):
            state = next(s for s in states if s.startswith("ADMITTED_"))
        elif "REJECTED" in states:
            state = "REJECTED"
        else:
            state = "MISSING"
        category_state[category] = state

    gate_state = {
        gate_id: category_state[category]
        for gate_id, category in gate_to_category.items()
    }

    ready = (
        pinned_release is not None
        and pinned_sha is not None
        and all(state == "SATISFIED" for state in category_state.values())
    )

    return {
        "ready": ready,
        "candidate_release": pinned_release,
        "candidate_git_sha": pinned_sha,
        "categories": category_state,
        "gates": gate_state,
    }


def main() -> None:
    result = evaluate_external_reviews()
    print(f"M20 external review intake: {'READY' if result['ready'] else 'BLOCKED'}")
    if result["candidate_git_sha"] is None:
        print("candidate: UNPINNED")
    else:
        print(
            "candidate: "
            f"{result['candidate_release']} @ {result['candidate_git_sha']}"
        )

    requirements, _ = validate_requirements()
    for category, requirement in requirements.items():
        gate_id = requirement["v1_gate_id"]
        print(f"- {category} -> {gate_id}: {result['categories'][category]}")


if __name__ == "__main__":
    main()
