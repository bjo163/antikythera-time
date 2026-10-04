#!/usr/bin/env python3
"""Fail-closed M11 hardware-selection and campaign readiness validator."""

from __future__ import annotations

import json
import math
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIR = ROOT / "hardware" / "m-clock"
SCHEMA = DIR / "qualification.schema.json"
TEMPLATE = DIR / "qualification-template.json"
SELECTION = DIR / "qualification.json"

EXPECTED_SCHEMA = "mtime-mclock-hardware-qualification-1"
REQUIRED_RUNS = {
    "boot_self_test": 0.0,
    "pps_lock": 0.0,
    "pps_latency_jitter": 0.0,
    "holdover_1h": 3600.0,
    "holdover_6h": 21600.0,
    "holdover_24h": 86400.0,
    "holdover_72h": 259200.0,
    "thermal_profile": 0.0,
    "gnss_loss_reacquisition": 0.0,
    "power_cycle_recovery": 0.0,
    "offline_mclock_output": 0.0,
}

def load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))

def nonempty(value: object, field: str) -> str:
    assert isinstance(value, str) and value.strip(), f"{field} must be non-empty"
    return value.strip()

def finite_positive(value: object, field: str, *, allow_zero: bool = False) -> float:
    assert isinstance(value, (int, float)) and not isinstance(value, bool), f"{field} must be numeric"
    value = float(value)
    assert math.isfinite(value), f"{field} must be finite"
    if allow_zero:
        assert value >= 0.0, f"{field} must be >= 0"
    else:
        assert value > 0.0, f"{field} must be > 0"
    return value

def validate_template() -> None:
    assert SCHEMA.exists(), "qualification schema missing"
    schema = load(SCHEMA)
    assert schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema"
    assert schema["properties"]["schema"]["const"] == EXPECTED_SCHEMA

    doc = load(TEMPLATE)
    assert doc["schema"] == EXPECTED_SCHEMA
    assert doc["status"] == "TEMPLATE_NOT_SELECTION"
    ids = [row["id"] for row in doc["campaign"]]
    assert set(ids) == set(REQUIRED_RUNS), "template campaign IDs mismatch"
    assert len(ids) == len(set(ids)), "template campaign IDs duplicate"

def validate_component(component: object, field: str) -> dict:
    assert isinstance(component, dict), f"{field} must be an object"
    for key in ("vendor", "model", "serial_or_build_id"):
        nonempty(component.get(key), f"{field}.{key}")
    return component

def validate_selection(doc: dict) -> str:
    assert doc.get("schema") == EXPECTED_SCHEMA
    status = doc.get("status")
    assert status in {"SELECTED_UNVERIFIED", "BENCH_READY"}, (
        "qualification.json cannot use TEMPLATE_NOT_SELECTION"
    )
    nonempty(doc.get("selection_date"), "selection_date")
    assert re.fullmatch(r"\d{4}-\d{2}-\d{2}", doc["selection_date"]), (
        "selection_date must be YYYY-MM-DD"
    )
    nonempty(doc.get("operator_id"), "operator_id")

    controller = validate_component(doc.get("controller"), "controller")
    for key in (
        "datasheet_reference", "architecture", "operating_environment",
        "rust_toolchain", "pps_timestamp_interface",
        "display_output_interface", "nonvolatile_storage", "power_input",
    ):
        nonempty(controller.get(key), f"controller.{key}")
    finite_positive(controller.get("timestamp_resolution_ns"), "controller.timestamp_resolution_ns")

    ref = validate_component(doc.get("reference_input"), "reference_input")
    for key in ("reference_type", "pps_interface", "timing_offset_plan", "provenance_plan"):
        nonempty(ref.get(key), f"reference_input.{key}")

    osc = validate_component(doc.get("oscillator"), "oscillator")
    for key in ("class", "stability_reference", "discipline_interface"):
        nonempty(osc.get(key), f"oscillator.{key}")
    finite_positive(osc.get("nominal_frequency_hz"), "oscillator.nominal_frequency_hz")

    rtc = validate_component(doc.get("rtc"), "rtc")
    nonempty(rtc.get("datasheet_reference"), "rtc.datasheet_reference")

    instrument = validate_component(doc.get("reference_instrument"), "reference_instrument")
    for key in ("traceability_plan", "calibration_record_plan", "capture_method"):
        nonempty(instrument.get(key), f"reference_instrument.{key}")
    finite_positive(
        instrument.get("expected_resolution_ns"),
        "reference_instrument.expected_resolution_ns",
    )

    power = doc.get("power")
    assert isinstance(power, dict), "power must be an object"
    nonempty(power.get("supply_id"), "power.supply_id")
    finite_positive(power.get("nominal_voltage_v"), "power.nominal_voltage_v")
    finite_positive(power.get("current_limit_a"), "power.current_limit_a")
    nonempty(power.get("power_cycle_method"), "power.power_cycle_method")

    thermal = doc.get("thermal")
    assert isinstance(thermal, dict), "thermal must be an object"
    nonempty(thermal.get("method"), "thermal.method")
    nonempty(thermal.get("temperature_sensor_id"), "thermal.temperature_sensor_id")
    tmin = thermal.get("planned_min_c")
    tmax = thermal.get("planned_max_c")
    assert isinstance(tmin, (int, float)) and isinstance(tmax, (int, float))
    assert math.isfinite(float(tmin)) and math.isfinite(float(tmax))
    assert float(tmin) < float(tmax), "thermal planned_min_c must be < planned_max_c"
    finite_positive(thermal.get("warmup_seconds"), "thermal.warmup_seconds", allow_zero=True)

    campaign = doc.get("campaign")
    assert isinstance(campaign, list), "campaign must be an array"
    by_id = {}
    for row in campaign:
        assert isinstance(row, dict), "campaign row must be an object"
        run_id = nonempty(row.get("id"), "campaign.id")
        assert run_id not in by_id, f"duplicate campaign id: {run_id}"
        by_id[run_id] = row
    assert set(by_id) == set(REQUIRED_RUNS), "campaign IDs mismatch"

    for run_id, required_duration in REQUIRED_RUNS.items():
        row = by_id[run_id]
        duration = finite_positive(
            row.get("planned_duration_seconds"),
            f"campaign.{run_id}.planned_duration_seconds",
            allow_zero=True,
        )
        assert duration >= required_duration, (
            f"campaign.{run_id} planned duration {duration}s is below {required_duration}s"
        )
        minimum_events = row.get("minimum_events")
        assert isinstance(minimum_events, int) and minimum_events >= 0, (
            f"campaign.{run_id}.minimum_events must be a non-negative integer"
        )
        nonempty(row.get("output_artifact"), f"campaign.{run_id}.output_artifact")
        nonempty(row.get("acceptance_evidence"), f"campaign.{run_id}.acceptance_evidence")

    return status

def main() -> None:
    validate_template()
    print("M11 hardware qualification framework: PASS")

    if not SELECTION.exists():
        print("hardware_selection: MISSING_BY_DESIGN")
        print("qualification_status: EXTERNAL_SELECTION_REQUIRED")
        return

    status = validate_selection(load(SELECTION))
    print(f"hardware_selection: {status}")
    if status == "SELECTED_UNVERIFIED":
        print("qualification_status: SELECTION_PRESENT_PREFLIGHT_REQUIRED")
    else:
        print("qualification_status: BENCH_PLAN_READY_PHYSICAL_BUILD_REQUIRED")
        print("note: BENCH_READY is not physical evidence and does not satisfy M11")

if __name__ == "__main__":
    main()
