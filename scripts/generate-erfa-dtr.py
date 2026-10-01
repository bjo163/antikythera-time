#!/usr/bin/env python3
import json, math
from datetime import datetime, timezone
import erfa

def jd_from_datetime(dt):
    unix = dt.timestamp()
    return unix / 86400.0 + 2440587.5

samples=[]
for year in range(1900,2101,5):
    for month in (1,4,7,10):
        dt=datetime(year,month,1,tzinfo=timezone.utc)
        jd=jd_from_datetime(dt)
        jd1=math.floor(jd-0.5)+0.5
        jd2=jd-jd1
        # Geocentric reference: observer longitude and distances set to zero.
        # UT fraction is irrelevant when topocentric terms are zero.
        dtr=float(erfa.dtdb(jd1,jd2,0.0,0.0,0.0,0.0))
        samples.append({"iso":dt.isoformat(),"jd1":jd1,"jd2":jd2,"erfaDtrSeconds":dtr})

payload={
  "source":"ERFA/PyERFA dtdb (SOFA-derived reference), geocentric u=v=0",
  "window":["1900-01-01","2100-10-01"],
  "samples":samples
}
import os
os.makedirs("artifacts",exist_ok=True)
with open("artifacts/erfa-dtr-reference.json","w") as f:
    json.dump(payload,f,indent=2)
print(json.dumps({"samples":len(samples),"first":samples[0],"last":samples[-1]},indent=2))
