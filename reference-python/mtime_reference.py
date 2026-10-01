#!/usr/bin/env python3
import math,subprocess
DAY=86400.0;LG=6.969290134e-10;MJD0=2400000.5;MJD1977=43144.0;TT_MINUS_TAI=32.184
def tt_to_tcg(d1,d2):
    t77t=MJD1977+TT_MINUS_TAI/DAY;rate=LG/(1-LG)
    return (d1,d2+((d1-MJD0)+(d2-t77t))*rate) if abs(d1)>abs(d2) else (d1+((d2-MJD0)+(d1-t77t))*rate,d2)
def simpson(f,n=400000):
    if n%2:n+=1
    h=1/n;s=f(0)+f(1)
    for i in range(1,n):s+=(4 if i%2 else 2)*f(i*h)
    return s*h/3
def planck_age():
    h0=67.36;h=h0/100;orad=2.4728e-5*(1+0.22710731766*3.046)/(h*h);om=0.3153;ol=1-orad-om
    def f(x):
        if x==0:return 0.0
        a=x*x;e=math.sqrt(orad/a**4+om/a**3+ol);return 2/(x*e)
    mpc=3.085677581491367e22;gyr=1e9*31557600
    return simpson(f)/(h0*1000/mpc)/gyr
rust=subprocess.check_output(["cargo","run","--quiet","-p","mtime-cli","--example","golden"],text=True)
got={}
for line in rust.strip().splitlines():
    k,v=line.split("=",1);got[k]=v
_,tcg=tt_to_tcg(2453750.5,0.892482639)
expected={"tt_to_tcg_d2":tcg,"saros_jd":2460409.263+6585.3223,"mabims_pass":True,"planck_age_gyr":planck_age()}
assert abs(float(got["tt_to_tcg_d2"])-expected["tt_to_tcg_d2"])<=1e-12
assert abs(float(got["saros_jd"])-expected["saros_jd"])<=1e-9
assert (got["mabims_pass"].lower()=="true")==expected["mabims_pass"]
assert abs(float(got["planck_age_gyr"])-expected["planck_age_gyr"])<=5e-6
print("M-Time Rust/Python compatibility: PASS")
for k,v in expected.items():print(k,v)
