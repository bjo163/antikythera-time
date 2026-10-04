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


def test_j2000_identity():
    state=m.digital_v1_from_tt(2451545.0)
    assert state.linear_si_nanoseconds_from_j2000_tt==0
    assert len(state.cycle_phase)==8

def test_bad_magic_rejected():
    try:
        m.decode_mts2(b"MTS1\x00\x00")
    except ValueError:
        return
    raise AssertionError("bad magic accepted")

def test_nonfinite_tt_rejected():
    try:
        m.digital_v1_from_tt(float("nan"))
    except ValueError:
        return
    raise AssertionError("non-finite TT accepted")

if __name__ == "__main__":
    test_j2000_identity()
    test_bad_magic_rejected()
    test_nonfinite_tt_rejected()
    print("M-Time Python SDK tests: PASS")
