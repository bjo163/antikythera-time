#!/usr/bin/env python3
"""Fail-closed admission gate for M11/M12 physical evidence.

This script distinguishes repository readiness from actual physical evidence.
Templates and synthetic fixtures can prove the software path is ready, but they
must never satisfy M11/M12.
"""

from __future__ import annotations

import csv
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "hardware" / "m-clock" / "physical-evidence"
TEMPLATE = EVIDENCE_DIR / "manifest-template.json"
CANDIDATE = EVIDENCE_DIR / "manifest.json"
SCHEMA = EVIDENCE_DIR / "manifest.schema.json"

EXPECTED_SCHEMA = "mtime-mclock-physical-evidence-2"
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
GIT_SHA_RE = re.compile(r"^[0-9a-f]{40}$")
CSV_COLUMNS = [
    "elapsed_seconds",
    "offset_nanoseconds",
    "temperature_c",
    "reference_id",
    "device_id",
    "firmware_sha",
]
REQUIRED_EVIDENCE = [
    "wiring_or_schematic",
    "boot_self_test_log",
    "pps_lock_log",
    "display_capture",
    "thermal_log",
    "gnss_loss_reacquisition_log",
    "power_cycle_log",
    "metrology_report",
]
REQUIRED_DATASETS = {
    "pps_latency_jitter": 0.0,
    "holdover_1h": 3600.0,
    "holdover_6h": 21600.0,
    "holdover_24h": 86400.0,
    "holdover_72h": 259200.0,
}


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def require_nonempty(value: object, field: str) -> str:
    assert isinstance(value, str) and value.strip(), f"{field} must be non-empty"
    return value.strip()


def validate_hash(value: object, field: str) -> str:
    text = require_nonempty(value, field).lower()
    assert SHA256_RE.fullmatch(text), f"{field} must be lowercase SHA-256 hex"
    return text


def safe_repo_path(value: object, field: str) -> Path:
    raw = require_nonempty(value, field)
    candidate = (ROOT / raw).resolve()
    assert ROOT == candidate or ROOT in candidate.parents, f"{field} escapes repository root"
    assert candidate.exists() and candidate.is_file(), f"{field} does not exist: {raw}"
    return candidate


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def validate_artifact(ref: dict, field: str) -> Path:
    assert isinstance(ref, dict), f"{field} must be an object"
    path = safe_repo_path(ref.get("path"), f"{field}.path")
    expected = validate_hash(ref.get("sha256"), f"{field}.sha256")
    actual = sha256_file(path)
    assert actual == expected, f"{field} SHA-256 mismatch: expected {expected}, got {actual}"
    return path


def validate_metrology_csv(
    path: Path,
    minimum_elapsed_seconds: float,
    field: str,
    *,
    expected_device_id: str,
    expected_reference_id: str,
    expected_firmware_sha: str,
) -> None:
    with path.open("r", encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle)
        assert reader.fieldnames == CSV_COLUMNS, (
            f"{field} header must be exactly {','.join(CSV_COLUMNS)}"
        )
        rows = list(reader)

    assert len(rows) >= 2, f"{field} must contain at least two measurement rows"
    elapsed = []
    for index, row in enumerate(rows, start=2):
        try:
            t = float(row["elapsed_seconds"])
            float(row["offset_nanoseconds"])
            if row["temperature_c"].strip():
                float(row["temperature_c"])
        except (TypeError, ValueError) as exc:
            raise AssertionError(f"{field} has invalid numeric value at line {index}") from exc
        assert t >= 0.0, f"{field} elapsed_seconds must be >= 0 at line {index}"
        reference_id = require_nonempty(row["reference_id"], f"{field}.reference_id line {index}")
        device_id = require_nonempty(row["device_id"], f"{field}.device_id line {index}")
        firmware_sha = require_nonempty(row["firmware_sha"], f"{field}.firmware_sha line {index}").lower()
        assert reference_id == expected_reference_id, (
            f"{field} reference_id mismatch at line {index}: {reference_id}"
        )
        assert device_id == expected_device_id, (
            f"{field} device_id mismatch at line {index}: {device_id}"
        )
        assert firmware_sha == expected_firmware_sha, (
            f"{field} firmware_sha mismatch at line {index}: {firmware_sha}"
        )
        elapsed.append(t)

    assert elapsed == sorted(elapsed), f"{field} elapsed_seconds must be non-decreasing"

    if minimum_elapsed_seconds > 0.0:
        observed = max(elapsed) - min(elapsed)
        assert observed >= minimum_elapsed_seconds, (
            f"{field} duration too short: observed span {observed}s, "
            f"requires >= {minimum_elapsed_seconds}s"
        )


