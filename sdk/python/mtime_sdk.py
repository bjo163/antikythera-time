"""Minimal public M-Time Python SDK.

Implements Digital Antikythera V1 from the public specification and decodes
canonical MTS-2 binary. No network access is required.
"""
from dataclasses import dataclass
import math
import struct

J2000_JD_TT=2451545.0
DAY_NS=86_400_000_000_000

def _wrap(x): return x%360.0
def _sin(x): return math.sin(math.radians(x))
def _phase(d,p): return (d%p)/p

@dataclass(frozen=True)
class AntikytheraState:
    jd_tt: float
    linear_si_nanoseconds_from_j2000_tt: int
    sun_deg: float
    moon_deg: float
    lunar_phase_deg: float
    node_deg: float
    cycle_phase: tuple

def digital_v1_from_tt(jd_tt: float) -> AntikytheraState:
    if not math.isfinite(jd_tt):
        raise ValueError("non-finite TT")
    d=jd_tt-J2000_JD_TT
    t=d/36525.0
    l0=_wrap(280.46646+0.98564736*d)
    ms=_wrap(357.52911+0.98560028*d)
    sun=_wrap(l0+(1.914602-0.004817*t-0.000014*t*t)*_sin(ms)
        +(0.019993-0.000101*t)*_sin(2*ms)+0.000289*_sin(3*ms))
    l=_wrap(218.3164477+13.17639648*d)
    mm=_wrap(134.9633964+13.06499295*d)
    de=_wrap(297.8501921+12.19074912*d)
    f=_wrap(93.272095+13.22935024*d)
    c=(6.289*_sin(mm)+1.274*_sin(2*de-mm)+0.658*_sin(2*de)+0.214*_sin(2*mm)
       -0.186*_sin(ms)-0.059*_sin(2*de-2*mm)-0.057*_sin(2*de-ms-mm)
       +0.053*_sin(2*de+mm)+0.046*_sin(2*de-ms)+0.041*_sin(ms-mm)
       -0.035*_sin(de)-0.031*_sin(ms+mm)-0.015*_sin(2*f-2*de)+0.011*_sin(2*de-4*mm))
    moon=_wrap(l+c)
    periods=(365.2421897,29.530588853,27.321661547,27.55454988,27.212220817,6939.688,6585.3223,19755.9669)
    cycles=(
        _phase(d,periods[0]),
        _wrap(297.8501921+d*360/periods[1])/360,
        _wrap(218.3164477+d*360/periods[2])/360,
        _wrap(134.9633964+d*360/periods[3])/360,
        _wrap(93.272095+d*360/periods[4])/360,
        _phase(d,periods[5]),_phase(d,periods[6]),_phase(d,periods[7])
    )
    linear=math.floor(d*DAY_NS+0.5) if d>=0 else math.ceil(d*DAY_NS-0.5)
    node=_wrap(125.04452+_phase(d,6798.383)*360)
    return AntikytheraState(jd_tt,linear,sun,moon,_wrap(moon-sun),node,cycles)

def decode_mts2(blob: bytes) -> dict:
    if blob[:4]!=b"MTS2": raise ValueError("bad MTS2 magic")
    n=struct.unpack(">H",blob[4:6])[0]; pos=6
    profile=blob[pos:pos+n].decode("utf-8"); pos+=n
    linear=int.from_bytes(blob[pos:pos+16],"big",signed=True); pos+=16
    vals=[]
    for _ in range(15):
        vals.append(struct.unpack(">d",blob[pos:pos+8])[0]); pos+=8
    if pos!=len(blob): raise ValueError("bad MTS2 length")
    return {
        "schema":"MTS-2","profile_id":profile,
        "linear_si_nanoseconds_from_j2000_tt":linear,
        "tt_jd":vals[0],"tdb_jd":vals[1],"reference_uncertainty_seconds":vals[2],
        "dial_deg":tuple(vals[3:7]),"cycle_phase":tuple(vals[7:15]),
    }
