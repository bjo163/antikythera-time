#!/usr/bin/env python3
"""Independent MTS-2 / Digital Antikythera V1 conformance implementation.

This file implements the public equations in spec/ANTIKYTHERA-DIGITAL-V1.md.
It does not call Rust through FFI or parse Rust source code.
"""
import math
import struct
import subprocess

J2000=2451545.0
DAY_NS=86_400_000_000_000
TROPICAL=365.2421897
SYNODIC=29.530588853
SIDEREAL=27.321661547
ANOMALISTIC=27.55454988
DRACONIC=27.212220817
METONIC=6939.688
SAROS=6585.3223
EXELIGMOS=19755.9669
NODE=6798.383

def wrap(x): return x%360.0
def phase(d,p): return (d%p)/p
def sind(x): return math.sin(math.radians(x))
def angle_diff(a,b): return abs((a-b+180.0)%360.0-180.0)

def solar(d):
    t=d/36525.0
    mean=wrap(280.46646+0.98564736*d)
    m=wrap(357.52911+0.98560028*d)
    c=(1.914602-0.004817*t-0.000014*t*t)*sind(m)+(0.019993-0.000101*t)*sind(2*m)+0.000289*sind(3*m)
    return wrap(mean+c)

def moon(d):
    l=wrap(218.3164477+13.17639648*d)
    mm=wrap(134.9633964+13.06499295*d)
    de=wrap(297.8501921+12.19074912*d)
    f=wrap(93.272095+13.22935024*d)
    ms=wrap(357.52911+0.98560028*d)
    c=(6.289*sind(mm)+1.274*sind(2*de-mm)+0.658*sind(2*de)+0.214*sind(2*mm)
       -0.186*sind(ms)-0.059*sind(2*de-2*mm)-0.057*sind(2*de-ms-mm)
       +0.053*sind(2*de+mm)+0.046*sind(2*de-ms)+0.041*sind(ms-mm)
       -0.035*sind(de)-0.031*sind(ms+mm)-0.015*sind(2*f-2*de)+0.011*sind(2*de-4*mm))
    return wrap(l+c)

def rust_round_i128(x):
    return math.floor(x+0.5) if x>=0 else math.ceil(x-0.5)

def independent(jd):
    d=jd-J2000
    s=solar(d); m=moon(d)
    node=wrap(125.04452+phase(d,NODE)*360.0)
    cycles=[
        phase(d,TROPICAL),
        wrap(297.8501921+d*360.0/SYNODIC)/360.0,
        wrap(218.3164477+d*360.0/SIDEREAL)/360.0,
        wrap(134.9633964+d*360.0/ANOMALISTIC)/360.0,
        wrap(93.272095+d*360.0/DRACONIC)/360.0,
        phase(d,METONIC),phase(d,SAROS),phase(d,EXELIGMOS)
    ]
    return rust_round_i128(d*DAY_NS), [s,m,wrap(m-s),node], cycles

def parse_mts2(blob):
    assert blob[:4]==b"MTS2"
    n=struct.unpack(">H",blob[4:6])[0]
    pos=6
    profile=blob[pos:pos+n].decode(); pos+=n
    linear=int.from_bytes(blob[pos:pos+16],"big",signed=True); pos+=16
    vals=[]
    for _ in range(15):
        vals.append(struct.unpack(">d",blob[pos:pos+8])[0]); pos+=8
    assert pos==len(blob)
    return profile,linear,vals

def reencode(profile,linear,vals):
    p=profile.encode()
    out=b"MTS2"+struct.pack(">H",len(p))+p+int(linear).to_bytes(16,"big",signed=True)
    for v in vals: out+=struct.pack(">d",v)
    return out

out=subprocess.check_output(["cargo","run","--quiet","-p","mtime-temporal","--example","mts2_conformance"],text=True)
count=0
for line in out.splitlines():
    if not line.startswith("V|"): continue
    parts=line.split("|")
    jd=float(parts[1]); linear=int(parts[2])
    rust_dials=list(map(float,parts[3:7]))
    rust_cycles=list(map(float,parts[7:15]))
    blob=bytes.fromhex(parts[15])
    py_linear,py_dials,py_cycles=independent(jd)
    assert linear==py_linear, (jd,linear,py_linear)
    for a,b in zip(rust_dials,py_dials):
        assert angle_diff(a,b)<2e-9,(jd,a,b)
    for a,b in zip(rust_cycles,py_cycles):
        assert abs(a-b)<2e-12,(jd,a,b)
    profile,binary_linear,vals=parse_mts2(blob)
    assert profile=="MTIME_DIGITAL_ANTIKYTHERA_V1"
    assert binary_linear==linear
    assert reencode(profile,binary_linear,vals)==blob
    count+=1
assert count==5
print(f"M15 independent Python conformance: PASS ({count} vectors)")
