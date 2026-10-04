#!/usr/bin/env python3
import importlib.util
from pathlib import Path
p=Path(__file__).with_name("mtime_sdk.py")
spec=importlib.util.spec_from_file_location("mtime_sdk",p)
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
s=m.digital_v1_from_tt(2451545.0)
assert s.linear_si_nanoseconds_from_j2000_tt==0
assert abs(s.sun_deg-280.0)<2.0
assert len(s.cycle_phase)==8
print("M-Time Python SDK smoke test: PASS")