def validate_template() -> None:
    assert SCHEMA.exists(), "physical evidence JSON schema missing"
    schema_doc = load_json(SCHEMA)
    assert schema_doc.get("$schema") == "https://json-schema.org/draft/2020-12/schema"
    template = load_json(TEMPLATE)
    assert template["schema"] == EXPECTED_SCHEMA
    assert template["status"] == "TEMPLATE_NOT_MEASUREMENT"
    assert set(template["evidence"]) == set(REQUIRED_EVIDENCE)
    assert set(template["datasets"]) == set(REQUIRED_DATASETS)


def validate_candidate(data: dict) -> str:
    assert data.get("schema") == EXPECTED_SCHEMA
    status = data.get("status")
    assert status in {"MEASURED_UNREVIEWED", "REVIEWED_ACCEPTED", "REVIEWED_REJECTED"}, (
        "physical-evidence/manifest.json cannot use TEMPLATE_NOT_MEASUREMENT"
    )

    device_id = require_nonempty(data.get("device_id"), "device_id")
    for name in ("controller", "gnss_pps", "oscillator", "rtc"):
        component = data.get(name)
        assert isinstance(component, dict), f"{name} must be an object"
        require_nonempty(component.get("model"), f"{name}.model")
        require_nonempty(component.get("serial_or_build_id"), f"{name}.serial_or_build_id")

    firmware = data.get("firmware")
    assert isinstance(firmware, dict), "firmware must be an object"
    git_sha = require_nonempty(firmware.get("git_sha"), "firmware.git_sha").lower()
    assert GIT_SHA_RE.fullmatch(git_sha), "firmware.git_sha must be a 40-character Git SHA"
    validate_hash(firmware.get("binary_sha256"), "firmware.binary_sha256")
    require_nonempty(firmware.get("build_instructions"), "firmware.build_instructions")

    reference = data.get("reference_time_source")
    assert isinstance(reference, dict), "reference_time_source must be an object"
    reference_id = require_nonempty(reference.get("id"), "reference_time_source.id")
    for field in ("description", "traceability", "calibration_record"):
        require_nonempty(reference.get(field), f"reference_time_source.{field}")

    build_date = require_nonempty(data.get("build_date"), "build_date")
    assert re.fullmatch(r"\d{4}-\d{2}-\d{2}", build_date), "build_date must be YYYY-MM-DD"

    evidence = data.get("evidence")
    assert isinstance(evidence, dict), "evidence must be an object"
    assert set(evidence) == set(REQUIRED_EVIDENCE), "evidence keys mismatch"
    for name in REQUIRED_EVIDENCE:
        validate_artifact(evidence[name], f"evidence.{name}")

    datasets = data.get("datasets")
    assert isinstance(datasets, dict), "datasets must be an object"
    assert set(datasets) == set(REQUIRED_DATASETS), "dataset keys mismatch"
    for name, required_duration in REQUIRED_DATASETS.items():
        entry = datasets[name]
        assert isinstance(entry, dict), f"datasets.{name} must be an object"
        declared_minimum = float(entry.get("minimum_elapsed_seconds"))
        assert declared_minimum == required_duration, (
            f"datasets.{name}.minimum_elapsed_seconds must be {required_duration}"
        )
        path = validate_artifact(entry, f"datasets.{name}")
        validate_metrology_csv(
            path,
            required_duration,
            f"datasets.{name}",
            expected_device_id=device_id,
            expected_reference_id=reference_id,
            expected_firmware_sha=git_sha,
        )

    report_path = validate_artifact(
        evidence["metrology_report"], "evidence.metrology_report"
    )
    report = load_json(report_path)
    assert report.get("schema") == "mtime-mclock-metrology-suite-1", (
        "metrology report schema mismatch"
    )
    report_identity = report.get("identity")
    assert isinstance(report_identity, dict), "metrology report identity missing"
    assert report_identity.get("device_id") == device_id, "metrology report device mismatch"
    assert report_identity.get("reference_id") == reference_id, "metrology report reference mismatch"
    assert str(report_identity.get("firmware_sha", "")).lower() == git_sha, (
        "metrology report firmware mismatch"
    )
    assert report.get("interpretation") == (
        "measurement_characterization_only_no_accuracy_class_assigned"
    ), "metrology report must not silently assign an accuracy class"

    report_datasets = report.get("datasets")
    assert isinstance(report_datasets, list), "metrology report datasets missing"
    by_label = {
        item.get("label"): item
        for item in report_datasets
        if isinstance(item, dict)
    }
    assert set(by_label) == set(REQUIRED_DATASETS), "metrology report dataset labels mismatch"
    for name, required_duration in REQUIRED_DATASETS.items():
        item = by_label[name]
        manifest_entry = datasets[name]
        assert item.get("source_csv") == manifest_entry.get("path"), (
            f"metrology report source path mismatch for {name}"
        )
        assert float(item.get("minimum_duration_seconds")) == required_duration, (
            f"metrology report minimum duration mismatch for {name}"
        )
        assert int(item.get("samples")) >= 2, f"metrology report has too few samples for {name}"
        assert float(item.get("duration_seconds")) >= required_duration, (
            f"metrology report duration too short for {name}"
        )

    review = data.get("review")
    assert isinstance(review, dict), "review must be an object"
    result = review.get("result")
    if status == "MEASURED_UNREVIEWED":
        assert result == "NOT_REVIEWED"
    elif status == "REVIEWED_ACCEPTED":
        assert result == "ACCEPTED"
        require_nonempty(review.get("reviewer"), "review.reviewer")
        require_nonempty(review.get("reviewed_at"), "review.reviewed_at")
    elif status == "REVIEWED_REJECTED":
        assert result == "REJECTED"
        require_nonempty(review.get("reviewer"), "review.reviewer")
        require_nonempty(review.get("reviewed_at"), "review.reviewed_at")
        require_nonempty(review.get("notes"), "review.notes")

    return status


def main() -> None:
    validate_template()

    bench = ROOT / "hardware" / "m-clock" / "BENCH_BUILD.md"
    metrology = ROOT / "hardware" / "m-clock" / "METROLOGY.md"
    assert bench.exists() and metrology.exists()

    print("M11/M12 repository readiness: PASS")

    if not CANDIDATE.exists():
        print("physical_evidence: MISSING_BY_DESIGN")
        print("milestone_status: SOFTWARE_READY_EXTERNAL_BENCH_REQUIRED")
        return

    status = validate_candidate(load_json(CANDIDATE))
    print(f"physical_evidence: {status}")
    if status == "MEASURED_UNREVIEWED":
        print("milestone_status: PHYSICAL_EVIDENCE_PRESENT_REVIEW_REQUIRED")
    elif status == "REVIEWED_ACCEPTED":
        print("milestone_status: PHYSICAL_EVIDENCE_ADMITTED")
        print("note: M11/M12 and v1 gates still require explicit issue/review updates; this script does not auto-promote them")
    else:
        print("milestone_status: PHYSICAL_EVIDENCE_REJECTED")


if __name__ == "__main__":
    main()
