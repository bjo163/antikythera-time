#!/usr/bin/env python3
import json
from pathlib import Path

p=Path("hardware/m-clock/physical-evidence/manifest-template.json")
d=json.loads(p.read_text())
assert d["schema"]=="mtime-mclock-physical-evidence-1"
assert d["status"]=="TEMPLATE_NOT_MEASUREMENT"
required=[
    "wiring_or_schematic","boot_self_test_log","pps_lock_log",
    "offline_holdover_log","display_capture","metrology_csv"
]
assert all(k in d["evidence"] for k in required)

bench=Path("hardware/m-clock/BENCH_BUILD.md")
metro=Path("hardware/m-clock/METROLOGY.md")
assert bench.exists() and metro.exists()

print("M11/M12 repository readiness: PASS")
print("physical_evidence: MISSING_BY_DESIGN")
print("milestone_status: SOFTWARE_READY_EXTERNAL_BENCH_REQUIRED")
