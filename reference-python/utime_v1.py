#!/usr/bin/env python3
import json, math, os

DAY=86400.0
LG=6.969290134e-10
MJD0=2400000.5
MJD1977=43144.0
TT_MINUS_TAI=32.184
MPC_M=149597870700.0*(648000.0/math.pi)*1_000_000.0
GYR_S=1e9*31557600.0

def tt_to_tcg(d1,d2):
    t77t=MJD1977+TT_MINUS_TAI/DAY
    rate=LG/(1.0-LG)
    if abs(d1)>abs(d2):
        return d1,d2+((d1-MJD0)+(d2-t77t))*rate
    return d1+((d2-MJD0)+(d1-t77t))*rate,d2

def omega_r(H0):
    h=H0/100.0
    return 2.4728e-5*(1+0.22710731766*3.046)/(h*h)

def simpson(f,n=400000):
    if n%2:n+=1
    h=1.0/n
    s=f(0.0)+f(1.0)
    for i in range(1,n):
        s+=(4 if i%2 else 2)*f(i*h)
    return s*h/3.0

def planck_age():
    H0=67.36; om=0.3153; orad=omega_r(H0); ol=1-orad-om
    def f(x):
        if x==0:return 0.0
        a=x*x
        E=math.sqrt(orad/a**4+om/a**3+ol)
        return 2.0/(x*E)
    I=simpson(f)
    h0si=H0*1000/MPC_M
    return I/h0si/GYR_S

def mars_j2000():
    a=1.52371034;e=0.09339410;I=1.84969142;L=-4.55343205;peri=-23.94362959;node=49.55953891
    w=peri-node;M=((L-peri+180)%360)-180;d2r=math.pi/180
    estar=180/math.pi*e
    E=M+estar*math.sin(M*d2r)
    for _ in range(20):
        dm=M-(E-estar*math.sin(E*d2r));de=dm/(1-e*math.cos(E*d2r));E+=de
        if abs(de)<=1e-10:break
    xp=a*(math.cos(E*d2r)-e);yp=a*math.sqrt(1-e*e)*math.sin(E*d2r)
    w*=d2r;node*=d2r;I*=d2r
    cw,sw=math.cos(w),math.sin(w);co,so=math.cos(node),math.sin(node);ci,si=math.cos(I),math.sin(I)
    x=(cw*co-sw*so*ci)*xp+(-sw*co-cw*so*ci)*yp
    y=(cw*so+sw*co*ci)*xp+(-sw*so+cw*co*ci)*yp
    z=(sw*si)*xp+(cw*si)*yp
    return {"x":x,"y":y,"z":z}

_,tcg2=tt_to_tcg(2453750.5,0.892482639)
out={
  "implementation":"python-independent",
  "ttToTcgD2":tcg2,
  "planckAgeGyr":planck_age(),
  "sarosJdTt":2460409.263+6585.3223,
  "mars":mars_j2000()
}
os.makedirs("artifacts",exist_ok=True)
with open("artifacts/v1-python.json","w") as f:json.dump(out,f,indent=2)
print(json.dumps(out,indent=2))
