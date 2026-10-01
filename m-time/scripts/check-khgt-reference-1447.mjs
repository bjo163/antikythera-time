import fs from 'node:fs/promises';
const r=JSON.parse(await fs.readFile(process.argv[2]??'/tmp/khgt-reference.json','utf8'));
const jd=(iso)=>Date.parse(iso)/86400000+2440587.5;
const expected={
  conjunction_jd_utc:jd('2026-03-19T01:23:28Z'),
  makkah_sunset_jd_utc:jd('2026-03-19T15:34:04Z'),
  makkah_geocentric_altitude_deg:6+9/60+9/3600,
  makkah_elongation_deg:8+5/60+24/3600,
};
const err={
  conjunction_seconds:Math.abs(r.conjunction_jd_utc-expected.conjunction_jd_utc)*86400,
  sunset_seconds:Math.abs(r.makkah_sunset_jd_utc-expected.makkah_sunset_jd_utc)*86400,
  altitude_deg:Math.abs(r.makkah_geocentric_altitude_deg-expected.makkah_geocentric_altitude_deg),
  elongation_deg:Math.abs(r.makkah_elongation_deg-expected.makkah_elongation_deg),
};
const gates={conjunction_seconds:60,sunset_seconds:120,altitude_deg:0.1,elongation_deg:0.1};
const pass=Object.keys(gates).every(k=>err[k]<=gates[k]);
const report={source:'Muhammadiyah KHGT 1447 H published calculation',computed:r,expected,errors:err,gates,pass};
console.log(JSON.stringify(report,null,2));
await fs.mkdir('artifacts',{recursive:true});await fs.writeFile('artifacts/khgt-reference-1447.json',JSON.stringify(report,null,2));
if(!pass)process.exit(1);
